use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CrearUsuario {
    pub nombre: String,
    pub email: String,
}
