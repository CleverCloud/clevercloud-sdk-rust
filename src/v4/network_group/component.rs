use oauth10a::rest::RestClient;

use crate::{
    Client, EndpointError, RestError,
    v4::{
        ErrorResponse,
        network_group::{
            NetworkGroup, clever_peer::CleverPeer, external_peer::ExternalPeer, member::Member,
            owner_id::OwnerId,
        },
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error("failed to get Network Group component for owner '{0}', query '{1}', {0}")]
    Get(OwnerId, Box<str>, RestError),
    #[error(transparent)]
    Status(#[from] ErrorResponse),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum NetworkGroupComponent {
    CleverPeer(CleverPeer),
    ExternalPeer(ExternalPeer),
    Member(Member),
    NetworkGroup(NetworkGroup),
}

impl NetworkGroupComponent {
    /// Search a Network Group component (network group, member, or peer by its id or label).
    pub async fn get(client: &Client, owner_id: &OwnerId, query: String) -> Result<Self, Error> {
        let endpoint = client.endpoint(format_args!(
            "/v4/networkgroups/organisations/{owner_id}/networkgroups/search?{query}"
        ))?;

        debug!(
            %endpoint,
            owner = %owner_id,
            query = %query,
            "execute a request to search a Network Group component"
        );

        Ok(client
            .get(endpoint)
            .await
            .map_err(|e| Error::Get(*owner_id, query.into_boxed_str(), e))??)
    }
}
