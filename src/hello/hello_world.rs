use actix_web::{web, Route};

pub fn route() -> Route {
    web::get().to(index)
}

async fn index() -> String {
    format!("Hello, From Actix!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[test]
    async fn success() {
        assert!(true);
    }

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = index().await;
        assert_eq!(resp, "Hello, From Actix!");
    }
}
