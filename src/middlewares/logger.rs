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

pub struct RequestLogger;

#[async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

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
