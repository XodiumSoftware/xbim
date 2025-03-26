/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod middleware {
    pub mod auth;
}

pub mod routes {
    pub mod flutter_service_worker;
    pub mod health;
    pub mod index;
}

pub mod constants;
pub mod database;
pub mod errors;

use constants::ROCKET_PORT;
use database::Database;
use errors::catchers;
use rocket::{build, launch, routes, Build, Config, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{flutter_service_worker::flutter_service_worker, health::health, index::index};

/// Launches the Rocket application.
///
/// # Returns
/// A Rocket instance.
#[launch]
async fn rocket() -> Rocket<Build> {
    build()
        .configure(Config {
            port: ROCKET_PORT,
            ..Config::debug_default()
        })
        .manage(Database::new().await)
        .mount("/", routes![index, flutter_service_worker])
        .mount("api", routes![health])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .register("/", catchers())
}
