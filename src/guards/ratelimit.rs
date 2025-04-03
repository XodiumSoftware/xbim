/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};

/// RateLimit Guard.
pub struct RateLimitGuard;

#[async_trait]
impl<'r> FromRequest<'r> for RateLimitGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(status) = request.local_cache(|| Option::<Status>::None) {
            return Outcome::Error((*status, ()));
        }
        Outcome::Success(RateLimitGuard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::{Client, LocalResponse};
    use rocket::{build, get, routes};

    #[get("/ratelimited")]
    fn ratelimited_endpoint(_guard: RateLimitGuard) -> &'static str {
        "Rate limit not exceeded"
    }

    #[get("/simulate_ratelimit")]
    fn simulate_ratelimit(request: &Request<'_>) -> &'static str {
        request.local_cache(|| Some(Status::TooManyRequests));
        "Rate limit simulation set"
    }

    struct TestContext {
        client: Client,
    }

    impl TestContext {
        fn new() -> Self {
            let rocket = build().mount("/", routes![ratelimited_endpoint, simulate_ratelimit]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client }
        }

        fn get<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
            self.client.get(path).dispatch()
        }
    }

    #[test]
    fn test_ratelimit_guard() {
        let ctx = TestContext::new();

        // Test normal access - should succeed
        let response = ctx.get("/ratelimited");
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Rate limit not exceeded");

        // Simulate rate limit by setting Status in local cache
        let _ = ctx.get("/simulate_ratelimit");

        // Next request should be rate limited
        let response = ctx.get("/ratelimited");
        assert_eq!(response.status(), Status::TooManyRequests);
    }
}
