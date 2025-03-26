/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use chrono::{DateTime, Utc};
use rocket::{get, serde::json::Json};
use serde::Serialize;

use super::auth::Auth;

#[derive(Serialize)]
pub struct Response {
    status: &'static str,
    version: &'static str,
    timestamp: DateTime<Utc>,
}

/// Health check endpoint to confirm the service is running.
///
/// # Arguments
/// * `_auth` - The authentication guard for the request.
///
/// # Returns
/// A JSON response with the status, version, and timestamp.
#[get("/health")]
pub fn health(_auth: Auth) -> Json<Response> {
    Json(Response {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now(),
    })
}
