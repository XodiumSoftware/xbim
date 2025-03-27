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

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{get, http::Status, local::blocking::Client, routes, Build, Rocket};

    #[get("/test")]
    fn test_endpoint() -> &'static str {
        "Test successful"
    }

    fn rocket() -> Rocket<Build> {
        rocket::build()
            .attach(Logger)
            .mount("/", routes![test_endpoint])
    }

    #[test]
    fn test_logger_fairing_doesnt_interfere_with_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/test").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Test successful");
    }

    #[test]
    fn test_logger_info() {
        let logger = Logger;
        let info = logger.info();

        assert_eq!(info.name, "Request Logger");
        assert!(info.kind.is(Kind::Request));
        assert!(info.kind.is(Kind::Response));
    }
}
