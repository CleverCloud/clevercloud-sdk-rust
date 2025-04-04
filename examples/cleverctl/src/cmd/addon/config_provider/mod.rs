//! # ConfigProvider module
//!
//! This module provides commands to interact with ConfigProvider addon

use std::sync::Arc;

use clap::Subcommand;

use crate::{
    cfg::Configuration,
    cmd::{Executor, addon::config_provider::environment::Environment},
};

pub mod environment;

// -----------------------------------------------------------------------------
// Error enumeration

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to execute command on config-provider environment, {0}")]
    Environment(environment::Error),
}

// -----------------------------------------------------------------------------
// ConfigProvider structure

#[derive(Subcommand, Eq, PartialEq, Clone, Debug)]
pub enum ConfigProvider {
    #[clap(name = "environment", aliases = &["env"], subcommand, about = "Interact with config-provider environment")]
    Environment(Environment),
}

impl Executor for ConfigProvider {
    type Error = Error;

    async fn execute(&self, config: Arc<Configuration>) -> Result<(), Self::Error> {
        match self {
            Self::Environment(environment) => environment
                .execute(config)
                .await
                .map_err(Error::Environment),
        }
    }
}
