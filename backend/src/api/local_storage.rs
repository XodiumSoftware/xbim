/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ Copyright (c) 2025. Xodium.
+ All rights reserved.
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use thiserror::Error;
use web_sys::{window, Storage};

/// Custom error type for local storage operations.
#[derive(Debug, Error)]
pub enum LocalStorageError {
    #[error("No window object available")]
    NoWindow,
    #[error("Failed to access local storage: {0}")]
    AccessError(String),
    #[error("Local storage is not available")]
    NotAvailable,
    #[error("Failed to set item: {0}")]
    SetItemError(String),
    #[error("Failed to get item: {0}")]
    GetItemError(String),
}

/// Type alias for local storage results.
type LocalStorageResult<T> = Result<T, LocalStorageError>;

/// A struct representing local storage functionality.
pub struct LocalStorage;
impl LocalStorage {
    /// Retrieves the local storage.
    ///
    /// # Returns
    /// A result containing the local storage or an error.
    fn get_storage() -> LocalStorageResult<Storage> {
        window()
            .ok_or(LocalStorageError::NoWindow)?
            .local_storage()
            .map_err(|e| {
                LocalStorageError::AccessError(
                    e.as_string().unwrap_or_else(|| "Unknown error".to_string()),
                )
            })?
            .ok_or(LocalStorageError::NotAvailable)
    }
    /// Sets an item in the local storage.
    ///
    /// # Arguments
    /// * `key` - A string slice that holds the key of the item.
    /// * `value` - A string slice that holds the value of the item.
    ///
    /// # Errors
    /// Returns an error if the item could not be set.
    pub fn set_item(key: &str, value: &str) -> LocalStorageResult<()> {
        Self::get_storage()?.set_item(key, value).map_err(|e| {
            LocalStorageError::SetItemError(
                e.as_string().unwrap_or_else(|| "Unknown error".to_string()),
            )
        })
    }

    /// Gets an item from the local storage.
    ///
    /// # Arguments
    /// * `key` - A string slice that holds the key of the item.
    ///
    /// # Returns
    /// A string slice that holds the value of the item.
    pub fn get_item(key: &str) -> LocalStorageResult<Option<String>> {
        Self::get_storage()?.get_item(key).map_err(|e| {
            LocalStorageError::GetItemError(
                e.as_string().unwrap_or_else(|| "Unknown error".to_string()),
            )
        })
    }
}
