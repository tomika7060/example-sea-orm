use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionManager {
    async fn begin(&mut self) -> Result<()>;
    async fn commit(&mut self) -> Result<()>;
    async fn rollback(&mut self) -> Result<()>;
}