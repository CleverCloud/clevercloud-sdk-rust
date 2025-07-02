//! # Addon module
//!
//! This module expose structures and helpers to interact with the addon api
//! version 2

use std::collections::BTreeMap;

use oauth10a::rest::RestClient;
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Client, EndpointError, RestError, v2::ErrorResponse,
    v4::addon_provider::config_provider::addon::environment::Variable,
};

// -----------------------------------------------------------------------------
// Provider structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    #[serde(default, rename = "plans")]
    pub plans: Vec<Plan>,
}

// -----------------------------------------------------------------------------
// Feature structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    pub price_id: Option<String>,
    #[serde(rename = "features")]
    pub features: Vec<Feature>,
    #[serde(rename = "zones")]
    pub zones: Vec<String>,
}

// -----------------------------------------------------------------------------
// Addon structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error("failed to list addons of organisation '{0}', {1}")]
    List(String, RestError),
    #[error("failed to get addon '{0}' of organisation '{1}', {2}")]
    Get(String, String, RestError),
    #[error("failed to get addon '{0}' environment of organisation '{1}', {2}")]
    Environment(String, String, RestError),
    #[error("failed to create addon for organisation '{0}', {1}")]
    Create(String, RestError),
    #[error("failed to delete addon '{0}' for organisation '{1}', {2}")]
    Delete(String, String, RestError),
    #[error(transparent)]
    StatusCode(#[from] ErrorResponse),
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns the list of addons for the given organisation
pub async fn list(client: &Client, organisation_id: &str) -> Result<Vec<Addon>, Error> {
    let endpoint = client.endpoint(format_args!("/v2/organisations/{organisation_id}/addons"))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        "execute a request to get the list of addons"
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::List(organisation_id.to_owned(), e))??)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns the addon for the given the organisation and identifier
pub async fn get(client: &Client, organisation_id: &str, addon_id: &str) -> Result<Addon, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v2/organisations/{organisation_id}/addons/{addon_id}"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        addon = addon_id,
        "execute a request to get information about an addon",
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::Get(addon_id.to_owned(), organisation_id.to_owned(), e))??)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// create the addon and returns it
pub async fn create(
    client: &Client,
    organisation_id: &str,
    opts: &CreateOpts,
) -> Result<Addon, Error> {
    let endpoint = client.endpoint(format_args!("/v2/organisations/{organisation_id}/addons"))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        name = opts.name,
        region = opts.region,
        plan = opts.plan,
        provider_id = opts.provider_id,
        "execute a request to create an addon",

    );

    Ok(client
        .post(endpoint, opts)
        .await
        .map_err(|e| Error::Create(organisation_id.to_owned(), e))??)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// delete the given addon
pub async fn delete(client: &Client, organisation_id: &str, addon_id: &str) -> Result<(), Error> {
    let endpoint = client.endpoint(format_args!(
        "/v2/organisations/{organisation_id}/addons/{addon_id}"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        addon = addon_id,
        "execute a request to delete an addon",
    );

    Ok(client
        .delete(endpoint)
        .await
        .map_err(|e| Error::Delete(addon_id.to_owned(), organisation_id.to_owned(), e))??)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns environment variables for an addon
pub async fn environment(
    client: &Client,
    organisation_id: &str,
    addon_id: &str,
) -> Result<BTreeMap<String, String>, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v2/organisations/{organisation_id}/addons/{addon_id}/env"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        addon = addon_id,
        "execute a request to get secret of a addon"
    );

    let env: Vec<Variable> = client
        .get(endpoint)
        .await
        .map_err(|e| Error::Environment(addon_id.to_owned(), organisation_id.to_owned(), e))??;

    Ok(env.into_iter().map(|var| (var.name, var.value)).collect())
}
