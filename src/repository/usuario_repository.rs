use crate::models::usuario::{CrearUsuario, Usuario};
use sqlx::PgPool;

pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Usuario>, sqlx::Error> {
    let usuarios = sqlx::query_as::<_, Usuario>(
        "SELECT id_usuario, nombre, direccion, fecha_registro FROM usuarios",
    )
    .fetch_all(pool)
    .await?;
    Ok(usuarios)
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Usuario>, sqlx::Error> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT id_usuario, nombre, direccion, fecha_registro FROM usuarios WHERE id_usuario = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(usuario)
}

pub async fn crear(pool: &PgPool, data: CrearUsuario) -> Result<Usuario, sqlx::Error> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "INSERT INTO usuarios (nombre, direccion) VALUES ($1, $2) RETURNING id_usuario, nombre, direccion, fecha_registro"
    )
    .bind(data.nombre)
    .bind(data.direccion)
    .fetch_one(pool)
    .await?;
    Ok(usuario)
}

pub async fn actualizar(
    pool: &PgPool,
    id: i32,
    data: CrearUsuario,
) -> Result<Option<Usuario>, sqlx::Error> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "UPDATE usuarios SET nombre=$1, direccion=$2 WHERE id_usuario=$3 RETURNING id_usuario, nombre, direccion, fecha_registro"
    )
    .bind(data.nombre)
    .bind(data.direccion)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(usuario)
}

pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query("DELETE FROM usuarios WHERE id_usuario = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(resultado.rows_affected() > 0)
}
