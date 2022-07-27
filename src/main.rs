extern crate actix;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use std::env;

use actix::SyncArbiter;
use actix_files as fs;

use actix_web::{middleware::Logger, App, HttpServer};
use blog::blog_actor::BlogActor;
use env_logger::Env;
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
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let db_url = env::var("DATABASE_URL").expect("Error No DATABASE_URL specified");
    
    let app_url: String = match env::var("APP_URL") {
        Ok(val) => val,
        Err(_) => String::from("127.0.0.1"),
    };

    let app_port: u16 = match env::var("APP_PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_) => 8090,
    };

    let pool = db::get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || BlogActor(pool.clone()));

    let counter = try_state::count::counter_state();
    let visit_state = hello::hello_name::VisitBoard::init_state();
    let db = blog::models::AppState::init_state(db_addr.clone());

    info!("Server Starting on {app_url} port {app_port}");

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
    .bind((app_url, app_port))?
    .run()
    .await
}
