use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use log::error;

use crate::{domain::entity::new_post::NewPost, infrastructure::server::RequestContext};

pub async fn get_post_handler(
    data: web::Data<Arc<RequestContext>>,
    id: web::Path<i32>,
) -> impl Responder {
    match data.post_controller.get_post(id.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            error!("Error: {}", e);
            return HttpResponse::InternalServerError().body(format!("Error: {}", e));
        }
    }
}

pub async fn create_post_handler(
    data: web::Data<Arc<RequestContext>>,
    post: web::Json<NewPost>,
) -> impl Responder {
    match data.post_controller.create_post(post.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error: {}", e);
            return HttpResponse::InternalServerError().body(format!("Error: {}", e));
        }
    }
}
