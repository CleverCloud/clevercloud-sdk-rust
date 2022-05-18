//! # Command line interface example
//!
//! Clever Cloud command line interface using the
//! [clevercloud-sdk-rust](https://github.com/CleverCloud/clevercloud-sdk-rust)
//! project
use std::sync::Arc;

use slog::{o, Drain, Level, LevelFilter, Logger};
use slog_async::Async;
use slog_scope::{crit, debug, info, set_global_logger, GlobalLoggerGuard as Guard};
use slog_term::{FullFormat, TermDecorator};

use crate::{
    cfg::Configuration,
    cmd::{Args, Executor},
};

pub mod cfg;
pub mod cmd;

// -----------------------------------------------------------------------------
// helpers

pub fn initialize(verbosity: &usize) -> Guard {
    let level = Level::from_usize(Level::Critical.as_usize() + verbosity).unwrap_or(Level::Trace);

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = LevelFilter::new(drain, level).fuse();
    let drain = Async::new(drain).build().fuse();

    set_global_logger(Logger::root(drain, o!()))
}

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
    let _guard = initialize(&args.verbosity);
    let result = match &args.config {
        Some(pb) => Configuration::try_from(pb).map_err(Error::Configuration),
        None => Configuration::try_default().map_err(Error::Configuration),
    };

    let config = match result {
        Ok(config) => Arc::new(config),
        Err(err) => {
            crit!("Could not load configuration"; "err" => err.to_string());
            return Err(err);
        }
    };

    if args.check {
        info!("Configuration is healthy!");
        debug!("Configuration is {:#?}", config);
        return Ok(());
    }

    if let Err(err) = args.cmd.execute(config).await.map_err(Error::Command) {
        crit!("Could not execute command"; "error" => err.to_string());
        return Err(err);
    }

    info!("Command successfully executed");
    Ok(())
}
