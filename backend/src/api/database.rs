/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::api::schema::data::dsl::data;
use crate::api::schema::data::{key, value};
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, QueryResult,
    RunQueryDsl,
};
use log::{error, info};

/// Represents the database operations.
pub struct Database;

impl Database {
    /// Creates a new data entry in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the PostgreSQL connection.
    /// * `k` - The key for the data entry.
    /// * `v` - The value for the data entry.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the number of records inserted.
    pub fn create_data(&self, conn: &mut PgConnection, k: &str, v: &str) -> QueryResult<usize> {
        match diesel::insert_into(data)
            .values((key.eq(k), value.eq(v)))
            .execute(conn)
        {
            Ok(count) => {
                info!("Successfully inserted {} record(s).", count);
                Ok(count)
            }
            Err(e) => {
                error!("Error inserting data: {:?}", e);
                Err(e)
            }
        }
    }

    /// Reads a data entry from the database by key.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the PostgreSQL connection.
    /// * `k` - The key for the data entry.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing an `Option` with the value if found, or `None` if not found.
    pub fn read_data(&self, conn: &mut PgConnection, k: &str) -> QueryResult<Option<String>> {
        match data
            .filter(key.eq(k))
            .select(value)
            .first::<String>(conn)
            .optional()
        {
            Ok(result) => {
                if let Some(ref val) = result {
                    info!("Successfully read data for key '{}': {}", k, val);
                } else {
                    info!("No data found for key '{}'", k);
                }
                Ok(result)
            }
            Err(e) => {
                error!("Error reading data for key '{}': {:?}", k, e);
                Err(e)
            }
        }
    }

    /// Updates a data entry in the database by key.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the PostgreSQL connection.
    /// * `k` - The key for the data entry.
    /// * `v` - The new value for the data entry.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the number of records updated.
    pub fn update_data(&self, conn: &mut PgConnection, k: &str, v: &str) -> QueryResult<usize> {
        match diesel::update(data.filter(key.eq(k)))
            .set(value.eq(v))
            .execute(conn)
        {
            Ok(count) => {
                info!("Successfully updated {} record(s) for key '{}'.", count, k);
                Ok(count)
            }
            Err(e) => {
                error!("Error updating data for key '{}': {:?}", k, e);
                Err(e)
            }
        }
    }

    /// Deletes a data entry from the database by key.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the PostgreSQL connection.
    /// * `k` - The key for the data entry.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the number of records deleted.
    pub fn delete_data(&self, conn: &mut PgConnection, k: &str) -> QueryResult<usize> {
        match diesel::delete(data.filter(key.eq(k))).execute(conn) {
            Ok(count) => {
                info!("Successfully deleted {} record(s) for key '{}'.", count, k);
                Ok(count)
            }
            Err(e) => {
                error!("Error deleting data for key '{}': {:?}", k, e);
                Err(e)
            }
        }
    }

    /// Lists all data entries in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the PostgreSQL connection.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing a vector of tuples with the key and value of each data entry.
    pub fn list_all_data(&self, conn: &mut PgConnection) -> QueryResult<Vec<(String, String)>> {
        match data.select((key, value)).load::<(String, String)>(conn) {
            Ok(results) => {
                info!("Successfully listed all data.");
                Ok(results)
            }
            Err(e) => {
                error!("Error listing all data: {:?}", e);
                Err(e)
            }
        }
    }

    /// Sets multiple data entries in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the PostgreSQL connection.
    /// * `entries` - A vector of tuples containing the key and value for each data entry.
    ///
    /// # Returns
    ///
    /// A `QueryResult` indicating success or failure.
    pub fn set_multiple_data(
        &self,
        conn: &mut PgConnection,
        entries: Vec<(&str, &str)>,
    ) -> QueryResult<()> {
        match conn.transaction::<_, diesel::result::Error, _>(|| {
            for (k, v) in entries {
                diesel::insert_into(data)
                    .values((key.eq(k), value.eq(v)))
                    .execute(conn)?;
            }
            Ok(())
        }) {
            Ok(_) => {
                info!("Successfully set multiple data entries.");
                Ok(())
            }
            Err(e) => {
                error!("Error setting multiple data entries: {:?}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use std::env;

    fn establish_connection() -> PgConnection {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    #[test]
    fn test_create_data() {
        let conn = &mut establish_connection();
        let db = Database;
        let result = db.create_data(conn, "test_key", "test_value");
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_data() {
        let conn = &mut establish_connection();
        let db = Database;
        db.create_data(conn, "test_key", "test_value").unwrap();
        let result = db.read_data(conn, "test_key");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("test_value".to_string()));
    }

    #[test]
    fn test_update_data() {
        let conn = &mut establish_connection();
        let db = Database;
        db.create_data(conn, "test_key", "test_value").unwrap();
        let result = db.update_data(conn, "test_key", "new_value");
        assert!(result.is_ok());
        let updated_value = db.read_data(conn, "test_key").unwrap();
        assert_eq!(updated_value, Some("new_value".to_string()));
    }

    #[test]
    fn test_delete_data() {
        let conn = &mut establish_connection();
        let db = Database;
        db.create_data(conn, "test_key", "test_value").unwrap();
        let result = db.delete_data(conn, "test_key");
        assert!(result.is_ok());
        let deleted_value = db.read_data(conn, "test_key").unwrap();
        assert_eq!(deleted_value, None);
    }

    #[test]
    fn test_list_all_data() {
        let conn = &mut establish_connection();
        let db = Database;
        db.create_data(conn, "key1", "value1").unwrap();
        db.create_data(conn, "key2", "value2").unwrap();
        let result = db.list_all_data(conn);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.contains(&("key1".to_string(), "value1".to_string())));
        assert!(data.contains(&("key2".to_string(), "value2".to_string())));
    }

    #[test]
    fn test_set_multiple_data() {
        let conn = &mut establish_connection();
        let db = Database;
        let entries = vec![("key1", "value1"), ("key2", "value2")];
        let result = db.set_multiple_data(conn, entries);
        assert!(result.is_ok());
        let data = db.list_all_data(conn).unwrap();
        assert!(data.contains(&("key1".to_string(), "value1".to_string())));
        assert!(data.contains(&("key2".to_string(), "value2".to_string())));
    }
}
