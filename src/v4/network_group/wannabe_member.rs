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
    #[error(transparent)]
    Status(#[from] ErrorResponse),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WannabeNetworkGroupMember {
    #[serde(rename = "id")]
    member_id: MemberId,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(rename = "domainName")]
    domain_name: String,
    #[serde(rename = "kind")]
    kind: MemberKind,
}

impl WannabeNetworkGroupMember {
    /// Add a Member to a Network Group.
    pub async fn post(
        &self,
        client: &Client,
        owner_id: &OwnerId,
        ng_id: NetworkGroupId,
    ) -> Result<(), Error> {
        let endpoint = client.endpoint(format_args!(
            "/v4/networkgroups/organisations/{owner_id}/networkgroups/{ng_id}/members"
        ))?;

        debug!(
            %endpoint,
            owner = %owner_id,
            network_group = %ng_id,
            "execute a request to add member to Network Group"
        );

        Ok(client
            .post(endpoint, self)
            .await
            .map_err(|e| Error::Post(*owner_id, ng_id, e))??)
    }
}
