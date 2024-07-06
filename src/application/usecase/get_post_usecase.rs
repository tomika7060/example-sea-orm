use std::sync::Arc;
use anyhow::Result;

use async_trait::async_trait;

use crate::domain::{entity::post::Post, repository::post_repository::PostRepository};

#[async_trait]
pub trait GetPostUsecase {
    async fn execute(&self, id: i32) -> Result<Post>;
}

pub struct GetPostUsecaseImpl<P>
where
    P: PostRepository,
{
    post_repository: Arc<P>,
}

impl<P> GetPostUsecaseImpl<P>
where
    P: PostRepository,
{
    pub fn new(post_repository: Arc<P>) -> Self {
        Self { post_repository }
    }
}

#[async_trait]
impl<P> GetPostUsecase for GetPostUsecaseImpl<P>
where
    P: PostRepository + Send + Sync,
{
    async fn execute(&self, id: i32) -> Result<Post> {
        let post = self.post_repository.find_by_id(id).await?;

        Ok(post)
    }
}
