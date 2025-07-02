use std::net::{IpAddr, SocketAddr, ToSocketAddrs};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ClientEndpoint {
    #[serde(rename = "ngIp")]
    ng_ip: IpAddr,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename = "ngTerm")]
pub struct NetworkGroupTerm {
    #[serde(rename = "host")]
    host: IpAddr,
    #[serde(rename = "port")]
    port: u16,
}

impl ToSocketAddrs for NetworkGroupTerm {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        (self.host, self.port).to_socket_addrs()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ServerEndpoint {
    #[serde(rename = "ngTerm")]
    ng_term: NetworkGroupTerm,
    #[serde(rename = "publicTerm")]
    public_term: NetworkGroupTerm,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Endpoint {
    Client(ClientEndpoint),
    Server(ServerEndpoint),
}
