use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    adapter::controller::post_controller::{PostController, PostControllerImpl},
    application::usecase::{
        create_post_usecase::CreatePostUsecaseImpl, get_post_usecase::GetPostUsecaseImpl,
    },
};

use super::{
    datastore::{db_client::DBClient, transaction_manager_impl::TransactionManagerImpl},
    repository_impl::post_repository_impl::PostRepositoryImpl,
};

#[async_trait]
pub trait DIContainer {
    async fn post_container(&self) -> Arc<dyn PostController>;
}

pub struct DIContainerImpl<D>
where
    D: DBClient,
{
    db_client: D,
}

impl<D> DIContainerImpl<D>
where
    D: DBClient,
{
    pub fn new(db_client: D) -> Self {
        Self { db_client }
    }
}

#[async_trait]
impl<D> DIContainer for DIContainerImpl<D>
where
    D: DBClient + Send + Sync,
{
    async fn post_container(&self) -> Arc<dyn PostController> {
        let post_repository = Arc::new(PostRepositoryImpl::new(Arc::clone(
            &self.db_client.get_connection(),
        )));

        let get_post_usecase = GetPostUsecaseImpl::new(Arc::clone(&post_repository));

        let transaction_manager = Arc::new(Mutex::new(TransactionManagerImpl::new(Arc::clone(
            &self.db_client.get_connection(),
        ))));

        let create_post_usecase =
            CreatePostUsecaseImpl::new(Arc::clone(&post_repository), transaction_manager);

        let post_controller = PostControllerImpl::new(get_post_usecase, create_post_usecase);
        Arc::new(post_controller)
    }
}
