use std::sync::Arc;
use anyhow::Result;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    application::transaction_manager::TransactionManager, domain::{
        entity::{new_post::NewPost, post::Post},
        repository::post_repository::PostRepository,
    },
};

#[async_trait]
pub trait CreatePostUsecase {
    async fn execute(&self, post: NewPost) -> Result<Post>;
}

pub struct CreatePostUsecaseImpl<P, TM>
where
    P: PostRepository,
    TM: TransactionManager,
{
    post_repository: Arc<P>,
    transaction_manager: Arc<Mutex<TM>>,
}

impl<P, TM> CreatePostUsecaseImpl<P, TM>
where
    P: PostRepository,
    TM: TransactionManager,
{
    pub fn new(post_repository: Arc<P>, transaction_manager: Arc<Mutex<TM>>) -> Self {
        Self {
            post_repository,
            transaction_manager,
        }
    }
}

#[async_trait]
impl<P, TM> CreatePostUsecase for CreatePostUsecaseImpl<P, TM>
where
    P: PostRepository + Send + Sync,
    TM: TransactionManager + Send + Sync,
{
    async fn execute(&self, post: NewPost) -> Result<Post> {
        self.transaction_manager.lock().await.begin().await?;


        let res = self.post_repository.create(post).await;

        match res.as_ref().map_err(|e| e.to_string()) {
            Ok(_) => {
                self.transaction_manager.lock().await.commit().await?;
            }
            Err(e) => {
                self.transaction_manager.lock().await.rollback().await?;
                return Err(anyhow::anyhow!(e));
            }
        }

        res
    }
}
