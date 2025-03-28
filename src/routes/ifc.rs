/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use crate::database::{Database, StoredIfcModel};
use crate::middlewares::authentication::RAG;
use crate::middlewares::identification::RIG;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, State};

/// Upload a new IFC model to the database.
///
/// # Arguments
/// * `db` - The database instance.
/// * `rig` - Request Identification Guard.
/// * `_rag` - Request Authentication Guard.
/// * `model` - The IFC model to upload.
///
/// # Returns
/// The saved IFC model with its ID.
#[post("/ifc", data = "<model>")]
pub async fn upload_ifc_model(
    db: &State<Database>,
    rig: RIG,
    _rag: RAG,
    model: Json<StoredIfcModel>,
) -> Result<Json<StoredIfcModel>, Status> {
    println!("Processing IFC upload with request ID: {}", rig.0);
    match db.save_ifc_model(model.into_inner().into()).await {
        Ok(saved_model) => {
            println!("Successfully saved IFC model with request ID: {}", rig.0);
            Ok(Json(saved_model))
        }
        Err(e) => {
            println!("Error saving IFC model with request ID {}: {:?}", rig.0, e);
            Err(Status::InternalServerError)
        }
    }
}

/// Get an IFC model by ID.
///
/// # Arguments
/// * `db` - The database instance.
/// * `rig` - Request Identification Guard.
/// * `_rag` - Request Authentication Guard.
/// * `id` - The ID of the IFC model to retrieve.
///
/// # Returns
/// The retrieved IFC model.
#[get("/ifc/<id>")]
pub async fn get_ifc_model(
    db: &State<Database>,
    rig: RIG,
    _rag: RAG,
    id: String,
) -> Result<Json<StoredIfcModel>, Status> {
    println!("Retrieving IFC model {} with request ID: {}", id, rig.0);
    match db.get_ifc_model(id.clone()).await {
        Ok(model) => {
            println!(
                "Successfully retrieved IFC model {} with request ID: {}",
                id, rig.0
            );
            Ok(Json(model))
        }
        Err(e) => {
            println!(
                "Error retrieving IFC model {} with request ID {}: {:?}",
                id, rig.0, e
            );
            Err(Status::NotFound)
        }
    }
}
