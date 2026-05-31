use crate::models::usuario::{CrearUsuario, Usuario};
use crate::repository::usuario_repository;
use sqlx::PgPool;

pub async fn listar_usuarios(pool: &PgPool) -> Result<Vec<Usuario>, sqlx::Error> {
    usuario_repository::obtener_todos(pool).await
}

pub async fn obtener_usuario(pool: &PgPool, id: i32) -> Result<Option<Usuario>, sqlx::Error> {
    usuario_repository::obtener_por_id(pool, id).await
}

pub async fn crear_usuario(pool: &PgPool, data: CrearUsuario) -> Result<Usuario, sqlx::Error> {
    usuario_repository::crear(pool, data).await
}

pub async fn actualizar_usuario(
    pool: &PgPool,
    id: i32,
    data: CrearUsuario,
) -> Result<Option<Usuario>, sqlx::Error> {
    usuario_repository::actualizar(pool, id, data).await
}

pub async fn eliminar_usuario(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    usuario_repository::eliminar(pool, id).await
}
