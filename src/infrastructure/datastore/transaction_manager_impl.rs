use anyhow::Result;
use async_trait::async_trait;
use log::{error, info};
use sea_orm::TransactionTrait;
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::application::transaction_manager::TransactionManager;

pub struct TransactionManagerImpl {
    con: Arc<DatabaseConnection>,
    transaction: Mutex<Option<DatabaseTransaction>>,
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
