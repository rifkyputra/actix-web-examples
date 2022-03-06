use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    Route,
};

pub struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

pub fn counter_state() -> Data<AppStateWithCounter> {
    return web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
}

pub fn route() -> Route {
    web::get().to(index)
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}
