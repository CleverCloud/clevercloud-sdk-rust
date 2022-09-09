//! # Addon module
//!
//! This module expose structures and helpers to interact with the addon api
//! version 2

use std::{collections::BTreeMap, fmt::Debug};

use hyper::client::connect::Connect;
#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{v4::addon_provider::config_provider::addon::environment::Variable, Client};

// -----------------------------------------------------------------------------
// Provider structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Provider {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "website")]
    pub website: String,
    #[serde(rename = "supportEmail")]
    pub support_email: String,
    #[serde(rename = "googlePlusName")]
    pub google_plus_name: String,
    #[serde(rename = "twitterName")]
    pub twitter_name: String,
    #[serde(rename = "analyticsId")]
    pub analytics_id: String,
    #[serde(rename = "shortDesc")]
    pub short_description: String,
    #[serde(rename = "longDesc")]
    pub long_description: String,
    #[serde(rename = "logoUrl")]
    pub logo_url: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "openInNewTab")]
    pub open_in_new_tab: bool,
    #[serde(rename = "canUpgrade")]
    pub can_upgrade: bool,
    #[serde(rename = "regions")]
    pub regions: Vec<String>,
}

// -----------------------------------------------------------------------------
// Feature structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Feature {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "computable_value")]
    pub computable_value: Option<String>,
    #[serde(rename = "name_code")]
    pub name_code: Option<String>,
}

// -----------------------------------------------------------------------------
// Plan structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub struct Plan {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "price")]
    pub price: f32,
    #[serde(rename = "price_id")]
    pub price_id: String,
    #[serde(rename = "features")]
    pub features: Vec<Feature>,
    #[serde(rename = "zones")]
    pub zones: Vec<String>,
}

// -----------------------------------------------------------------------------
// Addon structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub struct Addon {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "realId")]
    pub real_id: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "plan")]
    pub plan: Plan,
    #[serde(rename = "creationDate")]
    pub creation_date: u64,
    #[serde(rename = "configKeys")]
    pub config_keys: Vec<String>,
}

// -----------------------------------------------------------------------------
// Opts enum

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Default)]
pub struct Opts {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "encryption", skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<String>,
}

// -----------------------------------------------------------------------------
// CreateOpts structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct CreateOpts {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "plan")]
    pub plan: String,
    #[serde(rename = "options")]
    pub options: Opts,
}

// -----------------------------------------------------------------------------
// Error enumerations

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to list addons of organisation '{0}', {1}")]
    List(String, ClientError),
    #[error("failed to get addon '{0}' of organisation '{1}', {2}")]
    Get(String, String, ClientError),
    #[error("failed to get addon '{0}' environment of organisation '{1}', {2}")]
    Environment(String, String, ClientError),
    #[error("failed to create addon for organisation '{0}', {1}")]
    Create(String, ClientError),
    #[error("failed to delete addon '{0}' for organisation '{1}', {2}")]
    Delete(String, String, ClientError),
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the list of addons for the given organisation
pub async fn list<C>(client: &Client<C>, organisation_id: &str) -> Result<Vec<Addon>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v2/organisations/{}/addons",
        client.endpoint, organisation_id,
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get the list of addons, path: '{}', organisation: '{}'",
            &path, organisation_id
        );
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::List(organisation_id.to_owned(), err))
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the addon for the given the organisation and identifier
pub async fn get<C>(client: &Client<C>, organisation_id: &str, id: &str) -> Result<Addon, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v2/organisations/{}/addons/{}",
        client.endpoint, organisation_id, id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to get information about an addon, path: '{}', organisation: '{}', id: '{}'", &path, organisation_id, id);
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::Get(id.to_owned(), organisation_id.to_owned(), err))
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// create the addon and returns it
pub async fn create<C>(
    client: &Client<C>,
    organisation_id: &str,
    opts: &CreateOpts,
) -> Result<Addon, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v2/organisations/{}/addons",
        client.endpoint, organisation_id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to create an addon, path: '{}', organisation: '{}', name: '{}', region: '{}', plan: '{}', provider-id: '{}'", &path, organisation_id, &opts.name, &opts.region, &opts.plan, &opts.provider_id.to_string());
    }

    client
        .post(&path, opts)
        .await
        .map_err(|err| Error::Create(organisation_id.to_owned(), err))
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// delete the given addon
pub async fn delete<C>(client: &Client<C>, organisation_id: &str, id: &str) -> Result<(), Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v2/organisations/{}/addons/{}",
        client.endpoint, organisation_id, id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to delete an addon, path: '{}', organisation: '{}', id: '{}'",
            &path, organisation_id, id
        );
    }

    client
        .delete(&path)
        .await
        .map_err(|err| Error::Delete(id.to_owned(), organisation_id.to_owned(), err))
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns environment variables for an addon
pub async fn environment<C>(
    client: &Client<C>,
    organisation_id: &str,
    id: &str,
) -> Result<BTreeMap<String, String>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v2/organisations/{}/addons/{}/env",
        client.endpoint, organisation_id, id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get secret of a addon, path: '{}', organisation: '{}', id: '{}'",
            &path, organisation_id, id
        );
    }

    let env: Vec<Variable> = client
        .get(&path)
        .await
        .map_err(|err| Error::Environment(id.to_owned(), organisation_id.to_owned(), err))?;

    Ok(env.iter().fold(BTreeMap::new(), |mut acc, var| {
        acc.insert(var.name.to_owned(), var.value.to_owned());
        acc
    }))
}
