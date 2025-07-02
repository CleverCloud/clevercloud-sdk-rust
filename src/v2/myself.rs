//! # Myself module
//!
//! This module provides structures and helpers to interact with the user api
//! version 2

use oauth10a::rest::RestClient;
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Client, EndpointError, RestError, v2::ErrorResponse};

// -----------------------------------------------------------------------------
// Myself structure and helpers

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Myself {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "phone")]
    pub phone: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "zipcode")]
    pub zipcode: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "avatar")]
    pub avatar: String,
    #[serde(rename = "creationDate")]
    pub creation_date: u64,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "emailValidated")]
    pub email_validated: bool,
    #[serde(rename = "oauthApps")]
    pub oauth_apps: Vec<String>,
    #[serde(rename = "admin")]
    pub admin: bool,
    #[serde(rename = "canPay")]
    pub can_pay: bool,
    #[serde(rename = "preferredMFA")]
    pub preferred_mfa: String,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
}

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error("failed to get information about the current user, {0}")]
    Get(RestError),
    #[error(transparent)]
    StatusCode(#[from] ErrorResponse),
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns information about the person logged in
pub async fn get(client: &Client) -> Result<Myself, Error> {
    let endpoint = client.endpoint("/v2/self")?;

    debug!(
        %endpoint,
        "execute a request to get information about the logged in user"
    );

    Ok(client.get(endpoint).await.map_err(Error::Get)??)
}
