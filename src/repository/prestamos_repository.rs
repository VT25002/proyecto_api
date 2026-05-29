use sqlx::PgPool;
use crate::models::prestamos::{Prestamo, CrearPrestamo};

pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Prestamo>, sqlx::Error> {
    let prestamos = sqlx::query_as::<_, Prestamo>(
        "SELECT id_prestamo, id_libro, id_usuario, fecha_salida, fecha_devolucion FROM Prestamos"
    )
    .fetch_all(pool)
    .await?;
    Ok(prestamos)
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Prestamo>, sqlx::Error> {
    let prestamo = sqlx::query_as::<_, Prestamo>(
        "SELECT id_prestamo, id_libro, id_usuario, fecha_salida, fecha_devolucion FROM Prestamos WHERE id_prestamo = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(prestamo)
}

pub async fn crear(pool: &PgPool, nuevo: CrearPrestamo) -> Result<Prestamo, sqlx::Error> {
    let prestamo = sqlx::query_as::<_, Prestamo>(
        "INSERT INTO Prestamos (id_libro, id_usuario, fecha_salida, fecha_devolucion)
         VALUES ($1, $2, $3, $4)
         RETURNING id_prestamo, id_libro, id_usuario, fecha_salida, fecha_devolucion"
    )
    .bind(nuevo.id_libro)
    .bind(nuevo.id_usuario)
    .bind(nuevo.fecha_salida)
    .bind(nuevo.fecha_devolucion)
    .fetch_one(pool)
    .await?;
    Ok(prestamo)
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearPrestamo) -> Result<Option<Prestamo>, sqlx::Error> {
    let prestamo = sqlx::query_as::<_, Prestamo>(
        "UPDATE Prestamos SET id_libro = $1, id_usuario = $2, fecha_salida = $3, fecha_devolucion = $4
         WHERE id_prestamo = $5
         RETURNING id_prestamo, id_libro, id_usuario, fecha_salida, fecha_devolucion"
    )
    .bind(datos.id_libro)
    .bind(datos.id_usuario)
    .bind(datos.fecha_salida)
    .bind(datos.fecha_devolucion)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(prestamo)
}

pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query(
        "DELETE FROM Prestamos WHERE id_prestamo = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(resultado.rows_affected() > 0)
}