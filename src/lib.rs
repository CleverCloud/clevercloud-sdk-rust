//! # Clever-Cloud Sdk
//!
//! This module provides a client and structures to interact with clever-cloud
//! api.

use std::{fmt::Debug, future::Future};

use ::oauth10a::{
    client::{Client as OAuthClient, ClientError as OAuthClientError},
    credentials::Credentials as OAuthCredentials,
    execute::ExecuteRequest,
    reqwest::{self, Method, StatusCode},
    rest::{ErrorResponse, RestClient as OAuthRestClient, RestError},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub mod v2;
pub mod v4;

// -----------------------------------------------------------------------------
// Exports
//
// oauth10a 3.0.0 reworked its public API (generic `RestClient<X>` returning a
// nested `Result<Result<O, ErrorResponse<E>>, RestError<_>>`, credentials and
// client split across modules). To avoid rippling that change through every
// `v2`/`v4` module and through downstream consumers, this crate keeps the
// historical surface: a `ClientError`, and `Request`/`RestClient` traits whose
// methods return a flat `Result<T, ClientError>`. The `oauth10a` module below
// re-exports them so `clevercloud_sdk::oauth10a::{ClientError, reqwest, ..}`
// keeps working.

/// Compatibility facade exposing the historical `oauth10a::client` surface.
pub mod oauth10a {
    pub use ::oauth10a::{reqwest, url};

    pub use crate::{ClientError, Request, ResponseError, RestClient};
}

// -----------------------------------------------------------------------------
// Constants

pub const PUBLIC_ENDPOINT: &str = "https://api.clever-cloud.com";
pub const PUBLIC_API_BRIDGE_ENDPOINT: &str = "https://api-bridge.clever-cloud.com";

// Consumer key and secret reported here are one from the clever-tools and is
// available publicly.
// the disclosure of these tokens is not considered as a vulnerability.
// Do not report this to our security service.
//
// See:
// - <https://github.com/CleverCloud/clever-tools/blob/fed085e2ba0339f55e966d7c8c6439d4dac71164/src/models/configuration.js#L128>
pub const DEFAULT_CONSUMER_KEY: &str = "T5nFjKeHH4AIlEveuGhB5S3xg8T19e";
pub fn default_consumer_key() -> String {
    DEFAULT_CONSUMER_KEY.to_string()
}

pub const DEFAULT_CONSUMER_SECRET: &str = "MgVMqTr6fWlf2M0tkC2MXOnhfqBWDT";
pub fn default_consumer_secret() -> String {
    DEFAULT_CONSUMER_SECRET.to_string()
}

// -----------------------------------------------------------------------------
// ResponseError / ClientError structures

/// Body of an error response returned by the Clever Cloud API.
pub type ResponseError = serde_json::Value;

/// Error returned by the [`Client`] when executing a request. It keeps the
/// historical shape (in particular the `StatusCode` variant) expected by the
/// rest of the SDK and by downstream consumers.
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("failed to execute request, {0}")]
    Request(reqwest::Error),
    #[error("failed to authorize or execute request, {0}")]
    Execute(String),
    #[error("failed to execute request, got status code {0}, {1}")]
    StatusCode(StatusCode, ResponseError),
    #[error("failed to serialize request body, {0}")]
    Serialize(serde_json::Error),
    #[error("failed to deserialize response body, {0}")]
    Deserialize(serde_json::Error),
}

impl From<RestError<OAuthClientError>> for ClientError {
    fn from(err: RestError<OAuthClientError>) -> Self {
        match err {
            RestError::Execute(err) => Self::Execute(err.to_string()),
            RestError::Serialize(err) => Self::Serialize(err),
            RestError::Deserialize(err) => Self::Deserialize(err),
            RestError::Url(err) | RestError::BodyAggregation(err) => Self::Request(err),
            RestError::TooManyHeaders(err) => Self::Execute(err.to_string()),
        }
    }
}

impl From<OAuthClientError> for ClientError {
    fn from(err: OAuthClientError) -> Self {
        Self::Execute(err.to_string())
    }
}

// -----------------------------------------------------------------------------
// Request / RestClient traits

/// Low-level request execution, kept for the historical surface.
pub trait Request {
    type Error;

    fn request<T, U>(
        &self,
        method: &Method,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync;

    fn execute(
        &self,
        request: reqwest::Request,
    ) -> impl Future<Output = Result<reqwest::Response, Self::Error>> + Send;
}

/// `RESTful` helpers returning a flat `Result<T, Self::Error>`.
pub trait RestClient {
    type Error;

    fn get<T>(&self, endpoint: &str) -> impl Future<Output = Result<T, Self::Error>> + Send
    where
        T: DeserializeOwned + Debug + Send + Sync;

