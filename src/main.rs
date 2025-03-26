/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod api {
    pub mod health;
}

pub mod middleware {
    pub mod auth;
}

pub mod constants;
pub mod database;
pub mod errors;

use api::health::health;
use constants::ROCKET_PORT;
use database::Database;
use errors::{err_400, err_401, err_403, err_404, err_405, err_500, err_503};
use rocket::{
    build, catchers, get, http::Status, launch, response::Redirect, routes, Build, Config, Rocket,
};
use rocket_cors::{AllowedOrigins, CorsOptions};

/// Redirects to the main page.
///
/// # Returns
/// A redirect to the main page.
#[get("/")]
fn index() -> Redirect {
    Redirect::to("https://xodium.org")
}

/// Handle Flutter service worker requests
///
/// # Arguments
/// * `_v` - The version of the service worker.
///
/// # Returns
/// A 204 No Content status.
#[get("/flutter_service_worker.js?<_v>")]
fn flutter_service_worker(_v: Option<String>) -> Status {
    Status::NoContent
}

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
        .mount("/api", routes![health])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .register(
            "/",
            catchers![err_400, err_401, err_403, err_404, err_405, err_500, err_503],
        )
}
