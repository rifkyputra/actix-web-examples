use diesel::pg::PgConnection;
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
// use dotenv::dotenv;
// use std::env;

// pub fn connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("not set");

//     PgConnection::establish(&database_url).expect(&format!("Error Connecting {database_url}"))
// }

// use diesel::{
//     connection::Connection,
//     r2d2::{ConnectionManager, Pool},
//     PgConnection,
// };
// use diesel_migrations;

// pub mod schema;

// pub fn run_migrations(db_url: &str) {
//     embed_migrations!();
//     let connection = PgConnection::establish(db_url).expect("Error connecting to database");
//     embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
//         .expect("Error running migrations");
// }

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
