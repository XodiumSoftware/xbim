/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::guards::{auth::AuthGuard, id::IdGuard};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::{get, serde::json::Json};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct Response {
    status: Status,
    id: Uuid,
    version: &'static str,
    timestamp: DateTime<Utc>,
}

/// Health check endpoint to confirm the service is running.
///
/// # Arguments
/// * `ig` - Identification Guard.
/// * `_ag` - Authentication Guard.
///
/// # Returns
/// A JSON response with the status, request ID, version, and timestamp.
#[get("/health")]
pub fn health(idguard: IdGuard, _authguard: AuthGuard) -> Json<Response> {
    println!("Health check requested with request ID: {}", idguard.id);
    Json(Response {
        status: Status::Ok,
        id: idguard.id,
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now(),
    })
}
