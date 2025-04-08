/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::config::AppConfig;
use rocket::serde::uuid::Uuid;
use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

/// Authentication Guard
pub struct AuthGuard;

#[async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request
            .headers()
            .get_one("X-API-Key")
            .and_then(|key| Uuid::parse_str(key).ok())
            .filter(|&parsed_key| {
                parsed_key
                    == request
                        .rocket()
                        .state::<AppConfig>()
                        .expect("Config not found in Rocket state")
                        .api_key
            })
            .map(|_| Outcome::Success(AuthGuard))
            .unwrap_or(Outcome::Error((Status::Unauthorized, ())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Header;
    use rocket::local::blocking::{Client, LocalResponse};
    use rocket::{build, get, routes};

    #[get("/protected")]
    fn protected_endpoint(_authguard: AuthGuard) -> &'static str {
        "Protected content"
    }

    struct TestContext {
        client: Client,
        api_key: Uuid,
    }

    impl TestContext {
        fn new() -> Self {
            let config = AppConfig::default();
            let api_key = config.api_key;
            let rocket = build()
                .manage(config)
                .mount("/", routes![protected_endpoint]);
            let client = Client::tracked(rocket).expect("valid rocket instance");
            TestContext { client, api_key }
        }

        fn request_with_key(&self, key: Uuid) -> LocalResponse<'_> {
            self.client
                .get("/protected")
                .header(Header::new("X-API-Key", key.to_string()))
                .dispatch()
        }
    }

    #[test]
    fn test_api_key_authentication() {
        let ctx = TestContext::new();

        // Test valid key
        let valid_response = ctx.request_with_key(ctx.api_key);
        assert_eq!(valid_response.status(), Status::Ok);
        assert_eq!(valid_response.into_string().unwrap(), "Protected content");

        // Test invalid key (different UUID)
        let invalid_key = Uuid::now_v7();
        let invalid_response = ctx.request_with_key(invalid_key);
        assert_eq!(invalid_response.status(), Status::Unauthorized);

        // Test malformed key by directly sending a string
        let malformed_response = ctx
            .client
            .get("/protected")
            .header(Header::new("X-API-Key", "invalid-not-a-uuid"))
            .dispatch();
        assert_eq!(malformed_response.status(), Status::Unauthorized);
    }
}
