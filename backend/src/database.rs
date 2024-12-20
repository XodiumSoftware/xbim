#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use rocket::{http::Status, response::status::Custom};
use tokio_postgres::{Client, Error as PostgresError};

pub const CREATE_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL
)";
pub const INSERT_USER_QUERY: &str = "INSERT INTO users (name, email) VALUES ($1, $2)";
pub const GET_USERS_QUERY: &str = "SELECT id, name, email FROM users";
pub const UPDATE_USER_QUERY: &str = "UPDATE users SET name = $1, email = $2 WHERE id = $3";
pub const DELETE_USER_QUERY: &str = "DELETE FROM users WHERE id = $1";

pub struct Database {
    pub(crate) client: Client,
}

impl Database {
    pub async fn new(client: Client) -> Result<Self, PostgresError> {
        client.execute(CREATE_TABLE_QUERY, &[]).await?;
        Ok(Self { client })
    }

    pub async fn exec(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Custom<String>> {
        self.client
            .execute(query, params)
            .await
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
    }
}
