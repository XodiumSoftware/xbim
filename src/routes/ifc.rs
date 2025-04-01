/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use crate::{
    database::{Database, StoredIfcModel},
    guards::auth::AuthGuard,
    guards::id::IdGuard,
};
use rocket::{get, http::Status, post, serde::json::Json, State};

/// Upload a new IFC model to the database.
///
/// # Arguments
/// * `database` - The database instance.
/// * `idguard` - Identification Guard.
/// * `_authguard` - Authentication Guard.
/// * `model` - The IFC model to upload.
///
/// # Returns
/// The saved IFC model with its ID.
#[post("/ifc", data = "<model>")]
pub async fn upload_ifc_model(
    database: &State<Database>,
    idguard: IdGuard,
    _authguard: AuthGuard,
    model: Json<StoredIfcModel>,
) -> Result<Json<StoredIfcModel>, Status> {
    println!("Processing IFC upload with request ID: {}", idguard.id);
    match database.save_ifc_model(model.into_inner()).await {
        Ok(saved_model) => {
            println!(
                "Successfully saved IFC model with request ID: {}",
                idguard.id
            );
            Ok(Json(saved_model))
        }
        Err(e) => {
            println!(
                "Error saving IFC model with request ID {}: {:?}",
                idguard.id, e
            );
            Err(Status::InternalServerError)
        }
    }
}

/// Get an IFC model by ID.
///
/// # Arguments
/// * `database` - The database instance.
/// * `idguard` - Identification Guard.
/// * `_authguard` - Authentication Guard.
/// * `id` - The ID of the IFC model to retrieve.
///
/// # Returns
/// The retrieved IFC model.
#[get("/ifc/<id>")]
pub async fn get_ifc_model(
    database: &State<Database>,
    idguard: IdGuard,
    _authguard: AuthGuard,
    id: String,
) -> Result<Json<StoredIfcModel>, Status> {
    println!(
        "Retrieving IFC model {} with request ID: {}",
        id, idguard.id
    );
    match database.get_ifc_model(id.clone()).await {
        Ok(model) => {
            println!(
                "Successfully retrieved IFC model {} with request ID: {}",
                id, idguard.id
            );
            Ok(Json(model))
        }
        Err(e) => {
            println!(
                "Error retrieving IFC model {} with request ID {}: {:?}",
                id, idguard.id, e
            );
            Err(Status::NotFound)
        }
    }
}