    fn post<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync;

    fn put<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync;

    fn patch<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync;

    fn delete(&self, endpoint: &str) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

// -----------------------------------------------------------------------------
// Credentials structure

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(untagged)]
pub enum Credentials {
    OAuth1 {
        #[serde(rename = "token")]
        token: String,
        #[serde(rename = "secret")]
        secret: String,
        #[serde(rename = "consumer-key", default = "default_consumer_key")]
        consumer_key: String,
        #[serde(rename = "consumer-secret", default = "default_consumer_secret")]
        consumer_secret: String,
    },
    Basic {
        #[serde(rename = "username")]
        username: String,
        #[serde(rename = "password")]
        password: String,
    },
    Bearer {
        #[serde(rename = "token")]
        token: String,
    },
}

impl Default for Credentials {
    #[tracing::instrument(skip_all)]
    fn default() -> Self {
        Self::OAuth1 {
            token: String::new(),
            secret: String::new(),
            consumer_key: DEFAULT_CONSUMER_KEY.to_string(),
            consumer_secret: DEFAULT_CONSUMER_SECRET.to_string(),
        }
    }
}

impl From<OAuthCredentials> for Credentials {
    #[tracing::instrument(skip_all)]
    fn from(credentials: OAuthCredentials) -> Self {
        match credentials {
            OAuthCredentials::Bearer { token } => Self::Bearer {
                token: token.into(),
            },
            OAuthCredentials::Basic { username, password } => Self::Basic {
                username: username.into(),
                password: password.map(Into::into).unwrap_or_default(),
            },
            OAuthCredentials::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            } => Self::OAuth1 {
                token: token.into(),
                secret: secret.into(),
                consumer_key: consumer_key.into(),
                consumer_secret: consumer_secret.into(),
            },
        }
    }
}

impl From<Credentials> for OAuthCredentials {
    #[tracing::instrument(skip_all)]
    fn from(credentials: Credentials) -> Self {
        match credentials {
            Credentials::Bearer { token } => Self::Bearer {
                token: token.into(),
            },
            Credentials::Basic { username, password } => Self::Basic {
                username: username.into(),
                password: Some(password.into()),
            },
            Credentials::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            } => Self::OAuth1 {
                token: token.into(),
                secret: secret.into(),
                consumer_key: consumer_key.into(),
                consumer_secret: consumer_secret.into(),
            },
        }
    }
}

impl Credentials {
    #[tracing::instrument(skip_all)]
    pub fn bearer(token: String) -> Self {
        Self::Bearer { token }
    }

    #[tracing::instrument(skip_all)]
    pub fn basic(username: String, password: String) -> Self {
        Self::Basic { username, password }
    }

    #[tracing::instrument(skip_all)]
    pub fn oauth1(
        token: String,
        secret: String,
        consumer_key: String,
        consumer_secret: String,
    ) -> Self {
        Self::OAuth1 {
            token,
            secret,
            consumer_key,
            consumer_secret,
        }
    }
}

// -----------------------------------------------------------------------------
// Builder structure

#[derive(Clone, Debug, Default)]
pub struct Builder {
    endpoint: Option<String>,
    credentials: Option<Credentials>,
}

impl Builder {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn with_credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn build(self, client: reqwest::Client) -> Client {
        let endpoint = match self.endpoint {
            Some(endpoint) => endpoint,
            None => {
                if matches!(self.credentials, Some(Credentials::Bearer { .. })) {
                    PUBLIC_API_BRIDGE_ENDPOINT.to_string()
                } else {
                    PUBLIC_ENDPOINT.to_string()
                }
            }
        };

        let inner = OAuthClient::from(client)
            .with_credentials(self.credentials.map(Into::<OAuthCredentials>::into));

        Client { inner, endpoint }
    }
}

// -----------------------------------------------------------------------------
// Client structure

#[derive(Clone, Debug)]
pub struct Client {
    inner: OAuthClient,
    endpoint: String,
}

impl Request for Client {
    type Error = ClientError;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn request<T, U>(
        &self,
        method: &Method,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        let fut =
            OAuthRestClient::request::<T, U, ResponseError>(&self.inner, method, endpoint, payload);
        async move { flatten(fut.await) }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(request)))]
    fn execute(
        &self,
        request: reqwest::Request,
    ) -> impl Future<Output = Result<reqwest::Response, Self::Error>> + Send {
        let fut = self.inner.execute_request(request);
        async move { fut.await.map_err(ClientError::from) }
    }
}

