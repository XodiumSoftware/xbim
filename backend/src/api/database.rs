/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use log::{error, info};
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Value;
use surrealdb::Surreal;

/// Represents the database operations.
pub struct Database {
    pub client: Surreal<Client>,
}

impl Database {
    /// Creates a new `Database` by establishing an asynchronous connection to SurrealDB.
    ///
    /// # Returns
    /// A `Database` with a ready-to-use `Surreal<Client>`.
    pub async fn new() -> Self {
        let client = Surreal::new("localhost:8000")
            .await
            .expect("Failed to create SurrealDB client");

        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .expect("Failed to sign in to SurrealDB");

        client
            .use_ns("namespace_name")
            .use_db("database_name")
            .await
            .expect("Failed to select namespace/database");

        Database { client }
    }

    /// Creates a new data entry in the database.
    ///
    /// # Arguments
    ///
    /// * `k` - The key for the data entry.
    /// * `v` - The value for the data entry.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of records inserted.
    pub async fn create_data(&self, k: String, v: String) -> Result<(), surrealdb::Error> {
        match self
            .client
            .query("CREATE data SET key = $key, value = $value")
            .bind(("key", k))
            .bind(("value", v))
            .await
        {
            Ok(_) => {
                info!("Successfully inserted record.");
                Ok(())
            }
            Err(e) => {
                error!("Error inserting data: {:?}", e);
                Err(e)
            }
        }
    }

    /// Reads a data entry from the database by key.
    ///
    /// # Arguments
    ///
    /// * `k` - The key for the data entry.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the value if found, or `None` if not found.
    pub async fn read_data(&self, k: String) -> Result<Option<String>, surrealdb::Error> {
        match self
            .client
            .query("SELECT value FROM data WHERE key = $key")
            .bind(("key", k))
            .await
        {
            Ok(result) => {
                if let Some(record) = result.iterator().next() {
                    if let Some(Value::Str(val)) = record.get("value") {
                        info!("Successfully read data for key '{}': {}", k, val);
                        return Ok(Some(val.to_string()));
                    }
                }
                info!("No data found for key '{}'", k);
                Ok(None)
            }
            Err(e) => {
                error!("Error reading data for key '{}': {:?}", k, e);
                Err(e)
            }
        }
    }

    /// Updates a data entry in the database by key.
    ///
    /// # Arguments
    ///
    /// * `k` - The key for the data entry.
    /// * `v` - The new value for the data entry.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of records updated.
    pub async fn update_data(&self, k: String, v: String) -> Result<(), surrealdb::Error> {
        match self
            .client
            .query("UPDATE data SET value = $value WHERE key = $key")
            .bind(("key", k))
            .bind(("value", v))
            .await
        {
            Ok(_) => {
                info!("Successfully updated record.");
                Ok(())
            }
            Err(e) => {
                error!("Error updating data: {:?}", e);
                Err(e)
            }
        }
    }

    /// Deletes a data entry from the database by key.
    ///
    /// # Arguments
    ///
    /// * `k` - The key for the data entry.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of records deleted.
    pub async fn delete_data(&self, k: String) -> Result<(), surrealdb::Error> {
        match self
            .client
            .query("DELETE FROM data WHERE key = $key")
            .bind(("key", k))
            .await
        {
            Ok(_) => {
                info!("Successfully deleted record.");
                Ok(())
            }
            Err(e) => {
                error!("Error deleting data: {:?}", e);
                Err(e)
            }
        }
    }
}
