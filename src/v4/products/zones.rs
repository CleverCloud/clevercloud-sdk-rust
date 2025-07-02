//! # Zones module
//!
//! This module provide helpers and structures to interact with zones of products

use oauth10a::rest::RestClient;

#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Client, EndpointError, RestError, v4::ErrorResponse};

// -----------------------------------------------------------------------------
// Constants

pub const TAG_APPLICATION: &str = "for:applications";
pub const TAG_HDS: &str = "certification:hds";

// -----------------------------------------------------------------------------
// Zone structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Zone {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "countryCode")]
    pub coutry_code: String,
    #[serde(rename = "lat")]
    pub latitude: f64,
    #[serde(rename = "lon")]
    pub longitude: f64,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error("failed to list available zones, {0}")]
    List(#[from] RestError),
    #[error(transparent)]
    Status(#[from] ErrorResponse),
}

// -----------------------------------------------------------------------------
// List zones

/// Returns the list of available zones.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn list(client: &Client) -> Result<Vec<Zone>, Error> {
    let endpoint = client.endpoint("/v4/products/zones")?;

    debug!(%endpoint, "execute a request to list zones");

    Ok(client.get(endpoint).await??)
}

/// Returns the list of zones available for applications and addons.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn applications(client: &Client) -> Result<Vec<Zone>, Error> {
    Ok(list(client)
        .await?
        .into_iter()
        .filter(|zone| zone.tags.contains(&TAG_APPLICATION.to_string()))
        .collect())
}

/// Returns the list of zones available for applications and addons with
/// Health Data Hosting (HDS) certification.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn hds(client: &Client) -> Result<Vec<Zone>, Error> {
    Ok(list(client)
        .await?
        .into_iter()
        .filter(|zone| zone.tags.contains(&TAG_APPLICATION.to_string()))
        .filter(|zone| zone.tags.contains(&TAG_HDS.to_string()))
        .collect())
}
