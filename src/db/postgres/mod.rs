use tokio::sync::OnceCell;
use tokio_postgres::Client;
use tokio_postgres::{Error as PostgresError, NoTls};

pub mod blobs;
pub mod manifests;
pub mod repositories;
pub mod tags;

pub(crate) static CLIENT: OnceCell<Client> = OnceCell::const_new();

#[async_backtrace::framed]
pub async fn cleanup() -> Result<(), PostgresError> {
    let mut db = db().await;

    let trans = db.transaction().await?;

    // delete assocations we don't have a manifest for
    let assocs = trans
        .execute(
            "DELETE FROM manifest_blobs WHERE manifest NOT IN (SELECT digest FROM manifests)",
            &[],
        )
        .await?;
    tracing::info!("deleted {} orphaned assocations", assocs);

    // delete blobs we don't have an association for
    let blobs = trans
        .execute(
            "DELETE FROM blobs WHERE digest NOT IN (SELECT blob FROM manifest_blobs)",
            &[],
        )
        .await?;
    tracing::info!("deleted {} orphaned blobs", blobs);

    // delete tags that have no associated manifest
    let tags = trans
        .execute(
            "DELETE FROM tags WHERE tags.manifest NOT IN (SELECT digest FROM manifests)",
            &[],
        )
        .await?;
    tracing::info!("deleted {} orphaned tags", tags);

    // delete repositories that have no associated manifests
    let repositories = trans
        .execute(
            "DELETE FROM repositories WHERE name NOT IN (SELECT repository FROM manifests)",
            &[],
        )
        .await?;
    tracing::info!("deleted {} orphaned repositories", repositories);

    trans.commit().await?;

    db.execute("VACUUM", &[]).await?;
    tracing::info!("vacuumed database");

    Ok(())
}

#[async_backtrace::framed]
async fn db() -> Client {
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "postgresql://postgres:{}@localhost:5432",
            std::env::var("POSTGRES_PASSWORD").unwrap_or_default()
        ),
        NoTls,
    )
    .await
    .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}
