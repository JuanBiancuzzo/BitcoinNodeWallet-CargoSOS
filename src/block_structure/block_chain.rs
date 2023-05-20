use super::{
    block::Block, 
    block_header::BlockHeader, 
    node_chain::NodeChain,
    error_block::ErrorBlock,
};

use crate::serialization::{
    serializable_little_endian::SerializableLittleEndian,
    serializable_internal_order::SerializableInternalOrder,
    deserializable_little_endian::DeserializableLittleEndian,
    deserializable_internal_order::DeserializableInternalOrder,
    error_serialization::ErrorSerialization,
};

use std::io::{
    Read,
    Write,
};

const LAST_BLOCK_COUNT_SIZE: usize =  8;
const BLOCKS_COUNT_SIZE: usize =  8;
const BLOCK_CHAIN_SIZE: usize =  8;

const HEADER_SIZE: usize = LAST_BLOCK_COUNT_SIZE + BLOCKS_COUNT_SIZE + BLOCK_CHAIN_SIZE;

#[derive(Debug, Clone)]
pub struct BlockChain {
    
    blocks: Vec<NodeChain>,
    last_blocks: Vec<usize>,
}

use crate::block_structure::{
    transaction_output::TransactionOutput,
    hash::HashType,
};

impl BlockChain {

    pub fn new(block: Block) -> Result<Self, ErrorBlock> {

        let first_node: NodeChain = NodeChain::first(block)?;
        
        let blocks: Vec<NodeChain> = vec![first_node];
        let last_blocks: Vec<usize> = vec![0];
        
        Ok(BlockChain { 
            blocks,
            last_blocks,
        })
    }

    pub fn append_header(&mut self, header: BlockHeader) -> Result<(), ErrorBlock> {
        self.append_block(Block::new(header))
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), ErrorBlock> {

        for (i, index_last_block) in self.last_blocks.clone().iter().enumerate() {

            let last_block = self.get_block_at(*index_last_block)?;

            if last_block.is_equal(&block) {
                return Err(ErrorBlock::TransactionAlreadyInBlock);
            }

            if last_block.is_previous_of(&block) {

                let node = NodeChain::new(block, *index_last_block)?;
                self.blocks.push(node);

                self.last_blocks[i] = self.blocks.len() - 1;

                return Ok(());
            }

            while let Some(index_previous_node) = last_block.index_previous_node {

                let last_block = self.get_block_at_mut(index_previous_node)?;

                if last_block.is_equal(&block) {
                    return Err(ErrorBlock::TransactionAlreadyInBlock);
                }
    
                if last_block.is_previous_of(&block) {
    
                    let node = NodeChain::new(block, *index_last_block)?;
                    self.blocks.push(node);

                    self.last_blocks.push(self.blocks.len() - 1);
    
                    return Ok(());
                }                
            }
        }

        Err(ErrorBlock::CouldNotAppendBlock)
    }

    pub fn update_block(&mut self, block: Block) -> Result<(), ErrorBlock> {

        for current_block in self.blocks.iter_mut() {

            if current_block.is_equal(&block) {

                return current_block.update_block(block);
            }
        }

        Err(ErrorBlock::CouldNotUpdate)
    }

    pub fn get_blocks_after_timestamp(&self, timestamp: u32) -> Result<Vec<Block>, ErrorBlock> {
        
        let mut blocks_after_timestamp: Vec<Block> = Vec::new();

        for current_block in self.blocks.iter() {

            if current_block.block.header.time > timestamp {

                blocks_after_timestamp.push(current_block.block.clone());
            }
        }

        Ok(blocks_after_timestamp)
    }

    pub fn latest(&self) -> Vec<Block> {
        
        let mut latest: Vec<Block> = Vec::new();

        println!("Last blocks: {:?}", self.last_blocks);

        for index_last_block in self.last_blocks.iter() {

            let last_block = match self.get_block_at(*index_last_block) {
                Ok(block) => block,
                Err(_) => continue,
            };

            latest.push(last_block.block.clone());
        }

        latest
    }

    fn get_block_at(&self, index: usize) -> Result<NodeChain, ErrorBlock> {
        match self.blocks.get(index) {
            Some(block) => Ok(block.clone()),
            None => Err(ErrorBlock::NodeChainReferenceNotFound),
        }
    }

