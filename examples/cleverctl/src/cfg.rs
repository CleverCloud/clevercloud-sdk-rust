//! # Configuration module
//!
//! This module provides utilities to retrieve and parse configuration

use std::path::PathBuf;

use clevercloud_sdk::oauth10a::Credentials;
use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to load configuration from file '{0}', {1}")]
    LoadConfiguration(String, ConfigError),
    #[error("failed to load configuration from default paths, {0}")]
    LoadDefaultConfiguration(ConfigError),
    #[error("failed to cast configuration, {0}")]
    Cast(ConfigError),
}

// -----------------------------------------------------------------------------
// Configuration structure

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Configuration {
    #[serde(rename = "credentials")]
    pub credentials: Credentials,
}

impl TryFrom<&PathBuf> for Configuration {
    type Error = Error;

    fn try_from(pb: &PathBuf) -> Result<Self, Self::Error> {
        Config::builder()
            .add_source(File::from(pb.as_path()).required(true))
            .build()
            .map_err(|err| Error::LoadConfiguration(pb.display().to_string(), err))?
            .try_deserialize()
            .map_err(Error::Cast)
    }
}

impl Configuration {
    pub fn try_default() -> Result<Self, Error> {
        let paths = [
            format!("/usr/share/{}/config", env!("CARGO_PKG_NAME")),
            format!("/etc/{}/config", env!("CARGO_PKG_NAME")),
            format!(
                "{}/.local/usr/share/{}/config",
                env!("HOME"),
                env!("CARGO_PKG_NAME")
            ),
            format!(
                "{}/.local/etc/{}/config",
                env!("HOME"),
                env!("CARGO_PKG_NAME")
            ),
            format!("{}/.config/{}/config", env!("HOME"), env!("CARGO_PKG_NAME")),
            "config".to_string(),
        ];

        Config::builder()
            .add_source(
                paths
                    .iter()
                    .map(|path| File::with_name(path).required(false))
                    .collect::<Vec<_>>(),
            )
            .build()
            .map_err(Error::LoadDefaultConfiguration)?
            .try_deserialize()
            .map_err(Error::Cast)
    }
}
