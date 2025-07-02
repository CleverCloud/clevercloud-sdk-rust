//! # Network Group module
//!
//! This module provide structures and helpers to interact with Clever Cloud's
//! Network Group API.

use std::net::IpAddr;

use cidr::IpCidr;

use crate::v4::network_group::{
    clever_peer::CleverPeer, network_group_id::NetworkGroupId, owner_id::OwnerId,
};

pub mod clever_peer;
pub mod component;
pub mod delete;
pub mod endpoint;
pub mod external_peer;
pub mod member;
pub mod member_id;
pub mod member_kind;
pub mod network_group_id;
pub mod owner_id;
pub mod peer_id;
pub mod peer_kind;
pub mod wannabe_external_peer;
pub mod wannabe_member;
pub mod wireguard;
pub mod wireguard_configuration;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NetworkGroup {
    #[serde(rename = "id")]
    pub ng_id: NetworkGroupId,
    #[serde(rename = "ownerId")]
    pub owner_id: OwnerId,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "description", default)]
    pub description: Option<String>,
    #[serde(rename = "networkIp")]
    pub ng_ip: IpCidr,
    #[serde(rename = "lastAllocatedIp")]
    pub last_allocated_ip: IpAddr,
    #[serde(rename = "tags", default)]
    pub tags: Vec<String>,
    #[serde(rename = "peers", default)]
    pub peers: Vec<CleverPeer>,
}

// /v4/networkgroups/organisations/{ownerId}/networkgroups/{networkGroupId}
