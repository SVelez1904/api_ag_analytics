use sqlx::PgPool;


/// Inicializa el pool de conexiones asíncronas hacia PostgreSQL.
/// 
/// # Parámetros
/// * `database_url` - String de conexión en formato `postgres://user:pass@host:port/dbname`
///
/// # Panics
/// La función hará `panic!` y detendrá la aplicación si las credenciales son incorrectas
/// o el servidor de la base de datos no es accesible.
pub async fn init_pool(database_url: &str) -> PgPool {
    PgPool::connect(database_url)
        .await
        .expect("Error al conectar con la base de datos PostgreSQL")
}