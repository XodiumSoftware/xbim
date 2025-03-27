/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, State};

use crate::database::{Database, StoredIfcModel};
use crate::middlewares::authentication::Authenticator;

/// Upload a new IFC model to the database.
///
/// # Arguments
/// * `db` - The database instance.
/// * `_auth` - Authentication guard.
/// * `model` - The IFC model to upload.
///
/// # Returns
/// The saved IFC model with its ID.
#[post("/ifc", data = "<model>")]
pub async fn upload_ifc_model(
    db: &State<Database>,
    _auth: Authenticator,
    model: Json<StoredIfcModel>,
) -> Result<Json<StoredIfcModel>, Status> {
    match db.save_ifc_model(model.into_inner().into()).await {
        Ok(saved_model) => Ok(Json(saved_model)),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Get an IFC model by ID.
///
/// # Arguments
/// * `db` - The database instance.
/// * `_auth` - Authentication guard.
/// * `id` - The ID of the IFC model to retrieve.
///
/// # Returns
/// The retrieved IFC model.
#[get("/ifc/<id>")]
pub async fn get_ifc_model(
    db: &State<Database>,
    _auth: Authenticator,
    id: String,
) -> Result<Json<StoredIfcModel>, Status> {
    match db.get_ifc_model(id).await {
        Ok(model) => Ok(Json(model)),
        Err(_) => Err(Status::NotFound),
    }
}
