//! # Deployment module
//!
//! This module provides all necessary commands to interact with a function's deployment including
//! uploading and deploying it.

use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

use clap::Subcommand;
use clevercloud_sdk::{
    oauth10a::{reqwest, Credentials},
    v4::functions::deployments::{self, Opts, Platform},
    Client,
};
use tokio::fs::read;
use tracing::info;

use crate::{
    cfg::Configuration,
    cmd::{self, parse_btreemap, Executor, Output},
};

// ----------------------------------------------------------------------------
// Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to format output, {0}")]
    FormatOutput(Box<cmd::Error>),
    #[error("failed to create http client, {0}")]
    CreateClient(reqwest::Error),
    #[error("failed to list deployments of function '{0}' on organisation '{1}', {2}")]
    List(String, String, deployments::Error),
    #[error("failed to create deployment on function '{0}' for organisation '{1}', {2}")]
    Create(String, String, deployments::Error),
    #[error("failed to get deployment '{0}' of function '{1}' for organisation '{2}', {3}")]
    Get(String, String, String, deployments::Error),
    #[error("failed to delete deployment '{0}' of function '{1}' for organisation '{2}', {3}")]
    Delete(String, String, String, deployments::Error),
    #[error("failed to read file '{0}', {1}")]
    Read(String, std::io::Error),
    #[error("failed to upload file located at '{0}' for deployment '{1}' of function '{2}' on organisation '{3}', {4}")]
    Upload(String, String, String, String, deployments::Error),
    #[error("failed to trigger deployment '{0}' of function '{1}' for organisation '{2}', {3}")]
    Trigger(String, String, String, deployments::Error),
}

// ----------------------------------------------------------------------------
// Command

#[derive(Subcommand, PartialEq, Eq, Clone, Debug)]
pub enum Command {
    #[clap(name = "list", aliases = &["l"], about = "List functions information of an organisation")]
    List {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Specify the function identifier
        #[clap(name = "function-identifier")]
        function_id: String,
    },
    #[clap(name = "create", aliases = &["c"], about = "Create a function within the given organisation")]
    Create {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Specify the function identifier
        #[clap(name = "function-identifier")]
        function_id: String,
        /// Specify a name for the function
        #[clap(short = 'n', long = "name")]
        name: Option<String>,
        /// Specify a description for the function
        #[clap(short = 'd', long = "description")]
        description: Option<String>,
        /// Specify tags for the function (format: k1=v1,k2=v2)
        #[clap(short = 't', long = "tags", value_parser =  parse_btreemap)]
        tags: Option<BTreeMap<String, String>>,
        /// Specify the WebAssembly file to upload
        #[clap(short = 'f', long = "file")]
        file: PathBuf,
        /// Specify the language of the functions (available options are 'rust', 'javascript', 'tinygo' and 'assemblyscript')
        #[clap(short = 'p', long = "platform")]
        platform: Platform,
    },
    #[clap(name = "get", aliases = &["g"], about = "Get information about a function")]
    Get {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Specify the function identifier
        #[clap(name = "function-identifier")]
        function_id: String,
        /// Specify the deployment identifier
        #[clap(name = "deployment-identifier")]
        deployment_id: String,
    },
    #[clap(name = "delete", aliases = &["d"], about = "Delete a function")]
    Delete {
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Function identifier
        #[clap(name = "function-id")]
        function_id: String,
        /// Specify the deployment identifier
        #[clap(name = "deployment-identifier")]
        deployment_id: String,
    },
}

#[async_trait::async_trait]
impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::List {
                output,
                organisation_id,
                function_id,
            } => list(config, output, organisation_id, function_id).await,
            Self::Create {
                output,
                organisation_id,
                function_id,
                name,
                description,
                tags,
                file,
                platform,
            } => {
                let tag = tags.as_ref().map(|tags| {
                    tags.iter()
                        .fold(vec![], |mut acc, (k, v)| {
                            acc.push(format!("{k}:{v}"));
                            acc
                        })
                        .join(" ")
                });

                let opts = deployments::Opts {
                    name: name.to_owned(),
                    description: description.to_owned(),
                    tag,
                    platform: platform.to_owned(),
                };

                create(config, output, organisation_id, function_id, file, &opts).await
            }
            Self::Get {
                output,
                organisation_id,
                function_id,
                deployment_id,
            } => get(config, output, organisation_id, function_id, deployment_id).await,
            Self::Delete {
                organisation_id,
                function_id,
                deployment_id,
            } => delete(config, organisation_id, function_id, deployment_id).await,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Commands

pub async fn list(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    function_id: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());
    let deploymentz = deployments::list(&client, organisation_id, function_id)
        .await
        .map_err(|err| Error::List(function_id.to_string(), organisation_id.to_string(), err))?;

    println!(
        "{}",
        output
            .format(&deploymentz)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn create(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    function_id: &str,
    file: &PathBuf,
    opts: &Opts,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());

    info!(
        organisation_id = organisation_id,
        function_id = function_id,
        "Create deployment for function"
    );
    let deployment_c = deployments::create(&client, organisation_id, function_id, opts)
        .await
        .map_err(|err| Error::Create(function_id.to_string(), organisation_id.to_string(), err))?;

    info!(
        organisation_id = organisation_id,
        function_id = function_id,
        file = file.display().to_string(),
        deployment_id = deployment_c.id,
        "Read WebAssembly to a buffer"
    );
    let buf = read(file)
        .await
        .map_err(|err| Error::Read(file.display().to_string(), err))?;

    info!(
        organisation_id = organisation_id,
        function_id = function_id,
        file = file.display().to_string(),
        deployment_id = deployment_c.id,
        "Upload WebAssembly for deployment"
    );

    deployments::upload(&client, &deployment_c.upload_url, buf)
        .await
        .map_err(|err| {
            Error::Upload(
                file.display().to_string(),
                deployment_c.id.to_string(),
                function_id.to_string(),
                organisation_id.to_string(),
                err,
            )
        })?;

    info!(
        organisation_id = organisation_id,
        function_id = function_id,
        deployment_id = deployment_c.id,
        "Trigger deployment of the function"
    );

    deployments::trigger(&client, organisation_id, function_id, &deployment_c.id)
        .await
        .map_err(|err| {
            Error::Trigger(
                deployment_c.id.to_string(),
                function_id.to_string(),
                organisation_id.to_string(),
                err,
            )
        })?;

    info!(
        organisation_id = organisation_id,
        function_id = function_id,
        deployment_id = deployment_c.id,
        "Retrieve deployment"
    );

    let deployment = deployments::get(&client, organisation_id, function_id, &deployment_c.id)
        .await
        .map_err(|err| {
            Error::Get(
                deployment_c.id.to_string(),
                function_id.to_string(),
                organisation_id.to_string(),
                err,
            )
        })?;

    println!(
        "{}",
        output
            .format(&deployment)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn get(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());

    let deployment = deployments::get(&client, organisation_id, function_id, deployment_id)
        .await
        .map_err(|err| {
            Error::Get(
                deployment_id.to_string(),
                function_id.to_string(),
                organisation_id.to_string(),
                err,
            )
        })?;

    println!(
        "{}",
        output
            .format(&deployment)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn delete(
    config: Arc<Configuration>,
    organisation_id: &str,
    function_id: &str,
    deployment_id: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());

    deployments::delete(&client, organisation_id, function_id, deployment_id)
        .await
        .map_err(|err| {
            Error::Delete(
                deployment_id.to_string(),
                function_id.to_string(),
                organisation_id.to_string(),
                err,
            )
        })
}
