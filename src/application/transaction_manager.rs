use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionManager {
    async fn begin(&self) -> Result<()>;
    async fn commit(&self) -> Result<()>;
    async fn rollback(&self) -> Result<()>;
}
