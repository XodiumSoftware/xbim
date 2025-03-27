/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::constants::API_KEY;

/// Authentication guard for protected routes middleware
pub struct Authenticator;

#[async_trait]
impl<'r> FromRequest<'r> for Authenticator {
    type Error = ();

    /// Validates the API key in the request header
    ///
    /// # Arguments
    /// * `request` - The request to validate
    ///
    /// # Returns
    /// An `Outcome` with `ApiAuth` if the API key is valid, or an error if it is not
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("X-API-Key") {
            Some(key) if key == API_KEY => Outcome::Success(Authenticator),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
