#![allow(dead_code)]

#[derive(Debug)]
struct RegistryErrorDetails {
    code: String,
    message: String,
}

/// Errors that can be returned by the registry.
/// https://docs.docker.com/registry/spec/api/#errors-2
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("blob unknown to registry")]
    BlobUnknown,
    #[error("blob upload invalid")]
    BlobUploadInvalid,
    #[error("blob upload unknown to registry")]
    BlobUploadUnknown,
    #[error("provided digest did not match uploaded content")]
    DigestInvalid,
    #[error("blob unknown to registry")]
    ManifestBlobUnknown,
    #[error("manifest invalid")]
    ManifestInvalid,
    #[error("manifest unknown")]
    ManifestUnknown,
    #[error("manifest failed signature verification")]
    ManifestUnverified,
    #[error("invalid repository name")]
    NameInvalid,
    #[error("repository name not known to registry")]
    NameUnknown,
    #[error("the page number requested is outside the valid range")]
    PaginationNumberInvalid,
    #[error("provided range was invalid")]
    RangeInvalid,
    #[error("provided length did not match content length")]
    SizeInvalid,
    #[error("manifest tag did not match URI")]
    TagInvalid,
    #[error("authentication required")]
    Unauthorized,
    #[error("requested access to the resource is denied")]
    Denied,
    #[error("the operation is unsupported")]
    Unsupported,
}

impl From<RegistryError> for RegistryErrorDetails {
    fn from(err: RegistryError) -> Self {
        match err {
            RegistryError::BlobUnknown => RegistryErrorDetails {
                code: "BLOB_UNKNOWN".to_string(),
                message: "blob unknown to registry".to_string(),
            },
            RegistryError::BlobUploadInvalid => RegistryErrorDetails {
                code: "BLOB_UPLOAD_INVALID".to_string(),
                message: "blob upload invalid".to_string(),
            },
            RegistryError::BlobUploadUnknown => RegistryErrorDetails {
                code: "BLOB_UPLOAD_UNKNOWN".to_string(),
                message: "blob upload unknown to registry".to_string(),
            },
            RegistryError::DigestInvalid => RegistryErrorDetails {
                code: "DIGEST_INVALID".to_string(),
                message: "provided digest did not match uploaded content".to_string(),
            },
            RegistryError::ManifestBlobUnknown => RegistryErrorDetails {
                code: "MANIFEST_BLOB_UNKNOWN".to_string(),
                message: "blob unknown to registry".to_string(),
            },
            RegistryError::ManifestInvalid => RegistryErrorDetails {
                code: "MANIFEST_INVALID".to_string(),
                message: "manifest invalid".to_string(),
            },
            RegistryError::ManifestUnknown => RegistryErrorDetails {
                code: "MANIFEST_UNKNOWN".to_string(),
                message: "manifest unknown".to_string(),
            },
            RegistryError::ManifestUnverified => RegistryErrorDetails {
                code: "MANIFEST_UNVERIFIED".to_string(),
                message: "manifest failed signature verification".to_string(),
            },
            RegistryError::NameInvalid => RegistryErrorDetails {
                code: "NAME_INVALID".to_string(),
                message: "invalid repository name".to_string(),
            },
            RegistryError::NameUnknown => RegistryErrorDetails {
                code: "NAME_UNKNOWN".to_string(),
                message: "repository name not known to registry".to_string(),
            },
            RegistryError::PaginationNumberInvalid => RegistryErrorDetails {
                code: "PAGINATION_NUMBER_INVALID".to_string(),
                message: "the page number requested is outside the valid range".to_string(),
            },
            RegistryError::RangeInvalid => RegistryErrorDetails {
                code: "RANGE_INVALID".to_string(),
                message: "provided range was invalid".to_string(),
            },
            RegistryError::SizeInvalid => RegistryErrorDetails {
                code: "SIZE_INVALID".to_string(),
                message: "provided length did not match content length".to_string(),
            },
            RegistryError::TagInvalid => RegistryErrorDetails {
                code: "TAG_INVALID".to_string(),
                message: "manifest tag did not match URI".to_string(),
            },
            RegistryError::Unauthorized => RegistryErrorDetails {
                code: "UNAUTHORIZED".to_string(),
                message: "authentication required".to_string(),
            },
            RegistryError::Denied => RegistryErrorDetails {
                code: "DENIED".to_string(),
                message: "requested access to the resource is denied".to_string(),
            },
            RegistryError::Unsupported => RegistryErrorDetails {
                code: "UNSUPPORTED".to_string(),
                message: "the operation is unsupported".to_string(),
            },
        }
    }
}
