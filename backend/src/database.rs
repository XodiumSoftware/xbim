#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![allow(unused)]

use serde::de::DeserializeOwned;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Encode, Error, FromRow, Pool, Postgres, Type};

/// Represents a database connection handler using a PostgreSQL connection pool.
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    /// Creates a new `Database` instance with a connection pool.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if establishing the database connection fails.
    pub async fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            pool: PgPoolOptions::new().max_connections(5).connect(url).await?,
        })
    }

    /// Executes a SQL query (e.g., INSERT, UPDATE, DELETE) and returns the number of affected rows.
    ///
    /// # Type Parameters
    /// - `A`: The type of each query parameter, implementing `Encode` and `Type`.
    /// - `I`: An iterator of these parameters.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the execution fails.
    pub async fn execute_query<A, I>(&self, sql: &str, args: I) -> Result<u64, Error>
    where
        I: IntoIterator<Item = A>,
        for<'q> A: Encode<'q, Postgres> + Type<Postgres>,
    {
        let mut query = sqlx::query(sql);
        for arg in args {
            query = query.bind(arg);
        }
        Ok(query.execute(&self.pool).await?.rows_affected())
    }

    /// Executes a SQL SELECT query and returns a collection of deserialized results.
    ///
    /// # Type Parameters
    /// - `T`: The return type, implementing `FromRow`, `DeserializeOwned`, `Send`, and `Unpin`.
    /// - `A`: The type of each query parameter, implementing `Encode` and `Type`.
    /// - `I`: An iterator of these parameters.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the query or deserialization fails.
    pub async fn query_data<T, A, I>(&self, sql: &str, args: I) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, PgRow> + DeserializeOwned + Send + Unpin,
        I: IntoIterator<Item = A>,
        for<'q> A: Encode<'q, Postgres> + Type<Postgres>,
    {
        let mut query = sqlx::query_as::<_, T>(sql);
        for arg in args {
            query = query.bind(arg);
        }
        query.fetch_all(&self.pool).await
    }
}
