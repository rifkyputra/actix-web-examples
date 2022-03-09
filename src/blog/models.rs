use crate::actix::Addr;
use crate::blog::blog_actor::BlogActor;
use crate::schema::posts;
use actix_web::web::Data;
// use actix_web::error::BlockingError;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

// use uuid::Uuid;
// use serde::{Deserialize, Serialize};

// use super::blog_actor::BlogActor;

// pub struct AppState {
//     pub db: Addr<BlogActor>,
// }

// #[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
// pub struct Article {
//     pub uuid: Uuid,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }

// #[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
// #[table_name = "articles"]
// pub struct NewArticle {
//     pub uuid: Uuid,
//     pub title: String,
//     pub body: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ArticleData {
//     pub title: String,
//     pub body: String,
// }

#[derive(Debug, Clone)]

pub struct AppState {
    pub db: Addr<BlogActor>,
}

impl AppState {
    pub fn init_state(addr: Addr<BlogActor>) -> Data<AppState> {
        return Data::new(AppState { db: addr.clone() });
    }
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
// #[table_name = "posts"]
pub struct Posts {
    pub uuid: i32,

    pub title: String,

    pub body: String,

    pub published: bool,
}

#[derive(Debug, Insertable, Clone, Queryable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
