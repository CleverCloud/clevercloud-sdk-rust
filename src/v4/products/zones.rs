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
/// returns the list of plan for the postgresql addon provider
pub async fn list(client: &Client) -> Result<Vec<Zone>, Error> {
    let path = format!("{}/v4/products/zones", client.endpoint);

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to list zones, path: '{}'", &path);
    }

    client.get(&path).await.map_err(Error::List)
}
