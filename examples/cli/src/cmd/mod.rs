//! # Command line interface module
//!
//! This module provides structures and enums to interact with the command line
//! interface
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use serde::Serialize;
use structopt::StructOpt;

use crate::cfg::Configuration;

pub mod addon;
pub mod myself;
pub mod zone;

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse output '{0}', available options are 'json' or 'yaml'")]
    ParseOutput(String),
    #[error("failed to serialize object into json, {0}")]
    SerializeJson(serde_json::Error),
    #[error("failed to serialize object into yaml, {0}")]
    SerializeYaml(serde_yaml::Error),
    #[error("failed to execute command relative to the current user, {0}")]
    MyselfCommand(myself::Error),
    #[error("failed to execute command relative to addons, {0}")]
    AddonCommand(addon::Error),
    #[error("failed to execute command relative to zones, {0}")]
    ZoneCommand(zone::Error),
}

// -----------------------------------------------------------------------------
// Output enumeration

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Output {
    Json,
    Yaml,
}

impl FromStr for Output {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            _ => Err(Error::ParseOutput(s.to_owned())),
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

impl Output {
    pub fn format<T>(&self, obj: &T) -> Result<String, Error>
    where
        T: Serialize,
    {
        Ok(match self {
            Output::Json => serde_json::to_string_pretty(obj).map_err(Error::SerializeJson)?,
            Output::Yaml => serde_yaml::to_string(obj).map_err(Error::SerializeYaml)?,
        })
    }
}

// -----------------------------------------------------------------------------
// Excutor trait

/// Executor trait provides a common way to implement a command
#[async_trait::async_trait]
pub trait Executor {
    type Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error>;
}

// -----------------------------------------------------------------------------
// Command enumeration

/// Command enum contains all operations that the command line could handle
#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    /// Interact with the current user
    #[structopt(name = "self", aliases = &["sel", "se", "s"])]
    Myself(myself::Command),
    /// Interact with addons
    #[structopt(name = "addon", aliases = &["addo", "add", "ad", "a"])]
    Addon(addon::Command),
    /// Interact with zones
    #[structopt(name = "zone", aliases = &["zon", "zo", "z"])]
    Zone(zone::Command),
}

#[async_trait::async_trait]
impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Myself(cmd) => cmd.execute(config).await.map_err(Error::MyselfCommand),
            Self::Addon(cmd) => cmd.execute(config).await.map_err(Error::AddonCommand),
            Self::Zone(cmd) => cmd.execute(config).await.map_err(Error::ZoneCommand),
        }
    }
}

// -----------------------------------------------------------------------------
// Arguments structure

/// Args structure contains all commands and global flags that the command line
/// supports
#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub struct Args {
    /// Specify a configuration file
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
