//! # Zones module
//!
//! This module provide helpers and structures to interact with zones of products

#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Client;

// -----------------------------------------------------------------------------
// Constants

pub const TAG_APPLICATION: &str = "for:applications";
pub const TAG_HDS: &str = "certification:hds";

// -----------------------------------------------------------------------------
// Zone structure

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to list available zones, {0}")]
    List(ClientError),
}

// -----------------------------------------------------------------------------
// List zones

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the list of zones availables
pub async fn list(client: &Client) -> Result<Vec<Zone>, Error> {
    let path = format!("{}/v4/products/zones", client.endpoint);

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to list zones, path: '{}'", &path);
    }

    client.get(&path).await.map_err(Error::List)
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// applications returns the list of zones availables for applications and addons
pub async fn applications(client: &Client) -> Result<Vec<Zone>, Error> {
    Ok(list(client)
        .await?
        .iter()
        .filter(|zone| zone.tags.contains(&TAG_APPLICATION.to_string()))
        .map(ToOwned::to_owned)
        .collect())
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// hds returns the list of zones availables for applications and addons with
/// hds certification
pub async fn hds(client: &Client) -> Result<Vec<Zone>, Error> {
    Ok(list(client)
        .await?
        .iter()
        .filter(|zone| zone.tags.contains(&TAG_APPLICATION.to_string()))
        .filter(|zone| zone.tags.contains(&TAG_HDS.to_string()))
        .map(ToOwned::to_owned)
        .collect())
}
