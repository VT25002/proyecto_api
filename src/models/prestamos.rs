use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Prestamo {
    pub id_prestamo: i32,
    pub id_libro: Option<i32>,
    pub id_usuario: Option<i32>,
    #[serde(with = "date_format::option")]
    pub fecha_salida: Option<Date>,
    #[serde(with = "date_format::option")]
    pub fecha_devolucion: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrearPrestamo {
    pub id_libro: Option<i32>,
    pub id_usuario: Option<i32>,
    #[serde(with = "date_format")]
    pub fecha_salida: Date,
    #[serde(with = "date_format::option")]
    pub fecha_devolucion: Option<Date>,
}