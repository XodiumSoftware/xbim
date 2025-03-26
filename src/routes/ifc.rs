/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, State};

use crate::database::Database;
use crate::middlewares::auth::Auth;
use crate::schemas::ifc::{IfcModel, IfcModelUpload};

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
    _auth: Auth,
    model: Json<IfcModelUpload>,
) -> Result<Json<IfcModel>, Status> {
    match db.save_ifc_model(model.into_inner()).await {
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
    _auth: Auth,
    id: String,
) -> Result<Json<IfcModel>, Status> {
    match db.get_ifc_model(&id).await {
        Ok(Some(model)) => Ok(Json(model)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// List all IFC models.
///
/// # Arguments
/// * `db` - The database instance.
/// * `_auth` - Authentication guard.
///
/// # Returns
/// A list of all IFC models.
#[get("/ifc")]
pub async fn list_ifc_models(
    db: &State<Database>,
    _auth: Auth,
) -> Result<Json<Vec<IfcModel>>, Status> {
    match db.list_ifc_models().await {
        Ok(models) => Ok(Json(models)),
        Err(_) => Err(Status::InternalServerError),
    }
}
