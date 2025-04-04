//! # ElasticSearch addon provider module
//!
//! This module provides helpers and structures to interact with the elasticsearch
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
    #[error("failed to parse version from '{0}', available version are 7 and 8")]
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
    V7 = 7,
    V8 = 8,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "7" => Self::V7,
            "8" => Self::V8,
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::V7 => write!(f, "7"),
            Self::V8 => write!(f, "8"),
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers functions

/// returns information about the elasticsearch addon provider
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn get(client: &Client) -> Result<AddonProvider<Version>, Error> {
    let path = format!(
        "{}/v4/addon-providers/{}",
        client.endpoint,
        AddonProviderId::ElasticSearch
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get information about the elasticsearch addon-provider, path: '{}', name: '{}'",
            &path,
            AddonProviderId::ElasticSearch
        );
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::Get(AddonProviderId::ElasticSearch, err))
}
