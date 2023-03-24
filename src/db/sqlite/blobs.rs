use std::io::Read;

use bytes::Bytes;
use rusqlite::{Connection, Error as RusqliteError};

pub async fn get(digest: &str) -> Result<Bytes, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("SELECT value FROM blobs WHERE digest = ?")?;
    let mut rows = statement.query([digest])?;

    let row = rows.next()?;

    let result = match row {
        Some(row) => row.get::<usize, Vec<u8>>(0)?,
        None => {
            return Err(RusqliteError::QueryReturnedNoRows);
        }
    };

    let result = Bytes::from_iter(result);

    Ok(result)
}

pub async fn save(digest: &str, value: &Bytes) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("INSERT INTO blobs (digest, value) VALUES (?, ?)")?;
    statement.execute(rusqlite::params![
        digest,
        value.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>()
    ])?;

    Ok(())
}

pub async fn update_digest(old_digest: &str, new_digest: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("UPDATE blobs SET digest = ? WHERE digest = ?")?;
    statement.execute([new_digest, old_digest])?;

    Ok(())
}
