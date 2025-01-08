/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

mod schemas;
mod utils;

mod models {
    pub mod user;
}

pub mod api {
    pub mod github;
    pub mod user;
}

use crate::api::github::{github_callback, github_login, GitHub};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_oauth2::OAuth2;
use std::env;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(
            Pool::builder()
                .build(ConnectionManager::<PgConnection>::new(
                    env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                ))
                .expect("Failed to create pool."),
        )
        .mount("/", routes![github_login, github_callback])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(OAuth2::<GitHub>::fairing("github"))
}
