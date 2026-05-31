use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Libro {
    pub id_libro: i32,
    pub titulo: String,
    pub isbn: Option<String>,
    pub anio_publicacion: Option<i32>,
    pub id_autor: Option<i32>,
    pub id_editorial: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrearLibro {
    pub titulo: String,
    pub isbn: Option<String>,
    pub anio_publicacion: Option<i32>,
    pub id_autor: Option<i32>,
    pub id_editorial: Option<i32>,
}