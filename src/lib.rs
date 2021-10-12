//! # Clever-Cloud Sdk
//!
//! This module provide a client and structures to interact with clever-cloud
//! api.

pub use oauth10a::client as oauth10a;

use async_trait::async_trait;
use hyper::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::oauth10a::{ClientError, Credentials, Request, RestClient};

pub mod v2;
pub mod v4;

// -----------------------------------------------------------------------------
// Constants

pub const PUBLIC_ENDPOINT: &str = "https://api.clever-cloud.com";

// -----------------------------------------------------------------------------
// Client structures

#[derive(Clone, Debug)]
pub struct Client {
    pub inner: oauth10a::Client,
    pub endpoint: String,
}

#[async_trait]
impl Request for Client {
    type Error = ClientError;

    async fn request<T, U>(
        &self,
        method: &Method,
        endpoint: &str,
        payload: &T,
    ) -> Result<U, Self::Error>
    where
        T: Serialize + Send + Sync,
        U: DeserializeOwned + Send + Sync,
    {
        self.inner.request(method, endpoint, payload).await
    }
}

#[async_trait]
impl RestClient for Client {
    type Error = ClientError;

    async fn get<T>(&self, endpoint: &str) -> Result<T, Self::Error>
    where
        T: DeserializeOwned + Send + Sync,
    {
        self.inner.get(endpoint).await
    }

    async fn post<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Send + Sync,
        U: DeserializeOwned + Send + Sync,
    {
        self.inner.post(endpoint, payload).await
    }

    async fn put<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Send + Sync,
        U: DeserializeOwned + Send + Sync,
    {
        self.inner.put(endpoint, payload).await
    }

    async fn patch<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Send + Sync,
        U: DeserializeOwned + Send + Sync,
    {
        self.inner.patch(endpoint, payload).await
    }

    async fn delete(&self, endpoint: &str) -> Result<(), Self::Error> {
        self.inner.delete(endpoint).await
    }
}

impl From<Credentials> for Client {
    fn from(credentials: Credentials) -> Self {
        Self {
            inner: oauth10a::Client::from(credentials),
            ..Default::default()
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            inner: oauth10a::Client::default(),
            endpoint: PUBLIC_ENDPOINT.to_string(),
        }
    }
}

impl Client {
    pub fn new(endpoint: String, credentials: Option<Credentials>) -> Self {
        let mut inner = oauth10a::Client::default();
        inner.set_credentials(credentials);
        Self { inner, endpoint }
    }

    pub fn set_credentials(&mut self, credentials: Option<Credentials>) {
        self.inner.set_credentials(credentials);
    }
}
