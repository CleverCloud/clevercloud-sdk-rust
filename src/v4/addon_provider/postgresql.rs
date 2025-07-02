//! # Postgresql addon provider module
//!
//! This module provide helpers and structures to interact with the postgresql
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
    #[error(
        "failed to parse version from '{0}', available versions are 17, 16, 15, 14, 13, 12 and 11"
    )]
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
    V11 = 11,
    V12 = 12,
    V13 = 13,
    V14 = 14,
    V15 = 15,
    V16 = 16,
    V17 = 17,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "17" => Self::V17,
            "16" => Self::V16,
            "15" => Self::V15,
            "14" => Self::V14,
            "13" => Self::V13,
            "12" => Self::V12,
            "11" => Self::V11,
            _ => return Err(Error::ParseVersion(s.to_owned())),
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::V17 => write!(f, "17"),
            Self::V16 => write!(f, "16"),
            Self::V15 => write!(f, "15"),
            Self::V14 => write!(f, "14"),
            Self::V13 => write!(f, "13"),
            Self::V12 => write!(f, "12"),
            Self::V11 => write!(f, "11"),
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers functions

/// Returns information about the postgresql addon provider.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn get(client: &Client) -> Result<AddonProvider<Version>, Error> {
    const ADDON_PROVIDER_ID: AddonProviderId = AddonProviderId::PostgreSql;

    let endpoint = client.endpoint(format_args!("/v4/addon-providers/{ADDON_PROVIDER_ID}"))?;

    debug!(
        %endpoint,
        addon_provider = %ADDON_PROVIDER_ID,
        "execute a request to get information about the postgresql addon-provider"
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::Get(ADDON_PROVIDER_ID, e))??)
}
