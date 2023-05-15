use std::io::{
    Read,
    Write,
};

use crate::messages::{
    get_headers_message::GetHeadersMessage,
    headers_message::HeadersMessage,

    message_header::MessageHeader,
    command_name::CommandName,

    inventory_message::InventoryMessage,
    block_message::BlockMessage,

    error_message::ErrorMessage,
};

use crate::connections::{
    type_identifier::TypeIdentifier,
};

use crate::block_structure::{
    block_chain::BlockChain,
    block::Block,
    block_header::BlockHeader,
    hash::{
        HashType,
        hash256d
    }
};

use super::{
    error_node::ErrorNode
};

use crate::serialization::{
    serializable::Serializable,
    deserializable::Deserializable,
};

use crate::connections::{
    p2p_protocol::ProtocolVersionP2P, 
};

const TESTNET_MAGIC_NUMBERS: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];
const NO_STOP_HASH: HashType = [0; 32];

#[derive(Debug, Clone)]
pub struct InitialBlockDownload {
    pub protocol_version: ProtocolVersionP2P,
}

impl InitialBlockDownload {
    pub fn new(protocol_version: ProtocolVersionP2P) -> Self {
        InitialBlockDownload {
            protocol_version,
        }
    }

    pub fn send_get_headers_message<RW : Read + Write>(
        &self, peer_stream: &mut RW, 
        block_chain: &BlockChain
    ) -> Result<(), ErrorMessage>
    {
        let last_header: &BlockHeader = &block_chain.last().header;
        let mut serialized_header = Vec::new();

        last_header.serialize(&mut serialized_header)?;
        
        let hashed_header: HashType = hash256d(&serialized_header)?;
        
        let get_headers_message = GetHeadersMessage::new(
            TESTNET_MAGIC_NUMBERS,
            self.protocol_version,
            vec![hashed_header],
            NO_STOP_HASH,
        );
        get_headers_message.serialize(peer_stream)?;
        Ok(())
    }

    pub fn add_headers_to_blockchain(
        &self, 
        block_chain: &mut BlockChain, 
        received_headers_message: &HeadersMessage
    ) -> Result<u32,ErrorNode> 
    {
        let mut added_headers = 0;
        for header in &received_headers_message.headers {
            if !header.proof_of_work() {
                return Err(ErrorNode::ErrorWhileValidating("Error while validating proof of work".to_string()));
            }
            if block_chain.append_header(*header).is_ok() {
                added_headers += 1;
            }
        }
        Ok(added_headers)
    }

    pub fn get_headers<RW : Read + Write>(
        &self, 
        peer_stream: &mut RW, 
        block_chain: &mut BlockChain
    ) -> Result<u32,ErrorNode>
    {
        self.send_get_headers_message(peer_stream, block_chain)?;
        let received_headers_message = HeadersMessage::deserialize(peer_stream)?;
        let added_headers = self.add_headers_to_blockchain(block_chain, &received_headers_message)?;
        Ok(added_headers)
    }
  
    pub fn get_data<RW : Read + Write>(
        &self,
        peer_stream: &mut RW,
        hashed_header: &HashType,
    ) -> Result<Block, ErrorMessage> 
    {
        let inventory_message = InventoryMessage {
            type_identifier: TypeIdentifier::Block,
            hash_value: *hashed_header,
        };

        MessageHeader::serialize_message(
            peer_stream, 
            TESTNET_MAGIC_NUMBERS, 
            CommandName::Inventory, 
            &inventory_message,
        )?;

        let header = MessageHeader::deserialize_until_found(peer_stream, CommandName::Block)?;
        let block_message = BlockMessage::deserialize_message(peer_stream, header)?;
        
        match block_message.block.proof_of_inclusion() {
            true => Ok(block_message.block),
            false => Err(ErrorMessage::ErrorInDeserialization("Error while receiving block message".to_string())),
        }
    }
}