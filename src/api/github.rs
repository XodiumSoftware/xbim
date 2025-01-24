/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

// TEST

use once_cell::sync::OnceCell;
use reqwest::{Client, Error};
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Debug, Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};

static HTTP_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn get_http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("xBIM")
            .build()
            .expect("Failed to initialize HTTP client")
    })
}

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
    let token = token.access_token().to_string();

    cookies.add_private(
        Cookie::build(("token", token.clone()))
            .same_site(SameSite::Lax)
            .http_only(true)
            .secure(true)
            .build(),
    );

    Ok(Redirect::to("/"))
}
