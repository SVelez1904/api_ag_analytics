use serde::Serialize;

/*
    #[derive(Serialize)]: 
    Le dice a Rust que genere automáticamente el código para convertir esa estructura a un String JSON (vía serde).

    #[derive(sqlx::FromRow)]: 
    Le indica a sqlx que mapee automáticamente las columnas que devuelva una consulta SQL a las propiedades de esa estructura.
*/
#[derive(Serialize, sqlx::FromRow)] //
pub struct KPIGeneral {
    /// Número total de registros de reservas calculados con COUNT(*)
    pub total_reservas: i64,
    
    /// Suma total de los montos de reservas (`total_reserva`).
    /// Es `Option<f64>` porque si la tabla está vacía o el valor es NULL en la BD,
    /// devolverá `None` en lugar de romper con un NullPointer.
    pub ingresos_totales: Option<f64>,
}