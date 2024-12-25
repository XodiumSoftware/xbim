#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::api::user::User;
use sqlx::{query, query_as, PgPool};

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: PgPool::connect(url).await?,
        })
    }

    pub async fn add_user(&self, username: &str, email: &str) -> Result<(), sqlx::Error> {
        query!(
            "INSERT INTO users (username, email) VALUES ($1, $2)",
            username,
            email
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        Ok(query_as!(User, "SELECT id, username, email FROM users")
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn delete_user(&self, username: &str) -> Result<(), sqlx::Error> {
        query!("DELETE FROM users WHERE username = $1", username)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
