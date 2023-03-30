use crate::db::Tag;
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{Connection, Error as RusqliteError};

pub async fn list(repository: &str) -> Result<Vec<Tag>, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare(
        "SELECT name, updated, manifest FROM tags WHERE repository = ? ORDER BY updated DESC",
    )?;
    let rows = statement.query_map([repository], |row| {
        Ok(Tag {
            name: row.get(0)?,
            updated: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(row.get::<usize, i64>(1)?, 0).unwrap_or_default(),
                Utc,
            ),
            manifest: row.get(2)?,
        })
    })?;
    rows.into_iter().collect()
}

pub async fn save(repository: &str, tag: &str, digest: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement =
        conn.prepare("INSERT INTO tags (repository, name, updated, manifest) VALUES (?, ?, ?, ?)")?;
    statement.execute(rusqlite::params![
        repository,
        tag,
        chrono::Utc::now().timestamp(),
        digest
    ])?;

    Ok(())
}

pub async fn get_manifest(repository: &str, tag: &str) -> Result<String, RusqliteError> {
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

pub async fn get_size(digest: &str) -> Result<usize, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare(
        "SELECT sum(length(value)) AS size FROM blobs WHERE digest IN (SELECT blob FROM manifest_blobs WHERE manifest = ?)",
    )?;
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
