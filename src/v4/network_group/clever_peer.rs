use crate::v4::network_group::{
    endpoint::Endpoint, peer_id::PeerId, wireguard::WireGuardPublicKey,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CleverPeer {
    #[serde(rename = "id")]
    peer_id: PeerId,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "publicKey")]
    public_key: WireGuardPublicKey,
    #[serde(rename = "endpoint")]
    endpoint: Endpoint,
    #[serde(rename = "hostname")]
    hostname: String,
    #[serde(rename = "parentMember")]
    parent_member: String,
    #[serde(rename = "parentEvent")]
    parent_event: Option<String>,
    #[serde(rename = "hv")]
    hv: String,
}
