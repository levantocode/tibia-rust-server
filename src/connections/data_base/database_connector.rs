use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

use std::time::Duration;


pub async fn init_db() -> Result<MySqlPool, sqlx::Error> {
    new_db_pool().await
}

async fn new_db_pool() -> Result<MySqlPool, sqlx::Error> {
    let db_uri: String = dotenv::var("DB_URI").unwrap();

    let max_db_pool_connections: u32 = get_max_db_pool_connections();

    let db_connection_timeout_ms: u64 = get_db_connection_timeout();
    let timeout_ms: Duration = Duration::from_millis(db_connection_timeout_ms);

    MySqlPoolOptions::new()
        .max_connections(max_db_pool_connections)
        .connect_timeout(timeout_ms)
        .connect(&db_uri)
        .await
}

fn get_max_db_pool_connections() -> u32 {
    dotenv::var("MAX_DB_POOL_CONNECTIONS")
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn get_db_connection_timeout() -> u64 {
    dotenv::var("DB_CONNECTION_TIMEOUT_MS")
        .unwrap()
        .parse::<u64>()
        .unwrap()
}