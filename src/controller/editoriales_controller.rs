use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Router,
};
use sqlx::PgPool;
use crate::models::editoriales::{Editorial, CrearEditorial};
use crate::service::editoriales_service;

// Configuración de las rutas (endpoints) de la API para Editoriales
pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/editoriales", axum::routing::get(obtener_todos).post(crear_editorial))
        .route("/editoriales/:id", axum::routing::get(obtener_por_id).put(actualizar_editorial).delete(eliminar_editorial))
        .with_state(pool)
}

// GET /editoriales -> Retorna la lista de todas las editoriales
pub async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Editorial>>, (StatusCode, String)> {
    match editoriales_service::listar_editoriales(&pool).await {
        Ok(editoriales) => Ok(Json(editoriales)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// GET /editoriales/:id -> Busca una editorial por su ID
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

// POST /editoriales -> Crea una nueva editorial
pub async fn crear_editorial(
    State(pool): State<PgPool>,
    Json(nuevo): Json<CrearEditorial>,
) -> Result<(StatusCode, Json<Editorial>), (StatusCode, String)> {
    match editoriales_service::registrar_editorial(&pool, nuevo).await {
        Ok(editorial) => Ok((StatusCode::CREATED, Json(editorial))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// PUT /editoriales/:id -> Modifica una editorial existente
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

// DELETE /editoriales/:id -> Elimina una editorial por su ID
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