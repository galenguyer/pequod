use rusqlite::{Connection, Error as RusqliteError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Repository {
    pub name: String,
}

pub async fn list(repository: &str) -> Result<Vec<String>, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("SELECT name FROM tags WHERE repository = ?")?;
    let rows = statement.query_map([repository], |row| row.get::<usize, String>(0))?;
    rows.into_iter().collect()
}
