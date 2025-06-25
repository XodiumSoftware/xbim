/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub thumbnail: Option<String>,
    pub title: String,
    pub author: String,
    pub description: String,
    pub platform: String,
    pub downloads: u32,
    pub rating: f32,
    pub last_updated: f64,
}
