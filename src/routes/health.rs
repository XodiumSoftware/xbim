/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use crate::middlewares::{authentication::Authenticator, identification::IdGuard};
use chrono::{DateTime, Utc};
use rocket::{get, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    status: &'static str,
    version: &'static str,
    timestamp: DateTime<Utc>,
    request_id: String,
}

/// Health check endpoint to confirm the service is running.
///
/// # Arguments
/// * `request_id` - The request ID guard.
/// * `_auth` - The authentication guard for the request.
///
/// # Returns
/// A JSON response with the status, version, and timestamp.
#[get("/health")]
pub fn health(request_id: IdGuard, _auth: Authenticator) -> Json<Response> {
    println!("Health check requested with request ID: {}", request_id.0);
    Json(Response {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now(),
        request_id: request_id.0,
    })
}
