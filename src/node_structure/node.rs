use crate::connections::{
    p2p_protocol::ProtocolVersionP2P,
    ibd_methods::IBDMethod,
    suppored_services::SupportedServices
};

use std::net::{
    Ipv6Addr,
    SocketAddr
};
use crate::connections::socket_conversion::socket_to_ipv6_port;
use crate::messages::version_message::VersionMessage;
use chrono::offset::Utc;

const IGNORE_NONCE: u64 = 0;
const IGNORE_USER_AGENT: &str = "";
const NO_NEW_TRANSACTIONS: bool = false; 


pub struct Node {
    protocol_version: ProtocolVersionP2P,
    ibd_method: IBDMethod,
    peers_addrs: Vec<Ipv6Addr>,
    services: SupportedServices,
    blockchain_height: i32,
}


impl Node {
    
    pub fn new(
        protocol_version: ProtocolVersionP2P,
        ibd_method: IBDMethod,
        services: SupportedServices,
        blockchain_height: i32,
    ) -> Self {
        Node {
            protocol_version,
            ibd_method,
            peers_addrs: vec![],
            services,
            blockchain_height
        }
    }

    ///Function that tries to build a version message with the current information of the node
    pub fn build_version_message(
        &self,
        recv_socket_addr: SocketAddr,
        recv_services: SupportedServices,
        trans_socket_addr: SocketAddr,
        nonce: u64,
        user_agent: String,
        relay: bool
    ) ->  VersionMessage {

        let timestamp = Utc::now();
        let (recv_addr, recv_port) = socket_to_ipv6_port(&recv_socket_addr);
        let (trans_addr, trans_port) = socket_to_ipv6_port(&trans_socket_addr);
        
        VersionMessage::new(
            self.protocol_version, 
            self.services, 
            timestamp, 
            recv_services, 
            recv_addr, 
            recv_port, 
            trans_addr, 
            trans_port, 
            nonce, 
            user_agent, 
            self.blockchain_height, 
            relay)
    }

    

    /*
    ///Función que intenta hacer el handshake
    pub fn handshake(potential_peers: Vec<SocketAddr>) {

    }
    */

}