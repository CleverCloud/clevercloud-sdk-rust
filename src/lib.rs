//! # Clever-Cloud Software Development Kit (SDK)
//!
//! This module provides a client and structures to interact with the Clever-Cloud API.

use core::{fmt, str};
use std::sync::OnceLock;

use oauth10a::{
    client::Client as OAuthClient,
    credentials::{Credentials, CredentialsBuilder},
    execute::ExecuteRequest,
    reqwest::{self, Request, Response, Url},
};

#[macro_use]
mod logging;

pub mod v2;
pub mod v4;

// TYPE ALIASES ////////////////////////////////////////////////////////////////

pub type ClientError = oauth10a::client::ClientError<reqwest::Error>;

pub type RestError = oauth10a::rest::RestError<ClientError>;

type UrlParseError = <reqwest::Url as str::FromStr>::Err;

// RE-EXPORTS //////////////////////////////////////////////////////////////////

pub use oauth10a;

// CLEVER CLOUD API ////////////////////////////////////////////////////////////

pub const PUBLIC_API_ENDPOINT: &str = "https://api.clever-cloud.com";

pub fn public_api_endpoint() -> &'static Url {
    static URL: OnceLock<Url> = OnceLock::new();

    URL.get_or_init(|| {
        PUBLIC_API_ENDPOINT
            .parse()
            .expect("valid URL for public API endpoint")
    })
}

pub const PUBLIC_API_BRIDGE_ENDPOINT: &str = "https://api-bridge.clever-cloud.com";

pub fn public_api_bridge_endpoint() -> &'static Url {
    static URL: OnceLock<Url> = OnceLock::new();

    URL.get_or_init(|| {
        PUBLIC_API_BRIDGE_ENDPOINT
            .parse()
            .expect("valid URL for public API bridge endpoint")
    })
}

// CLEVER TOOLS ////////////////////////////////////////////////////////////////

/// Default OAuth1 consumer.
#[derive(Debug)]
pub struct CleverTools;

impl CleverTools {
    // Consumer key and secret of the clever-tools are publicly available.
    // The disclosure of these tokens is not considered a vulnerability.
    // Do not report this to our security service.
    //
    // See:
    // - <https://github.com/CleverCloud/clever-tools/blob/fed085e2ba0339f55e966d7c8c6439d4dac71164/src/models/configuration.js#L128>

    pub const CONSUMER_KEY: &'static str = "T5nFjKeHH4AIlEveuGhB5S3xg8T19e";
    pub const CONSUMER_SECRET: &'static str = "MgVMqTr6fWlf2M0tkC2MXOnhfqBWDT";
}

// ENDPOINT ERROR //////////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
#[error("failed to build request's endpoint URL on API: '{api}', path: '{path}', error: {error}")]
pub struct EndpointError {
    api: Url,
    path: Box<str>,
    error: UrlParseError,
}

// CLIENT //////////////////////////////////////////////////////////////////////

/// HTTP client specialized for Clever Cloud API.
///
///
///
///
#[derive(Debug, Default, Clone)]
pub struct Client {
    inner: OAuthClient,
    api_endpoint: Option<Url>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: OAuthClient::new(),
            api_endpoint: None,
        }
    }

    /// Sets the credentials that will be used by this client to authorize subsequent HTTP requests.
    ///
    /// When `consumer_keys` and/or `consumer_secret` are missing, the client will
    /// use the values of the [`CleverTools`].
    pub fn set_credentials<T: Into<CredentialsBuilder>>(&mut self, credentials: Option<T>) {
        self.inner.set_credentials(credentials.map(|credentials| {
            credentials
                .into()
                .with_consumer(CleverTools::CONSUMER_SECRET, CleverTools::CONSUMER_SECRET)
        }));
    }

    pub fn with_credentials<T: Into<CredentialsBuilder>>(mut self, credentials: Option<T>) -> Self {
        self.set_credentials(credentials);
        self
    }

    pub fn credentials(&self) -> Option<Credentials<&str>> {
        self.inner.credentials()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn set_endpoint(&mut self, api_endpoint: Option<Url>) {
        self.api_endpoint = api_endpoint;
    }

    pub fn with_endpoint(mut self, api_endpoint: Option<Url>) -> Self {
        self.set_endpoint(api_endpoint);
        self
    }

    pub fn api_endpoint(&self) -> &Url {
        if let Some(ref url) = self.api_endpoint {
            url
        } else if let Some(Credentials::Bearer { .. }) = self.credentials() {
            public_api_bridge_endpoint()
        } else {
            public_api_endpoint()
        }
    }

    /// Joins `path` to this client's API endpoint.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub(crate) fn endpoint<T: fmt::Display + fmt::Debug>(
        &self,
        path: T,
    ) -> Result<Url, EndpointError> {
        let api = self.api_endpoint();
        let path = path.to_string();

        Url::options()
            .base_url(Some(api))
            .parse(&path)
            .map_err(|error| EndpointError {
                api: api.clone(),
                path: path.into(),
                error,
            })
    }

    pub fn inner(&self) -> &reqwest::Client {
        self.inner.inner()
    }
}

impl From<reqwest::Client> for Client {
    fn from(value: reqwest::Client) -> Self {
        Self {
            inner: oauth10a::client::Client::from(value),
            ..Default::default()
        }
    }
}

impl From<CredentialsBuilder> for Client {
    fn from(value: CredentialsBuilder) -> Self {
        Self::new().with_credentials(Some(value))
    }
}

impl From<&CredentialsBuilder> for Client {
    fn from(value: &CredentialsBuilder) -> Self {
        Self::from(value.clone())
    }
}

impl<T: Into<Box<str>>> From<Credentials<T>> for Client {
    fn from(value: Credentials<T>) -> Self {
        Self {
            inner: oauth10a::client::Client::from(value),
            ..Default::default()
        }
    }
}

impl From<&Credentials> for Client {
    fn from(value: &Credentials) -> Self {
        Self {
            inner: oauth10a::client::Client::from(value),
            ..Default::default()
        }
    }
}

impl ExecuteRequest for Client {
    type Error = ClientError;

    #[inline]
    fn execute_request(
        &self,
        request: Request,
    ) -> impl Future<Output = Result<Response, Self::Error>> + Send + 'static {
        self.inner.execute_request(request)
    }
}
