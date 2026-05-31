use sqlx::PgPool;
use crate::models::libros::{Libro, CrearLibro};
use crate::repository::libros_repository;

pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Libro>, sqlx::Error> {
    libros_repository::obtener_todos(pool).await
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Libro>, sqlx::Error> {
    libros_repository::obtener_por_id(pool, id).await
}

pub async fn crear(pool: &PgPool, nuevo: CrearLibro) -> Result<Libro, sqlx::Error> {
    libros_repository::crear(pool, nuevo).await
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearLibro) -> Result<Option<Libro>, sqlx::Error> {
    libros_repository::actualizar(pool, id, datos).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    libros_repository::eliminar(pool, id).await
}