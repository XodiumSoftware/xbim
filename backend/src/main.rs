#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod database;
mod api {
    pub mod user;
}

use crate::database::Database;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(
            Database::new("postgres://username:password@localhost/dbname")
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
