use std::io::{
    Read,
    Write,
};

use crate::messages::{
    get_headers_message::GetHeadersMessage,
    headers_message::HeadersMessage,

    message::{
        self,
        Message,
    },
    command_name::CommandName,
    get_data_message::GetDataMessage,

    block_message::BlockMessage,

    error_message::ErrorMessage, 
};

use crate::logs::logger_sender::LoggerSender;

use crate::block_structure::{
    block::Block,
    block_chain::BlockChain,
    block_header::BlockHeader,
    hash::HashType,
};

use super::{
    error_node::ErrorNode
};


use crate::connections::{
    p2p_protocol::ProtocolVersionP2P, 
};

const TESTNET_MAGIC_NUMBERS: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];
const NO_STOP_HASH: HashType = [0; 32];

const MAX_HEADERS_COUNT: usize = 50_000;

#[derive(Debug, Clone)]
pub struct InitialBlockDownload {
    pub protocol_version: ProtocolVersionP2P,
    sender_log: LoggerSender,
}

impl InitialBlockDownload {
    pub fn new(
        protocol_version: ProtocolVersionP2P,
        sender_log: LoggerSender,
    ) -> Self 
    {
        InitialBlockDownload {
            protocol_version,
            sender_log,
        }
    }

    fn send_get_headers_message<RW : Read + Write>(
        &self, peer_stream: &mut RW, 
        block_chain: &BlockChain
    ) -> Result<(), ErrorMessage>
    {
        let _ = self.sender_log.log_connection(
            "Serializing last headers from blockchain".to_string()    
        );

        let mut header_locator_hashes: Vec<HashType> = Vec::new();

        for block in block_chain.latest().iter() {

            let last_header: &BlockHeader = &block.header;
            header_locator_hashes.push(last_header.get_hash256d()?);
        }

        let get_headers_message = GetHeadersMessage::new(
            self.protocol_version,
            header_locator_hashes,
            NO_STOP_HASH,
        );
        
        GetHeadersMessage::serialize_message(
            peer_stream, 
            TESTNET_MAGIC_NUMBERS, 
            &get_headers_message,
        )?;
        
        let _ = self.sender_log.log_connection(
            "Sending the message".to_string()    
        );

        Ok(())
    }

    fn add_headers_to_blockchain(
        &self, 
        block_chain: &mut BlockChain, 
        received_headers_message: &HeadersMessage
    ) -> Result<u32,ErrorNode> 
    {
        let _ = self.sender_log.log_connection(
            "Adding headers to the blockchain".to_string()    
        );

        let mut added_headers = 0;
        for header in received_headers_message.headers.iter() {

            if !header.proof_of_work() && false { // cambiar
                return Err(ErrorNode::WhileValidating("Error while validating proof of work".to_string()));
            }

            match block_chain.append_header(*header) {
                Ok(_) => added_headers += 1,
                Err(error) => {
                    let _ = self.sender_log.log_connection(format!(
                        "Could not append header, we get {:?}\nWith hash {:?}", 
                        error,
                        header.previous_block_header_hash,
                    ));
                    break;                    
                }
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
        let _ = self.sender_log.log_connection(
            "Sending get headers message".to_string()    
        );

        self.send_get_headers_message(peer_stream, block_chain)?;

        let header_headers_message = match message::deserialize_until_found(
            peer_stream, 
            CommandName::Headers,
        ) {
            Ok(header) => header,
            Err(error) => return Err(ErrorNode::NodeNotResponding(
                format!("Error while receiving headers message: {:?}", error)
            )),
        };

        let _ = self.sender_log.log_connection(
            "Receiving headers message".to_string()    
        );

        let received_headers_message = match HeadersMessage::deserialize_message(
            peer_stream, 
            header_headers_message
        ) {
            Ok(headers_message) => headers_message,
            Err(error) => return Err(ErrorNode::NodeNotResponding(
                format!("Error while receiving headers message: {:?}", error)
            )),
        };

        Ok(self.add_headers_to_blockchain(block_chain, &received_headers_message)?)
    }
  
    pub fn get_data<RW : Read + Write>(
        &self,
        peer_stream: &mut RW,
        hashed_headers: Vec<HashType>,
    ) -> Result<Vec<Block>, ErrorMessage> 
    {
        let _ = self.sender_log.log_connection(
            "Getting data".to_string()    
        );

        let headers_count = hashed_headers.len();

        if headers_count >= MAX_HEADERS_COUNT {
            let _ = self.sender_log.log_connection(
                "More headers than possible".to_string()    
            );
            return Err(ErrorMessage::RequestedDataTooBig);
        }

        let get_data_message = GetDataMessage::new(hashed_headers);

        GetDataMessage::serialize_message(
            peer_stream, 
            TESTNET_MAGIC_NUMBERS, 
            &get_data_message,
        )?;

        let mut blocks: Vec<Block> = Vec::new();
        let _ = self.sender_log.log_connection(format!(
            "Downloading {headers_count} blocks",
        ));
        
        for i in 0..headers_count {

            if i % 100 == 0 {
                let _ = self.sender_log.log_connection(format!(
                    "Getting blocks [{i}]"
                ));
            }

            let header = message::deserialize_until_found(peer_stream, CommandName::Block)?;
            let block_message = BlockMessage::deserialize_message(peer_stream, header)?;
            
            //println!("Block transaction count: {:?}", block_message.block.header.transaction_count);
            let block = match true || block_message.block.proof_of_inclusion() {
                true => block_message.block,
                false => return Err(ErrorMessage::InDeserialization(
                    "Error while receiving block message".to_string()
                )),
            };

            blocks.push(block);
        }

        Ok(blocks)
    }
}
