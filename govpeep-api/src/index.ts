/**
 * Welcome to Cloudflare Workers! This is your first worker.
 *
 * - Run `npm run dev` in your terminal to start a development server
 * - Open a browser tab at http://localhost:8787/ to see your worker in action
 * - Run `npm run deploy` to publish your worker
 *
 * Bind resources to your worker in `wrangler.jsonc`. After adding bindings, a type definition for the
 * `Env` object can be regenerated with `npm run cf-typegen`.
 *
 * Learn more at https://developers.cloudflare.com/workers/
 */

interface Env {
	govpeep_db: D1Database;
}

function withCors(headers?: HeadersInit): Headers {
	const h = new Headers(headers);
	h.set('access-control-allow-origin', '*');
	h.set('access-control-allow-methods', 'GET,OPTIONS');
	h.set('access-control-allow-headers', 'content-type');
	h.set('access-control-max-age', '86400');
	h.set('content-type', 'application/json; charset=utf-8');
	return h;
}

function json(data: unknown, init?: ResponseInit): Response {
	return new Response(JSON.stringify(data), {
		...init,
		headers: withCors(init?.headers),
	});
}

function escapeLike(input: string): string {
	return input.replace(/[\\%_]/g, (m) => `\\${m}`);
}

export default {
	async fetch(request, env, ctx): Promise<Response> {
		if (request.method === 'OPTIONS') {
			return new Response(null, { status: 204, headers: withCors() });
		}

		if (request.method !== 'GET') {
			return json({ error: 'Method Not Allowed' }, { status: 405 });
		}

		const url = new URL(request.url);
		if (url.pathname !== '/api/agencies') {
			return json({ error: 'Not Found' }, { status: 404 });
		}

		const q = (url.searchParams.get('q') ?? '').trim();

		try {
			if (!q) {
				const { results } = await env.govpeep_db
					.prepare(
						"SELECT id, name, description, website, phone_number, logo, governance, created_at, updated_at\n" +
						"FROM agencies\n" +
						"WHERE logo IS NOT NULL AND TRIM(logo) <> ''\n" +
						"ORDER BY name",
					)
					.all();

				return json(results);
			}

			const pattern = `%${escapeLike(q.toLowerCase())}%`;
			const { results } = await env.govpeep_db
				.prepare(
					"SELECT id, name, description, website, phone_number, logo, governance, created_at, updated_at\n" +
					"FROM agencies\n" +
					"WHERE (LOWER(name) LIKE ? ESCAPE '\\\\'\n" +
					"   OR LOWER(description) LIKE ? ESCAPE '\\\\'\n" +
					"   OR LOWER(governance) LIKE ? ESCAPE '\\\\')\n" +
					"ORDER BY\n" +
					"  CASE WHEN logo IS NULL OR TRIM(logo) = '' THEN 1 ELSE 0 END,\n" +
					"  name",
				)
				.bind(pattern, pattern, pattern)
				.all();

			return json(results);
		} catch (e) {
			return json({ error: 'Internal Server Error' }, { status: 500 });
		}
	},
} satisfies ExportedHandler<Env>;
