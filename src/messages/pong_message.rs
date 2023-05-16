use super::{
    message_header::MessageHeader,
};

use crate::serialization::{
    serializable::Serializable,
    deserializable::Deserializable,
    error_serialization::ErrorSerialization,
};

use crate::block_structure::{
    hash::hash256d_reduce,
};

use std::io::{
    Read,
    Write,
};

pub struct PongMessage {
    pub nonce: u64,
}

impl PongMessage {

    pub fn deserialize_message(
        stream: &mut dyn Read, 
        message_header: MessageHeader,
    ) -> Result<Self, ErrorSerialization> 
    {
        let mut buffer: Vec<u8> = vec![0; message_header.payload_size as usize];
        if stream.read_exact(&mut buffer).is_err() {
            return Err(ErrorSerialization::ErrorWhileReading);
        }
        let mut buffer: &[u8] = &buffer[..];

        let message = Self::deserialize(&mut buffer)?;

        let mut serialized_message: Vec<u8> = Vec::new();
        message.serialize(&mut serialized_message)?;
        
        let checksum = hash256d_reduce(&serialized_message)?;
        if !checksum.eq(&message_header.checksum) {
            return Err(ErrorSerialization::ErrorInDeserialization(
                format!("Checksum in pong isn't the same: {:?} != {:?}", checksum, message_header.checksum)
            ));
        }

        Ok(message)        
    }

}

impl Serializable for PongMessage {
    
    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        self.nonce.serialize(stream)
    }
}

impl Deserializable for PongMessage {

    fn deserialize(stream: &mut dyn Read) -> Result<Self, ErrorSerialization> {
        Ok(PongMessage {
            nonce: u64::deserialize(stream)?,
        })
    }
}