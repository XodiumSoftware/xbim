/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use rocket::{catch, catchers, http::Status, serde::json::Json, serde::Serialize, Catcher};
use rocket_governor::rocket_governor_catcher;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    status: Status,
    message: &'static str,
}

/// Returns a list of catchers for the application.
///
/// # Returns
/// A vector of catchers.
pub fn catchers() -> Vec<Catcher> {
    catchers![
        err_400,
        err_401,
        err_403,
        err_404,
        err_405,
        rocket_governor_catcher,
        err_500,
        err_503
    ]
}

#[catch(400)]
fn err_400() -> Json<Response> {
    Json(Response {
        status: Status::BadRequest,
        message: "Bad request format or invalid parameters",
    })
}

#[catch(401)]
fn err_401() -> Json<Response> {
    Json(Response {
        status: Status::Unauthorized,
        message: "Authentication required",
    })
}

#[catch(403)]
fn err_403() -> Json<Response> {
    Json(Response {
        status: Status::Forbidden,
        message: "Access forbidden - You don't have permission to access this resource",
    })
}

#[catch(404)]
fn err_404() -> Json<Response> {
    Json(Response {
        status: Status::NotFound,
        message: "Resource not found",
    })
}

#[catch(405)]
fn err_405() -> Json<Response> {
    Json(Response {
        status: Status::MethodNotAllowed,
        message: "Method not allowed for this resource",
    })
}

#[catch(500)]
fn err_500() -> Json<Response> {
    Json(Response {
        status: Status::InternalServerError,
        message: "Internal server error",
    })
}

#[catch(503)]
fn err_503() -> Json<Response> {
    Json(Response {
        status: Status::ServiceUnavailable,
        message: "Service temporarily unavailable",
    })
}
