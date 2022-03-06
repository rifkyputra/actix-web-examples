use actix_files as fs;
use actix_web::{App, HttpServer};

mod hello;
mod try_state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = try_state::count::counter_state();
    let visit_state = hello::hello_name::VisitBoard::init_state();

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .app_data(visit_state.clone())
            .route("/index.html", hello::index::route())
            .route("/assets/{filename:.*}", hello::assets::route())
            .route("/", hello::hello_world::route())
            .route("/visit", hello::hello_name::route())
            .route("/count", try_state::count::route())
            .service(fs::Files::new("/", "./static/assets").show_files_listing())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
