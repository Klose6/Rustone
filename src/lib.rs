//! Rustone — an LSM-tree storage engine inspired by RocksDB.

pub mod db;
pub mod error;

pub use db::Db;
pub use error::{Error, Result};
