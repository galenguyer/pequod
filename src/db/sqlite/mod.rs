use rusqlite::{Connection, Error as RusqliteError};

pub mod blobs;
pub mod manifests;
pub mod repositories;
pub mod tags;

pub async fn cleanup() -> Result<(), RusqliteError> {
    let mut conn = Connection::open("registry.db")?;
    let trans = conn.transaction()?;

    // delete assocations we don't have a manifest for
    let assocs = trans.execute(
        "DELETE FROM manifest_blobs WHERE manifest NOT IN (SELECT digest FROM manifests)",
        [],
    )?;
    tracing::info!("deleted {} orphaned assocations", assocs);

    // delete blobs we don't have an association for
    let blobs = trans.execute(
        "DELETE FROM blobs WHERE digest NOT IN (SELECT blob FROM manifest_blobs)",
        [],
    )?;
    tracing::info!("deleted {} orphaned blobs", blobs);

    // delete tags that have no associated manifest
    let tags = trans.execute(
        "DELETE FROM tags WHERE tags.manifest NOT IN (SELECT digest FROM manifests)",
        [],
    )?;
    tracing::info!("deleted {} orphaned tags", tags);

    // delete repositories that have no associated manifests
    let repositories = trans.execute(
        "DELETE FROM repositories WHERE name NOT IN (SELECT repository FROM manifests)",
        [],
    )?;
    tracing::info!("deleted {} orphaned repositories", repositories);

    trans.commit()?;

    conn.execute("VACUUM", [])?;

    Ok(())
}
