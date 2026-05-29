use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::models::prestamos::CrearPrestamo;
use crate::service::prestamos_service;

pub fn prestamos_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/prestamos", get(obtener_todos))
        .route("/prestamos/{id}", get(obtener_por_id))
        .route("/prestamos", post(crear))
        .route("/prestamos/{id}", put(actualizar))
        .route("/prestamos/{id}", delete(eliminar))
        .with_state(pool)
}

async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<crate::models::prestamos::Prestamo>>, StatusCode> {
    match prestamos_service::obtener_todos(&pool).await {
        Ok(prestamos) => Ok(Json(prestamos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<crate::models::prestamos::Prestamo>, StatusCode> {
    match prestamos_service::obtener_por_id(&pool, id).await {
        Ok(Some(prestamo)) => Ok(Json(prestamo)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn crear(
    State(pool): State<PgPool>,
    Json(nuevo): Json<CrearPrestamo>,
) -> Result<(StatusCode, Json<crate::models::prestamos::Prestamo>), StatusCode> {
    match prestamos_service::crear(&pool, nuevo).await {
        Ok(prestamo) => Ok((StatusCode::CREATED, Json(prestamo))),
        Err(e) => {
            println!("Error al crear prestamo: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn actualizar(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<CrearPrestamo>,
) -> Result<Json<crate::models::prestamos::Prestamo>, StatusCode> {
    match prestamos_service::actualizar(&pool, id, datos).await {
        Ok(Some(prestamo)) => Ok(Json(prestamo)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn eliminar(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> StatusCode {
    match prestamos_service::eliminar(&pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}