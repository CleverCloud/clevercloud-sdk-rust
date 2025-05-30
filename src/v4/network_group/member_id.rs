use core::fmt;
use core::str::FromStr;

use uuid::Uuid;

use crate::v4::network_group::{member_kind::MemberKind, owner_id::UnknownVariant};

// MEMBER ID PARSE ERROR KIND //////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
pub enum MemberIdParseErrorKind {
    #[error("missing prefix")]
    MissingPrefix,
    #[error(transparent)]
    Prefix(UnknownVariant),
    #[error("invalid UUID, {0}")]
    Uuid(uuid::Error),
}

// MEMBER ID PARSE ERROR ///////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
#[error("invalid owner identifier: {kind}")]
pub struct MemberIdParseError {
    kind: MemberIdParseErrorKind,
}

// MEMBER ID //////////////////////////////////////////////////////////////:////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberId {
    kind: MemberKind,
    uuid: Uuid,
}

impl fmt::Display for MemberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.kind.as_prefix(), self.uuid.as_hyphenated())
    }
}

impl FromStr for MemberId {
    type Err = MemberIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s.split_once('_') {
            Some((prefix, s)) => match (
                MemberKind::from_prefix(prefix),
                s.parse::<uuid::fmt::Hyphenated>(),
            ) {
                (Ok(kind), Ok(hyphenated)) => {
                    return Ok(Self {
                        kind,
                        uuid: hyphenated.into_uuid(),
                    });
                }
                (Err(e), _) => MemberIdParseErrorKind::Prefix(e),
                (_, Err(e)) => MemberIdParseErrorKind::Uuid(e),
            },
            None => MemberIdParseErrorKind::MissingPrefix,
        };
        Err(MemberIdParseError { kind })
    }
}

impl serde::Serialize for MemberId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for MemberId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
