//! # Clever-Cloud Sdk
//!
//! This module provide a client and structures to interact with clever-cloud
//! api.

use std::{fmt::Debug, marker::PhantomData};

pub use oauth10a::client as oauth10a;

use async_trait::async_trait;
use hyper::{Body, Method, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::oauth10a::{
    connector::{Connect, GaiResolver, HttpConnector, HttpsConnector, HttpsConnectorBuilder},
    ClientError, Credentials, Request, RestClient,
};

pub mod v2;
pub mod v4;

// -----------------------------------------------------------------------------
// Constants

pub const PUBLIC_ENDPOINT: &str = "https://api.clever-cloud.com";

// -----------------------------------------------------------------------------
// Builder structure

#[derive(Clone, Debug)]
pub struct Builder<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    endpoint: Option<String>,
    credentials: Option<Credentials>,
    phantom: PhantomData<C>,
}

impl<C> Default for Builder<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            endpoint: None,
            credentials: None,
            phantom: PhantomData::default(),
        }
    }
}

impl<C> Builder<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn with_credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn build(self, connector: C) -> Client<C> {
        let endpoint = match self.endpoint {
            Some(endpoint) => endpoint,
            None => PUBLIC_ENDPOINT.to_string(),
        };

        Client::<C>::new(connector, endpoint, self.credentials)
    }
}

// -----------------------------------------------------------------------------
// Client structure

#[derive(Clone, Debug)]
pub struct Client<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    inner: oauth10a::Client<C>,
    endpoint: String,
}

#[async_trait]
impl<C> Request for Client<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    type Error = ClientError;

    #[cfg_attr(feature = "trace", tracing::instrument)]
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

    #[cfg_attr(feature = "trace", tracing::instrument)]
    async fn execute(&self, request: hyper::Request<Body>) -> Result<Response<Body>, Self::Error> {
        self.inner.execute(request).await
    }
}

#[async_trait]
impl<C> RestClient for Client<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    type Error = ClientError;

    #[cfg_attr(feature = "trace", tracing::instrument)]
    async fn get<T>(&self, endpoint: &str) -> Result<T, Self::Error>
    where
        T: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.get(endpoint).await
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    async fn post<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.post(endpoint, payload).await
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    async fn put<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.put(endpoint, payload).await
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    async fn patch<T, U>(&self, endpoint: &str, payload: &T) -> Result<U, Self::Error>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.patch(endpoint, payload).await
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    async fn delete(&self, endpoint: &str) -> Result<(), Self::Error> {
        self.inner.delete(endpoint).await
    }
}

impl<C> From<C> for Client<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn from(connector: C) -> Self {
        Self::builder().build(connector)
    }
}

impl From<Credentials> for Client<HttpsConnector<HttpConnector<GaiResolver>>> {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn from(credentials: Credentials) -> Self {
        Self::builder().with_credentials(credentials).build(
            HttpsConnectorBuilder::new()
                .with_webpki_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        )
    }
}

impl Default for Client<HttpsConnector<HttpConnector<GaiResolver>>> {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn default() -> Self {
        Self::builder().build(
            HttpsConnectorBuilder::new()
                .with_webpki_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        )
    }
}

impl<C> Client<C>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn new(connector: C, endpoint: String, credentials: Option<Credentials>) -> Self {
        Self {
            inner: oauth10a::Client::<C>::new(connector, credentials),
            endpoint,
        }
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn builder() -> Builder<C> {
        Builder::<C>::default()
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn set_credentials(&mut self, credentials: Option<Credentials>) {
        self.inner.set_credentials(credentials);
    }

    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn inner(&self) -> &hyper::Client<C> {
        self.inner.inner()
    }
}
