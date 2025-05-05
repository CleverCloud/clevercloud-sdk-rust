use oauth10a::client::{ClientError, RestClient};
use tracing::debug;

use crate::Client;

use super::{OwnerId, network_group_id::NetworkGroupId, peer::PeerId};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to delete for owner '{0}', network group '{1}', peer id '{2}', {0}")]
    Delete(String, NetworkGroupId, PeerId, ClientError),
}

pub async fn delete(
    client: &Client,
    owner_id: &OwnerId,
    ng_id: &NetworkGroupId,
    peer_id: PeerId,
) -> Result<(), Error> {
    let endpoint = format!(
        "{}/v4/networkgroups/organisations/{owner_id}/networkgroups/{ng_id}/external-peers/{peer_id}",
        client.endpoint,
    );

    #[cfg(feature = "tracing")]
    debug!("execute a request to delete peer from Network Group, endpoint: '{endpoint}'");

    client
        .delete(&endpoint)
        .await
        .map_err(|e| Error::Delete(owner_id.to_owned(), *ng_id, peer_id, e))
}
