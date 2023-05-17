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
    serializable_internal_order::SerializableInternalOrder,
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

        let hashed_header: HashType = match self.block.header.get_hash256d() {
            Ok(hashed_header) => hashed_header,
            _ => return Err(ErrorBlock::CouldNotHash),
        };

        println!("nuestro hash: {:?}", hashed_header);
        println!("otro hash   : {:?}", previous_hashed_header);

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
        
        if self.block.header == block.header {
            self.block = block;
            return Ok(())
        }

        for next_block in self.next_blocks.iter_mut() {

            let block_clone = block.clone();
            match next_block.update_block(block_clone) {
                Err(ErrorBlock::CouldNotUpdate) | Ok(_) => continue,
                err => return err,
            }
        }

        Err(ErrorBlock::CouldNotUpdate)
    }

    pub fn get_block_after_timestamp(&self, timestamp: u32) -> Result<BlockChain, ErrorBlock> {
        
        if timestamp < self.block.header.time {
            return Ok(self.clone());
        }

        for next_block in self.next_blocks.iter() {
            match next_block.get_block_after_timestamp(timestamp) {
                Ok(block_after_timestamp) => return Ok(block_after_timestamp),
                Err(ErrorBlock::CouldNotFindBlockFarEnough) => continue,
                err => return err,
            }
        }

        Err(ErrorBlock::CouldNotFindBlockFarEnough)
    }

    pub fn last(&self) -> Vec<Block> {
        
        if self.next_blocks.is_empty() {
            return vec![self.block.clone()];
        }

        let mut last_blocks: Vec<Block> = Vec::new();
        for next_block in self.next_blocks.iter() {

            last_blocks.extend(next_block.last());
        }

        last_blocks
    }

    pub fn get_utxo(&self) -> Vec<TransactionOutput> {
        todo!()
    }
}
