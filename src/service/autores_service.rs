use sqlx::PgPool;
use crate::models::autores::{Autor, CrearAutor};
use crate::repository::autores_repository;

pub async fn listar_autores(pool: &PgPool) -> Result<Vec<Autor>, sqlx::Error> {
    autores_repository::obtener_todos(pool).await
}

pub async fn buscar_autor_por_id(pool: &PgPool, id: i32) -> Result<Option<Autor>, sqlx::Error> {
    autores_repository::obtener_por_id(pool, id).await
}

pub async fn registrar_autor(pool: &PgPool, nuevo: CrearAutor) -> Result<Autor, sqlx::Error> {
    // Aquí se podrían añadir validaciones más adelante si la rúbrica lo pide
    autores_repository::crear(pool, nuevo).await
}

pub async fn modificar_autor(pool: &PgPool, id: i32, datos: CrearAutor) -> Result<Option<Autor>, sqlx::Error> {
    autores_repository::actualizar(pool, id, datos).await
}

pub async fn remover_autor(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    autores_repository::eliminar(pool, id).await
}