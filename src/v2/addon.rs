use std::collections::BTreeMap;

#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Client;

// -----------------------------------------------------------------------------
// Provider structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
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
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
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
// AddonOpts enum

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord, Clone, Debug, Default)]
pub struct AddonOpts {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "encryption", skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
}

// -----------------------------------------------------------------------------
// CreateAddonOpts structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub struct CreateAddonOpts {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "plan")]
    pub plan: String,
    #[serde(rename = "options")]
    pub options: AddonOpts,
}

// -----------------------------------------------------------------------------
// EnvironmentVariable struct

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub struct EnvironmentVariable {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the list of addons for the given organisation
pub async fn list(client: &Client, organisation_id: &str) -> Result<Vec<Addon>, ClientError> {
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

    client.get(&path).await
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the addon for the given the organisation and identifier
pub async fn get(client: &Client, organisation_id: &str, id: &str) -> Result<Addon, ClientError> {
    let path = format!(
        "{}/v2/organisations/{}/addons/{}",
        client.endpoint, organisation_id, id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to get information about an addon, path: '{}', organisation: '{}', id: '{}'", &path, organisation_id, id);
    }

    client.get(&path).await
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// create the addon and returns it
pub async fn create(
    client: &Client,
    organisation_id: &str,
    opts: &CreateAddonOpts,
) -> Result<Addon, ClientError> {
    let path = format!(
        "{}/v2/organisations/{}/addons",
        client.endpoint, organisation_id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to create an addon, path: '{}', organisation: '{}', name: '{}', region: '{}', plan: '{}', provider-id: '{}'", &path, organisation_id, &opts.name, &opts.region, &opts.plan, &opts.provider_id.to_string());
    }

    client.post(&path, opts).await
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// delete the given addon
pub async fn delete(client: &Client, organisation_id: &str, id: &str) -> Result<(), ClientError> {
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

    client.delete(&path).await
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns environment variables for an addon
pub async fn environment(
    client: &Client,
    organisation_id: &str,
    id: &str,
) -> Result<BTreeMap<String, String>, ClientError> {
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

    let env: Vec<EnvironmentVariable> = client.get(&path).await?;
    Ok(env.iter().fold(BTreeMap::new(), |mut acc, var| {
        acc.insert(var.name.to_owned(), var.value.to_owned());
        acc
    }))
}
