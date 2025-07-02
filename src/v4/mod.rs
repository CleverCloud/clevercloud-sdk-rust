//! # Api version 4 module
//!
//! This module exposes resources under version 4 of the Clever-Cloud API.

mod error;
pub use error::HttpError;

pub type ErrorResponse = oauth10a::rest::ErrorResponse<Box<HttpError>>;

pub mod addon_provider;
pub mod functions;
#[cfg(feature = "network-group")]
pub mod network_group;
pub mod products;
