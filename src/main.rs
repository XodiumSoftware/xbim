/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod api {
    pub mod database;
}

use crate::api::database::Database;
use rocket::{build, launch, Build, Config, Rocket};
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
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
}
