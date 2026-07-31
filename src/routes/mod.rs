//! Módulo para la agregación y estructuración de todas las rutas de la API.
pub mod analytics;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;


/// Une todos los sub-módulos de rutas en un único Router principal.
///
/// Aplica el prefijo `/api/analytics` a las rutas de analítica
/// e inyecta el Pool de base de datos como estado compartido a toda la aplicación.
pub fn create_router(pool: Arc<PgPool>) -> Router {
    Router::new()
        // Prefijo para las rutas de analítica (/api/analytics/...)
        .nest("/api/analytics", analytics::analytics_routes())
        .with_state(pool)
}