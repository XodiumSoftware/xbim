/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::guards::{auth::AuthGuard, id::IdGuard};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::{get, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    status: Status,
    version: &'static str,
    timestamp: DateTime<Utc>,
    request_id: String,
}

/// Health check endpoint to confirm the service is running.
///
/// # Arguments
/// * `ig` - Identification Guard.
/// * `_ag` - Authentication Guard.
///
/// # Returns
/// A JSON response with the status, version, timestamp, and request ID.
#[get("/health")]
pub fn health(ig: IdGuard, _ag: AuthGuard) -> Json<Response> {
    println!("Health check requested with request ID: {}", ig.0);
    Json(Response {
        status: Status::Ok,
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now(),
        request_id: ig.0,
    })
}
