//! # Configuration module
//!
//! This module provides utilities to retrieve and parse configuration

use std::{error::Error, path::PathBuf};

use clevercloud_sdk::oauth10a::Credentials as CleverCloudCredentials;
use config::{Config, File};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Configuration {
    #[serde(rename = "credentials")]
    pub credentials: Credentials,
}

impl TryFrom<&PathBuf> for Configuration {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(pb: &PathBuf) -> Result<Self, Self::Error> {
        let mut config = Config::new();

        config
            .merge(File::from(pb.as_path()).required(true))
            .map_err(|err| {
                format!(
                    "faild to load configuration from file '{}', {}",
                    pb.display(),
                    err
                )
            })?;

        Ok(config
            .try_into()
            .map_err(|err| format!("failed to cast configuration, {}", err))?)
    }
}

impl Configuration {
    pub fn try_default() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut config = Config::new();
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

        config
            .merge(
                paths
                    .iter()
                    .map(PathBuf::from)
                    .map(|path| File::from(path).required(false))
                    .collect::<Vec<_>>(),
            )
            .map_err(|err| format!("faild to load configuration from default paths, {}", err))?;

        Ok(config
            .try_into()
            .map_err(|err| format!("failed to cast configuration, {}", err))?)
    }
}
