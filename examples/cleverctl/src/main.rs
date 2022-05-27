//! # Command line interface example
//!
//! Clever Cloud command line interface using the
//! [clevercloud-sdk-rust](https://github.com/CleverCloud/clevercloud-sdk-rust)
//! project
use std::sync::Arc;

use tracing::{info, debug, error};

use crate::{
    cfg::Configuration,
    cmd::{Args, Executor},
};

pub mod cfg;
pub mod cmd;
pub mod logging;

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to load configuration, {0}")]
    Configuration(cfg::Error),
    #[error("failed to execute command, {0}")]
    Command(cmd::Error),
    #[error("failed to parse command line, {0}")]
    ParseCommandLine(std::io::Error),
    #[error("failed to initialize logging system, {0}")]
    Logging(logging::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::ParseCommandLine(err)
    }
}

impl From<cmd::Error> for Error {
    fn from(err: cmd::Error) -> Self {
        Self::Command(err)
    }
}

// -----------------------------------------------------------------------------
// main

#[paw::main]
#[tokio::main]
pub async fn main(args: Args) -> Result<(), Error> {
    logging::initialize(args.verbosity).map_err(Error::Logging)?;

    let result = match &args.config {
        Some(pb) => Configuration::try_from(pb).map_err(Error::Configuration),
        None => Configuration::try_default().map_err(Error::Configuration),
    };

    let config = match result {
        Ok(config) => Arc::new(config),
        Err(err) => {
            error!("Could not load configuration, {}", err);
            return Err(err);
        }
    };

    if args.check {
        info!("Configuration is healthy!");
        debug!("Configuration is {:#?}", config);
        return Ok(());
    }

    if let Err(err) = args.cmd.execute(config).await.map_err(Error::Command) {
        error!("Could not execute command, {}", err);
        return Err(err);
    }

    info!("Command successfully executed");
    Ok(())
}
