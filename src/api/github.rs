/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use reqwest::Error;
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Debug, Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};

pub struct GitHub;

#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHub>, cookies: &CookieJar<'_>) -> Result<Redirect, String> {
    oauth2
        .get_redirect(cookies, &["user:read"])
        .map_err(|e| e.to_string())
}

#[get("/auth/github")]
pub async fn github_callback(
    token: TokenResponse<GitHub>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    cookies.add_private(
        Cookie::build(("token", token.access_token().to_string().clone()))
            .same_site(SameSite::Lax)
            .http_only(true)
            .secure(true)
            .build(),
    );
    Ok(Redirect::to("/"))
}
