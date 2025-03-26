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

    /// Runs a query on the database and returns the result.
    ///
    /// # Arguments
    /// * `query` - A string slice that holds the query to be executed.
    ///
    /// # Returns
    /// A `Result` which contains the query response if successful, or an error if it failed.
    pub async fn run_query_with_result<T>(&self, query: &str) -> surrealdb::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.client.query(query).await?;
        let processed = response.check()?;
        processed.take::<T>(0)
    }

    pub async fn save_ifc_model(
        &self,
        model_upload: IfcModelUpload,
    ) -> surrealdb::Result<IfcModel> {
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

        let model_json = serde_json::to_string(&model).map_err(|e| {
            surrealdb::Error::Db(surrealdb::error::Db::Serializing {
                value: "model".into(),
                message: e.to_string(),
            })
        })?;

        let query = format!("CREATE ifc_models CONTENT {};", model_json);

        let created: IfcModel = self.run_query_with_result(&query).await?;
        Ok(created)
    }

    /// Retrieves an IFC model from the database by ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the IFC model to retrieve.
    ///
    /// # Returns
    /// The retrieved IFC model.
    pub async fn get_ifc_model(&self, id: &str) -> surrealdb::Result<Option<IfcModel>> {
        let query = format!("SELECT * FROM ifc_models WHERE id = 'ifc_models:{}'", id);
        let models: Vec<IfcModel> = self.run_query_with_result(&query).await?;
        Ok(models.into_iter().next())
    }

    /// Lists all IFC models in the database.
    ///
    /// # Returns
    /// A vector of IFC models.
    pub async fn list_ifc_models(&self) -> surrealdb::Result<Vec<IfcModel>> {
        let query = "SELECT * FROM ifc_models";
        let models: Vec<IfcModel> = self.run_query_with_result(&query).await?;
        Ok(models)
    }
}
