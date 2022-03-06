use actix_web::{web, Route};

pub fn route() -> Route {
    web::get().to(index)
}

async fn index() -> String {
    format!("Hello, World!")
}
