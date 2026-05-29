use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Prestamo {
    pub id_prestamo: i32,
    pub id_libro: i32,
    pub id_usuario: i32,
    pub fecha_salida: chrono::NaiveDate,
    pub fecha_devolucion: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrearPrestamo {
    pub id_libro: i32,
    pub id_usuario: i32,
    pub fecha_salida: chrono::NaiveDate,
    pub fecha_devolucion: Option<chrono::NaiveDate>,
}