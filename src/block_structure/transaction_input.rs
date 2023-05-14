use super::outpoint::Outpoint;
use crate::serialization::{
    serializable::Serializable,
    error_serialization::ErrorSerialization,
};
use crate::messages::compact_size::CompactSize;
use std::io::Write;

pub struct TransactionInput {
    pub previous_output: Outpoint,
    pub signature_script: String,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: Outpoint, signature_script: String, sequence: u32) -> TransactionInput {
        TransactionInput {
            previous_output,
            signature_script,
            sequence,
        }
    }
}

impl Serializable for TransactionInput {
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        self.previous_output.serialize(stream)?;


        self.signature_script.serialize(stream)?;
        self.sequence.serialize(stream)?;

        Ok(())
    }
}