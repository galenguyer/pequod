use std::collections::HashMap;

use axum::body::Full;
use axum::extract::{Path, Query};
use axum::http::header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, RANGE};
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::Extension;
use bytes::{BufMut, Bytes, BytesMut};
use std::sync::{Arc, Mutex};

pub async fn get_blob(
    Path((_name, digest)): Path<(String, String)>,
    Extension(blobcache): Extension<Arc<Mutex<HashMap<String, Bytes>>>>,
) -> impl IntoResponse {
    let cache = blobcache.lock().unwrap();
    let blob = cache.get(&digest);
    if blob.is_none() {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    let blob = blob.unwrap();

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
    headers: HeaderMap,
    Extension(blobcache): Extension<Arc<Mutex<HashMap<String, Bytes>>>>,
    body: Bytes,
) -> impl IntoResponse {
    // dbg!(headers);
    let mut cache = blobcache.lock().unwrap();

    let (starting, ending) = match cache.get(&uuid) {
        None => {
            let body_len = body.len();
            cache.insert(uuid.clone(), body);
            (0, body_len)
        }
        Some(current) => {
            let mut new = BytesMut::new();
            let body_len = body.len();
            let current_len = current.len();
            new.put(current.to_owned());
            new.put(body);
            cache.insert(uuid.clone(), new.into());
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

#[axum::debug_handler]
pub async fn finish_uploads(
    Path((name, uuid)): Path<(String, String)>,
    Query(digest): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    Extension(blobcache): Extension<Arc<Mutex<HashMap<String, Bytes>>>>,
    body: Bytes,
) -> impl IntoResponse {
    let mut cache = blobcache.lock().unwrap();
    if body.len() > 0 {
        let current = cache.get(&uuid).unwrap();
        let mut new = BytesMut::new();
        new.put(current.to_owned());
        new.put(body);
        cache.insert(uuid.clone(), new.into());
    }

    if let Some(data) = cache.remove(&uuid) {
        cache.insert(digest.get("digest").unwrap().to_string(), data);
    }

    dbg!(&name, &uuid, &digest);
    (
        StatusCode::CREATED,
        [
            (LOCATION, format!("/v2/{}/blobs/uploads/{}", name, uuid)),
            (CONTENT_LENGTH, format!("0")),
            (HeaderName::from_static("docker-upload-uuid"), uuid),
        ],
    )
}
