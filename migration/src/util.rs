use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect_db(url: &str, max_connections: u32) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(url);

    // Configure the connection pool
    opt.max_connections(max_connections) // Maximum number of connections
        .min_connections(5) // Minimum connections to maintain
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true) // Enable SQLx logging
        .sqlx_logging_level(log::LevelFilter::Info);

    Ok(Database::connect(opt).await?)
}
