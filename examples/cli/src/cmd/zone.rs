//! # Zone module
//!
//! This module provides command implementation related to the zone API
use std::{error::Error, sync::Arc};

use clevercloud_sdk::{oauth10a::Credentials, v4::products::zones, Client};
use structopt::StructOpt;

use crate::{
    cfg::Configuration,
    cmd::{format, Executor, Output},
};

/// Zone enum contains all operations that could be achieved on the zone API
#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum Zone {
    /// List available zones
    #[structopt(name = "list", aliases = &["l"])]
    List {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
    },
}

#[async_trait::async_trait]
impl Executor for Zone {
    type Error = Box<dyn Error + Send + Sync>;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::List { output } => list(config, output)
                .await
                .map_err(|err| format!("failed to list available zones, {}", err).into()),
        }
    }
}

pub async fn list(
    config: Arc<Configuration>,
    output: &Output,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let client = Client::from(credentials);

    let zones = zones::list(&client).await.map_err(|err| {
        format!(
            "failed to list available zones information from Clever Cloud's api, {}",
            err
        )
    })?;

    let out = format(output, &zones)
        .map_err(|err| format!("failed to format current user information, {}", err))?;

    println!("{}", out);
    Ok(())
}
