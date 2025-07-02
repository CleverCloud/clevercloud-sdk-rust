//! # Redis addon provider module
//!
//! This module provide helpers and structures to interact with the redis
//! addon provider
#![allow(deprecated)]

use core::{fmt, str::FromStr};

use oauth10a::rest::RestClient;
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema_repr as JsonSchemaRepr;
use serde_repr::{Deserialize_repr as DeserializeRepr, Serialize_repr as SerializeRepr};

use crate::{
    Client, EndpointError, RestError,
    v4::{
        ErrorResponse,
        addon_provider::{AddonProvider, AddonProviderId},
    },
};

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error("failed to parse version from {0}, available version is 7.2.4")]
    ParseVersion(String),
    #[error("failed to get information about addon provider '{0}', {1}")]
    Get(AddonProviderId, RestError),
    #[error(transparent)]
    StatusCode(#[from] ErrorResponse),
}

// -----------------------------------------------------------------------------
// Version enum

#[cfg_attr(feature = "jsonschemas", derive(JsonSchemaRepr))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, SerializeRepr, DeserializeRepr)]
#[serde(untagged)]
#[repr(i32)]
pub enum Version {
    V7dot2dot4 = 724,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "7.2.4" => Self::V7dot2dot4,
            _ => {
                return Err(Error::ParseVersion(s.to_owned()));
            }
        })
    }
}

impl TryFrom<String> for Version {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for Version {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V7dot2dot4 => write!(f, "7.2.4"),
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns information about the redis addon provider
pub async fn get(client: &Client) -> Result<AddonProvider<Version>, Error> {
    const ADDON_PROVIDER_ID: AddonProviderId = AddonProviderId::Redis;

    let endpoint = client.endpoint(format_args!("/v4/addon-providers/{ADDON_PROVIDER_ID}",))?;

    debug!(
        %endpoint,
        name = %ADDON_PROVIDER_ID,
        "execute a request to get information about the redis addon-provider"
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::Get(ADDON_PROVIDER_ID, e))??)
}
