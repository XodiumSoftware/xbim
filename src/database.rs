/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::config::Config;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::error::Api;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Uuid,
    Error, Surreal,
};

pub struct Database {
    pub client: Surreal<Client>,
    pub session_token: Uuid,
}

impl Database {
    /// Creates a new `Database` instance.
    ///
    /// # Arguments
    /// * `config` - The application configuration.
    ///
    /// # Returns
    /// A new `Database` instance.
    pub async fn new(config: &Config) -> Self {
        let client = Surreal::new::<Ws>(&config.database_url)
            .await
            .unwrap_or_else(|_| {
                panic!("Failed to connect to SurrealDB at {}", config.database_url)
            });

        client
            .signin(Root {
                username: &config.database_username,
                password: &config.database_password,
            })
            .await
            .expect("Failed to sign in to SurrealDB");

        Self {
            client,
            session_token: Uuid::new(),
        }
    }

    /// Creates a new record in the specified table.
    ///
    /// # Arguments
    /// * `table` - The table name to create the record in.
    /// * `data` - The data to create.
    ///
    /// # Returns
    /// A `Result` containing the created record with its ID.
    pub async fn create<T>(&self, table: &str, data: T) -> Result<T, Error>
    where
        T: Serialize + for<'a> Deserialize<'a> + 'static,
    {
        self.client
            .create(table)
            .content(data)
            .await?
            .take()
            .ok_or_else(|| Error::Api(Api::ParseError(String::from("Failed to create record"))))
    }

    /// Retrieves a record from the specified table by its ID.
    ///
    /// # Arguments
    /// * `table` - The table name to retrieve from.
    /// * `id` - The ID of the record to retrieve.
    ///
    /// # Returns
    /// A `Result` containing the retrieved record.
    pub async fn read<T>(&self, table: &str, id: &str) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a> + 'static,
    {
        self.client
            .select((table, id))
            .await?
            .take()
            .ok_or_else(|| Error::Api(Api::ParseError(String::from("Failed to retrieve record"))))
    }

    /// Updates a record in the specified table.
    ///
    /// # Arguments
    /// * `table` - The table name where the record is stored.
    /// * `id` - The ID of the record to update.
    /// * `data` - The updated data.
    ///
    /// # Returns
    /// A `Result` containing the updated record.
    pub async fn update<T>(&self, table: &str, id: &str, data: T) -> Result<T, Error>
    where
        T: Serialize + for<'a> Deserialize<'a> + 'static,
    {
        self.client
            .update((table, id))
            .content(data)
            .await?
            .take()
            .ok_or_else(|| Error::Api(Api::ParseError(String::from("Failed to update record"))))
    }

    /// Deletes a record from the specified table.
    ///
    /// # Arguments
    /// * `table` - The table name where the record is stored.
    /// * `id` - The ID of the record to delete.
    ///
    /// # Returns
    /// A `Result` indicating whether the deletion was successful.
    pub async fn delete<T>(&self, table: &str, id: &str) -> Result<bool, Error>
    where
        T: for<'a> Deserialize<'a> + 'static,
    {
        let result: Option<T> = self.client.delete((table, id)).await?.take();
        Ok(result.is_some())
    }
}
