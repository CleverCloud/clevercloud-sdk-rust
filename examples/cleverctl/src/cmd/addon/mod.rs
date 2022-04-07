//! # Addon module
//!
//! This module provides command implementation related to addons
use std::sync::Arc;

use clevercloud_sdk::{
    oauth10a::{
        proxy::{self, ProxyConnectorBuilder},
        Credentials,
    },
    v2::addon,
    Client,
};
use structopt::StructOpt;

use crate::{
    cfg::Configuration,
    cmd::{self, addon::config_provider::ConfigProvider, Executor, Output},
};

pub mod config_provider;

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to format output, {0}")]
    FormatOutput(Box<cmd::Error>),
    #[error("failed to list addons of organisation '{0}', {1}")]
    List(String, addon::Error),
    #[error("failed to get addon '{0}' of organisation '{1}', {2}")]
    Get(String, String, addon::Error),
    #[error("failed to build proxy connector, {0}")]
    ProxyConnector(proxy::Error),
    #[error("failed to execute command on config-provider addon, {0}")]
    ConfigProvider(config_provider::Error),
}

// -----------------------------------------------------------------------------
// Addon enumeration

#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    /// List addons of an organisation
    #[structopt(name = "list")]
    List {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
        /// Specify the organisation identifier
        #[structopt(name = "organisation-identifier")]
        organisation_id: String,
    },
    /// Get addon of an organisation
    #[structopt(name = "get")]
    Get {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
        /// Specify the organisation identifier
        #[structopt(name = "organisation-identifier")]
        organisation_id: String,
        /// Specify the addon identifier
        #[structopt(name = "addon-identifier")]
        addon_id: String,
    },
    /// Interact with ConfigProvider addon
    #[structopt(name = "config-provider", aliases = &["cp"])]
    ConfigProvider(ConfigProvider),
}

#[async_trait::async_trait]
impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::List {
                output,
                organisation_id,
            } => list(config, output, organisation_id).await,
            Self::Get {
                output,
                organisation_id,
                addon_id,
            } => get(config, output, organisation_id, addon_id).await,
            Self::ConfigProvider(cmd) => cmd.execute(config).await.map_err(Error::ConfigProvider),
        }
    }
}

// -----------------------------------------------------------------------------
// helpers

pub async fn list(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
) -> Result<(), Error> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let connector = ProxyConnectorBuilder::try_from_env().map_err(Error::ProxyConnector)?;
    let client = Client::builder()
        .with_credentials(credentials)
        .build(connector);

    let addons = addon::list(&client, organisation_id)
        .await
        .map_err(|err| Error::List(organisation_id.to_owned(), err))?;

    println!(
        "{}",
        output
            .format(&addons)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );
    Ok(())
}

pub async fn get(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    addon_id: &str,
) -> Result<(), Error> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let connector = ProxyConnectorBuilder::try_from_env().map_err(Error::ProxyConnector)?;
    let client = Client::builder()
        .with_credentials(credentials)
        .build(connector);

    let addons = addon::get(&client, organisation_id, addon_id)
        .await
        .map_err(|err| Error::Get(addon_id.to_owned(), organisation_id.to_owned(), err))?;

    println!(
        "{}",
        output
            .format(&addons)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );
    Ok(())
}
