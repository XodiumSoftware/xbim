#![warn(clippy::all)]
#![forbid(unsafe_code)]

use crate::models::user::User;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    pub id: Option<Thing>,
    pub thumbnail: Option<String>,
    pub title: String,
    pub author: User,
    pub description: String,
    pub platform: String,
    pub downloads: u32,
    pub rating: f32,
    pub last_updated: f64,
}
