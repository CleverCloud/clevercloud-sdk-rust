// PEER KIND ///////////////////////////////////////////////////////////////////

use crate::v4::network_group::owner_id::UnknownVariant;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize,
)]
pub enum PeerKind {
    #[default]
    #[serde(rename = "CLEVER")]
    Clever,
    #[serde(rename = "EXTERNAL")]
    External,
}

impl PeerKind {
    /// Returns `true` if the peer kind is [`External`](PeerKind::External).
    pub const fn is_external(&self) -> bool {
        matches!(self, Self::External)
    }

    pub const fn prefix(self) -> Option<&'static str> {
        match self {
            Self::Clever => None,
            Self::External => Some("external"),
        }
    }

    pub fn from_prefix(prefix: &str) -> Result<Self, UnknownVariant> {
        match prefix {
            "external" => Ok(Self::External),
            _ => Err(UnknownVariant {
                name: "peer kind",
                variants: &["external"],
                found: Some(prefix.into()),
            }),
        }
    }
}
