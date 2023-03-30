use axum::extract::Path;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::db;

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
            let digest = db::tags::get(&name, &reference).await.unwrap();
            tracing::info!("resolved tag {}:{} to digest {}", name, reference, digest);
            digest
        }
    };
    let raw = db::manifests::get(&name, &digest).await.unwrap();
    (
        [
            (HeaderName::from_static("docker-content-digest"), digest),
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

    db::repositories::save(&name).await.unwrap();
    db::manifests::save(&name, &digest, &body)
        .await
        .unwrap();

    if !crate::DIGEST_REGEX.is_match(&reference) {
        db::tags::save(&name, &reference, &digest)
            .await
            .unwrap();
    }

    tracing::info!("manifest saved: {}", digest);

    let parsed = serde_json::from_str::<Manifest>(&body);
    if let Ok(manifest) = parsed {
        match manifest {
            Manifest::Image(image) => {
                tracing::info!(
                    "associating layer {} with manifest {}",
                    image.config.digest,
                    digest
                );
                if let Err(e) = db::blobs::associate(&digest, &image.config.digest).await {
                    tracing::error!("failed to associate layer with manifest: {}", e);
                }

                for layer in image.layers {
                    tracing::info!(
                        "associating layer {} with manifest {}",
                        layer.digest,
                        digest
                    );
                    if let Err(e) = db::blobs::associate(&digest, &layer.digest).await {
                        tracing::error!("failed to associate layer with manifest: {}", e);
                    }
                }
            }
            Manifest::List(_) => {
                tracing::warn!("manifest list not implemented")
            }
        }
    }

    (
        StatusCode::CREATED,
        [(HeaderName::from_static("docker-content-digest"), digest)],
    )
}

pub async fn delete(Path((name, reference)): Path<(String, String)>) -> impl IntoResponse {
    db::manifests::delete(&name, &reference).await.unwrap();
}
