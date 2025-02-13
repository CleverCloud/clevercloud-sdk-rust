use std::{path::PathBuf, sync::Arc};

use clap::Subcommand;
use clevercloud_sdk::{
    oauth10a::{reqwest, Credentials},
    v4::addon_provider::config_provider::addon::environment::{self, Variable},
    Client,
};
use tokio::{fs, task::spawn_blocking as blocking};

use crate::{
    cfg::Configuration,
    cmd::{self, Executor, Output},
};

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to format output, {0}")]
    FormatOutput(Box<cmd::Error>),
    #[error("failed to get environment for config-provider, {0}")]
    Get(environment::Error),
    #[error("failed to update environment for config-provider, {0}")]
    Put(environment::Error),
    #[error("failed to create http client, {0}")]
    CreateClient(reqwest::Error),
    #[error("failed to read file, {0}")]
    Read(std::io::Error),
    #[error("failed to wait for thread to finish, {0}")]
    Join(tokio::task::JoinError),
    #[error("failed to serialize content of the file into json, {0}")]
    Serialize(serde_json::Error),
}

// -----------------------------------------------------------------------------
// Environment enumeration

#[derive(Subcommand, PartialEq, Eq, Clone, Debug)]
pub enum Environment {
    #[clap(name = "get", about = "Get environment variables")]
    Get {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the config-provider identifier
        #[clap(name = "config-provider-identifier")]
        id: String,
    },
    #[clap(name = "insert", aliases = &["i"], about = "Insert an environment variable")]
    Insert {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the config-provider identifier
        #[clap(name = "config-provider-identifier")]
        id: String,
        /// Specify the name of the environment variable to insert
        #[clap(name = "name")]
        name: String,
        /// Specify the value of the environment variable to insert
        #[clap(name = "value")]
        value: String,
    },
    #[clap(name = "put", about = "Update environment variables")]
    Put {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the config-provider identifier
        #[clap(name = "config-provider-identifier")]
        id: String,
        /// Specify the json file to read
        #[clap(name = "file")]
        file: PathBuf,
    },
    #[clap(name = "remove", aliases = &["r"], about = "Remove an environment variable")]
    Remove {
        /// Specify the output format
        #[clap(short = 'o', long = "output", default_value_t)]
        output: Output,
        /// Specify the config-provider identifier
        #[clap(name = "config-provider-identifier")]
        id: String,
        /// Specify the name of the environment variable to remove
        #[clap(name = "name")]
        name: String,
    },
}

#[async_trait::async_trait]
impl Executor for Environment {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Get { output, id } => get(config, output, id).await,
            Self::Insert {
                output,
                id,
                name,
                value,
            } => insert(config, output, id, name, value).await,
            Self::Put { output, id, file } => put(config, output, id, file).await,
            Self::Remove { output, id, name } => remove(config, output, id, name).await,
        }
    }
}

// -----------------------------------------------------------------------------
// Helpers

pub async fn get(config: Arc<Configuration>, output: &Output, id: &str) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());
    let variables = environment::get(&client, id).await.map_err(Error::Get)?;

    println!(
        "{}",
        output
            .format(&variables)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn insert(
    config: Arc<Configuration>,
    output: &Output,
    id: &str,
    name: &str,
    value: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());
    let variables = environment::insert(
        &client,
        id,
        Variable::from((name.to_owned(), value.to_owned())),
    )
    .await
    .map_err(Error::Get)?;

    println!(
        "{}",
        output
            .format(&variables)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn put(
    config: Arc<Configuration>,
    output: &Output,
    id: &str,
    file: &PathBuf,
) -> Result<(), Error> {
    let content = fs::read_to_string(file).await.map_err(Error::Read)?;
    let variables = blocking(move || serde_json::from_str(&content))
        .await
        .map_err(Error::Join)?
        .map_err(Error::Serialize)?;

    let client = Client::from(config.credentials.to_owned());
    let variables = environment::put(&client, id, &variables)
        .await
        .map_err(Error::Put)?;

    println!(
        "{}",
        output
            .format(&variables)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}

pub async fn remove(
    config: Arc<Configuration>,
    output: &Output,
    id: &str,
    name: &str,
) -> Result<(), Error> {
    let client = Client::from(config.credentials.to_owned());
    let variables = environment::remove(&client, id, name)
        .await
        .map_err(Error::Get)?;

    println!(
        "{}",
        output
            .format(&variables)
            .map_err(|err| Error::FormatOutput(Box::new(err)))?
    );

    Ok(())
}
