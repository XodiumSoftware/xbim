/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::models::user::{NewUser, User};
use crate::schemas::users;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::serde::json::Json;
use rocket::{delete, post, State};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[post("/user", data = "<user>")]
pub async fn add_user(user: Json<NewUser>, db: &State<DbPool>) -> Result<Json<User>, String> {
    db.get()
        .map_err(|_| "Failed to get DB connection".to_string())
        .and_then(|mut conn| {
            diesel::insert_into(users::table)
                .values(&*user)
                .get_result(&mut conn)
                .map(Json)
                .map_err(|e| e.to_string())
        })
}

#[delete("/user/<id>")]
pub async fn remove_user(id: i32, db: &State<DbPool>) -> Json<Result<String, String>> {
    match db.get() {
        Ok(mut conn) => {
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
        Err(_) => Json(Err("Failed to get DB connection".to_string())),
    }
}
