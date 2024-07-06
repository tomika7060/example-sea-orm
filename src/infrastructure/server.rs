use actix_web::{web, App, HttpServer};
use anyhow::Result;
use log::error;
use std::sync::Arc;

use crate::adapter::{
    controller::post_controller::PostController,
    handler::post_handler::{create_post_handler, get_post_handler},
};

use super::{
    datastore::db_client::DBClientImpl,
    di::{DIContainer, DIContainerImpl},
};

#[derive(Clone)]
pub struct RequestContext {
    pub post_controller: Arc<dyn PostController>,
}

impl RequestContext {
    pub async fn new() -> Result<Self> {
        let db_client = DBClientImpl::new().await.map_err(|e| {
            error!("Failed to create DBClient: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create DBClient: {}", e),
            )
        })?;

        let container = DIContainerImpl::new(*db_client);

        let post_controller = container.post_container().await;

        Ok(RequestContext { post_controller })
    }
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let context = match RequestContext::new().await {
        Ok(ctx) => web::Data::new(Arc::new(ctx)),
        Err(e) => {
            error!("Failed to create RequestContext: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create RequestContext: {}", e),
            ));
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .route("/post/{id}", web::get().to(get_post_handler))
            .route("/post", web::post().to(create_post_handler))
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}
