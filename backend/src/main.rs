/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod api {
    pub mod github;
    pub mod local_storage;
}

use crate::api::github::{github_callback, github_login, GitHub};
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_oauth2::OAuth2;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![github_login, github_callback])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Failed to build CORS"),
        )
        .attach(OAuth2::<GitHub>::fairing("github"))
}
