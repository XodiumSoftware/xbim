/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::{async_trait, Request};
use rocket::{request::FromRequest, request::Outcome};
use uuid::Uuid;

/// Identification Guard
pub struct IdGuard {
    pub id: Uuid,
}

#[async_trait]
impl<'r> FromRequest<'r> for IdGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let id_string = request.local_cache::<String, _>(String::new).clone();
        let id = Uuid::parse_str(&id_string).unwrap_or_else(|_| Uuid::nil());
        Outcome::Success(IdGuard { id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fairings::id::IdGenerator;
    use rocket::http::Status;
    use rocket::local::blocking::{Client, LocalResponse};
    use rocket::{build, routes};

    #[rocket::get("/test")]
    fn test_endpoint() -> &'static str {
        "Hello, world!"
    }

    #[rocket::get("/guard")]
    fn guard_endpoint(ig: IdGuard) -> String {
        format!("Request ID: {}", ig.id)
    }

    struct TestContext {
        client: Client,
    }

    impl TestContext {
        fn new() -> Self {
            let rocket = build()
                .attach(IdGenerator)
                .mount("/", routes![test_endpoint, guard_endpoint]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client }
        }

        fn get<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch()
        }
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
