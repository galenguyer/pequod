use rusqlite::{Connection, Error as RusqliteError};

pub async fn get(repository: &str, digest: &str) -> Result<String, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("SELECT value FROM manifests WHERE repository = ? AND digest = ?")?;
    let mut rows = statement.query([repository, digest])?;

    let row = rows.next()?;

    let result = match row {
        Some(row) => row.get(0)?,
        None => {
            return Err(RusqliteError::QueryReturnedNoRows);
        }
    };

    Ok(result)
}

pub async fn save(repository: &str, digest: &str, value: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("INSERT INTO manifests (repository, digest, value) VALUES (?, ?, ?)")?;
    statement.execute([repository, digest, value])?;

    Ok(())
}

pub async fn delete(repository: &str, digest: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;

    let mut statement = conn.prepare("DELETE FROM manifests WHERE repository = ? AND digest = ?")?;
    statement.execute([repository, digest])?;
    tracing::info!("deleted manifest {} from repository {}", digest, repository);

    Ok(())
}
