/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Data, Request, Response};
use uuid::Uuid;

/// Request and Response Identification Middleware
pub struct RRIM;

#[async_trait]
impl Fairing for RRIM {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        request.local_cache(|| Uuid::new_v4().to_string());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "X-Request-ID",
            request.local_cache::<String, _>(|| String::new()).clone(),
        ));
    }
}

/// Request Identification Guard
pub struct RIG(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for RIG {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RIG(request
            .local_cache::<String, _>(|| String::new())
            .clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::{Client, LocalResponse};
    use rocket::{build, routes};

    #[rocket::get("/test")]
    fn test_endpoint() -> &'static str {
        "Hello, world!"
    }

    #[rocket::get("/guard")]
    fn guard_endpoint(id_guard: RIG) -> String {
        format!("Request ID: {}", id_guard.0)
    }

    struct TestContext {
        client: Client,
    }

    impl TestContext {
        fn new() -> Self {
            let rocket = build()
                .attach(RRIM)
                .mount("/", routes![test_endpoint, guard_endpoint]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client }
        }

        fn get<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch()
        }
    }

    #[test]
    fn test_request_id_header() {
        let ctx = TestContext::new();
        let response = ctx.get("/test");

        assert_eq!(response.status(), Status::Ok);
        assert!(response.headers().get_one("X-Request-ID").is_some());

        let request_id = response.headers().get_one("X-Request-ID").unwrap();
        assert!(!request_id.is_empty());
        assert_eq!(request_id.len(), 36);
    }

    #[test]
    fn test_multiple_requests_different_ids() {
        let ctx = TestContext::new();

        let response1 = ctx.get("/test");
        let response2 = ctx.get("/test");

        let id1 = response1.headers().get_one("X-Request-ID").unwrap();
        let id2 = response2.headers().get_one("X-Request-ID").unwrap();

        assert_ne!(id1, id2, "Different requests should have different IDs");
    }

    #[test]
    fn test_id_guard() {
        let ctx = TestContext::new();
        let response = ctx.get("/guard");

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_string().unwrap();
        assert!(body.starts_with("Request ID: "));

        let request_id = body.strip_prefix("Request ID: ").unwrap();
        assert_eq!(request_id.len(), 36);
    }
}
