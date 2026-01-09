use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::env_vars::AppConfig;

pub async fn create_database() -> eyre::Result<DatabaseConnection> {
    let database_url = &AppConfig::global().database_url;

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8 * 60))
        .max_lifetime(Duration::from_secs(30 * 60))
        .sqlx_logging(true)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;
    Ok(db)
}
