use crate::db::Repository;
use tokio_postgres::Error as PostgresError;

#[async_backtrace::framed]
pub async fn list() -> Result<Vec<Repository>, PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    let rows = db
        .query("SELECT name FROM repositories ORDER BY name ASC", &[])
        .await?;

    let repositories = rows
        .iter()
        .map(|row| Repository { name: row.get(0) })
        .collect();

    Ok(repositories)
}

#[async_backtrace::framed]
pub async fn save(name: &str) -> Result<(), PostgresError> {
    let db = super::CLIENT.get_or_init(super::db).await;

    db.execute(
        "
    INSERT INTO repositories (name)
        VALUES ($1)
    ON CONFLICT(name)
        DO NOTHING
        ",
        &[&name],
    )
    .await?;

    Ok(())
}
