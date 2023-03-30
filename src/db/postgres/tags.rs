use crate::db::Tag;
use chrono::{DateTime, NaiveDateTime, Utc};
use tokio_postgres::Error as PostgresError;

#[async_backtrace::framed]
pub async fn list(repository: &str) -> Result<Vec<Tag>, PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let rows = db
        .query(
            "SELECT name, updated, manifest FROM tags WHERE repository = $1 ORDER BY updated DESC",
            &[&repository],
        )
        .await?;

    let tags = rows
        .iter()
        .map(|row| Tag {
            name: row.get(0),
            updated: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(row.get::<usize, i64>(1), 0).unwrap_or_default(),
                Utc,
            ),
            manifest: row.get(2),
        })
        .collect();
    Ok(tags)
}

#[async_backtrace::framed]
pub async fn save(repository: &str, tag: &str, digest: &str) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    db.execute(
        "INSERT INTO tags (repository, name, updated, manifest) VALUES ($1, $2, $3, $4) ON CONFLICT (repository, name) DO UPDATE SET updated = $3, manifest = $4",
        &[&repository, &tag, &Utc::now().timestamp(), &digest],
    )
    .await?;

    Ok(())
}

#[async_backtrace::framed]
pub async fn get_manifest(repository: &str, tag: &str) -> Result<String, PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let manifest = db
        .query_one(
            "SELECT manifest FROM tags WHERE repository = $1 AND name = $2",
            &[&repository, &tag],
        )
        .await
        .map(|row| row.get(0))?;

    Ok(manifest)
}

#[async_backtrace::framed]
pub async fn get_size(digest: &str) -> Result<u32, PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let size = db.query_one("SELECT sum(length(value))::OID AS size FROM blobs WHERE digest IN (SELECT blob FROM manifest_blobs WHERE manifest = $1)",
&[&digest])
        .await
        .map(|row| row.try_get(0).unwrap_or_default())?;
    Ok(size)
}
