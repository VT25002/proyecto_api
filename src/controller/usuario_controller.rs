use crate::models::usuario::CrearUsuario;
use crate::service::usuario_service;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;

pub fn rutas() -> Router<PgPool> {
    Router::new()
        .route("/usuarios", get(listar).post(crear))
        .route(
            "/usuarios/:id",
            get(obtener).put(actualizar).delete(eliminar),
        )
}

async fn listar(State(pool): State<PgPool>) -> Result<Json<serde_json::Value>, StatusCode> {
    match usuario_service::listar_usuarios(&pool).await {
        Ok(usuarios) => Ok(Json(serde_json::json!(usuarios))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn obtener(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match usuario_service::obtener_usuario(&pool, id).await {
        Ok(Some(u)) => Ok(Json(serde_json::json!(u))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn crear(
    State(pool): State<PgPool>,
    Json(data): Json<CrearUsuario>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match usuario_service::crear_usuario(&pool, data).await {
        Ok(u) => Ok(Json(serde_json::json!(u))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn actualizar(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(data): Json<CrearUsuario>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match usuario_service::actualizar_usuario(&pool, id, data).await {
        Ok(Some(u)) => Ok(Json(serde_json::json!(u))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn eliminar(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match usuario_service::eliminar_usuario(&pool, id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
