use core::{fmt, str};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v4::network_group::{owner_id::UnknownVariant, peer_kind::PeerKind};

// PEER ID PARSE ERROR KIND ////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
enum PeerIdParseErrorKind {
    #[error(transparent)]
    PeerKind(UnknownVariant),
    #[error("invalid UUID, {0}")]
    Uuid(uuid::Error),
}

// PEER ID PARSE ERROR /////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
#[error("invalid peer identifier: {kind}")]
pub struct PeerIdParseError {
    kind: PeerIdParseErrorKind,
}

// PEER ID /////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerId {
    peer_kind: PeerKind,
    uuid: Uuid,
}

impl PeerId {
    pub const fn peer_kind(&self) -> PeerKind {
        self.peer_kind
    }

    pub const fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub const fn is_external(&self) -> bool {
        self.peer_kind.is_external()
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(prefix) = self.peer_kind.prefix() {
            write!(f, "{prefix}_")?;
        }
        write!(f, "{}", self.uuid.as_hyphenated())
    }
}

impl str::FromStr for PeerId {
    type Err = PeerIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (peer_kind, s) = match s.split_once('_') {
            None => (PeerKind::default(), s),
            Some((prefix, s)) => match PeerKind::from_prefix(prefix) {
                Ok(peer_kind) => (peer_kind, s),
                Err(e) => {
                    return Err(PeerIdParseError {
                        kind: PeerIdParseErrorKind::PeerKind(e),
                    });
                }
            },
        };
        match s.parse::<uuid::fmt::Hyphenated>() {
            Ok(hyphenated) => Ok(Self {
                uuid: hyphenated.into_uuid(),
                peer_kind,
            }),
            Err(e) => Err(PeerIdParseError {
                kind: PeerIdParseErrorKind::Uuid(e),
            }),
        }
    }
}

impl<'de> Deserialize<'de> for PeerId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for PeerId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
