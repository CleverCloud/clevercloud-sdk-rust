//! Clever Tools

use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

// CLEVER TOOLS ////////////////////////////////////////////////////////////////

/// Default OAuth1 consumer.
#[derive(Debug)]
pub struct CleverTools;

impl CleverTools {
    // Consumer key and secret of the clever-tools are publicly available.
    // The disclosure of these tokens is not considered a vulnerability.
    // Do not report this to our security service.
    //
    // See:
    // - <https://github.com/CleverCloud/clever-tools/blob/fed085e2ba0339f55e966d7c8c6439d4dac71164/src/models/configuration.js#L128>

    pub const CONSUMER_KEY: &'static str = "T5nFjKeHH4AIlEveuGhB5S3xg8T19e";
    pub const CONSUMER_SECRET: &'static str = "MgVMqTr6fWlf2M0tkC2MXOnhfqBWDT";
}

// CLEVER TOOLS CONFIG ERROR ///////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
pub enum CleverToolsConfigError {
    #[error("failed to resolve home directory")]
    HomeDir,
    #[error("failed to open clever-tools configuration file")]
    Open(io::Error),
    #[error("failed to read clever-tools configuration file's contents")]
    Read(io::Error),
    #[error("failed to parse clever-tools configuration file's contents")]
    Json(serde_json::Error),
}

// CLEVER TOOLS CONFIG /////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CleverToolsConfig {
    #[serde(rename = "token")]
    pub oauth_token: Box<str>,
    #[serde(rename = "secret")]
    pub oauth_secret: Box<str>,
}

fn config_dir() -> Result<PathBuf, CleverToolsConfigError> {
    Ok(match env::var_os("XDG_CONFIG_HOME") {
        Some(config_dir) => PathBuf::from(config_dir),
        None => env::home_dir()
            .ok_or(CleverToolsConfigError::HomeDir)?
            .join(".config"),
    })
}

impl CleverToolsConfig {
    pub fn default_config_dir() -> Result<PathBuf, CleverToolsConfigError> {
        Ok(config_dir()?.join("clever-cloud"))
    }

    pub fn config_path_in(config_dir: &Path) -> PathBuf {
        config_dir.join("clever-tools.json")
    }

    pub fn config_path() -> Result<PathBuf, CleverToolsConfigError> {
        Ok(Self::config_path_in(&Self::default_config_dir()?))
    }

    pub fn from_path(path: &Path) -> Result<Self, CleverToolsConfigError> {
        let buf = {
            let mut buf = String::new();

            let _ = fs::File::open(path)
                .map_err(CleverToolsConfigError::Open)?
                .read_to_string(&mut buf)
                .map_err(CleverToolsConfigError::Read)?;

            buf
        };

        serde_json::from_str(&buf).map_err(CleverToolsConfigError::Json)
    }
}
