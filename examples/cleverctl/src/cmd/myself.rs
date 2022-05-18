//! # Myself module
//!
//! This module provides command implementation related to the current user
use std::sync::Arc;

use clap::Subcommand;
use clevercloud_sdk::{
    oauth10a::{
        proxy::{self, ProxyConnectorBuilder},
        Credentials,
    },
    v2::myself,
    Client,
};

use crate::{
    cfg::Configuration,
    cmd::{self, Executor, Output},
};

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to format output, {0}")]
    FormatOutput(Box<cmd::Error>),
    #[error("failed to get current user information, {0}")]
    Get(myself::Error),
    #[error("failed to build proxy connector, {0}")]
    ProxyConnector(proxy::Error),
}

// -----------------------------------------------------------------------------
// Command enumeration

/// Command enum contains all operations that could be achieved on the user
#[derive(Subcommand, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    /// Get information about the current user
    #[clap(name = "get", aliases = &["ge", "g"])]
    Get {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
    },
}

#[async_trait::async_trait]
impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Get { output } => get(config, output).await,
        }
    }
}

pub async fn get(config: Arc<Configuration>, output: &Output) -> Result<(), Error> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let connector = ProxyConnectorBuilder::try_from_env().map_err(Error::ProxyConnector)?;
    let client = Client::builder()
        .with_credentials(credentials)
        .build(connector);

    let user = myself::get(&client).await.map_err(Error::Get)?;

    println!(
        "{}",
        output
            .format(&user)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}
