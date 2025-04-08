/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use reqwest::Client as HttpClient;
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Flash, Redirect};
use rocket::serde::{Deserialize, Serialize};
use rocket_oauth2::{OAuth2, TokenResponse};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[get("/auth/github/login")]
pub fn github_login(oauth2: OAuth2<GitHubUser>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(cookies, &["user:email", "read:user"])
        .unwrap()
}

#[get("/auth/github/callback")]
pub async fn github_callback(
    token: TokenResponse<GitHubUser>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Flash<Redirect>> {
    cookies.add_private(
        Cookie::build((
            "user_session",
            serde_json::to_string::<()>(
                &HttpClient::new()
                    .get("https://api.github.com/user")
                    .header("User-Agent", "xBIM-App")
                    .bearer_auth(token.access_token())
                    .send()
                    .await
                    .map_err(|_| Flash::error(Redirect::to("/"), "Failed to get GitHub user data"))?
                    .json()
                    .await
                    .map_err(|_| {
                        Flash::error(Redirect::to("/"), "Failed to parse GitHub user data")
                    })?,
            )
            .unwrap(),
        ))
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(Duration::from_secs(86400).try_into().unwrap())
        .build(),
    );
    Ok(Redirect::to("/"))
}
