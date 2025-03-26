/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Uuid,
    Surreal,
};

use crate::constants::{SURREALDB_PASSWORD, SURREALDB_URL, SURREALDB_USERNAME};

/// Represents the database operations.
pub struct Database {
    pub client: Surreal<Client>,
    pub session_token: Uuid,
}

impl Database {
    /// Creates a new `Database` instance.
    ///
    /// # Returns
    /// A new `Database` instance.
    pub async fn new() -> Self {
        let client = Surreal::new::<Ws>(SURREALDB_URL).await.expect(&format!(
            "Failed to connect to SurrealDB at {}",
            SURREALDB_URL
        ));

        client
            .signin(Root {
                username: SURREALDB_USERNAME,
                password: SURREALDB_PASSWORD,
            })
            .await
            .expect("Failed to sign in to SurrealDB");

        Self {
            client,
            session_token: Uuid::new(),
        }
    }

    /// Runs a query on the database.
    ///
    /// # Arguments
    /// * `query` - A string slice that holds the query to be executed.
    ///
    /// # Returns
    /// A `Result` which is `Ok` if the query was successful, or an error if it failed.
    pub async fn run_query(&self, query: &str) -> surrealdb::Result<()> {
        self.client.query(query).await.map(|_| ())
    }
}
