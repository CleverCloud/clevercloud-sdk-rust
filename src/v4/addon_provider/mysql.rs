//! # MySql addon provider module
//!
//! This module provide helpers and structures to interact with the mysql
//! addon provider

use std::{
    convert::TryFrom,
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

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
// Version enum

#[cfg_attr(feature = "jsonschemas", derive(JsonSchemaRepr))]
#[derive(SerializeRepr, DeserializeRepr, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(untagged)]
#[repr(i32)]
pub enum Version {
    V5dot7 = 57,
    V8dot0 = 80,
}

impl FromStr for Version {
    type Err = Box<dyn Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "5.7" => Self::V5dot7,
            "8.0" => Self::V8dot0,
            _ => {
                return Err(format!(
                    "failed to parse version from {}, available versions are 5.7 and 8.0",
                    s
                )
                .into());
            }
        })
    }
}

impl TryFrom<String> for Version {
    type Error = Box<dyn Error + Send + Sync>;

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
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns information about the mysql addon provider
pub async fn get(client: &Client) -> Result<AddonProvider<Version>, ClientError> {
    let path = format!(
        "{}/v4/addon-providers/{}",
        client.endpoint,
        AddonProviderId::MySql
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to get information about the mysql addon-provider, path: '{}', name: '{}'", &path, AddonProviderId::MySql.to_string());
    }

    client.get(&path).await
}
