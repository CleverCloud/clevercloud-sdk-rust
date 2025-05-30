use core::{fmt, str};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// NETWORK GROUP ID PARSE ERROR ////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
enum NetworkGroupIdParseErrorKind {
    #[error("expected `ng_` prefix")]
    MissingPrefix,
    #[error("invalid UUID, {0}")]
    Uuid(uuid::Error),
}

#[derive(Debug, thiserror::Error)]
#[error("invalid network group identifier: {kind}")]
pub struct NetworkGroupIdParseError {
    kind: NetworkGroupIdParseErrorKind,
}

// NETWORK GROUP ID ////////////////////////////////////////////////////////////

/// A network group identifier.
///
/// It is represented as an hyphenate lowercased UUID prefixed with `ng_`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NetworkGroupId(Uuid);

impl NetworkGroupId {
    /// Returns the name of the file in which the peer identifier is stored.
    pub fn ng_info_file_name(&self) -> PathBuf {
        PathBuf::from(self.to_string()).with_extension("id")
    }

    /// Returns the associated Wireguard interface name.
    pub fn wg_interface_name(&self) -> String {
        // requirement: `^[a-zA-Z0-9_=+.-]{1,15}$`.
        let time_low = &self.0.to_string()[..8];
        format!("wgcc{time_low}")
    }
}

impl str::FromStr for NetworkGroupId {
    type Err = NetworkGroupIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("ng_") {
            Some(s) => match s.parse::<uuid::fmt::Hyphenated>() {
                Ok(hyphenated) => Ok(Self(hyphenated.into_uuid())),
                Err(e) => Err(NetworkGroupIdParseError {
                    kind: NetworkGroupIdParseErrorKind::Uuid(e),
                }),
            },
            None => Err(NetworkGroupIdParseError {
                kind: NetworkGroupIdParseErrorKind::MissingPrefix,
            }),
        }
    }
}

impl fmt::Display for NetworkGroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ng_{}", self.0.as_hyphenated())
    }
}

impl<'de> Deserialize<'de> for NetworkGroupId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for NetworkGroupId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
