//! # Command line interface module
//!
//! This module provides structures and enums to interact with the command line
//! interface
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    future::Future,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use clap::{ArgAction, Parser, Subcommand};
use paw::ParseArgs;
use serde::Serialize;

use crate::cfg::Configuration;

pub mod addon;
pub mod functions;
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
    #[error("failed to execute command relative to functions, {0}")]
    FunctionCommand(functions::Error),
}

// -----------------------------------------------------------------------------
// Output enumeration

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Default)]
pub enum Output {
    #[default]
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
// Executor trait

/// Executor trait provides a common way to implement a command
pub trait Executor {
    type Error;

    fn execute(
        &self,
        config: Arc<Configuration>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

// -----------------------------------------------------------------------------
// Command enumeration

/// Command enum contains all operations that the command line could handle
#[derive(Subcommand, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    #[clap(name = "self", aliases = &["sel", "se", "s"], subcommand, about = "Interact with the current user")]
    Myself(myself::Command),
    #[clap(name = "addon", aliases = &["addo", "add", "ad", "a"], subcommand, about = "Interact with addons")]
    Addon(addon::Command),
    #[clap(name = "zone", aliases = &["zon", "zo", "z"], subcommand, about = "Interact with zones")]
    Zone(zone::Command),
    #[clap(name = "functions", aliases = &["functio", "functi", "funct", "func", "fun", "fu", "f"], subcommand, about = "Interact with functions")]
    Function(functions::Command),
}

impl Executor for Command {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Myself(cmd) => cmd.execute(config).await.map_err(Error::MyselfCommand),
            Self::Addon(cmd) => cmd.execute(config).await.map_err(Error::AddonCommand),
            Self::Zone(cmd) => cmd.execute(config).await.map_err(Error::ZoneCommand),
            Self::Function(cmd) => cmd.execute(config).await.map_err(Error::FunctionCommand),
        }
    }
}

// -----------------------------------------------------------------------------
// Arguments structure

/// Args structure contains all commands and global flags that the command line
/// supports
#[derive(Parser, Eq, PartialEq, Clone, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Specify a configuration file
    #[clap(short = 'c', long = "config", global = true)]
    pub config: Option<PathBuf>,
    /// Increase log verbosity
    #[clap(short = 'v', global = true, action = ArgAction::Count)]
    pub verbosity: u8,
    /// Check the healthiness of the configuration
    #[clap(long = "check", global = true)]
    pub check: bool,
    #[clap(subcommand)]
    pub cmd: Command,
}

impl ParseArgs for Args {
    type Error = Error;

    fn parse_args() -> Result<Self, Self::Error> {
        Ok(Args::parse())
    }
}

// -------------------------------------------------------------------------------------------------
// Helpers

pub fn parse_btreemap(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut btreemap: BTreeMap<String, String> = BTreeMap::new();

    for s in value.split(',') {
        if let Some((key, value)) = s.trim().split_once('=') {
            btreemap.insert(key.to_owned(), value.to_owned());
        } else {
            return Err(format!(
                "failed to parse '{value}' as a map (a.k.a k1=v1,k2=v2)"
            ));
        }
    }

    Ok(btreemap)
}
