use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::models::agency::Agency;

#[derive(Deserialize, Serialize)]
pub struct AgencyPayload {
    name: String,
    description: String,
    website: String,
    phone_number: String,
    logo: String,
    governance: String,
}

// Fetch all
pub async fn get_agencies(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let agencies = query_as!(
    Agency,
    "SELECT id, name, description, website, phone_number, logo, governance, created_at, updated_at FROM agencies"
)
        .fetch_all(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(agencies))
}

// Fetch single by ID
pub async fn get_agency(
    pool: web::Data<PgPool>,
    agency_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let agency = query_as!(
        Agency,
        r#"
        SELECT * FROM agencies WHERE id = $1
        "#,
        agency_id.into_inner()
    )
        .fetch_optional(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match agency {
        Some(agency) => Ok(HttpResponse::Ok().json(agency)),
        None => Ok(HttpResponse::NotFound().body("Agency not found")),
    }
}

// Create
pub async fn create_agency(
    pool: web::Data<PgPool>,
    payload: web::Json<AgencyPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let agency = sqlx::query_as!(
    Agency,
    "INSERT INTO agencies (id, name, description, website, phone_number, logo, governance, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
    RETURNING id, name, description, website, phone_number, logo, governance, created_at, updated_at",
    Uuid::new_v4(),
    payload.name,
    payload.description,
    payload.website,
    payload.phone_number,
    payload.logo,
    payload.governance
)

        .fetch_one(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(agency))
}

// Update
pub async fn update_agency(
    pool: web::Data<PgPool>,
    agency_id: web::Path<Uuid>,
    payload: web::Json<AgencyPayload>,
) -> Result<HttpResponse, actix_web::Error> {
    let agency = query_as!(
        Agency,
        r#"
        UPDATE agencies
        SET name = $1, description = $2, website = $3, phone_number = $4, logo = $5, governance = $6, updated_at = NOW()
        WHERE id = $7
        RETURNING *
        "#,
        payload.name,
        payload.description,
        payload.website,
        payload.phone_number,
        payload.logo,
        payload.governance,
        agency_id.into_inner()
    )
        .fetch_optional(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match agency {
        Some(agency) => Ok(HttpResponse::Ok().json(agency)),
        None => Ok(HttpResponse::NotFound().body("Agency not found")),
    }
}

// Delete
pub async fn delete_agency(
    pool: web::Data<PgPool>,
    agency_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM agencies WHERE id = $1
        "#,
        agency_id.into_inner()
    )
        .execute(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().body("Agency not found"))
    }
}
