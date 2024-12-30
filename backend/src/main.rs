#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod database;
mod api {
    pub mod user;
}

use crate::database::Database;
use dotenv::dotenv;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::env;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(
            Database::new(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
                .await
                .expect("Failed to initialize the database"),
        )
        .mount("/", routes![])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Error while building CORS"),
        )
}
