use anyhow::Result;
use async_trait::async_trait;

use crate::{
    application::usecase::{create_post_usecase::CreatePostUsecase, get_post_usecase::GetPostUsecase},
    domain::entity::{new_post::NewPost, post::Post},
};

#[async_trait]
pub trait PostController: Send + Sync {
    async fn get_post(&self, id: i32) -> Result<Post>;
    async fn create_post(&self, post: NewPost) -> Result<Post>;
}

pub struct PostControllerImpl<GU,CU>
where
    GU: GetPostUsecase,
    CU: CreatePostUsecase
{
    get_post_usecase: GU,
    create_post_usecase: CU,
}

impl<GU,CU> PostControllerImpl<GU,CU>
where
    GU: GetPostUsecase,
    CU: CreatePostUsecase
{
    pub fn new(get_post_usecase: GU,create_post_usecase:CU) -> Self {
        PostControllerImpl { get_post_usecase,create_post_usecase }
    }
}

#[async_trait]
impl<GU,CU> PostController for PostControllerImpl<GU,CU>
where
    GU: GetPostUsecase + Send + Sync,
    CU: CreatePostUsecase + Send + Sync
{
    async fn get_post(&self, id: i32) -> Result<Post> {
        self.get_post_usecase.execute(id).await
    }

    async fn create_post(&self, post: NewPost) -> Result<Post> {
        self.create_post_usecase.execute(post).await
    }
}
