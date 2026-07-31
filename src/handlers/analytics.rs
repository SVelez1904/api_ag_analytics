use axum::{extract::State, Json};
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::analytics::KPIGeneral;


/// Handler encargado de procesar la consulta analítica del resumen general.
///
/// Recibe la referencia al pool de la base de datos mediante el `State` de Axum,
/// ejecuta la agregación en la tabla `hechos_reservas` y retorna el DTO serializado en JSON.
pub async fn obtener_resumen_general(
    State(pool): State<Arc<PgPool>>,
) -> Json<KPIGeneral> {
    let query = "
        SELECT 
            COUNT(*)::BIGINT as total_reservas,
            SUM(total_reserva)::FLOAT as ingresos_totales
        FROM hechos_reservas;
    ";

    // Ejecuta la query y asocia las columnas retornadas con la struct KPIGeneral
    let resultado = sqlx::query_as::<_, KPIGeneral>(query)
        .fetch_one(&*pool)
        .await
        .unwrap_or(KPIGeneral {
            total_reservas: 0,
            ingresos_totales: Some(0.0),
        });

    Json(resultado)
}