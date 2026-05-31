use crate::models::usuario::{CrearUsuario, Usuario};
use sqlx::PgPool;

pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Usuario>, sqlx::Error> {
    sqlx::query_as!(Usuario, "SELECT id, nombre, email FROM usuarios")
        .fetch_all(pool)
        .await
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Usuario>, sqlx::Error> {
    sqlx::query_as!(
        Usuario,
        "SELECT id, nombre, email FROM usuarios WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn crear(pool: &PgPool, data: CrearUsuario) -> Result<Usuario, sqlx::Error> {
    sqlx::query_as!(
        Usuario,
        "INSERT INTO usuarios (nombre, email) VALUES ($1, $2) RETURNING id, nombre, email",
        data.nombre,
        data.email
    )
    .fetch_one(pool)
    .await
}

pub async fn actualizar(
    pool: &PgPool,
    id: i32,
    data: CrearUsuario,
) -> Result<Option<Usuario>, sqlx::Error> {
    sqlx::query_as!(
        Usuario,
        "UPDATE usuarios SET nombre=$1, email=$2 WHERE id=$3 RETURNING id, nombre, email",
        data.nombre,
        data.email,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM usuarios WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
