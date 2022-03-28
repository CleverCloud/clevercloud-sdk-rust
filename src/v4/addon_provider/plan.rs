//! # Postgresql addon provider plan module
//!
//! This module provide helpers and structures to interact with the plan api of
//! the postgresql addon provider

use std::fmt::Debug;

use hyper::client::connect::Connect;
#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{v2::addon::Feature, v4::addon_provider::AddonProviderId, Client};

// -----------------------------------------------------------------------------
// Constants

/// Config Provider addon have an unique and hard-coded plan as it is free to use
pub const CONFIG_PROVIDER: &str = "plan_5d8e9596-dd73-4b73-84d9-e165372c5324";

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
    pub price: f64,
    #[serde(rename = "price_id")]
    pub price_id: String,
    #[serde(rename = "features")]
    pub features: Vec<Feature>,
    #[serde(rename = "zones")]
    pub zones: Vec<String>,
}

// -----------------------------------------------------------------------------
// AddonProviderPlan structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub struct AddonProviderPlan {
    #[serde(rename = "id")]
    pub id: AddonProviderId,
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
    #[serde(rename = "plans")]
    pub plans: Vec<Plan>,
}

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to list plan of addon provider '{0}' of organisation '{1}', {2}")]
    List(AddonProviderId, String, ClientError),
}

// -----------------------------------------------------------------------------
// Helpers method

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the list of plan for the postgresql addon provider
pub async fn list<C>(
    client: &Client<C>,
    addon_provider_id: &AddonProviderId,
    organisation_id: &str,
) -> Result<AddonProviderPlan, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v2/products/addonproviders/{}?orga_id={}",
        client.endpoint, addon_provider_id, organisation_id
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to list plans of the addon-provider, path: '{}', name: '{}'",
            &path, addon_provider_id
        );
    }

    client.get(&path).await.map_err(|err| {
        Error::List(
            addon_provider_id.to_owned(),
            organisation_id.to_owned(),
            err,
        )
    })
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// list plans for the organisation and try to find one matching the pattern
/// returns the plan if found
pub async fn find<C>(
    client: &Client<C>,
    addon_provider_id: &AddonProviderId,
    organisation_id: &str,
    pattern: &str,
) -> Result<Option<Plan>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    Ok(list(client, addon_provider_id, organisation_id)
        .await?
        .plans
        .iter()
        .find(|plan| plan.slug == pattern || plan.id == pattern || plan.name == pattern)
        .map(ToOwned::to_owned))
}
