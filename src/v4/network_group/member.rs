use oauth10a::rest::RestClient;

use crate::{
    Client, EndpointError, RestError,
    v4::{
        ErrorResponse,
        network_group::{
            member_id::MemberId, member_kind::MemberKind, network_group_id::NetworkGroupId,
            owner_id::OwnerId,
        },
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error(
        "failed to post Wannabe Network Group member for owner '{0}', network group '{1}', {0}"
    )]
    Post(OwnerId, NetworkGroupId, RestError),
    #[error("failed to get Network Group member for owner '{0}', network group '{1}', {0}")]
    Get(OwnerId, NetworkGroupId, RestError),
    #[error(transparent)]
    Status(#[from] ErrorResponse),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Member {
    #[serde(rename = "id")]
    member_id: MemberId,
    #[serde(rename = "label")]
    label: String,
    #[serde(rename = "domainName")]
    domain_name: String,
    #[serde(rename = "kind")]
    kind: MemberKind,
}

impl Member {
    /// Get a Member of a Network Group.
    pub async fn get(
        client: &Client,
        owner_id: &OwnerId,
        ng_id: NetworkGroupId,
        member_id: MemberId,
    ) -> Result<Self, Error> {
        let endpoint = client.endpoint(format_args!(
            "/v4/networkgroups/organisations/{owner_id}/networkgroups/{ng_id}/members/{member_id}"
        ))?;

        debug!(
            %endpoint,
            owner = %owner_id,
            network_group = %ng_id,
            "execute a request to add member to Network Group"
        );

        let response: Self = client
            .get(endpoint)
            .await
            .map_err(|e| Error::Get(owner_id.to_owned(), ng_id, e))??;

        debug_assert_eq!(response.member_id, member_id);

        Ok(response)
    }
}
