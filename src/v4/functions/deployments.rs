//! # Deployment module
//!
//! This module provides structures to interact with functions' deployments.

use core::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use oauth10a::{
    execute::ExecuteRequest,
    reqwest::{
        self, Body, IntoUrl, Method,
        header::{self, HeaderValue},
    },
    rest::RestClient,
};
use serde::{Deserialize, Serialize};

use crate::{Client, ClientError, EndpointError, RestError, v4::ErrorResponse};

// -----------------------------------------------------------------------------
// Constants

pub const MIME_APPLICATION_WASM: HeaderValue = HeaderValue::from_static("application/wasm");

// ----------------------------------------------------------------------------
// Error

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error(
        "failed to parse the webassembly platform '{0}', available values are 'rust', 'javascript' ('js'), 'tiny_go' ('go') and 'assemblyscript'"
    )]
    ParsePlatform(String),
    #[error(
        "failed to parse the status '{0}', available values are 'waiting_for_upload', 'deploying', 'packaging', 'ready' and 'error'"
    )]
    ParseStatus(String),
    #[error("failed to list deployments for function '{0}' of organisation '{1}', {2}")]
    List(String, String, RestError),
    #[error("failed to create deployment for function '{0}' on organisation '{1}', {2}")]
    Create(String, String, RestError),
    #[error("failed to get deployment '{0}' of function '{1}' on organisation '{2}', {3}")]
    Get(String, String, String, RestError),
    #[error("failed to trigger deployment '{0}' of function '{1}' on organisation '{2}', {3}")]
    Trigger(String, String, String, RestError),
    #[error("failed to delete deployment '{0}' of function '{1}' on organisation '{2}', {3}")]
    Delete(String, String, String, RestError),
    #[error("failed to create request, {0}")]
    Request(#[from] RestError),
    #[error("failed to execute request, {0}")]
    Execute(#[from] ClientError),
    #[error(transparent)]
    StatusCode(#[from] ErrorResponse),
}

// ----------------------------------------------------------------------------
// Platform

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
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

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "WAITING_FOR_UPLOAD")]
    WaitingForUpload,
    #[serde(rename = "PACKAGING")]
    Packaging,
    #[serde(rename = "DEPLOYING")]
    Deploying,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "ERROR")]
    Error,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns the list of deployments for a function
pub async fn list(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
) -> Result<Vec<Deployment>, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        function = function_id,
        "execute a request to list deployments for functions"
    );

    Ok(client
        .get(endpoint)
        .await
        .map_err(|e| Error::List(function_id.to_string(), organisation_id.to_string(), e))??)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// create a deployment on the given function
pub async fn create(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
    opts: &Opts,
) -> Result<DeploymentCreation, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        function = function_id,
        "execute a request to create deployment"
    );

    Ok(client
        .post(endpoint, opts)
        .await
        .map_err(|e| Error::Create(function_id.to_string(), organisation_id.to_string(), e))??)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// returns the deployment information of the function
pub async fn get(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<Deployment, Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments/{deployment_id}"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        function = function_id,
        deployment = deployment_id,
        "execute a request to get deployment"
    );

    Ok(client.get(endpoint).await.map_err(|e| {
        Error::Get(
            deployment_id.to_string(),
            function_id.to_string(),
            organisation_id.to_string(),
            e,
        )
    })??)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// trigger the deployment of the function once the WebAssembly has been uploaded
pub async fn trigger(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<(), Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments/{deployment_id}/trigger"
    ))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        function = function_id,
        deployment = deployment_id,
        "execute a request to get deployment"
    );

    let request = reqwest::Request::new(Method::POST, endpoint);

    let response = client
        .execute_request(request)
        .await
        .map_err(RestError::Execute)?;

    let status_code = response.status();

    if !status_code.is_success() {
        let full = response.bytes().await.map_err(RestError::BodyAggregation)?;
        let value = serde_json::from_slice(&full).map_err(RestError::Deserialize)?;
        return Err(Error::StatusCode(ErrorResponse { status_code, value }));
    }

    Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// Upload the WebAssembly on the endpoint
pub async fn upload<X: IntoUrl + fmt::Debug>(
    client: &Client,
    endpoint: X,
    buf: Vec<u8>,
) -> Result<(), Error> {
    let url = endpoint.into_url().map_err(RestError::Url)?;

    let mut request = reqwest::Request::new(Method::PUT, url);

    let headers = request.headers_mut();
    let _ = headers.insert(header::CONTENT_TYPE, MIME_APPLICATION_WASM);
    let _ = headers.insert(header::CONTENT_LENGTH, HeaderValue::from(buf.len()));

    *request.body_mut() = Some(Body::from(buf));

    debug!(
        endpoint = %request.url(),
        "execute a request to upload webassembly"
    );

    let response = client
        .inner()
        .execute(request)
        .await
        .map_err(ClientError::Execute)?;

    let status_code = response.status();
    if !status_code.is_success() {
        let full = response.bytes().await.map_err(RestError::BodyAggregation)?;
        let value = serde_json::from_slice(&full).map_err(RestError::Deserialize)?;
        return Err(Error::StatusCode(ErrorResponse { status_code, value }));
    }

    Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
/// delete the deployment from the function
pub async fn delete(
    client: &Client,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<(), Error> {
    let endpoint = client.endpoint(format_args!("/v4/functions/organisations/{organisation_id}/functions/{function_id}/deployments/{deployment_id}"))?;

    debug!(
        %endpoint,
        organisation = organisation_id,
        function = function_id,
        deployment = deployment_id,
        "execute a request to delete deployment"
    );

    Ok(client.delete(endpoint).await.map_err(|e| {
        Error::Delete(
            deployment_id.to_string(),
            function_id.to_string(),
            organisation_id.to_string(),
            e,
        )
    })??)
}
