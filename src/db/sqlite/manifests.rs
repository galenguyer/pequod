use rusqlite::{Connection, Error as RusqliteError};

pub async fn get(digest: &str) -> Result<String, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("SELECT value FROM manifests WHERE digest = ?")?;
    let mut rows = statement.query([digest])?;

    let row = rows.next()?;

    let result = match row {
        Some(row) => row.get(0)?,
        None => {
            return Err(RusqliteError::QueryReturnedNoRows);
        }
    };

    Ok(result)
}

pub async fn save(digest: &str, value: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("INSERT INTO manifests (digest, value) VALUES (?, ?)")?;
    statement.execute([digest, value])?;

    Ok(())
}
