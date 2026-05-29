use sqlx::PgPool;
use crate::models::prestamos::{Prestamo, CrearPrestamo};
use crate::repository::prestamos_repository;

pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Prestamo>, sqlx::Error> {
    prestamos_repository::obtener_todos(pool).await
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Prestamo>, sqlx::Error> {
    prestamos_repository::obtener_por_id(pool, id).await
}

pub async fn crear(pool: &PgPool, nuevo: CrearPrestamo) -> Result<Prestamo, sqlx::Error> {
    prestamos_repository::crear(pool, nuevo).await
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearPrestamo) -> Result<Option<Prestamo>, sqlx::Error> {
    prestamos_repository::actualizar(pool, id, datos).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    prestamos_repository::eliminar(pool, id).await
}