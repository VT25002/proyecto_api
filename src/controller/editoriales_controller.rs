use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use sqlx::PgPool;
use crate::models::editoriales::{Editorial, CrearEditorial};
use crate::service::editoriales_service;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/editoriales", get(obtener_todos).post(crear_editorial))
        .route("/editoriales/{id}", get(obtener_por_id).put(actualizar_editorial).delete(eliminar_editorial))
        .with_state(pool)
}

pub async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Editorial>>, (StatusCode, String)> {
    match editoriales_service::listar_editoriales(&pool).await {
        Ok(editoriales) => Ok(Json(editoriales)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Editorial>, (StatusCode, String)> {
    match editoriales_service::buscar_editorial_por_id(&pool, id).await {
        Ok(Some(editorial)) => Ok(Json(editorial)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Editorial no encontrada".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn crear_editorial(
    State(pool): State<PgPool>,
    Json(nuevo): Json<CrearEditorial>,
) -> Result<(StatusCode, Json<Editorial>), (StatusCode, String)> {
    match editoriales_service::registrar_editorial(&pool, nuevo).await {
        Ok(editorial) => Ok((StatusCode::CREATED, Json(editorial))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn actualizar_editorial(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<CrearEditorial>,
) -> Result<Json<Editorial>, (StatusCode, String)> {
    match editoriales_service::modificar_editorial(&pool, id, datos).await {
        Ok(Some(editorial)) => Ok(Json(editorial)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Editorial no encontrada".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn eliminar_editorial(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match editoriales_service::remover_editorial(&pool, id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((StatusCode::NOT_FOUND, "Editorial no encontrada".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}