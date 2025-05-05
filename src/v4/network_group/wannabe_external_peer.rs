use core::net::IpAddr;

use oauth10a::client::{ClientError, RestClient};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::Client;

use super::{
    MemberId, OwnerId, network_group_id::NetworkGroupId, peer::PeerId,
    wireguard::WireGuardPublicKey,
};

// PEER ROLE ///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerRole {
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
}

// PEER CREATED ////////////////////////////////////////////////////////////////

/// Response from [`WannabePeer`] and [`WannabeExternalPeer`] requests.
#[derive(Debug, Deserialize)]
pub struct PeerCreated {
    #[serde(rename = "peerId")]
    pub peer_id: PeerId,
}

// ERROR ///////////////////////////////////////////////////////////////////////

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to post Wannabe External Peer for owner '{0}', network group '{1}', {0}")]
    Post(String, NetworkGroupId, ClientError),
}

// WANNABE EXTERNAL PEER ///////////////////////////////////////////////////////

/// Request to join a Network Group as an external peer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WannabeExternalPeer {
    /// Label of a Network Group peer.
    #[serde(rename = "label")]
    pub label: String,
    /// The public IP v4 or v6 on which the peer is listening,
    /// when `peer_role` is [`PeerRole::Server`].
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpAddr>,
    /// The public TCP or UDP port number on which the peer is listening,
    /// when `peer_role` is [`PeerRole::Server`].
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// The role of this peer in the Network Group.
    #[serde(rename = "peerRole")]
    pub peer_role: PeerRole,
    /// Base64-encoded WireGuard public key of the peer.
    #[serde(rename = "publicKey")]
    pub public_key: WireGuardPublicKey,
    /// Host name of the peer.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Event that created the peer within the Network Group.
    #[serde(rename = "parentEvent", skip_serializing_if = "Option::is_none")]
    pub parent_event: Option<String>,
    /// Unique ID of a Network Group member.
    #[serde(rename = "parentMember")]
    pub parent_member: MemberId,
}

impl WannabeExternalPeer {
    pub async fn post(
        &self,
        client: &Client,
        owner_id: &OwnerId,
        ng_id: &NetworkGroupId,
    ) -> Result<PeerCreated, Error> {
        let endpoint = format!(
            "{}/v4/networkgroups/organisations/{owner_id}/networkgroups/{ng_id}/external-peers",
            client.endpoint,
        );

        #[cfg(feature = "tracing")]
        debug!("execute a request to join Network Group, endpoint: '{endpoint}'");

        client
            .post(&endpoint, self)
            .await
            .map_err(|e| Error::Post(owner_id.to_owned(), *ng_id, e))
    }
}