    fn get_block_at_mut(&mut self, index: usize) -> Result<NodeChain, ErrorBlock> {
        match self.blocks.get(index) {
            Some(block) => Ok(block.clone()),
            None => Err(ErrorBlock::NodeChainReferenceNotFound),
        }
    }

    pub fn get_utxo(&self) -> Vec<TransactionOutput> {
        let mut utxo: Vec<(TransactionOutput, HashType, u32)> = vec![];
        for node_chain in self.blocks.iter() {
            node_chain.block.update_utxo_list(&mut utxo);
        }
        utxo.retain(|(output, _, _)| output.value != -1);
        utxo.iter().map(|(output, _, _)| output.clone()).collect()
    }


}

impl SerializableInternalOrder for BlockChain {
    
    fn io_serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {

        let mut block_chain: Vec<u8> = Vec::new();

        for block in self.blocks.iter() {
            block.io_serialize(&mut block_chain)?;
        }

        for index_last_block in self.last_blocks.iter() {
            (index_last_block.clone() as u64).le_serialize(&mut block_chain)?;
        }
        
        let mut header: Vec<u8> = Vec::new();
        
        (self.last_blocks.len() as u64).le_serialize(&mut header)?;
        (self.blocks.len() as u64).le_serialize(&mut header)?;
        (block_chain.len() as u64).le_serialize(&mut header)?;

        header.io_serialize(stream)?;
        block_chain.io_serialize(stream)?;

        Ok(())
    }
}

impl DeserializableInternalOrder for BlockChain {

