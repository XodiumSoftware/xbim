/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::routes::github::GitHubUser;
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
            .cookies()
            .get_private("user_session")
            .and_then(|cookie| {
                serde_json::from_str::<GitHubUser>(cookie.value())
                    .map(|_| AuthGuard)
                    .ok()
            })
            .map(Outcome::Success)
            .unwrap_or(Outcome::Error((Status::Unauthorized, ())))
    }
}

// TODO: tests
