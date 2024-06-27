//! # Deployment module
//!
//! This module provides structures to interact with functions' deployments.

use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Body, Method,
};
use log::{debug, log_enabled, Level};
use oauth10a::client::{connector::Connect, ClientError, Request, RestClient};
use serde::{Deserialize, Serialize};

use crate::Client;

// -----------------------------------------------------------------------------
// Constants

pub const MIME_APPLICATION_WASM: &str = "application/wasm";

// ----------------------------------------------------------------------------
// Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse the webassembly platform '{0}', available values are 'rust', 'javascript' ('js'), 'tiny_go' ('go') and 'assemblyscript'")]
    ParsePlatform(String),
    #[error("failed to parse the status '{0}', available values are 'waiting_for_upload', 'deploying', 'packaging', 'ready' and 'error'")]
    ParseStatus(String),
    #[error("failed to list deployments for function '{0}' of organisation '{1}', {2}")]
    List(String, String, ClientError),
    #[error("failed to create deployment for function '{0}' on organisation '{1}', {2}")]
    Create(String, String, ClientError),
    #[error("failed to get deployment '{0}' of function '{1}' on organisation '{2}', {3}")]
    Get(String, String, String, ClientError),
    #[error("failed to trigger deployment '{0}' of function '{1}' on organisation '{2}', {3}")]
    Trigger(String, String, String, ClientError),
    #[error("failed to delete deployment '{0}' of function '{1}' on organisation '{2}', {3}")]
    Delete(String, String, String, ClientError),
    #[error("failed to create request, {0}")]
    Request(hyper::http::Error),
    #[error("failed to execute request, {0}")]
    Execute(ClientError),
    #[error("failed to execute request, got status code {0}")]
    StatusCode(u16),
}

// ----------------------------------------------------------------------------
// Platform

#[derive(Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub enum Platform {
    #[serde(rename = "RUST")]
    Rust,
    #[serde(rename = "ASSEMBLY_SCRIPT")]
    AssemblyScript,
    #[serde(rename = "TINY_GO")]
    TinyGo,
    #[serde(rename = "JAVA_SCRIPT")]
    JavaScript,
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Rust => write!(f, "RUST"),
            Self::AssemblyScript => write!(f, "ASSEMBLY_SCRIPT"),
            Self::JavaScript => write!(f, "JAVA_SCRIPT"),
            Self::TinyGo => write!(f, "TINY_GO"),
        }
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim().replace('_', "").as_str() {
            "rust" => Ok(Self::Rust),
            "javascript" | "js" => Ok(Self::JavaScript),
            "tinygo" | "go" => Ok(Self::TinyGo),
            "assemblyscript" => Ok(Self::AssemblyScript),
            _ => Err(Error::ParsePlatform(s.to_string())),
        }
    }
}

// ----------------------------------------------------------------------------
// Status

#[derive(Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub enum Status {
    #[serde(rename = "WAITING_FOR_UPLOAD")]
    WaitingForUpload,
    #[serde(rename = "PACKAGING")]
    Packaging,
    #[serde(rename = "PACKAGING")]
    Deploying,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "ERROR")]
    Error,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::WaitingForUpload => write!(f, "WAITING_FOR_UPLOAD"),
            Self::Packaging => write!(f, "PACKAGING"),
            Self::Deploying => write!(f, "DEPLOYING"),
            Self::Ready => write!(f, "READY"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim().replace('_', "").as_str() {
            "waitingforupload" => Ok(Self::WaitingForUpload),
            "packaging" => Ok(Self::Packaging),
            "deploying" => Ok(Self::Deploying),
            "ready" => Ok(Self::Ready),
            "error" => Ok(Self::Error),
            _ => Err(Error::ParseStatus(s.to_string())),
        }
    }
}

// ----------------------------------------------------------------------------
// Opts

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Opts {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "tag")]
    pub tag: Option<String>,
    #[serde(rename = "platform")]
    pub platform: Platform,
}

