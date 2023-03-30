use chrono::{DateTime, Utc};
use serde::Serialize;

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "sqlite")]
pub use sqlite::*;

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "postgres")]
pub use postgres::*;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub name: String,
    pub updated: DateTime<Utc>,
    pub manifest: String,
}

#[derive(Debug, Serialize)]
pub struct Repository {
    pub name: String,
}
