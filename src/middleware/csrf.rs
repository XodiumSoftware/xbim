/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::{
    http::{Cookie, CookieJar, Status},
    request::{FromRequest, Outcome},
    Request,
};
use surrealdb::Uuid;

const CSRF_TOKEN: &str = "csrf_token";

pub fn set_csrf_cookie(cookies: &CookieJar<'_>) {
    cookies.add_private(
        Cookie::build((CSRF_TOKEN, Uuid::new_v4().to_string()))
            .http_only(true)
            .secure(true),
    );
}

#[derive(Debug)]
pub struct CsrfToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CsrfToken {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> Outcome<CsrfToken, Self::Error> {
        if request.method() == rocket::http::Method::Get
            || request.method() == rocket::http::Method::Head
            || request.method() == rocket::http::Method::Options
        {
            return Outcome::Success(CsrfToken(String::new()));
        }
        match (
            request.headers().get_one("X-CSRF-Token"),
            request
                .cookies()
                .get(CSRF_TOKEN)
                .map(|cookie| cookie.value().to_string()),
        ) {
            (Some(header_value), Some(cookie_value)) if header_value == cookie_value => {
                Outcome::Success(CsrfToken(header_value.to_string()))
            }
            _ => Outcome::Error((Status::Forbidden, Status::Forbidden)),
        }
    }
}
