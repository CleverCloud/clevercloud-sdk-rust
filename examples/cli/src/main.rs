//! # Command line interface example
//!
//! Clever Cloud command line interface using the
//! [clevercloud-sdk-rust](https://github.com/CleverCloud/clevercloud-sdk-rust)
//! project
use std::{error::Error, sync::Arc};

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

pub fn initialize(verbosity: &usize) -> Guard {
    let level = Level::from_usize(Level::Critical.as_usize() + verbosity).unwrap_or(Level::Trace);

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = LevelFilter::new(drain, level).fuse();
    let drain = Async::new(drain).build().fuse();

    set_global_logger(Logger::root(drain, o!()))
}

#[paw::main]
#[tokio::main]
pub async fn main(args: Args) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _guard = initialize(&args.verbosity);
    let result: Result<_, Box<dyn Error + Send + Sync>> = match &args.config {
        Some(pb) => Configuration::try_from(pb).map_err(|err| {
            format!(
                "failed to load configuration from path {}, {}",
                pb.display(),
                err
            )
            .into()
        }),
        None => Configuration::try_default().map_err(|err| {
            format!("failed to load configuration from defaults paths, {}", err).into()
        }),
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

    if let Err(err) = args.cmd.execute(config).await {
        crit!("Could not execute command"; "error" => err.to_string());
        return Err(err);
    }

    info!("Command successfully executed");
    Ok(())
}