    fn io_deserialize(stream: &mut dyn Read) -> Result<Self, ErrorSerialization> {
        let mut header: Vec<u8> = vec![0; HEADER_SIZE];
        if stream.read_exact(&mut header).is_err() {

            return Err(ErrorSerialization::ErrorWhileReading);
        }
        let mut header = &header[..];

        let last_blocks_count = u64::le_deserialize(&mut header)?;
        let blocks_count = u64::le_deserialize(&mut header)?;
        let block_chain_size = u64::le_deserialize(&mut header)?;

        let mut block_chain: Vec<u8> = vec![0; block_chain_size as usize];
        if stream.read_exact(&mut block_chain).is_err() {

            return Err(ErrorSerialization::ErrorWhileReading);
        }

        let mut block_chain = &block_chain[..];

        let mut blocks: Vec<NodeChain> = Vec::new();
        for _ in 0..blocks_count {
            blocks.push(NodeChain::io_deserialize(&mut block_chain)?);
        }

        let mut last_blocks: Vec<usize> = Vec::new();
        for _ in 0..last_blocks_count {
            last_blocks.push(u64::le_deserialize(&mut block_chain)? as usize);
        }

        Ok(BlockChain { 
            blocks, 
            last_blocks,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::block_structure::{
        compact256::Compact256,
        block_version,
        transaction_input::TransactionInput,
        transaction_output::TransactionOutput,
        transaction::Transaction,
        outpoint::Outpoint,
        hash::hash256d,
    };
    use crate::messages::compact_size::CompactSize;
    use crate::serialization::serializable_little_endian::SerializableLittleEndian;
    use super::*;

    #[test]
    fn test_01_correct_append_header() {
        let block = Block::new(
            BlockHeader::new(
                block_version::BlockVersion::version(1),
                [0; 32],
                [0; 32],
                0,
                Compact256::from(10),
                0,
                CompactSize::new(0),
            )
        );

        let hash_of_first_block_header = block.header.get_hash256d().unwrap();

        let mut blockchain = BlockChain::new(block).unwrap();

        let header_to_append = BlockHeader::new(
            block_version::BlockVersion::version(1),
            hash_of_first_block_header.clone(),
            [0; 32],
            0,
            Compact256::from(10),
            0,
            CompactSize::new(0),
        );

        blockchain.append_header(header_to_append.clone()).unwrap();
        assert_eq!(blockchain.blocks[1].block.header, header_to_append);

    }

    #[test]
    fn test_02_correct_block_update() {

        let transaction_input = TransactionInput::new(
            Outpoint { hash: [1;32], index: 23 },
            String::from("Prueba in"),
            24
        );

        let transaction_output = TransactionOutput{
            value: 10, 
            pk_script: String::from("Prueba out")
        };

        let transaction = Transaction {
            version : 1,
            tx_in: vec![transaction_input.clone()],
            tx_out: vec![transaction_output.clone()],
            time: 0,
        };

        let empty_block = Block::new(
            BlockHeader::new(
                block_version::BlockVersion::version(1),
                [0; 32],
                [0; 32],
                0,
                Compact256::from(10),
                0,
                CompactSize::new(0),
            )
        );

        let mut block_with_transactions = empty_block.clone();
        block_with_transactions.append_transaction(transaction.clone()).unwrap();

        let mut blockchain = BlockChain::new(empty_block).unwrap();

        blockchain.update_block(block_with_transactions).unwrap();

        assert_eq!(blockchain.blocks[0].block.transactions[0], transaction);

    } 

    #[test]
    fn test_03_correct_get_block_after_timestamp() {

        let block = Block::new(
            BlockHeader::new(
                block_version::BlockVersion::version(1),
                [0; 32],
                [0; 32],
                0,
                Compact256::from(10),
                0,
                CompactSize::new(0),
            )
        );

        let hash_of_first_block_header = block.header.get_hash256d().unwrap();

        let mut blockchain = BlockChain::new(block).unwrap();

        let header_to_append = BlockHeader::new(
            block_version::BlockVersion::version(1),
            hash_of_first_block_header.clone(),
            [3; 32],
            5,
            Compact256::from(10),
            21,
            CompactSize::new(0),
        );

        blockchain.append_header(header_to_append.clone()).unwrap();

        let block_after_timestamp = blockchain.get_blocks_after_timestamp(3).unwrap();
        assert_eq!(block_after_timestamp[0].header, header_to_append);
    }

    #[test]
    fn test_04_correct_get_latest() {

        let block = Block::new(
            BlockHeader::new(
                block_version::BlockVersion::version(1),
                [0; 32],
                [0; 32],
                0,
                Compact256::from(10),
                0,
                CompactSize::new(0),
            )
        );

        let hash_of_first_block_header = block.header.get_hash256d().unwrap();
    
        let mut blockchain = BlockChain::new(block).unwrap();

        let header_to_append = BlockHeader::new(
            block_version::BlockVersion::version(1),
            hash_of_first_block_header.clone(),
            [3; 32],
            5,
            Compact256::from(10),
            21,
            CompactSize::new(0),
        );

        blockchain.append_header(header_to_append.clone()).unwrap();

        let last_blocks = blockchain.latest();
        assert_eq!(last_blocks[0].header, header_to_append);
    }

    #[test]
    fn test_05_correct_get_utxo() {

        let transaction_output_1 = TransactionOutput{
            value: 10, 
            pk_script: String::from("Prueba out")
        };

        let transaction_output_2 = TransactionOutput{
            value: 20, 
            pk_script: String::from("Prueba out")
        };

        let transaction_output = Transaction {
            version : 1,
            tx_in: vec![],
            tx_out: vec![transaction_output_1.clone(), transaction_output_2.clone()],
            time: 0,
        };

        let mut serialized_transaction = Vec::new();
        transaction_output.le_serialize(&mut serialized_transaction).unwrap();
        let hashed_transaction = hash256d(&serialized_transaction).unwrap();


        let mut block_transaction_output = Block::new(
            BlockHeader::new(
                block_version::BlockVersion::V1,
                [0; 32],
                [0; 32],
                0,
                Compact256::from(10),
                0,
                CompactSize::new(0),
            )
        );

        block_transaction_output.append_transaction(transaction_output).unwrap();


        let transaction_input_1 = TransactionInput::new(
            Outpoint { hash: hashed_transaction, index: 0 },
            String::from("Prueba in"),
            24
        );

        let transaction_input = Transaction {
            version : 1,
            tx_in: vec![transaction_input_1.clone()],
            tx_out: vec![],
            time: 0,
        };

        let hash_block_transaction_output = block_transaction_output.header.get_hash256d().unwrap();

        let mut block_transaction_input = Block::new(
            BlockHeader::new(
                block_version::BlockVersion::V1,
                hash_block_transaction_output,
                [0; 32],
                0,
                Compact256::from(10),
                0,
                CompactSize::new(0),
            )
        );

        block_transaction_input.append_transaction(transaction_input).unwrap();

        let mut blockchain = BlockChain::new(block_transaction_output).unwrap();
        blockchain.append_block(block_transaction_input).unwrap();

        let utxo = blockchain.get_utxo();
        assert_eq!(utxo[0], transaction_output_2);
    }

}