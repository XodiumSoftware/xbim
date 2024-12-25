#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(FromRow)]
pub struct UserPreferences {
    pub id: i32,
    pub user_id: i32,
    pub theme: String,
}
