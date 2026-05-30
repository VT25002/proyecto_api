use sqlx::PgPool;
use crate::models::autores::{Autor, CrearAutor};

// Obtiene todos los autores registrados en un Vector
pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Autor>, sqlx::Error> {
    let autores = sqlx::query_as::<_, Autor>(
        "SELECT id_autor, nombre, nacionalidad FROM Autores"
    )
    .fetch_all(pool)
    .await?;
    Ok(autores)
}

// Busca un autor por su ID (Retorna un Option por si no existe)
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Autor>, sqlx::Error> {
    let autor = sqlx::query_as::<_, Autor>(
        "SELECT id_autor, nombre, nacionalidad FROM Autores WHERE id_autor = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(autor)
}

// Inserta un nuevo autor y retorna el registro con su ID generado
pub async fn crear(pool: &PgPool, nuevo: CrearAutor) -> Result<Autor, sqlx::Error> {
    let autor = sqlx::query_as::<_, Autor>(
        "INSERT INTO Autores (nombre, nacionalidad) 
         VALUES ($1, $2) 
         RETURNING id_autor, nombre, nacionalidad"
    )
    .bind(nuevo.nombre)
    .bind(nuevo.nacionalidad)
    .fetch_one(pool)
    .await?;
    Ok(autor)
}

// Actualiza los datos de un autor existente según su ID
pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearAutor) -> Result<Option<Autor>, sqlx::Error> {
    let autor = sqlx::query_as::<_, Autor>(
        "UPDATE Autores SET nombre = $1, nacionalidad = $2 
         WHERE id_autor = $3 
         RETURNING id_autor, nombre, nacionalidad"
    )
    .bind(datos.nombre)
    .bind(datos.nacionalidad)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(autor)
}

// Elimina un autor por ID (Retorna true si se borró con éxito)
pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query(
        "DELETE FROM Autores WHERE id_autor = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(resultado.rows_affected() > 0)
}