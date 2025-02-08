use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors; // ✅ Import CORS
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // ✅ Allow frontend origin
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
            .allow_any_header()
            .allow_any_origin() // ✅ Allow all origins (you may restrict this in production)
            .supports_credentials()
            .max_age(3600); // ✅ Cache CORS options for 1 hour

        App::new()
            .wrap(cors) // ✅ Apply CORS middleware
            .app_data(web::Data::new(pool.clone()))
            .route("/api/agencies", web::get().to(handlers::agencies::get_agencies))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
