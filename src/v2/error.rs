use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[error("got response {kind} ({id}), {message}")]
pub struct InvalidResponseBody {
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
}
