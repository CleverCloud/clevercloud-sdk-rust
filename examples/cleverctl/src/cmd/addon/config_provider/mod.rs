//! # ConfigProvider module
//!
//! This module provides commands to interact with ConfigProvider addon

use std::sync::Arc;

use structopt::StructOpt;

use crate::{
    cfg::Configuration,
    cmd::{addon::config_provider::environment::Environment, Executor},
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

#[derive(StructOpt, Eq, PartialEq, Clone, Debug)]
pub enum ConfigProvider {
    /// Interact with config-provider environment
    #[structopt(name = "environment", aliases = &["env"])]
    Environment(Environment),
}

#[async_trait::async_trait]
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
