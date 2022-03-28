//! # Postgresql addon provider module
//!
//! This module provide helpers and structures to interact with the postgresql
//! addon provider

use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use hyper::client::connect::Connect;
#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema_repr as JsonSchemaRepr;
use serde_repr::{Deserialize_repr as DeserializeRepr, Serialize_repr as SerializeRepr};

use crate::{
    v4::addon_provider::{AddonProvider, AddonProviderId},
    Client,
};

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse version from '{0}', available versions are 13, 12, 11 and 10")]
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
    V14 = 14,
    V13 = 13,
    V12 = 12,
    V11 = 11,
    V10 = 10,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "14" => Self::V14,
            "13" => Self::V13,
            "12" => Self::V12,
            "11" => Self::V11,
            "10" => Self::V10,
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
            Self::V14 => write!(f, "14"),
            Self::V13 => write!(f, "13"),
            Self::V12 => write!(f, "12"),
            Self::V11 => write!(f, "11"),
            Self::V10 => write!(f, "10"),
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns information about the postgresql addon provider
pub async fn get<C>(client: &Client<C>) -> Result<AddonProvider<Version>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/addon-providers/{}",
        client.endpoint,
        AddonProviderId::PostgreSql
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to get information about the postgresql addon-provider, path: '{}', name: '{}'", &path, AddonProviderId::PostgreSql);
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::Get(AddonProviderId::PostgreSql, err))
}
