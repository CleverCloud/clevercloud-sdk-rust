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
    #[error(
        "failed to parse addon provider identifier '{0}', available options are \
        'postgresql-addon', 'redis-addon', 'mysql-addon', 'mongodb-addon', \
        'addon-pulsar', 'config-provider', 'es-addon', 'kv', 'metabase', 'keycloak', \
        'cellar-addon', 'addon-matomo', 'addon-otoroshi' and 'azimutt'"
    )]
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
    KV,
    ConfigProvider,
    ElasticSearch,
    Metabase,
    Keycloak,
    Cellar,
    Matomo,
    Otoroshi,
    Azimutt,
}

impl AddonProviderId {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::PostgreSql => "postgresql-addon",
            Self::Redis => "redis-addon",
            Self::MySql => "mysql-addon",
            Self::MongoDb => "mongodb-addon",
            Self::Pulsar => "addon-pulsar",
            Self::KV => "kv",
            Self::ConfigProvider => "config-provider",
            Self::ElasticSearch => "es-addon",
            Self::Metabase => "metabase",
            Self::Keycloak => "keycloak",
            Self::Cellar => "cellar-addon",
            Self::Matomo => "addon-matomo",
            Self::Otoroshi => "otoroshi",
            Self::Azimutt => "azimutt",
        }
    }
}

impl FromStr for AddonProviderId {
    type Err = Error;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "postgresql-addon" => Self::PostgreSql,
            "redis-addon" => Self::Redis,
            "mysql-addon" => Self::MySql,
            "mongodb-addon" => Self::MongoDb,
            "addon-pulsar" => Self::Pulsar,
            "kv" => Self::KV,
            "config-provider" => Self::ConfigProvider,
            "es-addon" => Self::ElasticSearch,
            "metabase" => Self::Metabase,
            "cellar-addon" => Self::Cellar,
            "keycloak" => Self::Keycloak,
            "addon-matomo" => Self::Matomo,
            "otoroshi" => Self::Otoroshi,
            "azimutt" => Self::Azimutt,
            _ => return Err(Error::Parse(s.to_owned())),
        })
    }
}

impl TryFrom<String> for AddonProviderId {
    type Error = Error;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for AddonProviderId {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn into(self) -> String {
        self.to_string()
    }
}

impl Display for AddonProviderId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}
