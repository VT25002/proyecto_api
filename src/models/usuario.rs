use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id_usuario: i32,
    pub nombre: String,
    pub direccion: Option<String>,
    pub fecha_registro: Option<time::Date>,
}

#[derive(Debug, Deserialize)]
pub struct CrearUsuario {
    pub nombre: String,
    pub direccion: Option<String>,
}
