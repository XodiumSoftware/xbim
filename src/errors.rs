/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::{catch, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    status: &'static str,
    code: &'static str,
    message: &'static str,
}

/// Handles bad request errors (400).
///
/// # Returns
/// A JSON response with an error message.
#[catch(400)]
pub fn err_400() -> Json<Response> {
    Json(Response {
        status: "error",
        code: "400",
        message: "Bad request format or invalid parameters",
    })
}

/// Handles unauthorized access attempts (401 errors).
///
/// # Returns
/// A JSON response with an error message.
#[catch(401)]
pub fn err_401() -> Json<Response> {
    Json(Response {
        status: "error",
        code: "401",
        message: "Authentication required",
    })
}

/// Handles forbidden access attempts (403 errors).
///
/// # Returns
/// A JSON response with an error message.
#[catch(403)]
pub fn err_403() -> Json<Response> {
    Json(Response {
        status: "error",
        code: "403",
        message: "Access forbidden - You don't have permission to access this resource",
    })
}

/// Handles not found errors (404).
///
/// # Returns
/// A JSON response with an error message.
#[catch(404)]
pub fn err_404() -> Json<Response> {
    Json(Response {
        status: "error",
        code: "403",
        message: "Access forbidden - You don't have permission to access this resource",
    })
}

/// Handles method not allowed errors (405).
///
/// # Returns
/// A JSON response with an error message.
#[catch(405)]
pub fn err_405() -> Json<Response> {
    Json(Response {
        status: "error",
        code: "405",
        message: "Method not allowed for this resource",
    })
}

/// Handles internal server errors (500).
///
/// # Returns
/// A JSON response with an error message.
#[catch(500)]
pub fn err_500() -> Json<Response> {
    Json(Response {
        status: "error",
        code: "500",
        message: "Internal server error",
    })
}

/// Handles service unavailable errors (503).
///
/// # Returns
/// A JSON response with an error message.
#[catch(503)]
pub fn err_503() -> Json<Response> {
    Json(Response {
        status: "error",
        code: "503",
        message: "Service temporarily unavailable",
    })
}
