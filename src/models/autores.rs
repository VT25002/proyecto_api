use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Autor {
    pub id_autor: i32,
    pub nombre: String,
    pub nacionalidad: Option<String>, // Es Option porque en SQL puede ser NULL
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrearAutor {
    pub nombre: String,
    pub nacionalidad: Option<String>,
}