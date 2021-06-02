pub mod controllers;
pub mod routes;
#[cfg(test)]
pub mod tests;
pub mod utils;

#[macro_use]
extern crate diesel; // This sounds like that the #[macro_use] annotation is missing on your extern crate diesel
pub mod errors;
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
