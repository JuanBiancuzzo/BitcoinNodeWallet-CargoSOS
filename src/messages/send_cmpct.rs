use super::{
    message_header::MessageHeader,
};

use crate::serialization::{
    deserializable::Deserializable, error_serialization::ErrorSerialization,
    serializable::Serializable,
};

use std::io::{
    Read, 
    Write
};

pub const SEND_HEADERS_CHECKSUM: [u8; 4] = [0x5d, 0xf6, 0xe0, 0xe2];

#[derive(Debug, std::cmp::PartialEq)]
pub struct SendCmpctMessage {
    pub announce: bool,
    pub version: u64,
}

impl SendCmpctMessage {
  
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
        
        if !SEND_HEADERS_CHECKSUM.eq(&message_header.checksum) {
            return Err(ErrorSerialization::ErrorInDeserialization(format!("Checksum isn't the same: {:?} != {:?}", SEND_HEADERS_CHECKSUM, message_header.checksum)));
        }

        Ok(message)
    }
}

impl Serializable for SendCmpctMessage {

    fn serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        self.announce.serialize(stream)?;
        self.version.serialize(stream)?;
        Ok(())
    }
}

impl Deserializable for SendCmpctMessage {
    fn deserialize(stream: &mut dyn Read) -> Result<Self, ErrorSerialization> {        
        Ok(SendCmpctMessage{
            announce: bool::deserialize(stream)?,
            version: u64::deserialize(stream)?,
        })
    }
}