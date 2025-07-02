//! # MongoDb addon provider module
//!
//! This module provides helpers and structures to interact with the mongodb
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
    #[error("failed to parse version from '{0}', available version is 4.0.3")]
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
    V4dot0dot3 = 403,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "4.0.3" => Self::V4dot0dot3,
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
            Self::V4dot0dot3 => write!(f, "4.0.3"),
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers functions

/// returns information about the mongodb addon provider
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn get(client: &Client) -> Result<AddonProvider<Version>, Error> {
    const ADDON_PROVIDER_ID: AddonProviderId = AddonProviderId::MongoDb;

    let endpoint = client.endpoint(format_args!("/v4/addon-providers/{ADDON_PROVIDER_ID}"))?;

    debug!(
        %endpoint,
        addon_provider = %ADDON_PROVIDER_ID,
        "execute a request to get information about the mongodb addon-provider"
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::Get(ADDON_PROVIDER_ID, e))??)
}
