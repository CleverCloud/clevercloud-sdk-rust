use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// UNKNOWN VARIANT ERROR ///////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct UnknownVariant {
    pub name: &'static str,
    pub variants: &'static [&'static str],
    pub found: Option<Box<str>>,
}

impl fmt::Display for UnknownVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            variants,
            ref found,
        } = *self;

        if let Some(found) = found {
            write!(f, "unknown {name} `{found}`")?;
        } else {
            write!(f, "missing {name}")?;
        }

        if let Some((tail, variants)) = variants.split_last() {
            f.write_str(", expected ")?;
            let mut variants = variants.iter();
            if let Some(head) = variants.next() {
                write!(f, "one of `{head}`")?;
                for variant in variants {
                    write!(f, ", `{variant}`")?;
                }
                write!(f, " or ")?;
            }
            write!(f, "`{tail}`")?;
        }

        Ok(())
    }
}

impl core::error::Error for UnknownVariant {}

// OWNER KIND //////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnerKind {
    User,
    Organisation,
}

impl OwnerKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Organisation => "orga",
        }
    }

    /// Returns `true` if the owner kind is [`User`].
    ///
    /// [`User`]: OwnerKind::User
    #[must_use]
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User)
    }

    /// Returns `true` if the owner kind is [`Organisation`].
    ///
    /// [`Organisation`]: OwnerKind::Organisation
    #[must_use]
    pub const fn is_organisation(&self) -> bool {
        matches!(self, Self::Organisation)
    }
}

impl fmt::Display for OwnerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for OwnerKind {
    type Err = UnknownVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "user" => Self::User,
            "orga" => Self::Organisation,
            _ => {
                return Err(UnknownVariant {
                    name: "owner kind",
                    variants: &["user", "kind"],
                    found: Some(s.into()),
                });
            }
        })
    }
}

// OWNER ID PARSE ERROR KIND ///////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
enum OwnerIdParseErrorKind {
    #[error("missing prefix `ng_` prefix")]
    MissingPrefix,
    #[error(transparent)]
    Prefix(UnknownVariant),
    #[error("invalid UUID, {0}")]
    Uuid(uuid::Error),
}

// OWNER ID PARSE ERROR ////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
#[error("invalid owner identifier: {kind}")]
pub struct OwnerIdParseError {
    kind: OwnerIdParseErrorKind,
}

// OWNER ID ////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerId {
    kind: OwnerKind,
    uuid: Uuid,
}

impl OwnerId {
    pub const fn kind(&self) -> OwnerKind {
        self.kind
    }

    pub const fn is_user(&self) -> bool {
        self.kind.is_user()
    }

    pub const fn is_organisation(&self) -> bool {
        self.kind.is_organisation()
    }
}

impl fmt::Display for OwnerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.kind, self.uuid.as_hyphenated())
    }
}

impl FromStr for OwnerId {
    type Err = OwnerIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s.split_once('_') {
            Some((prefix, s)) => match (prefix.parse(), s.parse::<uuid::fmt::Hyphenated>()) {
                (Ok(kind), Ok(hyphenated)) => {
                    return Ok(Self {
                        kind,
                        uuid: hyphenated.into_uuid(),
                    });
                }
                (Err(e), _) => OwnerIdParseErrorKind::Prefix(e),
                (_, Err(e)) => OwnerIdParseErrorKind::Uuid(e),
            },
            None => OwnerIdParseErrorKind::MissingPrefix,
        };
        Err(OwnerIdParseError { kind })
    }
}

impl Serialize for OwnerId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OwnerId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
