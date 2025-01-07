/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::models::user::*;
use crate::schemas::users;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::serde::json::Json;
use rocket::{delete, post, put, State};

type DbPool = Pool<ConnectionManager<PgConnection>>;

fn get_db_conn(
    db: &State<DbPool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
    db.get()
        .map_err(|_| "Failed to get DB connection".to_string())
}

#[post("/user", data = "<data>")]
pub async fn add_user(data: Json<NewUser>, db: &State<DbPool>) -> Result<Json<User>, String> {
    diesel::insert_into(users::table)
        .values(&*data)
        .get_result(&mut get_db_conn(db)?)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[delete("/user/<id>")]
pub async fn remove_user(id: i32, db: &State<DbPool>) -> Json<Result<String, String>> {
    let mut conn = match get_db_conn(db) {
        Ok(conn) => conn,
        Err(e) => return Json(Err(e)),
    };

    match diesel::delete(users::table.filter(users::id.eq(id))).execute(&mut conn) {
        Ok(num_deleted) => {
            if num_deleted == 0 {
                Json(Err("User not found".to_string()))
            } else {
                Json(Ok("User successfully deleted".to_string()))
            }
        }
        Err(e) => Json(Err(e.to_string())),
    }
}

#[put("/user/<id>/username", data = "<data>")]
pub async fn update_username(
    id: i32,
    data: Json<UpdateUsername>,
    db: &State<DbPool>,
) -> Json<Result<String, String>> {
    let mut conn = match get_db_connection(db) {
        Ok(conn) => conn,
        Err(e) => return Json(Err(e)),
    };

    // Check if the new username already exists
    let existing_user = users::table
        .filter(users::username.eq(&data.username))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|e| e.to_string())?;

    if existing_user.is_some() {
        return Json(Err("Username already exists".to_string()));
    }

    match diesel::update(users::table.filter(users::id.eq(id)))
        .set(users::username.eq(&data.username))
        .execute(&mut conn)
    {
        Ok(num_updated) => {
            if num_updated == 0 {
                Json(Err("User not found".to_string()))
            } else {
                Json(Ok("Username successfully updated".to_string()))
            }
        }
        Err(e) => Json(Err(e.to_string())),
    }
}

#[put("/user/<id>/email", data = "<data>")]
pub async fn update_email(
    id: i32,
    data: Json<UpdateEmail>,
    db: &State<DbPool>,
) -> Json<Result<String, String>> {
    let mut conn = match get_db_conn(db) {
        Ok(conn) => conn,
        Err(e) => return Json(Err(e)),
    };

    // Perform email confirmation check (pseudo-code, implement as needed)
    if !confirm_email_update(&data.email) {
        return Json(Err("Email confirmation failed".to_string()));
    }

    match diesel::update(users::table.filter(users::id.eq(id)))
        .set(users::email.eq(&data.email))
        .execute(&mut conn)
    {
        Ok(num_updated) => {
            if num_updated == 0 {
                Json(Err("User not found".to_string()))
            } else {
                Json(Ok("Email successfully updated".to_string()))
            }
        }
        Err(e) => Json(Err(e.to_string())),
    }
}

// Pseudo function for email confirmation (implement the actual logic as needed)
fn confirm_email_update(email: &str) -> bool {
    // Implement the email confirmation logic here
    true
}
