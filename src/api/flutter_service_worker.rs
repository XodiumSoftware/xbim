/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::{get, http::Status};

/// Handle Flutter service worker requests
///
/// # Arguments
/// * `_v` - The version of the service worker.
///
/// # Returns
/// A 204 No Content status.
#[get("/flutter_service_worker.js?<_v>")]
pub fn flutter_service_worker(_v: Option<String>) -> Status {
    Status::NoContent
}
