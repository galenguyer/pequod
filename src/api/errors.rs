#![allow(dead_code)]

use serde::Serialize;

/// The details of a [RegistryError] to be returned by the API
#[derive(Debug, Serialize)]
pub struct RegistryErrorDetails {
    code: String,
    message: String,
}

/// The error codes returned via the API
///
/// [Reference]
///
/// [Reference]: https://docs.docker.com/registry/spec/api/#errors-2
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// Code: `BLOB_UNKNOWN`
    ///
    /// Message: `blob unknown to registry`
    ///
    /// Description: This error may be returned when a blob is unknown to the registry in a specified repository.
    /// This can be returned with a standard get or if a manifest references an unknown layer during upload.
    #[error("blob unknown to registry")]
    BlobUnknown,
    /// Code: `BLOB_UPLOAD_INVALID`
    ///
    /// Message: `blob upload invalid`
    ///
    /// Description: The blob upload encountered an error and can no longer proceed.
    #[error("blob upload invalid")]
    BlobUploadInvalid,
    /// Code: `BLOB_UPLOAD_UNKNOWN`
    ///
    /// Message: `blob upload unknown to registry`
    ///
    /// Description: If a blob upload has been cancelled or was never started, this error code may be returned.
    #[error("blob upload unknown to registry")]
    BlobUploadUnknown,
    /// Code: `DIGEST_INVALID`
    ///
    /// Message: `provided digest did not match uploaded content`
    ///
    /// Description: When a blob is uploaded, the registry will check that the content matches the digest provided
    /// by the client.
    /// The error may include a detail structure with the key “digest”, including the invalid digest string.
    /// This error may also be returned when a manifest includes an invalid layer digest.
    #[error("provided digest did not match uploaded content")]
    DigestInvalid,
    /// Code: `MANIFEST_BLOB_UNKNOWN`
    ///
    /// Message: `blob unknown to registry`
    ///
    /// Description: This error may be returned when a manifest blob is unknown to the registry.
    #[error("blob unknown to registry")]
    ManifestBlobUnknown,
    /// Code: `MANIFEST_INVALID`
    ///
    /// Message: `manifest invalid`
    ///
    /// Description: During upload, manifests undergo several checks ensuring validity.
    /// If those checks fail, this error may be returned, unless a more specific error is included.
    /// The detail will contain information the failed validation.
    #[error("manifest invalid")]
    ManifestInvalid,
    /// Code: `MANIFEST_UNKNOWN`
    ///
    /// Message: `manifest unknown`
    ///
    /// Description: This error is returned when the manifest, identified by name and tag is unknown to the repository.
    #[error("manifest unknown")]
    ManifestUnknown,
    /// Code: `MANIFEST_UNVERIFIED`
    ///
    /// Message: `manifest failed signature verification`
    ///
    /// Description: During manifest upload, if the manifest fails signature verification, this error will be returned.
    #[error("manifest failed signature verification")]
    ManifestUnverified,
    /// Code: `NAME_INVALID`
    ///
    /// Message: `invalid repository name`
    ///
    /// Description: Invalid repository name encountered either during manifest validation or any API operation.
    #[error("invalid repository name")]
    NameInvalid,
    /// Code: `NAME_UNKNOWN`
    ///
    /// Message: `repository name not known to registry`
    ///
    /// Description: This is returned if the name used during an operation is unknown to the registry.
    #[error("repository name not known to registry")]
    NameUnknown,
    /// Code: `PAGINATION_NUMBER_INVALID`
    ///
    /// Message: `invalid number of results requested`
    ///
    /// Description: Returned when the "n" parameter (number of results to return) is not an integer, or "n" is negative.
    #[error("invalid number of results requested")]
    PaginationNumberInvalid,
    /// Code: `RANGE_INVALID`
    ///
    /// Message: `invalid content range`
    ///
    /// Description: When a layer is uploaded, the provided range is checked against the uploaded chunk.
    /// This error is returned if the range is out of order.
    #[error("invalid content range")]
    RangeInvalid,
    /// Code: `SIZE_INVALID`
    ///
    /// Message: `provided length did not match content length`
    ///
    /// Description: When a layer is uploaded, the provided size will be checked against the uploaded content.
    /// If they do not match, this error will be returned.
    #[error("provided length did not match content length")]
    SizeInvalid,
    /// Code: `TAG_INVALID`
    ///
    /// Message: `manifest tag did not match URI`
    ///
    /// Description: During a manifest upload, if the tag in the manifest does not match the uri tag, this error will be returned.
    #[error("manifest tag did not match URI")]
    TagInvalid,
    /// Code: `UNAUTHORIZED`
    ///
    /// Message: `authentication required`
    ///
    /// Description: The access controller was unable to authenticate the client.
    /// Often this will be accompanied by a Www-Authenticate HTTP response header indicating how to authenticate.
    #[error("authentication required")]
    Unauthorized,
    /// Code: `DENIED`
    ///
    /// Message: `requested access to the resource is denied`
    ///
    /// Description: The access controller denied access for the operation on a resource.
    #[error("requested access to the resource is denied")]
    Denied,
    /// Code: `UNSUPPORTED`
    ///
    /// Message: `The operation is unsupported.`
    ///
    /// Description: The operation was unsupported due to a missing implementation or invalid set of parameters.
    #[error("The operation is unsupported.")]
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
                message: "invalid number of results requested".to_string(),
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
