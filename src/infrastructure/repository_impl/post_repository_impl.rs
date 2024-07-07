use anyhow::Result;
use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::domain::entity::new_post::NewPost;
use crate::domain::{entity::post::Post, repository::post_repository::PostRepository};
use crate::infrastructure::tables::post;
use post::ActiveModel as PostActiveModel;
use post::Entity as PostEntity;
use post::Model as PostModel;

pub struct PostRepositoryImpl {
    con: Arc<DatabaseConnection>,
}

impl PostRepositoryImpl {
    pub fn new(con: Arc<DatabaseConnection>) -> Self {
        Self { con }
    }

    fn from_entity(entity: PostModel) -> Post {
        Post {
            id: entity.id,
            title: entity.title,
            text: entity.text,
        }
    }

    fn to_active_model(post: NewPost) -> PostActiveModel {
        PostActiveModel {
            title: Set(post.title),
            text: Set(post.text),
            ..Default::default()
        }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Post> {
        let res = PostEntity::find_by_id(id)
            .one(&*self.con)
            .await
            .map_err(|e| e)?;

        match res {
            Some(post) => Ok(Self::from_entity(post)),
            None => Err(anyhow::anyhow!("Post not found")),
        }
    }

    async fn create(&self, post: NewPost) -> Result<Post> {
        let post_active_model = Self::to_active_model(post);

        let res = post_active_model.insert(&*self.con).await.map_err(|e| e)?;

        Ok(Self::from_entity(res))
    }
}
