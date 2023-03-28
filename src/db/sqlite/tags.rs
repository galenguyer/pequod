use rusqlite::{Connection, Error as RusqliteError};

pub async fn list(repository: &str) -> Result<Vec<String>, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("SELECT name FROM tags WHERE repository = ? ORDER BY updated DESC")?;
    let rows = statement.query_map([repository], |row| row.get::<usize, String>(0))?;
    rows.into_iter().collect()
}

pub async fn save(repository: &str, tag: &str, digest: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement =
        conn.prepare("INSERT INTO tags (repository, name, updated, manifest) VALUES (?, ?, ?, ?)")?;
    statement.execute(rusqlite::params![repository, tag, chrono::Utc::now().timestamp(), digest])?;

    Ok(())
}

pub async fn get(repository: &str, tag: &str) -> Result<String, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement =
        conn.prepare("SELECT manifest FROM tags WHERE repository = ? AND name = ?")?;
    let mut rows = statement.query([repository, tag])?;

    let row = rows.next()?;

    let result = match row {
        Some(row) => row.get(0)?,
        None => {
            return Err(RusqliteError::QueryReturnedNoRows);
        }
    };

    Ok(result)
}