// ----------------------------------------------------------------------------
// DeploymentCreation

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DeploymentCreation {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "functionId")]
    pub function_id: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "tag")]
    pub tag: Option<String>,
    #[serde(rename = "platform")]
    pub platform: Platform,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "errorReason")]
    pub reason: Option<String>,
    #[serde(rename = "uploadUrl")]
    pub upload_url: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

// ----------------------------------------------------------------------------
// Deployment

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Deployment {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "functionId")]
    pub function_id: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "tag")]
    pub tag: Option<String>,
    #[serde(rename = "platform")]
    pub platform: Platform,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "errorReason")]
    pub reason: Option<String>,
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

// ----------------------------------------------------------------------------
// Helpers

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the list of deployments for a function
pub async fn list<C>(
    client: &Client<C>,
    organisation_id: &str,
    function_id: &str,
) -> Result<Vec<Deployment>, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to list deployments for functions, path: '{path}', organisation: '{organisation_id}', function_id: '{function_id}'");
    }

    client
        .get(&path)
        .await
        .map_err(|err| Error::List(function_id.to_string(), organisation_id.to_string(), err))
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// create a deployment on the given function
pub async fn create<C>(
    client: &Client<C>,
    organisation_id: &str,
    function_id: &str,
    opts: &Opts,
) -> Result<DeploymentCreation, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to create deployment, path: '{path}', organisation: {organisation_id}, function_id: '{function_id}'"
        );
    }

    client
        .post(&path, opts)
        .await
        .map_err(|err| Error::Create(function_id.to_string(), organisation_id.to_string(), err))
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns the deployment information of the function
pub async fn get<C>(
    client: &Client<C>,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<Deployment, Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments/{deployment_id}",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get deployment, path: '{path}', organisation: {organisation_id}, function: {function_id}, deployment: {deployment_id}"
        );
    }

    client.get(&path).await.map_err(|err| {
        Error::Get(
            deployment_id.to_string(),
            function_id.to_string(),
            organisation_id.to_string(),
            err,
        )
    })
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// trigger the deployment of the function once the WebAssembly has been uploaded
pub async fn trigger<C>(
    client: &Client<C>,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<(), Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments/{deployment_id}/trigger",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get deployment, path: '{path}', organisation: {organisation_id}, function: {function_id}, deployment: {deployment_id}"
        );
    }

    let req = hyper::Request::builder()
        .method(&Method::POST)
        .uri(&path)
        .body(Body::empty())
        .map_err(Error::Request)?;

    let res = client.execute(req).await.map_err(Error::Execute)?;
    let status = res.status();
    if !status.is_success() {
        return Err(Error::StatusCode(status.as_u16()));
    }

    Ok(())
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// Upload the WebAssembly on the endpoint
pub async fn upload<C>(client: &Client<C>, endpoint: &str, buf: Vec<u8>) -> Result<(), Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let req = hyper::Request::builder()
        .method(Method::PUT)
        .uri(endpoint)
        .header(CONTENT_TYPE, MIME_APPLICATION_WASM.to_string())
        .header(CONTENT_LENGTH, buf.len())
        .body(Body::from(buf))
        .map_err(Error::Request)?;

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!("execute a request to upload webassembly, endpoint: '{endpoint}'");
    }

    let res = client
        .inner()
        .request(req)
        .await
        .map_err(|err| Error::Execute(ClientError::Request(err)))?;

    let status = res.status();
    if !status.is_success() {
        return Err(Error::StatusCode(status.as_u16()));
    }

    Ok(())
}

#[cfg_attr(feature = "trace", tracing::instrument)]
/// delete the deployment from the function
pub async fn delete<C>(
    client: &Client<C>,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<(), Error>
where
    C: Connect + Clone + Debug + Send + Sync + 'static,
{
    let path = format!(
        "{}/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments/{deployment_id}",
        client.endpoint
    );

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to delete deployment, path: '{path}', organisation: {organisation_id}, function: {function_id}, deployment: {deployment_id}"
        );
    }

    client.delete(&path).await.map_err(|err| {
        Error::Delete(
            deployment_id.to_string(),
            function_id.to_string(),
            organisation_id.to_string(),
            err,
        )
    })
}
