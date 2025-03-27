/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod middlewares {
    pub mod authentication;
    pub mod compression;
    pub mod identification;
    pub mod logging;
}

pub mod routes {
    pub mod health;
    pub mod ifc;
}

pub mod config;
pub mod database;
pub mod errors;

use database::Database;
use errors::catchers;
use middlewares::{compression::Compressor, identification::Identificator, logging::Logger};
use rocket::{build, launch, routes, Build, Config, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{
    health::health,
    ifc::{get_ifc_model, upload_ifc_model},
};

/// Launches the Rocket application.
///
/// # Returns
/// A Rocket instance.
#[launch]
async fn rocket() -> Rocket<Build> {
    let config = match config::Config::init() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    build()
        .configure(Config {
            port: config.server_port,
            ..Config::debug_default()
        })
        .manage(Database::new(&config).await)
        .mount("/", routes![health, upload_ifc_model, get_ifc_model])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(Compressor)
        .attach(Identificator)
        .attach(Logger)
        .register("/", catchers())
}
