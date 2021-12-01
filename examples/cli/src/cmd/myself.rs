//! # Myself module
//!
//! This module provides command implementation related to the current user
use std::{error::Error, sync::Arc};

use clevercloud_sdk::{oauth10a::Credentials, v2::myself, Client};
use structopt::StructOpt;

use crate::{
    cfg::Configuration,
    cmd::{format, Executor, Output},
};

/// Myself enum contains all operations that could be achieved on the user
#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum Myself {
    /// Get information about the current user
    #[structopt(name = "get", aliases = &["ge", "g"])]
    Get {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
    },
}

#[async_trait::async_trait]
impl Executor for Myself {
    type Error = Box<dyn Error + Send + Sync>;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Get { output } => get(config, output)
                .await
                .map_err(|err| format!("failed to get current user information, {}", err).into()),
        }
    }
}

pub async fn get(
    config: Arc<Configuration>,
    output: &Output,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let client = Client::from(credentials);

    let user = myself::get(&client).await.map_err(|err| {
        format!(
            "failed to get current user information from Clever Cloud's api, {}",
            err
        )
    })?;

    let out = format(output, &user)
        .map_err(|err| format!("failed to format current user information, {}", err))?;

    println!("{}", out);
    Ok(())
}
