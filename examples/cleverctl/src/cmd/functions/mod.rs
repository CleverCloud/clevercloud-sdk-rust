//! # Functions module
//!
//! This module provides command implementation related to functions product
use std::{collections::BTreeMap, sync::Arc};

use clap::Subcommand;
use clevercloud_sdk::{Client, oauth10a::reqwest, v4::functions};
use tracing::info;

use crate::{
    cfg::Configuration,
    cmd::{self, Executor, Output, parse_btreemap},
};

pub mod deployments;

// -----------------------------------------------------------------------------
// Constants

pub const DEFAULT_INSTANCES: u64 = 1;
pub const DEFAULT_MAX_MEMORY: u64 = 64 * 1024 * 1024;

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to format output, {0}")]
    FormatOutput(Box<cmd::Error>),
    #[error("failed to create http client, {0}")]
    CreateClient(reqwest::Error),
    #[error("failed to list functions of organisation '{0}', {1}")]
    List(String, functions::Error),
    #[error("failed to create function on organisation '{0}', {1}")]
    Create(String, functions::Error),
    #[error("failed to get function '{0}' of organisation '{1}', {2}")]
    Get(String, String, functions::Error),
    #[error("failed to update function '{0}' of organisation '{1}', {2}")]
    Update(String, String, functions::Error),
    #[error("failed to delete function '{0}' of organisation '{1}', {2}")]
    Delete(String, String, functions::Error),
    #[error("{0}")]
    DeploymentCommand(deployments::Error),
    #[error("failed to list deployments of function '{0}' on organisation '{1}', {2}")]
    ListDeployment(String, String, functions::deployments::Error),
    #[error(
        "failed to execute function '{0}' of organisation '{1}', there is no such deployment to execute"
    )]
    NoSuchDeployment(String, String),
    #[error(
        "failed to execute request on endpoint '{0}' of deployment '{1}' for function '{2}', {3}"
    )]
    Execute(String, String, String, functions::Error),
}

// -----------------------------------------------------------------------------
// Command

/// Command enum contains all operations that could be achieved on the user
#[derive(Subcommand, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    #[clap(name = "list", aliases = &["l"], about = "List functions information of an organisation")]
    List {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
    },
    #[clap(name = "create", aliases = &["c"], about = "Create a function within the given organisation")]
    Create {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Specify a name for the function
        #[clap(short = 'n', long = "name")]
        name: Option<String>,
        /// Specify a description for the function
        #[clap(short = 'd', long = "description")]
        description: Option<String>,
        /// Specify tags for the function (format: k1=v1,k2=v2)
        #[clap(short = 't', long = "tags", value_parser =  parse_btreemap)]
        tags: Option<BTreeMap<String, String>>,
        /// Specify environment for the function (format: k1=v1,k2=v2)
        #[clap(short = 'e', long = "environment", value_parser =  parse_btreemap)]
        environment: Option<BTreeMap<String, String>>,
        /// Specify the max memory of the function in byte.
        #[clap(short = 'm', long = "max-memory")]
        max_memory: Option<u64>,
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
    },
    #[clap(name = "update", aliases = &["u"], about = "Update information about a function")]
    Update {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Function identifier
        #[clap(name = "function-id")]
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
        /// Specify environment for the function (format: k1=v1,k2=v2)
        #[clap(short = 'e', long = "environment", value_parser =  parse_btreemap)]
        environment: Option<BTreeMap<String, String>>,
        /// Specify the max memory of the function in byte.
        #[clap(short = 'm', long = "max-memory")]
        max_memory: Option<u64>,
    },
    #[clap(name = "delete", aliases = &["d"], about = "Delete a function")]
    Delete {
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Function identifier
        #[clap(name = "function-id")]
        function_id: String,
    },
    #[clap(name = "execute", aliases = &["exec", "e"], about = "Execute a function")]
    Execute {
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
    #[clap(name = "deployments", aliases = &["deploy"], about = "Interact with deployments of a function", subcommand)]
    Deployments(deployments::Command),
}

impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::List {
                output,
                organisation_id,
            } => list(config, output, organisation_id).await,
            Self::Create {
                output,
                organisation_id,
                name,
                description,
                tags,
                environment,
                max_memory,
            } => {
                let tag = tags.as_ref().map(|tags| {
                    tags.iter()
                        .fold(vec![], |mut acc, (k, v)| {
                            acc.push(format!("{k}:{v}"));
                            acc
                        })
                        .join(" ")
                });

                let opts = functions::Opts {
                    name: name.to_owned(),
                    description: description.to_owned(),
                    tag,
                    environment: environment.to_owned().unwrap_or_default(),
                    max_memory: max_memory.unwrap_or_else(|| DEFAULT_MAX_MEMORY),
                    max_instances: DEFAULT_INSTANCES,
                };

                create(config, output, organisation_id, &opts).await
            }
            Self::Get {
                output,
                organisation_id,
                function_id,
            } => get(config, output, organisation_id, function_id).await,
            Self::Update {
                output,
                organisation_id,
                function_id,
                name,
                description,
                tags,
                environment,
                max_memory,
            } => {
                let tag = tags.as_ref().map(|tags| {
                    tags.iter()
                        .fold(vec![], |mut acc, (k, v)| {
                            acc.push(format!("{k}:{v}"));
                            acc
                        })
                        .join(" ")
                });

                let opts = functions::Opts {
                    name: name.to_owned(),
                    description: description.to_owned(),
                    tag,
                    environment: environment.to_owned().unwrap_or_default(),
                    max_memory: max_memory.unwrap_or_else(|| DEFAULT_MAX_MEMORY),
                    max_instances: DEFAULT_INSTANCES,
                };

                update(config, output, organisation_id, function_id, &opts).await
            }
            Self::Delete {
                organisation_id,
                function_id,
            } => delete(config, organisation_id, function_id).await,
            Self::Execute {
                output,
                organisation_id,
                function_id,
            } => execute(config, output, organisation_id, function_id).await,
            Self::Deployments(cmd) => cmd.execute(config).await.map_err(Error::DeploymentCommand),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Commands

pub async fn list(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());

    let functionz = functions::list(&client, organisation_id)
        .await
        .map_err(|err| Error::List(organisation_id.to_string(), err))?;

    println!(
        "{}",
        output
            .format(&functionz)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn create(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    opts: &functions::Opts,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());

    let function = functions::create(&client, organisation_id, opts)
        .await
        .map_err(|err| Error::Create(organisation_id.to_string(), err))?;

    println!(
        "{}",
        output
            .format(&function)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn get(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    function_id: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());

    let function = functions::get(&client, organisation_id, function_id)
        .await
        .map_err(|err| Error::Get(function_id.to_string(), organisation_id.to_string(), err))?;

    println!(
        "{}",
        output
            .format(&function)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn update(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    function_id: &str,
    opts: &functions::Opts,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());
    let function = functions::update(&client, organisation_id, function_id, opts)
        .await
        .map_err(|err| Error::Update(function_id.to_string(), organisation_id.to_string(), err))?;

    println!(
        "{}",
        output
            .format(&function)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn delete(
    config: Arc<Configuration>,
    organisation_id: &str,
    function_id: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());

    functions::delete(&client, organisation_id, function_id)
        .await
        .map_err(|err| Error::Delete(function_id.to_string(), organisation_id.to_string(), err))
}

pub async fn execute(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    function_id: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());
    info!(
        organisation_id = organisation_id,
        function_id = function_id,
        "List deployment to take the latest one"
    );

    let mut deploymentz = functions::deployments::list(&client, organisation_id, function_id)
        .await
        .map_err(|err| {
            Error::ListDeployment(function_id.to_string(), organisation_id.to_string(), err)
        })?
        .into_iter()
        .filter(|deployment| {
            deployment.url.is_some() && functions::deployments::Status::Ready == deployment.status
        })
        .collect::<Vec<_>>();

    deploymentz.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    let deployment = deploymentz.first().ok_or_else(|| {
        Error::NoSuchDeployment(function_id.to_string(), organisation_id.to_string())
    })?;

    info!(
        organisation_id = organisation_id,
        function_id = function_id,
        deployment_id = deployment.id,
        endpoint = deployment.url,
        "Execute a GET request on function endpoint"
    );

    match &deployment.url {
        None => Err(Error::NoSuchDeployment(
            function_id.to_string(),
            organisation_id.to_string(),
        )),
        Some(url) => {
            let result = functions::execute(&client, url).await.map_err(|err| {
                Error::Execute(
                    url.to_string(),
                    deployment.id.to_owned(),
                    function_id.to_string(),
                    err,
                )
            })?;

            println!(
                "{}",
                output
                    .format(&result)
                    .map_err(|err| Error::FormatOutput(Box::new(err)))?
            );

            Ok(())
        }
    }
}
