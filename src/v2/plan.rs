//! # Addon provider plan module
//!
//! This module provides helpers and structures to interact with the plan api of
//! the addon providers

#[cfg(feature = "logging")]
use log::{Level, debug, log_enabled};
use oauth10a::client::{ClientError, RestClient};

use crate::{
    Client,
    v2::addon::{Plan, Provider},
    v4::addon_provider::AddonProviderId,
};

// -----------------------------------------------------------------------------
// Constants

/// Config Provider addon have an unique and hard-coded plan as it is free to use
pub const CONFIG_PROVIDER: &str = "plan_5d8e9596-dd73-4b73-84d9-e165372c5324";

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to fetch list of addon providers, {0}")]
    List(ClientError),
    #[error("failed to fetch details of addon provider '{0}'")]
    Get(AddonProviderId),
    #[error("failed to find plan '{0}' for addon provider '{1}' amongst available options: {2}")]
    Plan(String, AddonProviderId, String),
}

// -----------------------------------------------------------------------------
// Helpers method

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// Returns the list of details relative to the addon providers.
pub async fn list(client: &Client) -> Result<Vec<Provider>, Error> {
    let path = format!("{}/v2/products/addonproviders", client.endpoint);

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to list plans of the addon-provider, path: '{path}'");
    }

    client.get(&path).await.map_err(Error::List)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// Returns the plan matching `pattern` for the given addon provider, if any.
///
/// # Errors
///
/// * [`Error::List`]: failed to fetch list of details relative to addon providers.
/// * [`Error::Get`]: failed to fetch details of addon provider.
/// * [`Error::Plan`]: failed to find plan.
pub async fn find(
    client: &Client,
    addon_provider_id: &AddonProviderId,
    pattern: &str,
) -> Result<Option<Plan>, Error> {
    let providers = list(client).await?;
    let addon_provider_id_str = addon_provider_id.as_str();

    // Find the provider matching the addon provider id
    let provider = providers
        .into_iter()
        .find(|provider| provider.id == addon_provider_id_str)
        .ok_or(Error::Get(addon_provider_id.to_owned()))?;

    // It seems that some addon providers may validly not have plans
    // NOTE: this is error prone, maybe we should set the field to `Option<Vec<Plan>>` instead
    if provider.plans.is_empty() {
        return Ok(None);
    }

    // Find the plan matching the pattern
    if let Some(plan) = provider.plans.iter().find(|plan| {
        plan.slug.eq_ignore_ascii_case(pattern)
            || plan.name.eq_ignore_ascii_case(pattern)
            || plan.id.eq_ignore_ascii_case(pattern)
    }) {
        return Ok(Some(plan.to_owned()));
    }

    // No match
    Err(Error::Plan(
        pattern.to_owned(),
        addon_provider_id.to_owned(),
        provider
            .plans
            .into_iter()
            .map(|plan| format!("'{}' ('{}')", plan.name, plan.slug))
            .collect::<Vec<_>>()
            .join(", "),
    ))
}
