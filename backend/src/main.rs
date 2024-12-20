#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;
mod database;

use crate::database::{
    Database, DELETE_USER_QUERY, GET_USERS_QUERY, INSERT_USER_QUERY, UPDATE_USER_QUERY,
};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{http::Status, response::status::Custom, State};
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio_postgres::NoTls;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

#[post("/api/users", data = "<user>")]
async fn add_user(
    db_handler: &State<Database>,
    user: Json<User>,
) -> Result<Json<Vec<User>>, Custom<String>> {
    db_handler
        .execute_query(INSERT_USER_QUERY, &[&user.name, &user.email])
        .await?;
    get_users(db_handler).await
}

#[get("/api/users")]
async fn get_users(db_handler: &State<Database>) -> Result<Json<Vec<User>>, Custom<String>> {
    db_handler
        .client
        .query(GET_USERS_QUERY, &[])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
        .map(|rows| {
            Json(
                rows.iter()
                    .map(|row| User {
                        id: Some(row.get(0)),
                        name: row.get(1),
                        email: row.get(2),
                    })
                    .collect(),
            )
        })
}

#[put("/api/users/<id>", data = "<user>")]
async fn update_user(
    db_handler: &State<Database>,
    id: i32,
    user: Json<User>,
) -> Result<Json<Vec<User>>, Custom<String>> {
    db_handler
        .execute_query(UPDATE_USER_QUERY, &[&user.name, &user.email, &id])
        .await?;
    get_users(db_handler).await
}

#[delete("/api/users/<id>")]
async fn delete_user(db_handler: &State<Database>, id: i32) -> Result<Status, Custom<String>> {
    db_handler.execute_query(DELETE_USER_QUERY, &[&id]).await?;
    Ok(Status::NoContent)
}

#[launch]
async fn rocket() -> _ {
    let (client, conn) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    )
    .await
    .expect("Failed to connect to Postgres");

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("Failed to connect to Postgres: {}", e);
        }
    });

    rocket::build()
        .manage(
            Database::new(client)
                .await
                .expect("Failed to initialize the database"),
        )
        .mount("/", routes![add_user, get_users, update_user, delete_user])
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .to_cors()
                .expect("Error while building CORS"),
        )
}
