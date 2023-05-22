use std::io::{
    Read,
    Write,
};

use crate::messages::{
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
    hash::HashType,
};

const TESTNET_MAGIC_NUMBERS: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];

const MAX_HEADERS_COUNT: usize = 50_000;

#[derive(Debug, Clone)]
pub struct BlockDownload {
    sender_log: LoggerSender,
}

impl BlockDownload {
    pub fn new(
        sender_log: LoggerSender,
    ) -> Self 
    {
        BlockDownload {
            sender_log,
        }
    }

    fn send_get_data_message<RW : Read + Write>(
        &self, 
        peer_stream: &mut RW, 
        hashed_headers: Vec<HashType>,
    ) -> Result<(), ErrorMessage>
    {
        let _ = self.sender_log.log_connection(
            "Getting data".to_string()    
        );

        let get_data_message = GetDataMessage::new(hashed_headers);

        GetDataMessage::serialize_message(
            peer_stream, 
            TESTNET_MAGIC_NUMBERS, 
            &get_data_message,
        )?;

        Ok(())
    }

    fn receive_blocks<RW : Read + Write>(
        &self, 
        peer_stream: &mut RW,
        headers_count: usize,
    ) -> Result<Vec<Block>, ErrorMessage> 
    {
        let mut blocks: Vec<Block> = Vec::new();
        for i in 0..headers_count {

            if i % 100 == 0 {
                let _ = self.sender_log.log_connection(format!(
                    "Getting blocks [{i}]"
                ));
            }

            let header = message::deserialize_until_found(peer_stream, CommandName::Block)?;
            let block_message = BlockMessage::deserialize_message(peer_stream, header)?;
            
            /* Por ahora no funciona pero no encontramos el error
            let block = match block_message.block.proof_of_inclusion() {
                true => block_message.block,
                false => return Err(ErrorMessage::InDeserialization(
                    "Error while receiving block message".to_string()
                )),
            };
            */

            blocks.push(block_message.block);
        }

        Ok(blocks)
    }
  
    pub fn get_data<RW : Read + Write>(
        &self,
        peer_stream: &mut RW,
        hashed_headers: Vec<HashType>,
    ) -> Result<Vec<Block>, ErrorMessage> 
    {
        let headers_count = hashed_headers.len();

        if headers_count >= MAX_HEADERS_COUNT {
            let _ = self.sender_log.log_connection(
                "More headers than possible".to_string()    
            );
            return Err(ErrorMessage::RequestedDataTooBig);
        }

        self.send_get_data_message(peer_stream, hashed_headers)?;

        let _ = self.sender_log.log_connection(format!(
            "Downloading {headers_count} blocks",
        ));
        
        self.receive_blocks(peer_stream, headers_count)
    }
}