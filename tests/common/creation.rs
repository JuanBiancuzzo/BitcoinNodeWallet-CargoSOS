use cargosos_bitcoin::{
    logs::logger,
    connections::{
        p2p_protocol::ProtocolVersionP2P,
        supported_services::SupportedServices,
        ibd_methods::IBDMethod,
        type_identifier::TypeIdentifier,
    },
    messages::{
        compact_size::CompactSize,
    },
    block_structure::{
        block::Block,
        block_header::BlockHeader,
        block_chain::BlockChain,
        transaction::Transaction,
        transaction_input::TransactionInput,
        transaction_output::TransactionOutput,
        outpoint::Outpoint,
        block_version::BlockVersion,
        compact256::Compact256,
        hash::HashType,
    },
    node_structure::{
        handshake::Handshake,
        handshake_data::HandshakeData,
        initial_headers_download::InitialHeaderDownload,
        block_download::BlockDownload,
        peer_manager::PeerManager,
        message_response::MessageResponse,
    },
};

pub fn create_transaction(time: u32) -> Transaction {
    let transaction_input =
        TransactionInput::new(Outpoint::new([1; 32], 23), vec![1, 2, 3], 24);

    let transaction_output = TransactionOutput {
        value: 10,
        pk_script: vec![4, 5, 6],
    };

    Transaction {
        version: 1,
        tx_in: vec![transaction_input.clone()],
        tx_out: vec![transaction_output.clone()],
        time,
    }
}

pub fn create_empty_header(transaction_count: u64) -> BlockHeader {
    BlockHeader::new(
        BlockVersion::version(1),
        [0; 32],
        [0; 32],
        0,
        Compact256::from(u32::MAX),
        0,
        CompactSize::new(transaction_count),
    )
}

pub fn create_header(previous_header: HashType, transaction_count: u64) -> BlockHeader {
    BlockHeader::new(
        BlockVersion::version(1),
        previous_header,
        [0; 32],
        0,
        Compact256::from(u32::MAX),
        0,
        CompactSize::new(transaction_count),
    )
}

pub fn create_genesis_block() -> Block {
    Block::new(BlockHeader::generate_genesis_block_header())
}

pub fn create_empty_block(transaction_count: u64) -> Block {
    Block::new(create_empty_header(transaction_count))
}

pub fn create_block(previous_header: HashType, transaction_count: u64) -> Block {
    Block::new(create_header(previous_header, transaction_count))
}
