use anyhow::Result;
use async_trait::async_trait;
use log::error;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, sync::Arc, time::Duration};

#[async_trait]
pub trait DBClient {
    fn get_connection(&self) -> Arc<DatabaseConnection>;
}

pub struct DBClientImpl {
    con: Arc<DatabaseConnection>,
}

impl DBClientImpl {
    pub async fn new() -> Result<Box<Self>> {
        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let db_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let db_pass = env::var("DATABASE_PASSWORD").expect("DATABASE_PASS must be set");
        let db_port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
        let db_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
        let db_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            db_user, db_pass, db_host, db_port, db_name
        );

        let max_connections: u32 = env::var("DB_MAX_ACTIVE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("DB_MAX_ACTIVE must be an unsigned integer");
        let min_connections: u32 = env::var("DB_MAX_IDLE")
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .expect("DB_MAX_IDLE must be an unsigned integer");
        let idle_timeout_secs: u64 = env::var("DB_IDLE_TIMEOUT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("DB_IDLE_TIMEOUT must be an unsigned integer");
        let is_logging_enabled: bool = env::var("DB_LOGGING")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("DB_LOGGING must be a boolean");

        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(max_connections)
            .min_connections(min_connections)
            .idle_timeout(Duration::from_secs(idle_timeout_secs))
            .sqlx_logging(is_logging_enabled);

        let con = match Database::connect(opt).await {
            Ok(con) => con,
            Err(e) => {
                error!("Failed to connect to database: {}", e.to_string());
                return Err(anyhow::anyhow!("Failed to connect to database"));
            }
        };

        Ok(Box::new(DBClientImpl { con: Arc::new(con) }))
    }
}

#[async_trait]
impl DBClient for DBClientImpl {
    fn get_connection(&self) -> Arc<DatabaseConnection> {
        self.con.clone()
    }
}
