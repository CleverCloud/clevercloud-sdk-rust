//! # Functions module
//!
//! This module provides all structures and helpers to interact with functions
//! product at Clever Cloud.

use core::fmt;
use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use oauth10a::{
    reqwest::{self, IntoUrl, Method},
    rest::RestClient,
};
use serde::{Deserialize, Serialize};

use crate::{Client, EndpointError, RestError, v4::ErrorResponse};

pub mod deployments;

// -----------------------------------------------------------------------------
// Error

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error("failed to list functions for organisation '{0}', {1}")]
    List(String, RestError),
    #[error("failed to create function on organisation '{0}', {1}")]
    Create(String, RestError),
    #[error("failed to get function '{0}' for organisation '{1}', {2}")]
    Get(String, String, RestError),
    #[error("failed to update function '{0}' of organisation '{1}', {2}")]
    Update(String, String, RestError),
    #[error("failed to delete function '{0}' of organisation '{1}', {2}")]
    Delete(String, String, RestError),
    #[error(transparent)]
    StatusCode(#[from] ErrorResponse),

    #[error("failed to execute request, {0}")]
    Execute(reqwest::Error),
    #[error("failed to aggregate body, {0}")]
    BodyAggregation(reqwest::Error),
    #[error("failed to deserialize execute response payload, {0}")]
    Deserialize(serde_json::Error),
}

// -----------------------------------------------------------------------------
// Opts structure

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opts {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "tag")]
    pub tag: Option<String>,
    #[serde(rename = "environment")]
    pub environment: BTreeMap<String, String>,
    #[serde(rename = "maxMemory")]
    pub max_memory: u64,
    #[serde(rename = "maxInstances")]
    pub max_instances: u64,
}

impl Opts {
    pub const DEFAULT_MAX_MEMORY: u64 = 512 * 1024 * 1024;

    pub const DEFAULT_MAX_INSTANCES: u64 = 1;
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            tag: None,
            environment: BTreeMap::new(),
            max_memory: Self::DEFAULT_MAX_MEMORY,
            max_instances: Self::DEFAULT_MAX_INSTANCES,
        }
    }
}

// -----------------------------------------------------------------------------
// Function structure

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "environment")]
    pub environment: BTreeMap<String, String>,
    #[serde(rename = "maxMemory")]
    pub max_memory: u64,
    #[serde(rename = "maxInstances")]
    pub max_instances: u64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// ExecuteResult structure

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecutionResult {
    Ok {
        #[serde(rename = "stdout")]
        stdout: String,
        #[serde(rename = "stderr")]
        stderr: String,
        #[serde(rename = "dmesg")]
        dmesg: String,
        #[serde(rename = "current_pages")]
        current_pages: Option<u64>,
    },
    Err {
        #[serde(rename = "error")]
        error: String,
    },
}

impl ExecutionResult {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn ok<T, U, V>(stdout: T, stderr: U, dmesg: V, current_pages: Option<u64>) -> Self
    where
        T: fmt::Display,
        U: fmt::Display,
        V: fmt::Display,
    {
        Self::Ok {
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            dmesg: dmesg.to_string(),
            current_pages,
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn err<T>(error: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Err {
            error: error.to_string(),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok { .. })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

// -----------------------------------------------------------------------------
// Helpers

/// Returns the list of function for an organisation.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn list(client: &Client, organisation_id: &str) -> Result<Vec<Function>, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        "execute a request to list functions"
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::List(organisation_id.to_string(), e))??)
}

/// Creates a function on the given organisation.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn create(
    client: &Client,
    organisation_id: &str,
    opts: &Opts,
) -> Result<Function, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        "execute a request to create function"
    );

    Ok(client
        .post(endpoint, opts)
        .await
        .map_err(|e| Error::Create(organisation_id.to_string(), e))??)
}

/// Returns the function information of the organisation.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn get(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
) -> Result<Function, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions/{function_id}",
    ))?;

    debug!(
        %endpoint,
        organization = organisation_id,
        function = function_id,
        "execute a request to get function"
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::Get(function_id.to_string(), organisation_id.to_string(), e))??)
}

/// Updates the function information of the organisation.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn update(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
    opts: &Opts,
) -> Result<Function, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions/{function_id}",
    ))?;

    debug!(
        %endpoint,
        organization = organisation_id,
        function = function_id,
        "execute a request to update function"
    );

    Ok(client
        .put(endpoint, opts)
        .await
        .map_err(|e| Error::Update(function_id.to_string(), organisation_id.to_string(), e))??)
}

/// Returns the function information of the organisation.
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn delete(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
) -> Result<(), Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions/{function_id}",
    ))?;

    debug!(
        %endpoint,
        organization = organisation_id,
        function = function_id,
        "execute a request to delete function"
    );

    Ok(client
        .delete(endpoint)
        .await
        .map_err(|e| Error::Delete(function_id.to_string(), organisation_id.to_string(), e))??)
}

/// Execute a GET HTTP request on the given endpoint
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn execute<X: IntoUrl + fmt::Debug>(
    client: &Client,
    endpoint: X,
) -> Result<ExecutionResult, Error> {
    let url = endpoint.into_url().map_err(Error::Execute)?;

    let req = reqwest::Request::new(Method::GET, url);

    let res = client.inner().execute(req).await.map_err(Error::Execute)?;

    let buf = res.bytes().await.map_err(Error::BodyAggregation)?;

    serde_json::from_slice(&buf).map_err(Error::Deserialize)
}
