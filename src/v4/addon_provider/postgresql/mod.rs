//! # Postgresql addon provider module
//!
//! This module provide helpers and structures to interact with the postgresql
//! addon provider

use std::{
    collections::BTreeMap,
    convert::TryFrom,
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::{JsonSchema, JsonSchema_repr as JsonSchemaRepr};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr as DeserializeRepr, Serialize_repr as SerializeRepr};

use crate::{
    v4::addon_provider::{AddonProviderId, Feature},
    Client,
};

pub mod plan;

// -----------------------------------------------------------------------------
// Version enum

#[cfg_attr(feature = "jsonschemas", derive(JsonSchemaRepr))]
#[derive(SerializeRepr, DeserializeRepr, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(untagged)]
#[repr(u8)]
pub enum Version {
    V13 = 13,
    V12 = 12,
    V11 = 11,
    V10 = 10,
    V9dot6 = 96,
}

impl FromStr for Version {
    type Err = Box<dyn Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "13" => Self::V13,
            "12" => Self::V12,
            "11" => Self::V11,
            "10" => Self::V10,
            "9.6" => Self::V9dot6,
            _ => {
                return Err(format!("failed to parse version from {}, available versions are 13, 12, 11, 10 and 9.6", s).into());
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
            Self::V13 => write!(f, "13"),
            Self::V12 => write!(f, "12"),
            Self::V11 => write!(f, "11"),
            Self::V10 => write!(f, "10"),
            Self::V9dot6 => write!(f, "9.6"),
        }
    }
}

// -----------------------------------------------------------------------------
// Cluster structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Cluster {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "zone")]
    pub zone: String,
    #[serde(rename = "version")]
    pub version: Version,
    #[serde(rename = "features")]
    pub features: Vec<Feature>,
}

// -----------------------------------------------------------------------------
// AddonProvider structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AddonProvider {
    #[serde(rename = "providerId")]
    pub provider_id: AddonProviderId,
    #[serde(rename = "clusters")]
    pub clusters: Vec<Cluster>,
    #[serde(rename = "dedicated")]
    pub dedicated: BTreeMap<Version, Vec<Feature>>,
    #[serde(rename = "defaultDedicatedVersion")]
    pub default: Version,
}

// -----------------------------------------------------------------------------
// Helpers functions

/// returns information about the postgresql addon provider
pub async fn get(client: &Client) -> Result<AddonProvider, ClientError> {
    let path = format!(
        "{}/v4/addon-providers/{}",
        client.endpoint,
        AddonProviderId::PostgreSql
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to get information about the postgresql addon-provider, path: '{}', name: '{}'", &path, AddonProviderId::PostgreSql.to_string());
    }

    Ok(client.get(&path).await?)
}
