use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::analytics::obtener_resumen_general;


/// Define el sub-enrutador para los endpoints del recurso de analítica.
/// 
/// Asocia el path `/summary` con la función handler `obtener_resumen_general`.
pub fn analytics_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/summary", get(obtener_resumen_general))
}