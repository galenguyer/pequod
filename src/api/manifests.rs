use axum::extract::Path;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let manifest: Manifest = serde_json::from_str(r#"{"schemaVersion":2,"mediaType":"application/vnd.docker.distribution.manifest.v2+json","config":{"mediaType":"application/vnd.docker.container.image.v1+json","size":1546,"digest":"sha256:35b811f5c18761eb6bf7c48d934e542188fc601f0c95fcc0b545320f30e57ec7"},"layers":[{"mediaType":"application/vnd.docker.image.rootfs.diff.tar.gzip","size":3374447,"digest":"sha256:63b65145d645c1250c391b2d16ebe53b3747c295ca8ba2fcb6b0cf064a4dc21c"},{"mediaType":"application/vnd.docker.image.rootfs.diff.tar.gzip","size":4218428,"digest":"sha256:84c353bd7b164c886102cd79e89030f830082e64a292d5033318241238ed2514"}]}"#).unwrap();

    (
        [
            (
                HeaderName::from_static("docker-content-digest"),
                "sha256:448aa840670266671338b7ccd9069777be3e5f2bf698c29c71f086b3582cf377",
            ),
            (
                CONTENT_TYPE,
                "application/vnd.docker.distribution.manifest.v2+json",
            ),
            (
                ACCEPT,
                "application/vnd.docker.distribution.manifest.v2+json",
            ),
            (
                HeaderName::from_static("docker-distribution-api-version"),
                "registry/2.0",
            ),
        ],
        Json(json!(manifest)),
    )
}

pub async fn put(
    Path((name, reference)): Path<(String, String)>,
    body: String,
) -> impl IntoResponse {
    println!("{}", &body);

    let manifest: Manifest = serde_json::from_str(&body).unwrap();

    dbg!(&manifest);

    let normalized = serde_json::to_string(&manifest).unwrap();

    println!("{}", &normalized);

    let hash = sha256::digest(normalized.clone());
    let digest = format!("sha256:{hash}");

    (
        StatusCode::CREATED,
        [(
            HeaderName::from_static("docker-content-digest"),
            dbg!(digest),
        )],
    )
}
