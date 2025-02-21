//! # Addon module
//!
//! This module provides command implementation related to addons
use std::sync::Arc;

use clap::Subcommand;
use clevercloud_sdk::{Client, oauth10a::reqwest, v2::addon};

use crate::{
    cfg::Configuration,
    cmd::{self, Executor, Output, addon::config_provider::ConfigProvider},
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
    #[error("failed to create http client, {0}")]
    CreateClient(reqwest::Error),
    #[error("failed to execute command on config-provider addon, {0}")]
    ConfigProvider(config_provider::Error),
}

// -----------------------------------------------------------------------------
// Addon enumeration

#[derive(Subcommand, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    #[clap(name = "list", about = "List addons of an organisation")]
    List {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
    },
    #[clap(name = "get", about = "Get addon of an organisation")]
    Get {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the organisation identifier
        #[clap(name = "organisation-identifier")]
        organisation_id: String,
        /// Specify the addon identifier
        #[clap(name = "addon-identifier")]
        addon_id: String,
    },
    #[clap(name = "config-provider", aliases = &["cp"], subcommand, about = "Interact with ConfigProvider addon")]
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
    let client = Client::from(config.credentials.to_owned());
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
    let client = Client::from(config.credentials.to_owned());
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
