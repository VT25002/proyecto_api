use dotenvy::dotenv;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use std::env;
use std::str::FromStr;

pub fn obtener_url_base_datos() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL no está configurada en el archivo .env")
}

pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let url_base_datos = obtener_url_base_datos();

    let opciones = PgConnectOptions::from_str(&url_base_datos)
        .expect("URL de base de datos inválida")
        .statement_cache_capacity(0);

    PgPoolOptions::new()
        .max_connections(1)
        .connect_with(opciones)
        .await
}