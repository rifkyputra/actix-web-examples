use actix::{io, MailboxError};
use actix_web::{
    error, get, post,
    web::{self, Data, Json},
    HttpResponse, Responder, Route,
};

use derive_more::{Display, Error};
use log::info;
use models::{AppState, NewPost};

use crate::blog::blog_actor::{Create, GetArticles};

pub mod blog;
pub mod blog_actor;
pub mod models;

#[post("/blog")]
pub async fn post_article(state: Data<AppState>, article: Json<NewPost>) -> impl Responder {
    let db = state.as_ref().db.clone();

    let article = article.into_inner();

    match db
        .send(Create {
            title: article.title,
            body: article.body,
        })
        .await
        .map_err(|e| {
            info!("{e}");

            error::ErrorInternalServerError::<MailboxError>(e)
        }) {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        _ => HttpResponse::InternalServerError().json("create failed"),
    }
}

#[get("/blog")]
pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    info!("Get Blog List ");
    let db = state.as_ref().db.clone();

    match db.send(GetArticles).await.map_err(|e| {
        info!("{e}");
        error::ErrorInternalServerError::<MailboxError>(e)
    }) {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        e => HttpResponse::InternalServerError().json("get error {e}"),
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct AppError {
    name: &'static str,
}

impl error::ResponseError for AppError {}
