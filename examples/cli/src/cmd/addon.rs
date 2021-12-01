//! # Addon module
//!
//! This module provides command implementation related to addons
use std::{error::Error, sync::Arc};

use clevercloud_sdk::{oauth10a::Credentials, v2::addon, Client};
use structopt::StructOpt;

use crate::{
    cfg::Configuration,
    cmd::{format, Executor, Output},
};

#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum Addon {
    /// List addons of an organisation
    List {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
        /// Specify the organisation identifier
        #[structopt(name = "organisation-identifier")]
        organisation_id: String,
    },
    /// Get addon of an organisation
    Get {
        /// Specify the output format
        #[structopt(short = "o", long = "output", default_value)]
        output: Output,
        /// Specify the organisation identifier
        #[structopt(name = "organisation-identifier")]
        organisation_id: String,
        /// Specify the addon identifier
        #[structopt(name = "addon-identifier")]
        addon_id: String,
    },
}

#[async_trait::async_trait]
impl Executor for Addon {
    type Error = Box<dyn Error + Send + Sync>;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::List {
                output,
                organisation_id,
            } => list(config, output, organisation_id).await.map_err(|err| {
                format!(
                    "failed to list addons of organisation '{}', {}",
                    organisation_id, err
                )
                .into()
            }),
            Self::Get {
                output,
                organisation_id,
                addon_id,
            } => get(config, output, organisation_id, addon_id)
                .await
                .map_err(|err| {
                    format!(
                        "failed to get addon '{}' of organisation '{}', {}",
                        addon_id, organisation_id, err
                    )
                    .into()
                }),
        }
    }
}

pub async fn list(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let client = Client::from(credentials);

    let addons = addon::list(&client, organisation_id)
        .await
        .map_err(|err| format!("failed to list addons from Clever Cloud's api, {}", err))?;

    let out = format(output, &addons).map_err(|err| format!("failed to format addons, {}", err))?;

    println!("{}", out);
    Ok(())
}

pub async fn get(
    config: Arc<Configuration>,
    output: &Output,
    organisation_id: &str,
    addon_id: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let credentials: Credentials = config.credentials.to_owned().into();
    let client = Client::from(credentials);

    let addons = addon::get(&client, organisation_id, addon_id)
        .await
        .map_err(|err| format!("failed to get addon from Clever Cloud's api, {}", err))?;

    let out = format(output, &addons).map_err(|err| format!("failed to format addon, {}", err))?;

    println!("{}", out);
    Ok(())
}
