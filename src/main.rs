use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;

// 1. DTO de respuesta con los genéricos explícitos en Option
#[derive(Serialize, sqlx::FromRow)]
pub struct KPIGeneral {
    pub total_reservas: i64,
    pub ingresos_totales: Option<f64>,
}

#[tokio::main]
async fn main() {
    // 2. Ajusta esta URL con tus credenciales locales de PostgreSQL
    let database_url = "postgresql://stays:stays123@localhost:5432/stays";


    let pool = PgPool::connect(&database_url)
        .await
        .expect("Error al conectar con la base de datos");

    // 3. Enrutador
    let app = Router::new()
        .route("/api/analytics/summary", get(obtener_resumen_general))
        .with_state(Arc::new(pool));

    // 4. Iniciar Servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
        
    println!("🚀 API corriendo en http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// 5. Handler con State<Arc<PgPool>> y Json<KPIGeneral> bien tipados
async fn obtener_resumen_general(
    State(pool): State<Arc<PgPool>>,
) -> Json<KPIGeneral> {
    let query = "
        SELECT 
            COUNT(*)::BIGINT as total_reservas,
            SUM(total_reserva)::FLOAT as ingresos_totales
        FROM hechos_reservas;
    ";

    let resultado = sqlx::query_as::<_, KPIGeneral>(query)
        .fetch_one(&*pool)
        .await
        .unwrap_or(KPIGeneral {
            total_reservas: 0,
            ingresos_totales: Some(0.0),
        });

    Json(resultado)
}