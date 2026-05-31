use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/** Tabla de Editoriales */
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Editorial {
    pub id_editorial: i32,
    pub nombre: String,
    pub pais: Option<String>, // Es Option porque en SQL puede ser NULL
}

// Para crear nuevas editoriales
#[derive(Debug, Serialize, Deserialize)]
pub struct CrearEditorial {
    pub nombre: String,
    pub pais: Option<String>,
}