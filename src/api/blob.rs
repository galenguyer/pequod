use axum::body::Full;
use axum::extract::{Path, Query};
use axum::http::header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, RANGE};
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use bytes::{BufMut, Bytes, BytesMut};

use crate::db::sqlite;

pub async fn get_blob(
    Path((_name, digest)): Path<(String, String)>,
) -> impl IntoResponse {
    let blob = sqlite::blobs::get(&digest).await;
    if blob.is_err() {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    let blob = blob.unwrap();

    tracing::info!("serving blob with digest {} (size: {})", digest,  blob.len());

    (
        StatusCode::OK,
        [
            (HeaderName::from_static("docker-content-digest"), digest),
            (CONTENT_LENGTH, format!("{}", blob.len())),
            (CONTENT_TYPE, "application/octet-stream".to_string()),
        ],
        Full::from(blob.to_owned()),
    )
        .into_response()
}

pub async fn post_uploads(Path(name): Path<String>) -> impl IntoResponse {
    let uuid = uuid::Uuid::new_v4().as_hyphenated().to_string();

    (
        StatusCode::ACCEPTED,
        [
            (LOCATION, format!("/v2/{}/blobs/uploads/{}", name, uuid)),
            (HeaderName::from_static("docker-upload-uuid"), uuid),
        ],
    )
}

pub async fn patch_uploads(
    Path((name, uuid)): Path<(String, String)>,
    _headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let current = sqlite::blobs::get(&uuid).await;
    let (starting, ending) = match current {
        Err(_) => {
            let body_len = body.len();
            sqlite::blobs::save(&uuid, &body).await.unwrap();
            (0, body_len)
        }
        Ok(current) => {
            let mut new = BytesMut::new();
            let body_len = body.len();
            let current_len = current.len();
            new.put(current.to_owned());
            new.put(body);
            sqlite::blobs::save(&uuid, &new.into()).await.unwrap();
            (current_len, current_len + body_len)
        }
    };

    (
        StatusCode::ACCEPTED,
        [
            (LOCATION, format!("/v2/{}/blobs/uploads/{}", name, uuid)),
            (RANGE, format!("{starting}-{ending}")),
            (CONTENT_LENGTH, format!("0")),
            (HeaderName::from_static("docker-upload-uuid"), uuid),
        ],
    )
}

pub async fn finish_uploads(
    Path((name, uuid)): Path<(String, String)>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    if body.len() > 0 {
        let current = sqlite::blobs::get(&uuid).await.unwrap();
        let mut new = BytesMut::new();
        new.put(current.to_owned());
        new.put(body);
        sqlite::blobs::save(&uuid, &new.into()).await.unwrap();
    }

    let digest = query.get("digest").unwrap().to_string();
    sqlite::blobs::update_digest(&uuid, &digest).await.unwrap();
    let blob = sqlite::blobs::get(&digest).await.unwrap();

    tracing::info!("saved blob with digest {} (size: {})", digest,  blob.len());

    (
        StatusCode::CREATED,
        [
            (LOCATION, format!("/v2/{}/blobs/{}", name, digest)),
            (CONTENT_LENGTH, format!("0")),
            (HeaderName::from_static("docker-content-digest"), digest),
        ],
    )
}
