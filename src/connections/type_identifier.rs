use crate::serialization::{
    deserializable_little_endian::DeserializableLittleEndian,
    error_serialization::ErrorSerialization, serializable_little_endian::SerializableLittleEndian,
};

use std::{
    cmp::PartialEq,
    io::{Read, Write},
};

const ERROR_VALUE: u32 = 0x00;
const TRANSACTION_ID_VALUE: u32 = 0x01;
const BLOCK_VALUE: u32 = 0x02;
const FILTERED_BLOCK_VALUE: u32 = 0x03;
const COMPACT_BLOCK_VALUE: u32 = 0x04;
const WITNESS_TRANSACTION_VALUE: u32 = 0x40000001;
const WITNESS_BLOCK_VALUE: u32 = 0x40000002;
const FILTERED_WITNESS_BLOCK_VALUE: u32 = 0x40000003;
const UNKNOWN_VALUE: u32 = 0x0201;

/// It's the representation of the type of data to request
#[derive(Debug, Clone, PartialEq)]
pub enum TypeIdentifier {
    Error,
    TransactionId,
    Block,
    FilteredBlock,
    CompactBlock,
    WitnessTransaction,
    WitnessBlock,
    FilteredWitnessBlock,
    Unknown,
}

impl SerializableLittleEndian for TypeIdentifier {
    fn le_serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        let value: u32 = match self {
            TypeIdentifier::Error => ERROR_VALUE,
            TypeIdentifier::TransactionId => TRANSACTION_ID_VALUE,
            TypeIdentifier::Block => BLOCK_VALUE,
            TypeIdentifier::FilteredBlock => FILTERED_BLOCK_VALUE,
            TypeIdentifier::CompactBlock => COMPACT_BLOCK_VALUE,
            TypeIdentifier::WitnessTransaction => WITNESS_TRANSACTION_VALUE,
            TypeIdentifier::WitnessBlock => WITNESS_BLOCK_VALUE,
            TypeIdentifier::FilteredWitnessBlock => FILTERED_WITNESS_BLOCK_VALUE,

            TypeIdentifier::Unknown => UNKNOWN_VALUE,
        };

        match value.le_serialize(stream) {
            Err(_) => Err(ErrorSerialization::ErrorInSerialization(format!(
                "While serializing the type identifier {:?}",
                self
            ))),
            _ => Ok(()),
        }
    }
}

impl DeserializableLittleEndian for TypeIdentifier {
    fn le_deserialize(stream: &mut dyn Read) -> Result<Self, ErrorSerialization> {
        let value = u32::le_deserialize(stream)?;

        match value {
            ERROR_VALUE => Ok(TypeIdentifier::Error),
            TRANSACTION_ID_VALUE => Ok(TypeIdentifier::TransactionId),
            BLOCK_VALUE => Ok(TypeIdentifier::Block),
            FILTERED_BLOCK_VALUE => Ok(TypeIdentifier::FilteredBlock),
            COMPACT_BLOCK_VALUE => Ok(TypeIdentifier::CompactBlock),
            WITNESS_TRANSACTION_VALUE => Ok(TypeIdentifier::WitnessTransaction),
            WITNESS_BLOCK_VALUE => Ok(TypeIdentifier::WitnessBlock),
            FILTERED_WITNESS_BLOCK_VALUE => Ok(TypeIdentifier::FilteredWitnessBlock),

            UNKNOWN_VALUE => {
                println!("We get a placeholder");
                Ok(TypeIdentifier::Unknown)
            }
            _ => Err(ErrorSerialization::ErrorInDeserialization(format!(
                "While deserializing the type identifier, we get: {}",
                value,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_serialize_correctly_type_identifier() -> Result<(), ErrorSerialization> {
        let expected_stream: Vec<u8> = vec![0x01, 0x00, 0x00, 0x40];
        
        let mut stream: Vec<u8> = Vec::new();
        let type_identifier = TypeIdentifier::WitnessTransaction;

        type_identifier.le_serialize(&mut stream)?;

        assert_eq!(expected_stream, stream);
        Ok(())
    } 

    #[test]
    fn test02_deserialize_correctly_type_identifier() -> Result<(), ErrorSerialization> {
        let mut stream: Vec<u8> = Vec::new();
        let type_identifier = TypeIdentifier::WitnessTransaction;

        type_identifier.le_serialize(&mut stream)?;

        let mut stream: &[u8] = &stream;

        let type_identifier_deserialized = TypeIdentifier::le_deserialize(&mut stream)?;

        assert_eq!(type_identifier, type_identifier_deserialized);

        Ok(())
    } 
}