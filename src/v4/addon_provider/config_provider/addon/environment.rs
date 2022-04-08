//! # ConfigProvider addon's environment module
//!
//! This module provide helpers and structures to interact with the config
//! provider addon's environment

use std::{collections::HashMap, fmt::Debug};

use hyper::client::connect::Connect;
#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{v4::addon_provider::AddonProviderId, Client};

// -----------------------------------------------------------------------------
// Variable structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Variable {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl From<(String, String)> for Variable {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    fn from((name, value): (String, String)) -> Self {
        Self::new(name, value)
    }
}

impl Variable {
    #[cfg_attr(feature = "trace", tracing::instrument)]
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to get variables of config-provider addon '{0}', {1}")]
    Get(String, ClientError),
    #[error("failed to update variables of config-provider addon '{0}', {1}")]
    Put(String, ClientError),
}

// -----------------------------------------------------------------------------
// Helpers

/// Retrieve environment variables of the config provider addon
#[cfg_attr(feature = "trace", tracing::instrument)]
pub async fn get<C>(client: &Client<C>, id: &str) -> Result<Vec<Variable>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/addon-providers/{}/addons/{}/env",
        client.endpoint,
        AddonProviderId::ConfigProvider,
        id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to get information about the config-provider addon, path: '{}', id: '{}'", &path, id);
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::Get(id.to_string(), err))
}

/// Update environment variables of the config provider addon
#[cfg_attr(feature = "trace", tracing::instrument)]
pub async fn put<C>(
    client: &Client<C>,
    id: &str,
    variables: &Vec<Variable>,
) -> Result<Vec<Variable>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/addon-providers/{}/addons/{}/env",
        client.endpoint,
        AddonProviderId::ConfigProvider,
        id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to update information about the config-provider addon, path: '{}', id: '{}'", &path, id);
    }

    client
        .put(&path, variables)
        .await
        .map_err(|err| Error::Put(id.to_string(), err))
}

/// Insert a new environment variable into config provider
#[cfg_attr(feature = "trace", tracing::instrument)]
pub async fn insert<C>(client: &Client<C>, id: &str, var: Variable) -> Result<Vec<Variable>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    bulk_insert(client, id, &[var]).await
}

/// Insert multiple new environment variables into config provider
#[cfg_attr(feature = "trace", tracing::instrument)]
pub async fn bulk_insert<C>(
    client: &Client<C>,
    id: &str,
    vars: &[Variable],
) -> Result<Vec<Variable>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let mut v = get(client, id)
        .await?
        .iter()
        .fold(HashMap::new(), |mut acc, v| {
            acc.insert(v.name.to_owned(), v.value.to_owned());
            acc
        });

    for var in vars {
        v.insert(var.name.to_owned(), var.value.to_owned());
    }

    let v = v.iter().fold(vec![], |mut acc, (k, v)| {
        acc.push(Variable::from((k.to_owned(), v.to_owned())));
        acc
    });

    put(client, id, &v).await
}

/// Remove an environment variable from config provider
#[cfg_attr(feature = "trace", tracing::instrument)]
pub async fn remove<C>(client: &Client<C>, id: &str, name: &str) -> Result<Vec<Variable>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    bulk_remove(client, id, &[name]).await
}

/// Remove multiples environment variables from config provider
#[cfg_attr(feature = "trace", tracing::instrument)]
pub async fn bulk_remove<C>(
    client: &Client<C>,
    id: &str,
    names: &[&str],
) -> Result<Vec<Variable>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let v: Vec<_> = get(client, id)
        .await?
        .iter()
        .filter(|v| !names.contains(&v.name.as_str()))
        .cloned()
        .collect();

    put(client, id, &v).await
}
