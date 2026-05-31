use sqlx::PgPool;
use crate::models::editoriales::{Editorial, CrearEditorial};
use crate::repository::editoriales_repository;

pub async fn listar_editoriales(pool: &PgPool) -> Result<Vec<Editorial>, sqlx::Error> {
    editoriales_repository::obtener_todos(pool).await
}

pub async fn buscar_editorial_por_id(pool: &PgPool, id: i32) -> Result<Option<Editorial>, sqlx::Error> {
    editoriales_repository::obtener_por_id(pool, id).await
}

pub async fn registrar_editorial(pool: &PgPool, nuevo: CrearEditorial) -> Result<Editorial, sqlx::Error> {
    editoriales_repository::crear(pool, nuevo).await
}

pub async fn modificar_editorial(pool: &PgPool, id: i32, datos: CrearEditorial) -> Result<Option<Editorial>, sqlx::Error> {
    editoriales_repository::actualizar(pool, id, datos).await
}

pub async fn remover_editorial(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    editoriales_repository::eliminar(pool, id).await
}