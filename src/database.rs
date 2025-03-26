/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use chrono::Utc;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Uuid,
    Surreal,
};

use crate::{
    constants::{SURREALDB_PASSWORD, SURREALDB_URL, SURREALDB_USERNAME},
    schemas::ifc::{IfcModel, IfcModelUpload},
};

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

    pub async fn save_ifc_model(&self, model_upload: IfcModelUpload) -> Result<IfcModel> {
        let now = Utc::now();

        let model = IfcModel {
            id: None,
            name: model_upload.name,
            version: model_upload.version,
            description: model_upload.description,
            created_at: now,
            updated_at: now,
            metadata: model_upload.metadata.unwrap_or_default(),
            file_content: model_upload.file_content,
        };

        let created: IfcModel = self.client.create("ifc_models").content(&model).await?;

        Ok(created)
    }

    /// Retrieves an IFC model from the database by ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the IFC model to retrieve.
    ///
    /// # Returns
    /// The retrieved IFC model.
    pub async fn get_ifc_model(&self, id: &str) -> Result<Option<IfcModel>> {
        let thing: Thing = Thing::from(("ifc_models", id));
        let model = self.client.select(thing).await?;
        Ok(model)
    }

    /// Lists all IFC models in the database.
    ///
    /// # Returns
    /// A vector of IFC models.
    pub async fn list_ifc_models(&self) -> Result<Vec<IfcModel>> {
        let models = self.client.select("ifc_models").await?;
        Ok(models)
    }
}
