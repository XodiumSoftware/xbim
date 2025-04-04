/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod fairings {
    pub mod filtering;
}

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

use database::Database;
use errors::catchers;
use fairings::filtering::IpFilter;
use rocket::shield::{
    ExpectCt, Feature, Frame, Hsts, NoSniff, Permission, Prefetch, Referrer, Shield, XssFilter,
};
use rocket::time::Duration;
use rocket::{build, launch, routes, Build, Config, Rocket};
use rocket_async_compression::{Compression, Level as CompressionLevel};
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
        .attach(IpFilter::default())
        .register("/", catchers())
}
