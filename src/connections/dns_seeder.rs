use std::net::{ToSocketAddrs, SocketAddr};
use super::connection_error::ConnectionError;

pub struct DNSSeeder {
    pub dns_addr: String,
    pub port_number: u16
}

impl DNSSeeder {
    pub fn new(dns_addr: &str, port_number: u16) -> Self {
        DNSSeeder {dns_addr: dns_addr.to_string() , port_number}
    }

    pub fn discover_peers(&self) -> Result<Vec<SocketAddr>, ConnectionError>{
        let mut peer_addrs: Vec<SocketAddr> = Vec::new();
        
        if let Ok(iter) = (self.dns_addr.clone(), self.port_number).to_socket_addrs() {
            for peer_addr in iter {
                peer_addrs.push(peer_addr);
            }
        
        } else {
            return Err(ConnectionError::ErrorInvalidIPOrPortNumber)
        } 

        Ok(peer_addrs)
    }
}