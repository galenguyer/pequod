use rusqlite::{Connection, Error as RusqliteError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Repository {
    pub name: String,
}

pub async fn list() -> Result<Vec<Repository>, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("SELECT name FROM repositories")?;
    let rows = statement.query_map([], |row| Ok(Repository { name: row.get(0)? }))?;
    rows.into_iter().collect()
}
