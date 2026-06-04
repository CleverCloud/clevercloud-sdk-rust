//! # MySql addon provider module
//!
//! This module provides helpers and structures to interact with the mysql
//! addon provider

use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::oauth10a::{ClientError, RestClient};
#[cfg(feature = "logging")]
use log::{Level, debug, log_enabled};
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
    #[error("failed to parse version from '{0}', available versions are 5.7, 8.0 and 8.4")]
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
    V5dot7 = 57,
    V8dot0 = 80,
    V8dot4 = 84,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "5.7" => Self::V5dot7,
            "8.0" => Self::V8dot0,
            "8.4" => Self::V8dot4,
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
            Self::V5dot7 => write!(f, "5.7"),
            Self::V8dot0 => write!(f, "8.0"),
            Self::V8dot4 => write!(f, "8.4"),
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers functions

/// returns information about the mysql addon provider
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn get(client: &Client) -> Result<AddonProvider<Version>, Error> {
    let path = format!(
        "{}/v4/addon-providers/{}",
        client.endpoint,
        AddonProviderId::MySql
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get information about the mysql addon-provider, path: '{}', name: '{}'",
            &path,
            AddonProviderId::MySql
        );
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::Get(AddonProviderId::MySql, err))
}

// -----------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Version;

    #[test]
    fn version_string_round_trip() {
        for (s, version) in [
            ("5.7", Version::V5dot7),
            ("8.0", Version::V8dot0),
            ("8.4", Version::V8dot4),
        ] {
            assert_eq!(Version::from_str(s).unwrap(), version);
            assert_eq!(version.to_string(), s);
        }
    }

    #[test]
    fn version_rejects_unknown() {
        assert!(Version::from_str("5.6").is_err());
    }
}
