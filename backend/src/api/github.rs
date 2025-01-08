/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use reqwest::Client;
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;

pub struct GitHub;

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub id: u64,
    pub name: String,
}

#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHub>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

#[get("/auth/github")]
pub async fn github_callback(token: TokenResponse<GitHub>, cookies: &CookieJar<'_>) -> Redirect {
    let token = token.access_token().to_string();

    cookies.add_private(
        Cookie::build(("token", token.clone()))
            .same_site(SameSite::Lax)
            .http_only(true)
            .secure(true)
            .build(),
    );

    let user_info = Client::new()
        .get("https://api.github.com/user")
        .bearer_auth(&token)
        .header("User-Agent", "xBIM")
        .send()
        .await
        .expect("Failed to send request to GitHub")
        .json::<GitHubUser>()
        .await
        .expect("Failed to parse GitHub user response");

    println!("User ID: {}", user_info.id);
    println!("User Name: {}", user_info.name);

    Redirect::to("/")
}
