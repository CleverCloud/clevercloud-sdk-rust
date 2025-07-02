//! # Api version 2 module
//!
//! This module expose resources under the version 2 of the Clever-Cloud Api.

pub mod error;

pub type ErrorResponse = oauth10a::rest::ErrorResponse<error::InvalidResponseBody>;

pub mod addon;
pub mod myself;
pub mod plan;
