//! # Zone module
//!
//! This module provides command implementation related to the zone API
use std::sync::Arc;

use clevercloud_sdk::{oauth10a::Credentials, v4::products::zones, Client};
use structopt::StructOpt;

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
    #[error("failed to list available zones, {0}")]
    List(zones::Error),
}

// -----------------------------------------------------------------------------
// Command enumeration

/// Command enum contains all operations that could be achieved on the zone API
#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    /// List available zones
    #[structopt(name = "list", aliases = &["l"])]
    List {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
    },
    /// List application available zones
    #[structopt(name = "application", aliases = &["app", "a"])]
    Application {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
    },
    /// List hds available zones
    #[structopt(name = "hds", aliases = &["h"])]
    Hds {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
    },
}

#[async_trait::async_trait]
impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::List { output } => list(config, output).await,
            Self::Application { output } => applications(config, output).await,
            Self::Hds { output } => hds(config, output).await,
        }
    }
}

// -----------------------------------------------------------------------------
// helpers

pub async fn list(config: Arc<Configuration>, output: &Output) -> Result<(), Error> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let client = Client::from(credentials);

    let zones = zones::list(&client).await.map_err(Error::List)?;

    println!(
        "{}",
        output
            .format(&zones)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );
    Ok(())
}

pub async fn applications(config: Arc<Configuration>, output: &Output) -> Result<(), Error> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let client = Client::from(credentials);

    let zones = zones::applications(&client).await.map_err(Error::List)?;

    println!(
        "{}",
        output
            .format(&zones)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );
    Ok(())
}

pub async fn hds(config: Arc<Configuration>, output: &Output) -> Result<(), Error> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let client = Client::from(credentials);

    let zones = zones::hds(&client).await.map_err(Error::List)?;

    println!(
        "{}",
        output
            .format(&zones)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );
    Ok(())
}
