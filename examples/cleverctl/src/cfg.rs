//! # Configuration module
//!
//! This module provides utilities to retrieve and parse configuration

use std::path::PathBuf;

use clevercloud_sdk::oauth10a::Credentials as CleverCloudCredentials;
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
// Credentials structure

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Credentials {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "consumerKey")]
    pub consumer_key: String,
    #[serde(rename = "consumerSecret")]
    pub consumer_secret: String,
}

#[allow(clippy::from_over_into)]
impl Into<CleverCloudCredentials> for Credentials {
    fn into(self) -> CleverCloudCredentials {
        CleverCloudCredentials {
            token: self.token,
            secret: self.secret,
            consumer_key: self.consumer_key,
            consumer_secret: self.consumer_secret,
        }
    }
}

// -----------------------------------------------------------------------------
// Configuration structure

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Configuration {
    #[serde(rename = "credentials")]
    pub credentials: Credentials,
    #[serde(rename = "endpoint")]
    pub endpoint: String,
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
        let paths = vec![
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
                    .map(PathBuf::from)
                    .map(|path| File::from(path).required(false))
                    .collect::<Vec<_>>(),
            )
            .build()
            .map_err(Error::LoadDefaultConfiguration)?
            .try_deserialize()
            .map_err(Error::Cast)
    }
}
