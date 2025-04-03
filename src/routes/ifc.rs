/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */
use crate::guards::ratelimit::RateLimitGuard;
use crate::{database::Database, guards::auth::AuthGuard, guards::id::IdGuard};
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, http::Status, post, put, serde::json::Json, State};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredIFC {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub file_content: Option<String>,
}

/// Upload a new IFC model to the database.
///
/// # Arguments
/// * `database` - The database instance.
/// * `idguard` - Identification Guard.
/// * `_authguard` - Authentication Guard.
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `model` - The IFC model to upload.
///
/// # Returns
/// The saved IFC model with its ID.
#[post("/ifc", data = "<model>")]
pub async fn upload_ifc(
    database: &State<Database>,
    idguard: IdGuard,
    _authguard: AuthGuard,
    _ratelimitguard: RateLimitGuard,
    model: Json<StoredIFC>,
) -> Result<Json<StoredIFC>, Status> {
    println!("Processing IFC upload with request ID: {}", idguard.id);
    match database.create("ifc_models", model.into_inner()).await {
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
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `id` - The ID of the IFC model to retrieve.
///
/// # Returns
/// The retrieved IFC model.
#[get("/ifc/<id>")]
pub async fn get_ifc(
    database: &State<Database>,
    idguard: IdGuard,
    _authguard: AuthGuard,
    _ratelimitguard: RateLimitGuard,
    id: String,
) -> Result<Json<StoredIFC>, Status> {
    println!(
        "Retrieving IFC model {} with request ID: {}",
        id, idguard.id
    );
    match database.read::<StoredIFC>("ifc_models", &id).await {
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

/// Update an existing IFC model.
///
/// # Arguments
/// * `database` - The database instance.
/// * `idguard` - Identification Guard.
/// * `_authguard` - Authentication Guard.
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `id` - The ID of the IFC model to update.
/// * `model` - The updated IFC model data.
///
/// # Returns
/// The updated IFC model.
#[put("/ifc/<id>", data = "<model>")]
pub async fn update_ifc(
    database: &State<Database>,
    idguard: IdGuard,
    _authguard: AuthGuard,
    _ratelimitguard: RateLimitGuard,
    id: String,
    model: Json<StoredIFC>,
) -> Result<Json<StoredIFC>, Status> {
    println!("Updating IFC model {} with request ID: {}", id, idguard.id);
    match database.update("ifc_models", &id, model.into_inner()).await {
        Ok(updated_model) => {
            println!(
                "Successfully updated IFC model {} with request ID: {}",
                id, idguard.id
            );
            Ok(Json(updated_model))
        }
        Err(e) => {
            println!(
                "Error updating IFC model {} with request ID {}: {:?}",
                id, idguard.id, e
            );
            Err(Status::InternalServerError)
        }
    }
}

/// Delete an IFC model by ID.
///
/// # Arguments
/// * `database` - The database instance.
/// * `idguard` - Identification Guard.
/// * `_authguard` - Authentication Guard.
/// * `_ratelimitguard` - Rate Limit Guard.
/// * `id` - The ID of the IFC model to delete.
///
/// # Returns
/// 204 No Content on success, error status otherwise.
#[delete("/ifc/<id>")]
pub async fn delete_ifc(
    database: &State<Database>,
    idguard: IdGuard,
    _authguard: AuthGuard,
    _ratelimitguard: RateLimitGuard,
    id: String,
) -> Status {
    println!("Deleting IFC model {} with request ID: {}", id, idguard.id);
    match database.delete::<StoredIFC>("ifc_models", &id).await {
        Ok(true) => {
            println!(
                "Successfully deleted IFC model {} with request ID: {}",
                id, idguard.id
            );
            Status::NoContent
        }
        Ok(false) => {
            println!(
                "IFC model {} not found for deletion, request ID: {}",
                id, idguard.id
            );
            Status::NotFound
        }
        Err(e) => {
            println!(
                "Error deleting IFC model {} with request ID {}: {:?}",
                id, idguard.id, e
            );
            Status::InternalServerError
        }
    }
}
