use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use crate::models::autores::{Autor, CrearAutor};
use crate::service::autores_service;

// Configuración de las rutas (endpoints) de la API para Autores
pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/autores", get(obtener_todos).post(crear_autor))
        .route("/autores/:id", get(obtener_por_id).put(actualizar_autor).delete(eliminar_autor))
        .with_state(pool)
}

// GET /autores -> Retorna la lista de todos los autores
pub async fn obtener_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Autor>>, (StatusCode, String)> {
    match autores_service::listar_autores(&pool).await {
        Ok(autores) => Ok(Json(autores)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// GET /autores/:id -> Busca un autor por su ID
pub async fn obtener_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Autor>, (StatusCode, String)> {
    match autores_service::buscar_autor_por_id(&pool, id).await {
        Ok(Some(autor)) => Ok(Json(autor)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Autor no encontrado".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// POST /autores -> Crea un nuevo autor
pub async fn crear_autor(
    State(pool): State<PgPool>,
    Json(nuevo): Json<CrearAutor>,
) -> Result<(StatusCode, Json<Autor>), (StatusCode, String)> {
    match autores_service::registrar_autor(&pool, nuevo).await {
        Ok(autor) => Ok((StatusCode::CREATED, Json(autor))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// PUT /autores/:id -> Modifica un autor existente
pub async fn actualizar_autor(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<CrearAutor>,
) -> Result<Json<Autor>, (StatusCode, String)> {
    match autores_service::modificar_autor(&pool, id, datos).await {
        Ok(Some(autor)) => Ok(Json(autor)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Autor no encontrado".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// DELETE /autores/:id -> Elimina un autor por su ID
pub async fn eliminar_autor(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match autores_service::remover_autor(&pool, id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((StatusCode::NOT_FOUND, "Autor no encontrado".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}