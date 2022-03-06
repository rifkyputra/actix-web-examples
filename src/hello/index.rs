use actix_files::NamedFile;
use actix_web::{error, web, Result, Route};

///
/// Actix-web provides ResponseError implementations for some common non-actix errors.
/// For example, if a handler responds with an io::Error, that error is converted into an HttpInternalServerError

pub fn route() -> Route {
    web::get().to(index)
}

async fn index() -> Result<NamedFile> {
    NamedFile::open("static/index.html").map_err(|_| error::ErrorNotFound("Not Found"))
}
