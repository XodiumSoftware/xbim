/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use crate::config::Config;
use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

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
        let config = request
            .rocket()
            .state::<Config>()
            .expect("Config not found in Rocket state");
        match request.headers().get_one("X-API-Key") {
            Some(key) if key == config.api_key => Outcome::Success(Authenticator),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Header;
    use rocket::local::blocking::Client;
    use rocket::{get, routes};

    #[get("/protected")]
    fn protected(_auth: Authenticator) -> &'static str {
        "Protected content"
    }

    #[test]
    fn test_valid_api_key() {
        let config = Config::default();
        let api_key = config.api_key.clone();
        let rocket = rocket::build().mount("/", routes![protected]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client
            .get("/protected")
            .header(Header::new("X-API-Key", api_key))
            .dispatch();

        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Protected content");
    }

    #[test]
    fn test_invalid_api_key() {
        let rocket = rocket::build().mount("/", routes![protected]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client
            .get("/protected")
            .header(Header::new("X-API-Key", "invalid-key"))
            .dispatch();

        assert_eq!(response.status(), rocket::http::Status::Unauthorized);
    }
}
