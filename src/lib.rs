//! # Clever-Cloud Software Development Kit (SDK)
//!
//! This crate provides an HTTP client and structures to interact with the Clever-Cloud API.

use core::fmt;
use std::sync::OnceLock;

use oauth10a::{
    authorize::Authorize,
    credentials::{AuthorizationError, Credentials},
    execute::ExecuteRequest,
    reqwest::{self, Request, Response, Url},
};

use crate::clever_tools::CleverTools;

#[macro_use]
pub mod logging;
pub mod clever_tools;
pub mod v2;
pub mod v4;

// TYPE ALIASES ////////////////////////////////////////////////////////////////

pub type RestError = oauth10a::rest::RestError<ClientError>;

pub type ClientError = oauth10a::client::ClientError<reqwest::Error, AuthorizationError>;

type UrlParseError = <reqwest::Url as core::str::FromStr>::Err;

/// Credentials that supports missing OAuth 1.0a consumer details.
pub type PartialCredentials = Credentials<Box<str>, Option<Box<str>>>;

type OAuthClient = oauth10a::client::Client<reqwest::Client, Authorizer>;

// RE-EXPORTS //////////////////////////////////////////////////////////////////

pub use oauth10a;

// CLEVER CLOUD API ////////////////////////////////////////////////////////////

pub const DEFAULT_API_HOST: &str = "https://api.clever-cloud.com";

pub fn default_api_host() -> &'static Url {
    static URL: OnceLock<Url> = OnceLock::new();

    URL.get_or_init(|| {
        DEFAULT_API_HOST
            .parse()
            .expect("valid URL for default API host")
    })
}

pub const DEFAULT_AUTH_BRIDGE_HOST: &str = "https://api-bridge.clever-cloud.com";

pub fn default_auth_bridge_host() -> &'static Url {
    static URL: OnceLock<Url> = OnceLock::new();

    URL.get_or_init(|| {
        DEFAULT_AUTH_BRIDGE_HOST
            .parse()
            .expect("valid URL for default auth bridge host")
    })
}

pub const DEFAULT_SSH_GATEWAY: &str =
    "ssh@sshgateway-clevercloud-customers.services.clever-cloud.com'";

// ENDPOINT ERROR //////////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
#[error("failed to build request's endpoint URL on API: '{api}', path: '{path}', error: {error}")]
pub struct EndpointError {
    api: Url,
    path: Box<str>,
    error: UrlParseError,
}

// INTO PARTIAL CREDENTIALS ////////////////////////////////////////////////////

/// Utility trait for types that can be used to create Credentials for Clever Cloud's HTTP [`Client`].
pub trait IntoPartialCredentials: fmt::Debug {
    fn into_partial_credentials(self) -> PartialCredentials;
}

impl<T: Into<Box<str>>> IntoPartialCredentials for Credentials<T, Option<T>> {
    fn into_partial_credentials(self) -> PartialCredentials {
        match self {
            Self::Bearer { token } => Credentials::Bearer {
                token: token.into(),
            },
            Self::Basic { username, password } => Credentials::Basic {
                username: username.into(),
                password: password.map(Into::into),
            },
            Self::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            } => Credentials::OAuth1 {
                token: token.into(),
                secret: secret.into(),
                consumer_key: consumer_key.map(Into::into),
                consumer_secret: consumer_secret.map(Into::into),
            },
        }
    }
}

impl<T: Into<Box<str>>> IntoPartialCredentials for Credentials<T> {
    #[inline]
    fn into_partial_credentials(self) -> PartialCredentials {
        match self {
            Self::Bearer { token } => Credentials::Bearer {
                token: token.into(),
            },
            Self::Basic { username, password } => Credentials::Basic {
                username: username.into(),
                password: password.map(Into::into),
            },
            Self::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            } => Credentials::OAuth1 {
                token: token.into(),
                secret: secret.into(),
                consumer_key: Some(consumer_key.into()),
                consumer_secret: Some(consumer_secret.into()),
            },
        }
    }
}

impl<T: IntoPartialCredentials + Clone> IntoPartialCredentials for &T {
    #[inline]
    fn into_partial_credentials(self) -> PartialCredentials {
        self.clone().into_partial_credentials()
    }
}

// AUTHORIZER //////////////////////////////////////////////////////////////////

/// HTTP Request [`Authorize`] implementation that supports missing OAuth 1.0a consumer details.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Authorizer {
    credentials: Option<PartialCredentials>,
}

impl Authorizer {
    #[inline(always)]
    pub const fn new(credentials: Option<PartialCredentials>) -> Self {
        Self { credentials }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline(always)]
    pub fn set_credentials(&mut self, credentials: PartialCredentials) {
        let _ = self.credentials.replace(credentials);
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline(always)]
    pub fn clear_credentials(&mut self) {
        let _ = self.credentials.take();
    }

