//! # Functions module
//!
//! This module provides all structures and helpers to interact with functions
//! product at Clever Cloud.

use std::{collections::BTreeMap, fmt::Debug};

use chrono::{DateTime, Utc};
use log::{Level, debug, log_enabled};
use oauth10a::client::{
    ClientError, RestClient,
    bytes::Buf,
    reqwest::{self, Method},
    url,
};
use serde::{Deserialize, Serialize};

use crate::Client;

pub mod deployments;

// -----------------------------------------------------------------------------
// Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse endpoint '{0}', {1}")]
    ParseUrl(String, url::ParseError),
    #[error("failed to list functions for organisation '{0}', {1}")]
    List(String, ClientError),
    #[error("failed to create function on organisation '{0}', {1}")]
    Create(String, ClientError),
    #[error("failed to get function '{0}' for organisation '{1}', {2}")]
    Get(String, String, ClientError),
    #[error("failed to update function '{0}' of organisation '{1}', {2}")]
    Update(String, String, ClientError),
    #[error("failed to delete function '{0}' of organisation '{1}', {2}")]
    Delete(String, String, ClientError),
    #[error("failed to aggregate body, {0}")]
    BodyAggregation(reqwest::Error),
    #[error("failed to deserialize execute response payload, {0}")]
    Deserialize(serde_json::Error),
    #[error("failed to execute request, {0}")]
    Execute(reqwest::Error),
    #[error("failed to execute request, got status code {0}")]
    StatusCode(u16),
}

// -----------------------------------------------------------------------------
// CreateOpts structure

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
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

impl Default for Opts {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            tag: None,
            environment: BTreeMap::new(),
            max_memory: 512 * 1024 * 1024,
            max_instances: 1,
        }
    }
}

// -----------------------------------------------------------------------------
// Function structure

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
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
        T: ToString,
        U: ToString,
        V: ToString,
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
        T: ToString,
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

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns the list of function for an organisation
pub async fn list(client: &Client, organisation_id: &str) -> Result<Vec<Function>, Error> {
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to list functions for organisation, path: '{path}', organisation: '{organisation_id}'"
        );
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::List(organisation_id.to_string(), err))
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// create a function on the given organisation
pub async fn create(
    client: &Client,
    organisation_id: &str,
    opts: &Opts,
) -> Result<Function, Error> {
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to create function, path: '{path}', organisation: {organisation_id}"
        );
    }

    client
        .post(&path, opts)
        .await
        .map_err(|err| Error::Create(organisation_id.to_string(), err))
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns the function information of the organisation
pub async fn get(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
) -> Result<Function, Error> {
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get function, path: '{path}', organisation: {organisation_id}, function: {function_id}"
        );
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::Get(function_id.to_string(), organisation_id.to_string(), err))
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// Update the function information of the organisation
pub async fn update(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
    opts: &Opts,
) -> Result<Function, Error> {
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to update function, path: '{path}', organisation: {organisation_id}, function: {function_id}"
        );
    }

    client
        .put(&path, opts)
        .await
        .map_err(|err| Error::Update(function_id.to_string(), organisation_id.to_string(), err))
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns the function information of the organisation
pub async fn delete(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
) -> Result<(), Error> {
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to delete function, path: '{path}', organisation: {organisation_id}, function: {function_id}"
        );
    }

    client
        .delete(&path)
        .await
        .map_err(|err| Error::Delete(function_id.to_string(), organisation_id.to_string(), err))
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// Execute a GET HTTP request on the given endpoint
pub async fn execute(client: &Client, endpoint: &str) -> Result<ExecutionResult, Error> {
    let req = reqwest::Request::new(
        Method::GET,
        endpoint
            .parse()
            .map_err(|err| Error::ParseUrl(endpoint.to_string(), err))?,
    );

    let res = client.inner().execute(req).await.map_err(Error::Execute)?;
    let buf = res.bytes().await.map_err(Error::BodyAggregation)?;

    serde_json::from_reader(buf.reader()).map_err(Error::Deserialize)
}
