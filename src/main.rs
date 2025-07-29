#![warn(clippy::all)]
#![forbid(unsafe_code)]

pub mod guards {
    pub mod auth;
    pub mod ratelimit;
}

pub mod models {
    pub mod card;
    pub mod user;
}

pub mod routes {
    pub mod data;
    pub mod github;
    pub mod health;
}

pub mod config;
pub mod database;
pub mod errors;
mod tls;
mod utils;

use crate::config::Config;
use crate::routes::data::{data_delete, data_get, data_update, data_upload};
use crate::routes::github::{GitHubUser, github_callback, github_login};
use crate::routes::health::health;
use crate::tls::Tls;
use crate::utils::Utils;
use database::Database;
use errors::catchers;
use rocket::config::SecretKey;
use rocket::routes;
use rocket::{
    Build, Rocket, build, config::TlsConfig, launch, shield::ExpectCt, shield::Feature,
    shield::Frame, shield::Hsts, shield::NoSniff, shield::Permission, shield::Prefetch,
    shield::Referrer, shield::Shield, shield::XssFilter, time::Duration,
};
use rocket_async_compression::{Compression, Level as CompressionLevel};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_oauth2::{HyperRustlsAdapter, OAuth2, OAuthConfig, StaticProvider};

#[launch]
async fn rocket() -> Rocket<Build> {
    let config_path = Utils::get_exec_path("config.toml");
    let mut config = Config::new();

    if config.tls_cert_path.is_empty() || config.tls_key_path.is_empty() {
        let cert_path = Utils::get_exec_path("certs/cert.pem");
        let key_path = Utils::get_exec_path("certs/key.pem");

        Tls::new(cert_path.clone(), key_path.clone()).expect("Failed to generate TLS certificates");

        config.tls_cert_path = cert_path.to_string_lossy().into_owned();
        config.tls_key_path = key_path.to_string_lossy().into_owned();
        config
            .save_to_file(&config_path)
            .expect("Failed to save updated config with TLS paths");

        println!("Auto-generated TLS certificates for development at:");
        println!("  Config: {}", config_path.display());
        println!("  Cert: {}", cert_path.display());
        println!("  Key: {}", key_path.display());
    }

    let config_clone = config.clone();

    build()
        .configure(rocket::Config {
            tls: (!config.tls_cert_path.is_empty() && !config.tls_key_path.is_empty())
                .then(|| TlsConfig::from_paths(&config.tls_cert_path, &config.tls_key_path)),
            secret_key: SecretKey::derive_from(config.secret_key.as_bytes()),
            ..rocket::Config::default()
        })
        .manage(config.clone())
        .manage(Database::new(&config_clone).await)
        .mount(
            "/",
            routes![
                github_login,
                github_callback,
                health,
                data_upload,
                data_get,
                data_update,
                data_delete,
            ],
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
        .attach(OAuth2::<GitHubUser>::custom(
            HyperRustlsAdapter::default(),
            OAuthConfig::new(
                StaticProvider::GitHub,
                config.github_client_id.clone(),
                config.github_client_secret.clone(),
                Some(config.github_redirect_url.clone()),
            ),
        ))
        .register("/", catchers())
}
