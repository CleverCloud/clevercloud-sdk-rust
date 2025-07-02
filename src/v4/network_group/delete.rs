use oauth10a::rest::RestClient;

use crate::{Client, EndpointError, RestError, v4::ErrorResponse};

use crate::v4::network_group::{
    network_group_id::NetworkGroupId, owner_id::OwnerId, peer_id::PeerId,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error("failed to delete for owner '{0}', network group '{1}', peer id '{2}', {0}")]
    Delete(OwnerId, NetworkGroupId, PeerId, RestError),
    #[error(transparent)]
    Status(#[from] ErrorResponse),
}

/// Delete a Network Group.
pub async fn delete(
    client: &Client,
    owner_id: &OwnerId,
    ng_id: &NetworkGroupId,
    peer_id: PeerId,
) -> Result<(), Error> {
    let endpoint = client.endpoint(format_args!(
        "/v4/networkgroups/organisations/{owner_id}/networkgroups/{ng_id}/external-peers/{peer_id}"
    ))?;

    debug!(
        %endpoint,
        owner = %owner_id,
        network_group = %ng_id,
        peer = %peer_id,
        "execute a request to delete peer from Network Group"
    );

    Ok(client
        .delete(endpoint)
        .await
        .map_err(|e| Error::Delete(*owner_id, *ng_id, peer_id, e))??)
}
