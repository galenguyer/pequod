use tokio_postgres::Error as PostgresError;

#[async_backtrace::framed]
pub async fn get(repository: &str, digest: &str) -> Result<String, PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let row = db
        .query_one(
            "SELECT value
                FROM manifests
            WHERE repository = $1
                AND digest = $2",
            &[&repository, &digest],
        )
        .await?;

    Ok(row.get::<usize, String>(0))
}

#[async_backtrace::framed]
pub async fn save(repository: &str, digest: &str, value: &str) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    db.execute(
        "INSERT INTO manifests (repository, digest, value) VALUES ($1, $2, $3) ON CONFLICT (digest) DO NOTHING",
        &[&repository, &digest, &value],
    )
    .await?;

    Ok(())
}

#[async_backtrace::framed]
pub async fn delete(repository: &str, digest: &str) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    db.execute(
        "DELETE FROM manifests WHERE repository = $1 AND digest = $2",
        &[&repository, &digest],
    )
    .await?;

    Ok(())
}
