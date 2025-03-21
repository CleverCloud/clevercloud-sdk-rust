//! # Redis addon provider module
//!
//! This module provide helpers and structures to interact with the redis
//! addon provider
#![allow(deprecated)]

use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "logging")]
use log::{Level, debug, log_enabled};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema_repr as JsonSchemaRepr;
use serde_repr::{Deserialize_repr as DeserializeRepr, Serialize_repr as SerializeRepr};

use crate::{
    Client,
    v4::addon_provider::{AddonProvider, AddonProviderId},
};

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse version from {0}, available version is 7.2.4")]
    ParseVersion(String),
    #[error("failed to get information about addon provider '{0}', {1}")]
    Get(AddonProviderId, ClientError),
}

// -----------------------------------------------------------------------------
// Version enum

#[cfg_attr(feature = "jsonschemas", derive(JsonSchemaRepr))]
#[derive(SerializeRepr, DeserializeRepr, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
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

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    let path = format!(
        "{}/v4/addon-providers/{}",
        client.endpoint,
        AddonProviderId::Redis
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get information about the redis addon-provider, path: '{}', name: '{}'",
            &path,
            AddonProviderId::Redis
        );
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::Get(AddonProviderId::Redis, err))
}
