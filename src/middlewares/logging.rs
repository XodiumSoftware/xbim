/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use chrono::Utc;
use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

/// Request and response logging middleware
pub struct Logger;

#[async_trait]
impl Fairing for Logger {
    /// Returns the name and kind of the middleware
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    /// Logs incoming requests
    ///
    /// # Arguments
    /// * `req` - The incoming request
    /// * `_data` - The request data
    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        println!(
            "{} - Incoming request: {} {} from {}",
            Utc::now(),
            req.method(),
            req.uri(),
            req.client_ip()
                .map_or("Unknown".into(), |ip| ip.to_string())
        );
    }

    /// Logs outgoing responses
    ///
    /// # Arguments
    /// * `req` - The incoming request
    /// * `res` - The outgoing response
    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        println!(
            "{} - Outgoing response: {} {} - Status: {}",
            Utc::now(),
            req.method(),
            req.uri(),
            res.status()
        );
    }
}
