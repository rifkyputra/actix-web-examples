use actix::{Actor, Handler, Message, SyncContext};
// use actix_web::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use super::models::*;
use crate::schema::posts::dsl::*;
use diesel::prelude::*;

// use crate::schema::posts::dsl::{posts};
// extern crate diesel_demo;
extern crate diesel;

pub struct BlogActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Message)]
#[rtype(result = "QueryResult<Posts>")]
pub struct Create {
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Posts>")]
pub struct Edit {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Posts>")]
pub struct Delete {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Posts>")]
pub struct Publish {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Posts>>")]
pub struct GetArticles;

impl Actor for BlogActor {
    type Context = SyncContext<Self>;
}

impl Handler<Create> for BlogActor {
    type Result = QueryResult<Posts>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Cannot connect");

        let new_article = NewPost {
            title: msg.title,
            body: msg.body,
        };

        diesel::insert_into(posts)
            .values(new_article)
            .get_result::<Posts>(&conn)
    }
}
impl Handler<GetArticles> for BlogActor {
    type Result = QueryResult<Vec<Posts>>;

    fn handle(&mut self, msg: GetArticles, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        posts.get_results::<Posts>(&conn)
    }
}
