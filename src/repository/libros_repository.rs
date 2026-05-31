use sqlx::PgPool;
use crate::models::libros::{Libro, CrearLibro};

// Obtiene todos los libros registrados en un vector
pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Libro>, sqlx::Error> {
    let libros = sqlx::query_as::<_, Libro>(
        "SELECT id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial FROM Libros"
    )
    .fetch_all(pool)
    .await?;
    Ok(libros)
}

// Busca un libro por su ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Libro>, sqlx::Error> {
    let libro = sqlx::query_as::<_, Libro>(
        "SELECT id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial
         FROM Libros WHERE id_libro = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(libro)
}

// Inserta un nuevo libro y retorna el registro con su ID generado
pub async fn crear(pool: &PgPool, nuevo: CrearLibro) -> Result<Libro, sqlx::Error> {
    let libro = sqlx::query_as::<_, Libro>(
        "INSERT INTO Libros (titulo, isbn, anio_publicacion, id_autor, id_editorial)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial"
    )
    .bind(nuevo.titulo)
    .bind(nuevo.isbn)
    .bind(nuevo.anio_publicacion)
    .bind(nuevo.id_autor)
    .bind(nuevo.id_editorial)
    .fetch_one(pool)
    .await?;
    Ok(libro)
}

// Actualiza los datos de un libro existente según su ID
pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearLibro) -> Result<Option<Libro>, sqlx::Error> {
    let libro = sqlx::query_as::<_, Libro>(
        "UPDATE Libros
         SET titulo = $1, isbn = $2, anio_publicacion = $3, id_autor = $4, id_editorial = $5
         WHERE id_libro = $6
         RETURNING id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial"
    )
    .bind(datos.titulo)
    .bind(datos.isbn)
    .bind(datos.anio_publicacion)
    .bind(datos.id_autor)
    .bind(datos.id_editorial)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(libro)
}

// Elimina un libro por ID (Retorna true si se borró con éxito)
pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query(
        "DELETE FROM Libros WHERE id_libro = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(resultado.rows_affected() > 0)
}