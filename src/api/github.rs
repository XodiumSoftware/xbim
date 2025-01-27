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

/// Represents the GitHub OAuth2 provider.
pub struct GitHub;

/// Initiates the GitHub OAuth2 login process.
///
/// # Errors
///
/// This function will return an error if the OAuth2 redirect URL cannot be generated.
#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHub>, cookies: &CookieJar<'_>) -> Result<Redirect, String> {
    oauth2
        .get_redirect(cookies, &["user:read"])
        .map_err(|e| e.to_string())
}

/// Handles the callback from GitHub OAuth2 login process.
///
/// # Errors
///
/// This function will return an error if the token cannot be retrieved or if there is an issue with the HTTP request.
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
