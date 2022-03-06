use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Route};

pub fn route() -> Route {
    return web::get().to(index);
}

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}
