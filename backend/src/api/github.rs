/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::api::local_storage::LocalStorage;
use reqwest::{Client, Error};
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Debug, Redirect};
use rocket::serde::json::serde_json::to_string;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};

pub struct GitHub;

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubUser {
    pub id: u64,
    pub name: String,
}

#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHub>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
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

    let response = Client::new()
        .get("https://api.github.com/user")
        .bearer_auth(&token)
        .header("User-Agent", "xBIM")
        .send()
        .await
        .map_err(Debug)?;

    let github_user: GitHubUser = response.json().await.map_err(Debug)?;
    let github_user_json = to_string(&github_user).map_err(Debug)?;
    LocalStorage::set_item("github_user", &github_user_json).unwrap();

    Ok(Redirect::to("/"))
}
