extern crate actix;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use std::env;

use actix::SyncArbiter;
use actix_files as fs;

use actix_web::{middleware::Logger, App, HttpServer};
use blog::blog_actor::BlogActor;
use log::info;
// use diesel_migrations::{run_migrations, run_pending_migrations};

mod blog;
mod db;
mod hello;
mod schema;
mod try_state;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Error");
    let pool = db::get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || BlogActor(pool.clone()));

    let counter = try_state::count::counter_state();
    let visit_state = hello::hello_name::VisitBoard::init_state();
    let db = blog::models::AppState::init_state(db_addr.clone());

    info!("Server Starting");

    // run_pending_migrations(&db_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(counter.clone())
            .app_data(visit_state.clone())
            .app_data(db.clone())
            .route("/index.html", hello::index::route())
            .route("/assets/{filename:.*}", hello::assets::route())
            .route("/", hello::hello_world::route())
            .route("/visit/{name_args}", hello::hello_name::route())
            .route("/count", try_state::count::route())
            .service(blog::post_article)
            .service(blog::get_posts)
            .service(fs::Files::new("/", "./static/assets").show_files_listing())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
