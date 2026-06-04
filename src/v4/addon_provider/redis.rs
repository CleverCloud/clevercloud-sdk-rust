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
    #[error("failed to parse version from {0}, available versions are 8.8.0 and 7.2.4")]
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
    V8dot8dot0 = 880,
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "8.8.0" => Self::V8dot8dot0,
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
            Self::V8dot8dot0 => write!(f, "8.8.0"),
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

// -----------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Version;

    #[test]
    fn version_string_round_trip() {
        for (s, version) in [
            ("8.8.0", Version::V8dot8dot0),
            ("7.2.4", Version::V7dot2dot4),
        ] {
            assert_eq!(Version::from_str(s).unwrap(), version);
            assert_eq!(version.to_string(), s);
        }
    }

    #[test]
    fn version_rejects_unknown() {
        assert!(Version::from_str("7.2").is_err());
    }
}