    #[inline(always)]
    pub const fn credentials(&self) -> Option<Credentials<&str>> {
        match &self.credentials {
            None => None,
            Some(Credentials::OAuth1 {
                token,
                secret,
                consumer_key,
                consumer_secret,
            }) => Some(Credentials::OAuth1 {
                token,
                secret,
                consumer_key: match consumer_key {
                    None => CleverTools::CONSUMER_KEY,
                    Some(v) => v,
                },
                consumer_secret: match consumer_secret {
                    None => CleverTools::CONSUMER_SECRET,
                    Some(v) => v,
                },
            }),
            Some(Credentials::Basic { username, password }) => Some(Credentials::Basic {
                username,
                password: match password {
                    None => None,
                    Some(v) => Some(v),
                },
            }),
            Some(Credentials::Bearer { token }) => Some(Credentials::Bearer { token }),
        }
    }
}

impl Authorize for Authorizer {
    type Error = <Credentials as Authorize>::Error;

    #[inline(always)]
    fn authorize(&self, request: &mut Request) -> Result<bool, Self::Error> {
        match self.credentials() {
            None => Ok(false),
            Some(credentials) => credentials.authorize(request),
        }
    }
}

impl Drop for Authorizer {
    fn drop(&mut self) {
        use zeroize::Zeroize;

        if let Some(mut credentials) = self.credentials.take() {
            credentials.zeroize();
        }
    }
}

// CLIENT //////////////////////////////////////////////////////////////////////

/// HTTP client specialized for interacting with the Clever Cloud API.
#[derive(Debug, Clone)]
pub struct Client {
    client: OAuthClient,
    api_host: Option<Url>,
    auth_bridge_host: Option<Url>,
}

impl Default for Client {
    fn default() -> Self {
        Self::from(reqwest::Client::default())
    }
}

impl From<reqwest::Client> for Client {
    fn from(value: reqwest::Client) -> Self {
        Self {
            client: OAuthClient::new(value, Authorizer::default()),
            api_host: None,
            auth_bridge_host: None,
        }
    }
}

impl Client {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn new(
        client: reqwest::Client,
        credentials: Option<PartialCredentials>,
        api_host: Option<Url>,
        auth_bridge_host: Option<Url>,
    ) -> Self {
        Self {
            api_host,
            auth_bridge_host,
            client: OAuthClient::new(client, Authorizer::new(credentials)),
        }
    }

    /// Sets the credentials that will be used by this client to authorize subsequent HTTP requests.
    ///
    /// When `consumer_keys` and/or `consumer_secret` are missing, the client will
    /// use the values of the [`CleverTools`].
    pub fn set_credentials(&mut self, credentials: impl IntoPartialCredentials) {
        self.client
            .authorizer_mut()
            .set_credentials(credentials.into_partial_credentials());
    }

    /// Fills the `credentials` to be used by the client to authorize HTTP request,
    /// discarding the current value, if any.
    ///
    /// When `consumer_keys` and/or `consumer_secret` are missing, the client will
    /// use the credentials of the default consumer: [`CleverTools`].
    pub fn with_credentials(mut self, credentials: impl IntoPartialCredentials) -> Self {
        self.set_credentials(credentials);
        self
    }

    pub fn clear_credentials(&mut self) {
        self.client.authorizer_mut().clear_credentials();
    }

    pub fn credentials(&self) -> Option<Credentials<&str>> {
        self.client.authorizer().credentials()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn set_api_host(&mut self, api_host: impl Into<Option<Url>> + fmt::Debug) {
        self.api_host = api_host.into();
    }

    pub fn with_api_host(mut self, api_host: impl Into<Option<Url>> + fmt::Debug) -> Self {
        self.set_api_host(api_host);
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn set_auth_bridge_host(&mut self, auth_bridge_host: impl Into<Option<Url>> + fmt::Debug) {
        self.auth_bridge_host = auth_bridge_host.into();
    }

    pub fn with_auth_bridge_host(
        mut self,
        auth_bridge_host: impl Into<Option<Url>> + fmt::Debug,
    ) -> Self {
        self.set_auth_bridge_host(auth_bridge_host);
        self
    }

    pub const fn api_host(&self) -> Option<&Url> {
        self.api_host.as_ref()
    }

    pub const fn auth_bridge_host(&self) -> Option<&Url> {
        self.auth_bridge_host.as_ref()
    }

    pub fn api_endpoint(&self) -> &Url {
        match self.credentials() {
            Some(Credentials::Bearer { .. }) => match self.auth_bridge_host {
                Some(ref url) => url,
                None => default_auth_bridge_host(),
            },
            _ => match self.api_host {
                Some(ref url) => url,
                None => default_api_host(),
            },
        }
    }

    /// Joins `path` to this client's API endpoint.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub(crate) fn endpoint(
        &self,
        path: impl fmt::Display + fmt::Debug,
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
        self.client.executer()
    }
}

impl<T: IntoPartialCredentials + fmt::Debug> From<T> for Client {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn from(value: T) -> Self {
        Client::default().with_credentials(value)
    }
}

impl ExecuteRequest for Client {
    type Error = ClientError;

    #[inline(always)]
    fn execute_request(
        &self,
        request: Request,
    ) -> impl Future<Output = Result<Response, Self::Error>> + Send + 'static {
        self.client.execute_request(request)
    }
}
