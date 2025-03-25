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
use rocket::{build, launch, Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

/// SurrealDB connection URL
const SURREALDB_URL: &str = "localhost:8000";

/// Launches the Rocket application.
///
/// # Returns
/// A Rocket instance.
#[launch]
async fn rocket() -> Rocket<Build> {
    build()
        .manage(Database::new(
            Surreal::new::<Ws>(SURREALDB_URL).await.expect(&format!(
                "Failed to connect to SurrealDB at {}",
                SURREALDB_URL
            )),
        ))
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
}
