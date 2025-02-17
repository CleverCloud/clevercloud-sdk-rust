//! # Clever-Cloud Sdk
//!
//! This module provides a client and structures to interact with clever-cloud
//! api.

use std::fmt::Debug;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::oauth10a::{
    reqwest::{self, Method},
    Client as OAuthClient, ClientError, Credentials, Request, RestClient,
};

pub mod v2;
pub mod v4;

// -----------------------------------------------------------------------------
// Exports

pub use oauth10a::client as oauth10a;

// -----------------------------------------------------------------------------
// Constants

pub const PUBLIC_ENDPOINT: &str = "https://api.clever-cloud.com";
pub const PUBLIC_OAUTHLESS_ENDPOINT: &str = "https://oauthless-api.clever-cloud.com";

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
                    PUBLIC_ENDPOINT.to_string()
                } else {
                    PUBLIC_OAUTHLESS_ENDPOINT.to_string()
                }
            }
        };

        Client::new(client, endpoint, self.credentials)
    }
}

// -----------------------------------------------------------------------------
// Client structure

#[derive(Clone, Debug)]
pub struct Client {
    inner: OAuthClient,
    endpoint: String,
}

#[async_trait]
impl Request for Client {
    type Error = ClientError;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn request<T, U>(
        &self,
        method: &Method,
        endpoint: &str,
        payload: &T,
    ) -> Result<U, Self::Error>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.request(method, endpoint, payload).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn execute(&self, request: reqwest::Request) -> Result<reqwest::Response, Self::Error> {
        self.inner.execute(request).await
    }
}

#[async_trait]
impl RestClient for Client {
    type Error = ClientError;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn get<T>(&self, endpoint: &str) -> Result<T, Self::Error>
    where
        T: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.get(endpoint).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn post<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.post(endpoint, payload).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn put<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.put(endpoint, payload).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn patch<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.patch(endpoint, payload).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn delete(&self, endpoint: &str) -> Result<(), Self::Error> {
        self.inner.delete(endpoint).await
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
                .with_credentials(credentials)
                .with_endpoint(PUBLIC_OAUTHLESS_ENDPOINT.to_string())
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
        Self {
            inner: OAuthClient::new(client, credentials),
            endpoint,
        }
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
        self.inner.set_credentials(credentials);
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn inner(&self) -> &reqwest::Client {
        self.inner.inner()
    }
}
