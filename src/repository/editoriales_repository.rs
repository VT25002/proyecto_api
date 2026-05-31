use sqlx::PgPool;
use crate::models::editoriales::{Editorial, CrearEditorial};

// Obtiene todas las editoriales registradas en un Vector
pub async fn obtener_editoriales(pool: &PgPool) -> Result<Vec<Editorial>, sqlx::Error> {
    let editoriales = sqlx::query_as::<_, Editorial>(
        "SELECT id_editorial, nombre, pais FROM Editoriales"
    )
    .fetch_all(pool)
    .await?;
    Ok(editoriales)
}

// Busca una editorial por su ID (Retorna un Option por si no existe)
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Editorial>, sqlx::Error> {
    let editorial = sqlx::query_as::<_, Editorial>(
        "SELECT id_editorial, nombre, pais FROM Editoriales WHERE id_editorial = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(editorial)
}

// Inserta una nueva editorial y retorna el registro con su ID generado
pub async fn crearEditorial(pool: &PgPool, nuevo: CrearEditorial) -> Result<Editorial, sqlx::Error> {
    let editorial = sqlx::query_as::<_, Editorial>(
        "INSERT INTO Editoriales (nombre, pais)
         VALUES ($1, $2)
         RETURNING id_editorial, nombre, pais"
    )
    .bind(nuevo.nombre)
    .bind(nuevo.pais)
    .fetch_one(pool)
    .await?;
    Ok(editorial)
}

// Actualiza los datos de una editorial existente según su ID
pub async fn actualizarEditorial(pool: &PgPool, id: i32, datos: CrearEditorial) -> Result<Option<Editorial>, sqlx::Error> {
    let editorial = sqlx::query_as::<_, Editorial>(
        "UPDATE Editoriales SET nombre = $1, pais = $2
         WHERE id_editorial = $3
         RETURNING id_editorial, nombre, pais"
    )
    .bind(datos.nombre)
    .bind(datos.pais)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(editorial)
}

// Elimina una editorial por ID (Retorna true si se borró con éxito)
pub async fn eliminarEditorial(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query(
        "DELETE FROM Editoriales WHERE id_editorial = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(resultado.rows_affected() > 0)
}