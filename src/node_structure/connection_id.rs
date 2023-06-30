use super::connection_type::ConnectionType;

use std::{cmp::PartialEq, fmt::Display, net::SocketAddr};

#[derive(Debug, Clone, Copy)]
pub struct ConnectionId {
    connection_type: ConnectionType,
    address: SocketAddr,
}

impl ConnectionId {
    pub fn new(address: SocketAddr, connection_type: ConnectionType) -> Self {
        ConnectionId {
            connection_type,
            address,
        }
    }
}

impl PartialEq for ConnectionId {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.connection_type, self.address)
    }
}
