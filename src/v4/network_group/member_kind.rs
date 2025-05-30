// MEMBER KIND /////////////////////////////////////////////////////////////////

use crate::v4::network_group::owner_id::UnknownVariant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MemberKind {
    #[serde(rename = "ADDON")]
    Addon,
    #[serde(rename = "APPLICATION")]
    Application,
    #[serde(rename = "EXTERNAL")]
    External,
}

impl MemberKind {
    pub const fn as_prefix(&self) -> &'static str {
        match self {
            Self::Addon => "addon",
            Self::Application => "app",
            Self::External => "external",
        }
    }

    pub fn from_prefix(prefix: &str) -> Result<Self, UnknownVariant> {
        Ok(match prefix {
            "addon" => Self::Addon,
            "app" => Self::Application,
            "external" => Self::External,
            _ => {
                return Err(UnknownVariant {
                    name: "member kind",
                    variants: &["addon", "app", "external"],
                    found: Some(prefix.into()),
                });
            }
        })
    }
}
