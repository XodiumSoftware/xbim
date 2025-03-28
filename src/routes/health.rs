/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use crate::middlewares::{authentication::RAG, identification::RIG};
use chrono::{DateTime, Utc};
use rocket::{get, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    status: String,
    version: String,
    timestamp: DateTime<Utc>,
    request_id: String,
}

/// Health check endpoint to confirm the service is running.
///
/// # Arguments
/// * `rig` - Request Identification Guard.
/// * `_rag` - Request Authentication Guard.
///
/// # Returns
/// A JSON response with the status, version, and timestamp.
#[get("/health")]
pub fn health(rig: RIG, _rag: RAG) -> Json<Response> {
    println!("Health check requested with request ID: {}", rig.0);
    Json(Response {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
        request_id: rig.0,
    })
}
