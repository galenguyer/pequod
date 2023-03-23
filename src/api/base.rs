//! Base V2 API route. Typically, this can be used for lightweight version checks and to validate registry authentication.
//! <https://docs.docker.com/registry/spec/api/#base>
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use super::{errors, RegistryErrorDetails};

/// The possible responses for the `GET /v2/` endpoint.
/// NotFound should never be returned by this API.
#[allow(dead_code)]
pub enum GetBaseResponse {
    Ok,
    NotFound,
    Unauthorized,
    TooManyRequests,
}

impl IntoResponse for GetBaseResponse {
    fn into_response(self) -> Response {
        match self {
            GetBaseResponse::Ok => (StatusCode::OK, Json(json!({}))).into_response(),
            GetBaseResponse::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "errors":
                        vec![RegistryErrorDetails::from(
                            errors::RegistryError::Unauthorized,
                        )]
                })),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "errors":
                        vec![RegistryErrorDetails::from(
                            errors::RegistryError::Unsupported,
                        )]
                })),
            )
                .into_response(),
        }
    }
}

/// # GET Base
///
/// Check that the endpoint implements Docker Registry API V2.
///
///
///
/// ```text
/// GET /v2/
/// Host: <registry host>
/// Authorization: <scheme> <token>
/// ```
/// The following parameters should be specified on the request:
///
/// |Name|Kind|Description|
/// |----|----|-----------|
/// |`Host`|header|Standard HTTP Host Header. Should be set to the registry host.|
/// |`Authorization`|header|An RFC7235 compliant authorization header.|
///
///
///
///
/// ## On Success: OK
///
/// ```text
/// 200 OK
///
/// {}
/// ```
///
/// The API implements V2 protocol and is accessible.
///
///
/// ## On Failure: Not Found
///
/// ```text
/// 404 Not Found
/// ```
///
/// The registry does not implement the V2 API.
///
///
/// ## On Failure: Authentication Required
///
/// ```text
/// 401 Unauthorized
/// WWW-Authenticate: <scheme> realm="<realm>", ..."
/// Content-Length: <length>
/// Content-Type: application/json
///
/// {
///     "errors": [
///         {
///             "code": <error code>,
///             "message": "<error message>",
///             "detail": ...
///         },
///         ...
///     ]
/// }
/// ```
///
/// The client is not authenticated.
///
/// The following headers will be returned on the response:
///
/// |Name|Description|
/// |----|-----------|
/// |`WWW-Authenticate`|An RFC7235 compliant authentication challenge header.|
/// |`Content-Length`|Length of the JSON response body.|
///
///
/// The error codes that may be included in the response body are enumerated below:
///
/// |Code|Message|Description|
/// |----|-------|-----------|
/// | `UNAUTHORIZED` | authentication required | The access controller was unable to authenticate the client. Often this will be accompanied by a Www-Authenticate HTTP response header indicating how to authenticate. |
///
///
/// ## On Failure: Too Many Requests
///
/// ```text
/// 429 Too Many Requests
/// Content-Length: <length>
/// Content-Type: application/json
///
/// {
///     "errors": [
///         {
///             "code": <error code>,
///             "message": "<error message>",
///             "detail": ...
///         },
///         ...
///     ]
/// }
/// ```
///
/// The client made too many requests within a time interval.
///
/// The following headers will be returned on the response:
///
/// |Name|Description|
/// |----|-----------|
/// |`Content-Length`|Length of the JSON response body.|
///
///
/// The error codes that may be included in the response body are enumerated below:
///
/// |Code|Message|Description|
/// |----|-------|-----------|
/// | `TOOMANYREQUESTS` | too many requests | Returned when a client attempts to contact a service too many times |
///
/// <https://docs.docker.com/registry/spec/api/#get-base>
pub async fn base() -> GetBaseResponse {
    GetBaseResponse::Ok
}
