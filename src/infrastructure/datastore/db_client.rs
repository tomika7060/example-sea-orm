use anyhow::Result;
use async_trait::async_trait;
use log::{error, info};
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};
use std::{env, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::application::transaction_manager::TransactionManager;

#[async_trait]
pub trait DBClient {
    fn get_connection(&self) -> Arc<DatabaseConnection>;
}

pub struct DBClientImpl {
    pub con: Arc<DatabaseConnection>,
}

pub struct TransactionManagerImpl {
    pub con: Arc<DatabaseConnection>,
    transaction: Mutex<Option<DatabaseTransaction>>,
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

impl TransactionManagerImpl {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self {
            con: connection,
            transaction: Mutex::new(None),
        }
    }
}

#[async_trait]
impl DBClient for DBClientImpl {
    fn get_connection(&self) -> Arc<DatabaseConnection> {
        self.con.clone()
    }
}

#[async_trait]
impl TransactionManager for TransactionManagerImpl {
    async fn begin(&mut self) -> Result<()> {
        let mut tx_guard = self.transaction.lock().await;
        if tx_guard.is_none() {
            let transaction = self.con.begin().await.map_err(|e| {
                error!("Failed to start transaction: {}", e.to_string());
                e
            })?;

            info!("Transaction started");
            *tx_guard = Some(transaction);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Transaction already exists"))
        }
    }

    async fn commit(&mut self) -> Result<()> {
        let mut tx_guard = self.transaction.lock().await;
        if let Some(transaction) = tx_guard.take() {
            transaction.commit().await.map_err(|e| {
                error!("Failed to commit transaction: {}", e.to_string());
                e
            })?;
            info!("Transaction committed");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Transaction already exists"))
        }
    }

    async fn rollback(&mut self) -> Result<()> {
        let mut tx_guard = self.transaction.lock().await;
        if let Some(transaction) = tx_guard.take() {
            transaction.rollback().await.map_err(|e| {
                error!("Failed to rollback transaction: {}", e.to_string());
                e
            })?;
            info!("Transaction rolled back");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Transaction already exists"))
        }
    }
}