impl RestClient for Client {
    type Error = ClientError;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn get<T>(&self, endpoint: &str) -> impl Future<Output = Result<T, Self::Error>> + Send
    where
        T: DeserializeOwned + Debug + Send + Sync,
    {
        let fut = OAuthRestClient::get::<T, ResponseError>(&self.inner, endpoint);
        async move { flatten(fut.await) }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn post<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        let fut = OAuthRestClient::post::<T, U, ResponseError>(&self.inner, endpoint, payload);
        async move { flatten(fut.await) }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn put<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        let fut = OAuthRestClient::put::<T, U, ResponseError>(&self.inner, endpoint, payload);
        async move { flatten(fut.await) }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn patch<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: ?Sized + Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        let fut = OAuthRestClient::patch::<T, U, ResponseError>(&self.inner, endpoint, payload);
        async move { flatten(fut.await) }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn delete(&self, endpoint: &str) -> impl Future<Output = Result<(), Self::Error>> + Send {
        let fut = OAuthRestClient::delete::<ResponseError>(&self.inner, endpoint);
        async move { flatten(fut.await) }
    }
}

/// Flattens the nested result returned by oauth10a 3.0.0's `RestClient` into the
/// historical flat `Result<T, ClientError>`.
fn flatten<T>(
    result: Result<Result<T, ErrorResponse<ResponseError>>, RestError<OAuthClientError>>,
) -> Result<T, ClientError> {
    match result {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(ErrorResponse { status_code, value })) => {
            Err(ClientError::StatusCode(status_code, value))
        }
        Err(err) => Err(ClientError::from(err)),
    }
}

impl From<reqwest::Client> for Client {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn from(client: reqwest::Client) -> Self {
        Self::builder().build(client)
    }
}

impl From<Credentials> for Client {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn from(credentials: Credentials) -> Self {
        match &credentials {
            Credentials::Bearer { .. } => Self::builder()
                .with_endpoint(PUBLIC_API_BRIDGE_ENDPOINT.to_string())
                .with_credentials(credentials)
                .build(reqwest::Client::new()),
            _ => Self::builder()
                .with_credentials(credentials)
                .build(reqwest::Client::new()),
        }
    }
}

impl Default for Client {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn default() -> Self {
        Self::builder().build(reqwest::Client::new())
    }
}

impl Client {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn new(
        client: reqwest::Client,
        endpoint: String,
        credentials: Option<Credentials>,
    ) -> Self {
        let mut builder = Self::builder().with_endpoint(endpoint);

        if let Some(credentials) = credentials {
            builder = builder.with_credentials(credentials);
        }

        builder.build(client)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn builder() -> Builder {
        Builder::default()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn set_credentials(&mut self, credentials: Option<Credentials>) {
        self.inner
            .set_credentials(credentials.map(Into::<OAuthCredentials>::into));
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn inner(&self) -> &reqwest::Client {
        self.inner.inner()
    }
}

// -----------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// A non-success status returned by the API must surface as
    /// `ClientError::StatusCode`, carrying both the status and the response body.
    ///
    /// This is the contract the operator relies on to tell "addon not found"
    /// (404) apart from a transport failure and trigger a recreate; if `flatten`
    /// ever routed a 404 through the `RestError` arm it would become
    /// `ClientError::Execute` and that reconciliation logic would silently break.
    #[test]
    fn flatten_maps_error_response_to_status_code() {
        let body = json!({ "id": 4004, "message": "addon not found" });
        let result: Result<Result<(), ErrorResponse<ResponseError>>, RestError<OAuthClientError>> =
            Ok(Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                value: body.clone(),
            }));

        match flatten(result) {
            Err(ClientError::StatusCode(status, value)) => {
                assert_eq!(status, StatusCode::NOT_FOUND);
                assert_eq!(value, body);
            }
            other => panic!("expected ClientError::StatusCode(404, _), got {other:?}"),
        }
    }

    /// A successful response is forwarded untouched.
    #[test]
    fn flatten_forwards_success() {
        let result: Result<Result<u32, ErrorResponse<ResponseError>>, RestError<OAuthClientError>> =
            Ok(Ok(42));

        assert!(matches!(flatten(result), Ok(42)));
    }

    /// Transport / (de)serialization failures travel through the `RestError` arm
    /// and must keep their own variant rather than being misreported as a status
    /// code.
    #[test]
    fn flatten_keeps_transport_errors_distinct_from_status_code() {
        let serde_err = serde_json::from_str::<u32>("not-a-number").unwrap_err();
        let result: Result<Result<(), ErrorResponse<ResponseError>>, RestError<OAuthClientError>> =
            Err(RestError::Serialize(serde_err));

        assert!(matches!(flatten(result), Err(ClientError::Serialize(_))));
    }
}
