/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod fairings {
    pub mod compression;
    pub mod filtering;
    pub mod id;
    pub mod limiting;
    pub mod logging;
    pub mod security;
}

pub mod guards {
    pub mod auth;
    pub mod id;
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
use fairings::{
    compression::ContentCompressor, filtering::IpFilter, id::IdGenerator, limiting::RateLimiter,
    logging::Logger, security::SecurityHeaders,
};
use rocket::{build, launch, routes, Build, Config, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{health::health, ifc::delete_ifc, ifc::get_ifc, ifc::update_ifc, ifc::upload_ifc};
use std::process::exit;

#[launch]
async fn rocket() -> Rocket<Build> {
    let config = match config::Config::init() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            exit(1);
        }
    };
    build()
        .configure(Config {
            port: config.server_port,
            ..Config::debug_default()
        })
        .manage(config.clone())
        .manage(Database::new(&config).await)
        .mount(
            "/",
            routes![health, upload_ifc, get_ifc, update_ifc, delete_ifc],
        )
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(ContentCompressor)
        .attach(IdGenerator)
        .attach(RateLimiter::new(100, 60))
        .attach(Logger)
        .attach(SecurityHeaders::default())
        .attach(IpFilter::default())
        .register("/", catchers())
}
