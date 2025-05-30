use std::{borrow::Cow, fs, io, path::Path};

use env_capture::{Env, IgnoreAsciiCase};
use oauth10a::{credentials::Credentials, url::Url};

use crate::{
    DEFAULT_SSH_GATEWAY, PartialCredentials,
    clever_tools::{CleverTools, CleverToolsConfig, CleverToolsConfigError},
    default_api_host, default_auth_bridge_host,
};

// PARTIAL OAUTH ERROR /////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
pub enum PartialOAuthError {
    #[error("missing token")]
    Token,
    #[error("missing secret")]
    Secret,
    #[error("missing consumer token")]
    ConsumerKey,
    #[error("missing consumer secret")]
    ConsumerSecret,
}

// CLEVER ENV //////////////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
pub enum CleverEnvError {
    #[error("failed to capture environnement, {0}")]
    Capture(#[from] env_capture::Error),
    #[error(transparent)]
    CleverToolsConfigFile(#[from] CleverToolsConfigError),
    #[error("partial OAuth credentials: {0}")]
    PartialOAuth(#[from] PartialOAuthError),
    #[error("failed to create configuration directory")]
    ConfigDir(io::Error),
}

/// Snapshot of the clever environment variables, hydrated with the OAuth
/// configuration from the main configuration file of the `clever-tools`, if any.
#[derive(Debug, serde::Deserialize)]
pub struct CleverEnv {
    #[serde(rename = "API_HOST")]
    pub(crate) api_host: Option<Url>,
    #[serde(rename = "AUTH_BRIDGE_API")]
    pub(crate) auth_bridge_host: Option<Url>,
    #[serde(rename = "SSH_GATEWAY")]
    pub(crate) ssh_gateway: Option<Box<str>>,
    #[serde(rename = "", flatten, default)]
    pub(crate) credentials: Option<PartialCredentials>,
    #[serde(skip)]
    pub(crate) config_dir: Option<Box<Path>>,
}

impl CleverEnv {
    pub fn from_env() -> Result<Self, CleverEnvError> {
        let env = Env::<IgnoreAsciiCase>::from_env();

        let mut env = env.with_prefix("CLEVER_").parse::<Self>()?;

        match &mut env.credentials {
            credentials @ None => {
                trace!("credentials not found in current process environment");

                let config_dir = CleverToolsConfig::default_config_dir()?;

                let config_path = CleverToolsConfig::config_path_in(&config_dir);

                if config_path.exists() {
                    env.config_dir.replace(config_dir.into());

                    let CleverToolsConfig {
                        oauth_token,
                        oauth_secret,
                    } = CleverToolsConfig::from_path(&config_path)?;

                    *credentials = Some(Credentials::OAuth1 {
                        token: oauth_token,
                        secret: oauth_secret,
                        consumer_key: None,
                        consumer_secret: None,
                    });

                    trace!("using credentials from `clever-tools` configuration file");
                }
            }
            Some(Credentials::OAuth1 {
                consumer_key: Some(_),
                consumer_secret: None,
                ..
            }) => {
                return Err(CleverEnvError::PartialOAuth(
                    PartialOAuthError::ConsumerSecret,
                ));
            }
            Some(Credentials::OAuth1 {
                consumer_key: None,
                consumer_secret: Some(_),
                ..
            }) => return Err(CleverEnvError::PartialOAuth(PartialOAuthError::ConsumerKey)),
            Some(_) => trace!("using credentials from environment"),
        }

        Ok(env)
    }

    pub fn env_api_host(&self) -> Option<&Url> {
        self.api_host.as_ref()
    }

    pub fn api_host(&self) -> &Url {
        match &self.api_host {
            None => default_api_host(),
            Some(v) => v,
        }
    }

    pub fn env_auth_bridge_host(&self) -> Option<&Url> {
        self.auth_bridge_host.as_ref()
    }

    pub fn auth_bridge_host(&self) -> &Url {
        match &self.auth_bridge_host {
            None => default_auth_bridge_host(),
            Some(v) => v,
        }
    }

    pub fn env_ssh_gateway(&self) -> Option<&str> {
        self.ssh_gateway.as_deref()
    }

    pub const fn ssh_gateway(&self) -> &str {
        match &self.ssh_gateway {
            None => DEFAULT_SSH_GATEWAY,
            Some(v) => v,
        }
    }

    pub const fn env_oauth_consumer_key(&self) -> Option<&str> {
        match self.credentials {
            Some(Credentials::OAuth1 {
                consumer_key: Some(ref v),
                ..
            }) => Some(v),
            _ => None,
        }
    }

    pub const fn oauth_consumer_key(&self) -> &str {
        match self.env_oauth_consumer_key() {
            Some(x) => x,
            None => CleverTools::CONSUMER_KEY,
        }
    }

    pub const fn env_oauth_consumer_secret(&self) -> Option<&str> {
        match self.credentials {
            Some(Credentials::OAuth1 {
                consumer_secret: Some(ref v),
                ..
            }) => Some(v),
            _ => None,
        }
    }

    pub const fn oauth_consumer_secret(&self) -> &str {
        match self.env_oauth_consumer_secret() {
            Some(v) => v,
            None => CleverTools::CONSUMER_SECRET,
        }
    }

    /// Returns the path to the directory where configuration files of clever apps are stored.
    pub fn config_dir(&self) -> Result<Cow<'_, Path>, CleverEnvError> {
        let path = match self.config_dir {
            Some(ref config_dir) => Cow::Borrowed(&**config_dir),
            None => Cow::Owned(CleverToolsConfig::default_config_dir()?),
        };

        fs::create_dir_all(&*path).map_err(CleverEnvError::ConfigDir)?;

        Ok(path)
    }

    pub fn credentials(&self) -> Option<&PartialCredentials> {
        self.credentials.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use env_capture::set_tmp_var;
    use oauth10a::credentials::Credentials;

    use crate::clever_env::CleverEnv;

    #[test]
    fn test_env() {
        let _ = unsafe { set_tmp_var("RUST_LOG", "trace") };

        tracing_subscriber::fmt::fmt()
            .with_level(true)
            .with_line_number(true)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();

        let _ = unsafe { set_tmp_var("CLEVER_TOKEN", "my_token") };
        let _ = unsafe { set_tmp_var("CLEVER_SECRET", "my_secret") };

        if let Credentials::OAuth1 {
            token,
            secret,
            consumer_key,
            consumer_secret,
        } = CleverEnv::from_env().unwrap().credentials().unwrap()
        {
            dbg!(token, secret, consumer_key, consumer_secret);
        }
    }
}
