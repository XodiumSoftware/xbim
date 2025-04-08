/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod guards {
    pub mod auth;
    pub mod ratelimit;
}

pub mod routes {
    pub mod health;
    pub mod ifc;
    pub mod index;
}

pub mod config;
pub mod database;
pub mod errors;
mod utils;

use crate::config::AppConfig;
use database::Database;
use errors::catchers;
use rocket::{
    build, config::TlsConfig, launch, routes, shield::ExpectCt, shield::Feature, shield::Frame,
    shield::Hsts, shield::NoSniff, shield::Permission, shield::Prefetch, shield::Referrer,
    shield::Shield, shield::XssFilter, time::Duration, Build, Config, Rocket,
};
use rocket_async_compression::{Compression, Level as CompressionLevel};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{
    health::health, ifc::delete_ifc, ifc::get_ifc, ifc::update_ifc, ifc::upload_ifc, index::index,
};

#[launch]
async fn rocket() -> Rocket<Build> {
    let config = AppConfig::new();
    build()
        .configure(Config {
            tls: (!config.tls_cert_path.is_empty() && !config.tls_key_path.is_empty())
                .then(|| TlsConfig::from_paths(&config.tls_cert_path, &config.tls_key_path)),
            ..Config::default()
        })
        .manage(config.clone())
        .manage(Database::new(&config).await)
        .mount(
            "/",
            routes![index, health, upload_ifc, get_ifc, update_ifc, delete_ifc],
        )
        .attach(
            Shield::new()
                .enable(ExpectCt::Enforce(Duration::days(30)))
                .enable(
                    Permission::default()
                        .block(Feature::Camera)
                        .block(Feature::Geolocation)
                        .block(Feature::Microphone),
                )
                .enable(Frame::SameOrigin)
                .enable(Hsts::IncludeSubDomains(Duration::days(365)))
                .enable(NoSniff::Enable)
                .enable(Prefetch::On)
                .enable(Referrer::StrictOriginWhenCrossOrigin)
                .enable(XssFilter::EnableBlock),
        )
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(Compression::with_level(CompressionLevel::Default))
        .register("/", catchers())
}
