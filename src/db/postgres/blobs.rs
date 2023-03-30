use bytes::Bytes;
use tokio_postgres::Error as PostgresError;

#[async_backtrace::framed]
pub async fn get(digest: &str) -> Result<Bytes, PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let row = db
        .query_one("SELECT value FROM blobs WHERE digest = $1", &[&digest])
        .await?;
    let value: Vec<u8> = row.get(0);

    Ok(Bytes::from_iter(value))
}

#[async_backtrace::framed]
pub async fn length(digest: &str) -> Result<u32, PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let size = db
        .query_one(
            "SELECT length(value)::OID FROM blobs WHERE digest = $1",
            &[&digest],
        )
        .await
        .map(|row| row.get(0))?;

    Ok(size)
}

#[async_backtrace::framed]
pub async fn save(digest: &str, value: &Bytes) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    db.execute(
        "INSERT INTO blobs (digest, value)
            VALUES ($1, $2)
        ON CONFLICT(digest)
            DO UPDATE SET value = $2",
        &[&digest, &value.to_vec()],
    )
    .await?;

    Ok(())
}

#[async_backtrace::framed]
pub async fn update_digest(old_digest: &str, new_digest: &str) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    db.execute(
        "UPDATE blobs SET digest = $1 WHERE digest = $2 AND NOT EXISTS (SELECT 1 FROM blobs WHERE digest = $1)",
        &[&new_digest, &old_digest],
    )
    .await?;

    Ok(())
}

#[async_backtrace::framed]
pub async fn associate(manifest_digest: &str, layer_digest: &str) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    db.execute(
        "INSERT INTO manifest_blobs(manifest, blob) VALUES ($1, $2) ON CONFLICT(manifest, blob) DO NOTHING",
        &[&manifest_digest, &layer_digest],
    ).await?;
    tracing::info!("associated {} -> {}", manifest_digest, layer_digest);

    Ok(())
}

#[async_backtrace::framed]
pub async fn disassociate(repository: &str, layer_digest: &str) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let deleted = db
        .query(
            "
    DELETE FROM manifest_blobs
        WHERE blob = $1
            AND manifest IN
                (SELECT digest
                    FROM manifests
                        WHERE repository = $2)
    RETURNING manifest, blob",
            &[&layer_digest, &repository],
        )
        .await?;

    for d in deleted {
        let manifest: String = d.get(0);
        let blob: String = d.get(1);

        tracing::info!("deleted association {} -> {}", manifest, blob);
    }

    Ok(())
}
