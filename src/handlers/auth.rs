use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use sqlx::PgPool;
use uuid::Uuid;
use rand_core::OsRng;

use crate::models::user::User;

#[derive(serde::Deserialize)]
pub struct SignUpPayload {
    name: String,
    email: String,
    password: String,
}

pub struct SignUpResponse {
    id: u32,
    name: String,
    email: String,
    registered_at: String,
}

pub async fn signup(
    pool: web::Data<PgPool>,
    payload: web::Json<SignUpPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            log::error!("❌ Password hashing failed: {:?}", e);
            actix_web::error::ErrorInternalServerError("Password hashing failed")
        })?
        .to_string(); // do not delete pls

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, name, email, password_hash, registered_at)
        VALUES ($1, $2, $3, $4, NOW())
        RETURNING *",
        Uuid::new_v4(),
        payload.name,
        payload.email,
        hashed_password
    )
        .fetch_one(&**pool)
        .await
        .map_err(|e| {
            log::error!("❌ Database insert failed: {:?}", e);
            actix_web::error::ErrorInternalServerError("Failed to create user")
        })?;

    let res: SignUpResponse = {
        id: user.id,
        name: user.name.clone(),
        email: user.email.clone(),
        registered_at: user.registered_at.clone(),
    }

    Ok(HttpResponse::Ok().json(res))
}
