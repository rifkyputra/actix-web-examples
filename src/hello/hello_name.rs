use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    Route,
};
use chrono::Utc;

pub struct VisitBoard {
    name: Mutex<String>,
    since: Mutex<String>,
    date: Mutex<String>,
}

impl VisitBoard {
    pub fn init_state() -> Data<VisitBoard> {
        return web::Data::new(VisitBoard {
            date: Mutex::new(String::from("Never")),
            name: Mutex::new(String::from("Guest")),
            since: Mutex::new(String::from("Never")),
        });
    }
}

pub fn route() -> Route {
    web::get().to(index)
}

async fn index(data: web::Data<VisitBoard>) -> String {
    let mut date = data.date.lock().unwrap();
    let since = data.since.lock().unwrap();
    let name = data.name.lock().unwrap();

    let text: String =
        format!("hello  {name} , last visit : {date} \n duration since last visit: {since}");

    // let duration : String = Utc::now()
    let now = Utc::now().to_string();

    *date = now;

    return text;
}
