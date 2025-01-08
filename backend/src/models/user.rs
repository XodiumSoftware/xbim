/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::schemas::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

/// Represents a user in the system.
///
/// This struct is used to query user data from the database.
///
/// # Fields
/// - `id`: The unique identifier of the user.
/// - `username`: The username of the user.
/// - `email`: The email address of the user.
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

/// Represents a new user to be inserted into the database.
///
/// This struct is used for creating new user records.
///
/// # Fields
/// - `username`: The username of the new user.
/// - `email`: The email address of the new user.
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

/// Represents the data for updating a user's username.
///
/// This struct is used when a request is made to update the username.
///
/// # Fields
/// - `username`: The new username for the user.
#[derive(Deserialize)]
pub struct UpdateUsername {
    pub username: String,
}

/// Represents the data for updating a user's email address.
///
/// This struct is used when a request is made to update the email address.
///
/// # Fields
/// - `email`: The new email address for the user.
#[derive(Deserialize)]
pub struct UpdateEmail {
    pub email: String,
}
