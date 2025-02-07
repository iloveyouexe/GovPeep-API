use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors) // CORS was missing
            .app_data(web::Data::new(pool.clone()))
            .route("/api/signup", web::post().to(handlers::auth::signup))
            .route("/api/agencies", web::get().to(handlers::agencies::get_agencies))
            .route("/api/agencies/{id}", web::get().to(handlers::agencies::get_agency))
            .route("/api/agencies", web::post().to(handlers::agencies::create_agency))
            .route("/api/agencies/{id}", web::put().to(handlers::agencies::update_agency))
            .route("/api/agencies/{id}", web::delete().to(handlers::agencies::delete_agency))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
