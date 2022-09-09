//! # Addon provider module
//!
//! This module provide structures and helpers to interact with clever-cloud's
//! addon-provider

use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    str::FromStr,
};

#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod config_provider;
pub mod elasticsearch;
pub mod mongodb;
pub mod mysql;
pub mod plan;
pub mod postgresql;
pub mod redis;

// -----------------------------------------------------------------------------
// Feature structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Feature {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

// -----------------------------------------------------------------------------
// Cluster structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Cluster<T> {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "zone")]
    pub zone: String,
    #[serde(rename = "features")]
    pub features: Vec<Feature>,
    #[serde(rename = "version")]
    pub version: T,
}

// -----------------------------------------------------------------------------
// AddonProvider structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AddonProvider<T>
where
    T: Ord,
{
    #[serde(rename = "providerId")]
    pub provider_id: AddonProviderId,
    #[serde(rename = "clusters")]
    pub clusters: Vec<Cluster<T>>,
    #[serde(rename = "dedicated")]
    pub dedicated: BTreeMap<T, Vec<Feature>>,
    #[serde(rename = "defaultDedicatedVersion")]
    pub default: T,
}

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse addon provider identifier {0}, available options are 'addon-pulsar', 'postgresql-addon', 'mysql-addon', 'mongodb-addon' or 'redis-addon'")]
    Parse(String),
}

// -----------------------------------------------------------------------------
// AddonProviderName structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
#[serde(untagged, try_from = "String", into = "String")]
pub enum AddonProviderId {
    PostgreSql,
    Redis,
    MySql,
    MongoDb,
    Pulsar,
    ConfigProvider,
    ElasticSearch,
}

impl FromStr for AddonProviderId {
    type Err = Error;

    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "mysql-addon" => Self::MySql,
            "redis-addon" => Self::Redis,
            "postgresql-addon" => Self::PostgreSql,
            "mongodb-addon" => Self::MongoDb,
            "addon-pulsar" => Self::Pulsar,
            "config-provider" => Self::ConfigProvider,
            "es-addon" => Self::ElasticSearch,
            _ => return Err(Error::Parse(s.to_owned())),
        })
    }
}

impl TryFrom<String> for AddonProviderId {
    type Error = Error;

    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for AddonProviderId {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn into(self) -> String {
        self.to_string()
    }
}

impl Display for AddonProviderId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::PostgreSql => write!(f, "postgresql-addon"),
            Self::Redis => write!(f, "redis-addon"),
            Self::MySql => write!(f, "mysql-addon"),
            Self::MongoDb => write!(f, "mongodb-addon"),
            Self::Pulsar => write!(f, "addon-pulsar"),
            Self::ConfigProvider => write!(f, "config-provider"),
            Self::ElasticSearch => write!(f, "es-addon"),
        }
    }
}
