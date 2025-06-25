/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use crate::{guards::auth::AuthGuard, guards::ratelimit::RateLimitGuard};
use chrono::{DateTime, Utc};
use rocket::{get, http::Status, serde::json::Json, serde::Serialize};
use rocket_governor::RocketGovernor;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    status: Status,
    version: &'static str,
    timestamp: DateTime<Utc>,
}

/// Health check endpoint to confirm the service is running.
///
/// # Arguments
/// * `_authguard`: An instance of `AuthGuard` to handle authentication.
/// * `_ratelimitguard`: An instance of `RateLimitGuard` to handle rate limiting.
///
/// # Returns
/// A JSON response with the status, request ID, version, and timestamp.
#[get("/health")]
pub fn health(
    _authguard: AuthGuard,
    _ratelimitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Json<Response> {
    Json(Response {
        status: Status::Ok,
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now(),
    })
}
