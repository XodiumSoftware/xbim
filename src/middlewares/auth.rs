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

/// Authentication guard for protected routes using API key
pub struct Auth;

#[async_trait]
impl<'r> FromRequest<'r> for Auth {
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
            Some(key) if key == API_KEY => Outcome::Success(Auth),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
