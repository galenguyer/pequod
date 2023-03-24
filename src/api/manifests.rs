use axum::extract::Path;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::db::sqlite;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Manifest {
    List(ManifestList),
    Image(ImageManifest),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestList {
    pub schema_version: u32,
    pub media_type: String,
    pub manifests: Vec<ManifestListManifest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestListManifest {
    pub media_type: String,
    pub size: u32,
    pub digest: String,
    pub platform: ManifestListManifestPlatform,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestListManifestPlatform {
    pub architecture: String,
    pub os: String,
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,
    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,
    pub variant: Option<String>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageManifest {
    pub schema_version: u32,
    pub media_type: String,
    pub config: ImageManifestConfig,
    pub layers: Vec<ImageManifestLayer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageManifestConfig {
    pub media_type: String,
    pub size: u32,
    pub digest: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageManifestLayer {
    pub media_type: String,
    pub size: u32,
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

pub async fn get(Path((name, reference)): Path<(String, String)>) -> impl IntoResponse {
    let digest = match crate::DIGEST_REGEX.is_match(&reference) {
        true => reference,
        false => {
            tracing::info!("resolving tag: {}:{}", name, reference);
            let digest =sqlite::tags::get(&name, &reference).await.unwrap();
            tracing::info!("resolved tag {}:{} to digest {}", name, reference, digest);
            digest
        }
    };
    let raw = sqlite::manifests::get(&digest).await.unwrap();
    (
        [
            (
                HeaderName::from_static("docker-content-digest"),
                digest,
            ),
            (
                CONTENT_TYPE,
                "application/vnd.docker.distribution.manifest.v2+json".to_string(),
            ),
            (
                ACCEPT,
                "application/vnd.docker.distribution.manifest.v2+json".to_string(),
            ),
            (
                HeaderName::from_static("docker-distribution-api-version"),
                "registry/2.0".to_string(),
            ),
        ],
        raw,
    )
        .into_response()
}

pub async fn put(
    Path((name, reference)): Path<(String, String)>,
    body: String,
) -> impl IntoResponse {
    let hash = sha256::digest(body.clone());
    let digest = format!("sha256:{hash}");

    crate::db::sqlite::manifests::save(&digest, &body)
        .await
        .unwrap();

    if !crate::DIGEST_REGEX.is_match(&reference) {
        crate::db::sqlite::tags::save(&name, &reference, &digest)
            .await
            .unwrap();
    }

    tracing::info!("manifest saved: {}", digest);

    (
        StatusCode::CREATED,
        [(
            HeaderName::from_static("docker-content-digest"),
            digest,
        )],
    )
}
