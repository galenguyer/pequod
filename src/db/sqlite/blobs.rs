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

pub async fn length(digest: &str) -> Result<usize, RusqliteError> {
    let conn = Connection::open("registry.db")?;
    let mut statement = conn.prepare("SELECT length(value) FROM blobs WHERE digest = ?")?;
    let mut rows = statement.query([digest])?;

    let row = rows.next()?;

    let result = match row {
        Some(row) => row.get::<usize, usize>(0)?,
        None => {
            return Err(RusqliteError::QueryReturnedNoRows);
        }
    };

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

pub async fn associate(manifest_digest: &str, layer_digest: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;

    let mut statement = conn.prepare("INSERT INTO manifest_blobs(manifest, blob) VALUES (?, ?)")?;
    statement.execute([manifest_digest, layer_digest])?;
    tracing::info!("associated {} -> {}", manifest_digest, layer_digest);

    Ok(())
}

pub async fn disassociate(repository: &str, layer_digest: &str) -> Result<(), RusqliteError> {
    let conn = Connection::open("registry.db")?;

    let mut statement = conn.prepare("DELETE FROM manifest_blobs WHERE blob = ? AND manifest IN (SELECT digest FROM manifests WHERE repository = ?) RETURNING manifest, blob")?;
    let mut deleted = statement.query([dbg!(layer_digest), repository])?;

    loop {
        let d = deleted.next()?;
        match d {
            Some(d) => {
                let manifest: String = d.get(0)?;
                let blob: String = d.get(1)?;

                tracing::info!("deleted association {} -> {}", manifest, blob);
            }
            None => {
                break;
            }
        }
    }

    Ok(())
}

pub async fn cleanup() -> Result<(), RusqliteError> {
    let mut conn = Connection::open("registry.db")?;
    let trans = conn.transaction()?;

    // delete blobs we don't have a manifest for
    trans.execute("DELETE FROM blobs WHERE digest IN (SELECT blob FROM manifest_blobs WHERE manifest NOT IN (SELECT digest FROM manifests))", [])?;
    // delete assocations we don't have a manifest for
    trans.execute(
        "DELETE FROM manifest_blobs WHERE manifest NOT IN (SELECT digest FROM manifests)",
        [],
    )?;

    // delete blobs we don't have an association for
    trans.execute(
        "DELETE FROM blobs WHERE digest NOT IN (SELECT blob FROM manifest_blobs)",
        [],
    )?;

    trans.commit()?;

    conn.execute("VACUUM", [])?;

    Ok(())
}
