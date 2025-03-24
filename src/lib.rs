//! # Clever-Cloud Sdk
//!
//! This module provides a client and structures to interact with clever-cloud
//! api.

use std::fmt::Debug;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::oauth10a::{
    Client as OAuthClient, ClientError, Request, RestClient,
    reqwest::{self, Method},
};

pub mod v2;
pub mod v4;

// -----------------------------------------------------------------------------
// Exports

pub use oauth10a::client as oauth10a;

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

impl From<oauth10a::Credentials> for Credentials {
    #[tracing::instrument(skip_all)]
    fn from(credentials: oauth10a::Credentials) -> Self {
        match credentials {
            oauth10a::Credentials::Bearer { token } => Self::Bearer { token },
            oauth10a::Credentials::Basic { username, password } => {
                Self::Basic { username, password }
            }
            oauth10a::Credentials::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            } => Self::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            },
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<oauth10a::Credentials> for Credentials {
    #[tracing::instrument(skip_all)]
    fn into(self) -> oauth10a::Credentials {
        match self {
            Self::Bearer { token } => oauth10a::Credentials::Bearer { token },
            Self::Basic { username, password } => {
                oauth10a::Credentials::Basic { username, password }
            }
            Self::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            } => oauth10a::Credentials::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
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

        Client {
            inner: OAuthClient::new(client, self.credentials.map(Into::into)),
            endpoint,
        }
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
    ) -> impl Future<Output = Result<U, Self::Error>>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.request(method, endpoint, payload)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn execute(
        &self,
        request: reqwest::Request,
    ) -> impl Future<Output = Result<reqwest::Response, Self::Error>> {
        self.inner.execute(request)
    }
}

impl RestClient for Client {
    type Error = ClientError;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn get<T>(&self, endpoint: &str) -> impl Future<Output = Result<T, Self::Error>>
    where
        T: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.get(endpoint)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn post<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.post(endpoint, payload)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn put<T, U>(&self, endpoint: &str, payload: &T) -> impl Future<Output = Result<U, Self::Error>>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.put(endpoint, payload)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn patch<T, U>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> impl Future<Output = Result<U, Self::Error>>
    where
        T: Serialize + Debug + Send + Sync,
        U: DeserializeOwned + Debug + Send + Sync,
    {
        self.inner.patch(endpoint, payload)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn delete(&self, endpoint: &str) -> impl Future<Output = Result<(), Self::Error>> {
        self.inner.delete(endpoint)
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
        self.inner.set_credentials(credentials.map(Into::into));
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn inner(&self) -> &reqwest::Client {
        self.inner.inner()
    }
}
