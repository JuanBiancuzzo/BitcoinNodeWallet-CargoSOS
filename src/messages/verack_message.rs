use super::{
    command_name::CommandName,
    message::{Message, CHECKSUM_EMPTY_PAYLOAD},
};

use crate::serialization::{
    deserializable_internal_order::DeserializableInternalOrder,
    error_serialization::ErrorSerialization,
    serializable_internal_order::SerializableInternalOrder,
};

use std::io::{Read, Write};

/// It's the verack message
#[derive(Debug, std::cmp::PartialEq)]
pub struct VerackMessage;

impl Message for VerackMessage {
    fn get_command_name() -> CommandName {
        CommandName::Verack
    }

    fn calculate_checksum(_: &[u8]) -> Result<[u8; 4], ErrorSerialization> {
        Ok(CHECKSUM_EMPTY_PAYLOAD)
    }
}

impl SerializableInternalOrder for VerackMessage {
    fn io_serialize(&self, _: &mut dyn Write) -> Result<(), ErrorSerialization> {
        Ok(())
    }
}

impl DeserializableInternalOrder for VerackMessage {
    fn io_deserialize(_: &mut dyn Read) -> Result<Self, ErrorSerialization> {
        Ok(VerackMessage)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DeserializableInternalOrder, ErrorSerialization, SerializableInternalOrder, VerackMessage,
        CHECKSUM_EMPTY_PAYLOAD,
    };

    use crate::messages::{
        command_name::CommandName, message::Message, message_header::MessageHeader,
    };

    use crate::serialization::serializable_little_endian::SerializableLittleEndian;

    #[test]
    fn test01_serialize() -> Result<(), ErrorSerialization> {
        let magic_bytes: [u8; 4] = [0x55, 0x66, 0xee, 0xee];

        let verack_message = VerackMessage;
        let mut stream: Vec<u8> = Vec::new();

        VerackMessage::serialize_message(&mut stream, magic_bytes, &verack_message)?;

        let mut expected_stream: Vec<u8> = Vec::new();
        magic_bytes.io_serialize(&mut expected_stream)?;
        CommandName::Verack.io_serialize(&mut expected_stream)?;
        vec![0, 0, 0, 0].le_serialize(&mut expected_stream)?;
        CHECKSUM_EMPTY_PAYLOAD.io_serialize(&mut expected_stream)?;

        assert_eq!(expected_stream, stream);

        Ok(())
    }

    #[test]
    fn test02_deserialize() -> Result<(), ErrorSerialization> {
        let magic_bytes: [u8; 4] = [0x55, 0x66, 0xee, 0xee];

        let header = MessageHeader {
            magic_numbers: magic_bytes,
            command_name: CommandName::Verack,
            payload_size: 0,
            checksum: CHECKSUM_EMPTY_PAYLOAD,
        };

        let mut stream: Vec<u8> = Vec::new();
        magic_bytes.io_serialize(&mut stream)?;
        CommandName::Verack.io_serialize(&mut stream)?;
        vec![0, 0, 0, 0].le_serialize(&mut stream)?;
        CHECKSUM_EMPTY_PAYLOAD.io_serialize(&mut stream)?;
        let mut stream: &[u8] = &stream;

        let expected_verack = VerackMessage::deserialize_message(&mut stream, header)?;

        let verack = VerackMessage::io_deserialize(&mut stream)?;

        assert_eq!(expected_verack, verack);

        Ok(())
    }
}
