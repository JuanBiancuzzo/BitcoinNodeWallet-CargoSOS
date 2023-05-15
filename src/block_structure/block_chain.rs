use super::{
    block::Block, 
    block_header::BlockHeader, 
    hash::{
        HashType,
        hash256d,
    },
    transaction_output::TransactionOutput,
    error_block::ErrorBlock,
};

use crate::serialization::{
    serializable::Serializable,
};

#[derive(Debug, Clone)]
pub struct BlockChain {
    pub next_blocks: Vec<BlockChain>,
    pub block: Block,   
}

impl BlockChain {
    pub fn new(block: Block) -> Self {
        BlockChain {
            next_blocks: vec![],
            block,
        }
    }

    pub fn append_header(&mut self, header: BlockHeader) -> Result<(), ErrorBlock> {
        self.append_block(Block::new(header))
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), ErrorBlock> {
    
        let previous_hashed_header: HashType = block.header.previous_block_header_hash;

        let mut serialized_header: Vec<u8> = Vec::new();
        if self.block.header.serialize(&mut serialized_header).is_err() {
            return Err(ErrorBlock::CouldNotSerialize);
        }

        let hashed_header: HashType = match hash256d(&serialized_header) {
            Ok(hashed_header) => hashed_header,
            _ => return Err(ErrorBlock::CouldNotHash),
        };

        if previous_hashed_header == hashed_header {
            self.next_blocks.push(BlockChain::new(block));    
            return Ok(())
        }
        
        for next_block in self.next_blocks.iter_mut() {

            let block_clone = block.clone();
            match next_block.append_block(block_clone) {
                Err(ErrorBlock::CouldNotAppendBlock) | Ok(_) => continue,
                err => return err,
            }
        }
        
        Err(ErrorBlock::CouldNotAppendBlock)
    }

    pub fn update_block(&mut self, block: Block) -> Result<(), ErrorBlock> {
        todo!()
    }

    pub fn get_block_after_timestamp(&self, timestamp: u32) -> Result<BlockChain, ErrorBlock> {
        todo!()
    }

    pub fn last<'b>(&self) -> &'b Block {
        todo!()
    }

    pub fn get_utxo(&self) -> Vec<TransactionOutput> {
        todo!()
    }
}
