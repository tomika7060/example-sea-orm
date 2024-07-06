use async_trait::async_trait;
use anyhow::Result;

use crate::domain::entity::{new_post::NewPost, post::Post};

#[async_trait]
pub trait PostRepository {
    async fn find_by_id(&self, id: i32) -> Result<Post>;
    async fn create(&self, post: NewPost) -> Result<Post>;
}
