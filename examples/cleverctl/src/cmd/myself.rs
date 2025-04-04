//! # Myself module
//!
//! This module provides command implementation related to the current user
use std::sync::Arc;

use clap::Subcommand;
use clevercloud_sdk::{Client, oauth10a::reqwest, v2::myself};

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
    #[error("failed to create http client, {0}")]
    CreateClient(reqwest::Error),
}

// -----------------------------------------------------------------------------
// Command enumeration

/// Command enum contains all operations that could be achieved on the user
#[derive(Subcommand, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    #[clap(name = "get", aliases = &["ge", "g"], about = "Get information about the current user")]
    Get {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
    },
}

impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Get { output } => get(config, output).await,
        }
    }
}

pub async fn get(config: Arc<Configuration>, output: &Output) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());
    let user = myself::get(&client).await.map_err(Error::Get)?;

    println!(
        "{}",
        output
            .format(&user)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}
