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
}

pub mod config;
pub mod database;
pub mod errors;

use crate::config::AppConfig;
use database::Database;
use errors::catchers;
use figment::providers::{Format, Serialized, Toml};
use figment::Figment;
use rocket::{
    build, config::TlsConfig, launch, routes, shield::ExpectCt, shield::Feature, shield::Frame,
    shield::Hsts, shield::NoSniff, shield::Permission, shield::Prefetch, shield::Referrer,
    shield::Shield, shield::XssFilter, time::Duration, Build, Config, Rocket,
};
use rocket_async_compression::{Compression, Level as CompressionLevel};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{health::health, ifc::delete_ifc, ifc::get_ifc, ifc::update_ifc, ifc::upload_ifc};

#[launch]
async fn rocket() -> Rocket<Build> {
    let figment =
        Figment::from(Serialized::defaults(AppConfig::default())).merge(Toml::file("config.toml"));

    let config = figment.extract::<AppConfig>().unwrap_or_else(|err| {
        println!("Configuration error (using defaults): {}", err);
        AppConfig::default()
    });

    let mut rocket_config = Config::default();

    if let (Some(cert_path), Some(key_path)) = (&config.tls_cert_path, &config.tls_key_path) {
        if !cert_path.is_empty() && !key_path.is_empty() {
            rocket_config.tls = Some(TlsConfig::from_paths(cert_path, key_path));
        }
    }

    let db = Database::new(&config).await;

    build()
        .configure(rocket_config)
        .manage(config)
        .manage(db)
        .mount(
            "/",
            routes![health, upload_ifc, get_ifc, update_ifc, delete_ifc],
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
