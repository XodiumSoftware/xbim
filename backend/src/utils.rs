/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::models::user::User;
use crate::schemas::users;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::http::Status;
use rocket::State;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Utility struct for common helper methods used across the application.
pub struct Utils;

impl Utils {
    ///
    /// Retrieves a database connection from the connection pool.
    ///
    /// # Parameters
    /// * `db`: A reference to the `DbPool` managed by Rocket's `State`.
    ///
    /// # Returns
    /// * `Result` with a `PooledConnection` on success, or an `(InternalServerError, String)` on failure.
    ///
    pub fn get_db_conn(
        db: &State<DbPool>,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, (Status, String)> {
        Result::map_err(db.get(), |e| (Status::InternalServerError, e.to_string()))
    }

    ///
    /// Checks if a user with the given username exists in the database.
    ///
    /// # Parameters
    /// * `conn`: A mutable reference to a `PgConnection` to interact with the database.
    /// * `username`: The username to search for in the `users` table.
    ///
    /// # Returns
    /// * `Ok(true)` if the user exists.
    /// * `Ok(false)` if the user does not exist.
    /// * `Err` with an `InternalServerError` status and error message if there is an issue with the database query.
    ///
    pub fn lookup_user(conn: &mut PgConnection, username: &str) -> Result<bool, (Status, String)> {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .optional()
            .map(|user| user.is_some())
            .map_err(|e| (Status::InternalServerError, e.to_string()))
    }
}
