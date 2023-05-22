use super::{
    error_node::ErrorNode
};

use crate::messages::{
    headers_message::HeadersMessage,

    message::{
        self,
        Message,
    },
    command_name::CommandName,
};

use crate::logs::logger_sender::LoggerSender;

use crate::block_structure::{
    block::Block,
    block_chain::BlockChain,
    block_header::BlockHeader,
    hash::HashType,
};

use std::io::{
    Read,
    Write,
};

#[derive(Debug, Clone)]
pub struct BlockBroadcasting {

    sender_log: LoggerSender,
}

impl BlockBroadcasting {

    pub fn get_new_headers<RW : Read + Write>(
        &self, 
        peer_stream: &mut RW,
        block_chain: &mut BlockChain,
    ) -> Result<u32, ErrorNode> {
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

        match block_chain.append_headers(received_headers_message.headers) {
            Ok(count) => Ok(count),
            Err(error) => Err(ErrorNode::WhileValidating(
                format!("Error while appending headers, we get: {:?}", error)
            )),
        }
    }
}