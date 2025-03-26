/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    error::Api,
    opt::auth::Root,
    sql::Uuid,
    Error, Surreal,
};

use crate::constants::{SURREALDB_PASSWORD, SURREALDB_URL, SURREALDB_USERNAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredIfcModel {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub file_content: Option<String>,
}

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
    async fn run_query(&self, query: &str) -> surrealdb::Result<()> {
        self.client.query(query).await.map(|_| ())
    }

    /// Saves an IFC model to the database.
    ///
    /// # Arguments
    /// * `model` - The IFC model upload data.
    ///
    /// # Returns
    /// A `Result` containing the saved model with its ID.
    pub async fn save_ifc_model(&self, model: StoredIfcModel) -> surrealdb::Result<StoredIfcModel> {
        let now = Utc::now();
        let stored_model = StoredIfcModel {
            id: None,
            name: model.name,
            version: model.version,
            description: model.description,
            created_at: now,
            updated_at: now,
            metadata: model.metadata,
            file_content: model.file_content,
        };
        self.client
            .create("ifc_models")
            .content(stored_model)
            .await?
            .take()
            .ok_or_else(|| {
                Error::Api(Api::ParseError(String::from(
                    "Failed to retrieve created IFC model",
                )))
            })
    }
}
