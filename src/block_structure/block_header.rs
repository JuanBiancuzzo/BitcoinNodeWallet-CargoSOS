use super::{block_version::BlockVersion, transaction::Transaction};

use crate::serialization::{
    serializable::Serializable,
    error_serialization::ErrorSerialization,
};

use std::io::{
    Write,
};


const GENESIS_BLOCK_VERSION: BlockVersion = BlockVersion::V1;
const GENESIS_PREVIOUS_BLOCK_HEADER_HASH: [u8; 32] = [0; 32];
const GENESIS_MERKLE_ROOT_HASH: [u8; 32] = [0x3b, 0xa3, 0xed, 0xfd, 0x7a, 0x7b, 0x12, 0xb2, 0x7a, 0xc7, 0x2c, 0x3e, 0x67, 0x76, 0x8f, 0x61, 0x7f, 0xc8, 0x1b, 0xc3, 0x88, 0x8a, 0x51, 0x32, 0x3a, 0x9f, 0xb8, 0xaa, 0x4b, 0x1e, 0x5e, 0x4a];
const GENESIS_TIME: u32 = 1231013705;
const GENESIS_N_BITS: u32 = 0x1d00ffff;
const GENESIS_NONCE: u32 = 2083236893;

pub struct BlockHeader {
    pub version: BlockVersion,
    pub previous_block_header_hash: [u8; 32],
    pub merkle_root_hash: [u8; 32],
    pub time: u32,
    pub n_bits: u32,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn new(
        version: BlockVersion,
        previous_block_header_hash: [u8; 32],
        merkle_root_hash: [u8; 32],
        time: u32,
        n_bits: u32,
        nonce: u32,
    ) -> Self {
        BlockHeader {
            version,
            previous_block_header_hash,
            merkle_root_hash,
            time,
            n_bits,
            nonce,
        }
    }

    pub fn generate_genesis_block_header() -> Self {
        let genesis_block_header = BlockHeader::new(
            GENESIS_BLOCK_VERSION,
            GENESIS_PREVIOUS_BLOCK_HEADER_HASH,
            GENESIS_MERKLE_ROOT_HASH,
            GENESIS_TIME,
            GENESIS_N_BITS,
            GENESIS_NONCE,
        );
        genesis_block_header
    }

    pub fn proof_of_work(&self) -> bool {
        todo!()
    }

    pub fn proof_of_inclusion(&self, transactions: &[Transaction]) -> bool {
        todo!()
    }

}

impl Serializable for BlockHeader {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        self.version.serialize(stream)?;
        self.previous_block_header_hash.serialize(stream)?;
        self.merkle_root_hash.serialize(stream)?;
        self.time.serialize(stream)?;
        self.n_bits.serialize(stream)?;
        self.nonce.serialize(stream)
    }
}