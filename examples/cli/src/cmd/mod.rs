//! # Command line interface module
//!
//! This module provides structures and enums to interact with the command line
//! interface
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use serde::Serialize;
use structopt::StructOpt;

use crate::cfg::Configuration;

pub mod myself;

// -----------------------------------------------------------------------------

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Output {
    Json,
    Yaml,
}

impl FromStr for Output {
    type Err = Box<dyn Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            _ => Err(
                "failed to parse output, available options are 'json' or 'yaml'"
                    .to_string()
                    .into(),
            ),
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Yaml => write!(f, "yaml"),
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::Json
    }
}

// -----------------------------------------------------------------------------

pub fn format<T>(output: &Output, obj: &T) -> Result<String, Box<dyn Error + Send + Sync>>
where
    T: Serialize,
{
    Ok(match output {
        Output::Json => serde_json::to_string_pretty(obj)
            .map_err(|err| format!("failed to serialize object into pretty json, {}", err))?,
        Output::Yaml => serde_yaml::to_string(obj)
            .map_err(|err| format!("failed to serialize object into yaml, {}", err))?,
    })
}

// -----------------------------------------------------------------------------

/// Executor trait provides a common way to implement a command
#[async_trait::async_trait]
pub trait Executor {
    type Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error>;
}

// -----------------------------------------------------------------------------

/// Command enum contains all operations that the command line could handle
#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    /// Interact with the current user
    #[structopt(name = "self", aliases = &["sel", "se", "s"])]
    Myself(myself::Myself),
}

#[async_trait::async_trait]
impl Executor for Command {
    type Error = Box<dyn Error + Send + Sync>;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Myself(cmd) => cmd
                .execute(config)
                .await
                .map_err(|err| format!("faild to execute current user command, {}", err).into()),
        }
    }
}

// -----------------------------------------------------------------------------

/// Args structure contains all commands and global flags that the command line
/// supports
#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub struct Args {
    /// Specified a configuration file
    #[structopt(short = "c", long = "config", global = true)]
    pub config: Option<PathBuf>,
    /// Increase log verbosity
    #[structopt(short = "v", global = true, parse(from_occurrences))]
    pub verbosity: usize,
    /// Check the healthiness of the configuration
    #[structopt(short = "t", long = "check", global = true)]
    pub check: bool,
    #[structopt(subcommand)]
    pub cmd: Command,
}
