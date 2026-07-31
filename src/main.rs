// File: src/main.rs

//! API REST Asíncrona de Analítica sobre PostgreSQL usando Axum y SQLx.

mod config;
mod handlers;
mod models;
mod routes;

use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 1. Cargar las variables de entorno desde el archivo .env
    dotenvy::dotenv().ok();

    // 2. Obtener la cadena de conexión desde las variables de entorno
    let database_url = env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL debe estar configurada en el .env");

    // 3. Inicializar el Pool de PostgreSQL a través de la capa de configuración
    let pool = config::db::init_pool(&database_url).await;

    // 4. Crear el árbol de rutas e inyectarle el pool mediante un Arc (Atomic Reference Counter)
    let app = routes::create_router(Arc::new(pool));

    // 5. Configurar el puerto y la dirección del servidor web
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("🚀 API de Analítica corriendo en http://{}", addr);

    // 6. Arrancar el servidor web HTTP de Axum
    axum::serve(listener, app).await.unwrap();
}