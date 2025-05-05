//! # Network Group module
//!
//! This module provide structures and helpers to interact with Clever Cloud's
//! Network Group API.

pub type MemberId = String;

pub type OwnerId = String;

pub mod delete;
pub mod network_group_id;
pub mod peer;
pub mod wannabe_external_peer;
pub mod wireguard;
pub mod wireguard_configuration;
