/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod api {
    pub mod database;
    pub mod health;
}

use crate::api::database::Database;
use api::health::health;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{build, catch, catchers, get, launch, routes, Build, Config, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

/// Rocket connection Port
const ROCKET_PORT: u16 = 8080;

/// SurrealDB connection URL
const SURREALDB_URL: &str = "localhost:8000";

/// SurrealDB username for authentication
const SURREALDB_USERNAME: &str = "root";

/// SurrealDB password for authentication
const SURREALDB_PASSWORD: &str = "root";

/// Redirects to the 404 page.
///
/// # Returns
/// A redirect to the 404 page.
#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to("https://xodium.org/404")
}

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
    let db = Surreal::new::<Ws>(SURREALDB_URL).await.expect(&format!(
        "Failed to connect to SurrealDB at {}",
        SURREALDB_URL
    ));

    db.signin(Root {
        username: SURREALDB_USERNAME,
        password: SURREALDB_PASSWORD,
    })
    .await
    .expect("Failed to sign in to SurrealDB");

    build()
        .configure(Config {
            port: ROCKET_PORT,
            ..Config::debug_default()
        })
        .manage(Database::new(db))
        .mount("/", routes![index, flutter_service_worker])
        .mount("/api", routes![health])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .register("/", catchers![not_found])
}
