/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::models::user::*;
use crate::schemas::users;
use crate::utils::{DbPool, Utils};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, post, put, State};

///
/// Adds a new user to the database.
///
/// # Parameters
/// * `data`: A JSON representation of the `NewUser` struct, containing the details of the user to be added.
/// * `db`: A reference to the `DbPool` state, used to establish a database connection.
///
/// # Returns
/// * `Ok(Json<User>)`: A JSON response containing the newly added `User` on success.
/// * `Err((Status, String))`: An error response with an HTTP status and error message if the operation fails.
///
/// # Errors
/// * `Status::InternalServerError` if there is an issue with the database operation or connection.
///
#[post("/user", data = "<data>")]
pub async fn add_user(
    data: Json<NewUser>,
    db: &State<DbPool>,
) -> Result<Json<User>, (Status, String)> {
    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&mut Utils::get_db_conn(db)?)
        .map(Json)
        .map_err(|e| (Status::InternalServerError, e.to_string()))
}

///
/// Removes an existing user from the database.
///
/// # Parameters
/// * `id`: The ID of the user to be removed.
/// * `db`: A reference to the `DbPool` state, used to establish a database connection.
///
/// # Returns
/// * `Ok(Json<String>)`: A JSON response with a success message if the user was successfully deleted.
/// * `Err((Status, String))`: An error response with an HTTP status and error message if the operation fails.
///
/// # Errors
/// * `Status::NotFound` if no user with the specified ID exists in the database.
/// * `Status::InternalServerError` if there is an issue with the database operation or connection.
#[delete("/user/<id>")]
pub async fn remove_user(id: i32, db: &State<DbPool>) -> Result<Json<String>, (Status, String)> {
    match diesel::delete(users::table.filter(users::id.eq(id)))
        .execute(&mut Utils::get_db_conn(db)?)
    {
        Ok(0) => Err((Status::NotFound, "User not found".to_string())),
        Ok(_) => Ok(Json("User successfully deleted".to_string())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

///
/// Updates the username of an existing user in the database.
///
/// # Parameters
/// * `id`: The ID of the user whose username is to be updated.
/// * `data`: A JSON representation of the `UpdateUsername` struct, containing the new username.
/// * `db`: A reference to the `DbPool` state, used to establish a database connection.
///
/// # Returns
/// * `Ok(Json<String>)`: A JSON response with a success message if the username was successfully updated.
/// * `Err((Status, String))`: An error response with an HTTP status and error message if the operation fails.
///
/// # Errors
/// * `Status::Conflict` if the new username already exists in the database.
/// * `Status::NotFound` if no user with the specified ID exists in the database.
/// * `Status::InternalServerError` if there is an issue with the database operation or connection.
#[put("/user/<id>/username", data = "<data>")]
pub async fn update_username(
    id: i32,
    data: Json<UpdateUsername>,
    db: &State<DbPool>,
) -> Result<Json<String>, (Status, String)> {
    let mut conn = Utils::get_db_conn(db)?;

    if Utils::lookup_user(&mut conn, &data.username)? {
        return Err((Status::Conflict, "Username already exists".to_string()));
    }

    match diesel::update(users::table.filter(users::id.eq(id)))
        .set(users::username.eq(&data.username))
        .execute(&mut conn)
    {
        Ok(0) => Err((Status::NotFound, "User not found".to_string())),
        Ok(_) => Ok(Json("Username successfully updated".to_string())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

///
/// Updates the email of an existing user in the database.
///
/// # Parameters
/// * `id`: The ID of the user whose email is to be updated.
/// * `data`: A JSON representation of the `UpdateEmail` struct, containing the new email.
/// * `db`: A reference to the `DbPool` state, used to establish a database connection.
///
/// # Returns
/// * `Ok(Json<String>)`: A JSON response with a success message if the email was successfully updated.
/// * `Err((Status, String))`: An error response with an HTTP status and error message if the operation fails.
///
/// # Errors
/// * `Status::NotFound` if no user with the specified ID exists in the database.
/// * `Status::InternalServerError` if there is an issue with the database operation or connection.
#[put("/user/<id>/email", data = "<data>")]
pub async fn update_email(
    id: i32,
    data: Json<UpdateEmail>,
    db: &State<DbPool>,
) -> Result<Json<String>, (Status, String)> {
    match diesel::update(users::table.filter(users::id.eq(id)))
        .set(users::email.eq(&data.email))
        .execute(&mut Utils::get_db_conn(db)?)
    {
        Ok(0) => Err((Status::NotFound, "User not found".to_string())),
        Ok(_) => Ok(Json("Email successfully updated".to_string())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}
