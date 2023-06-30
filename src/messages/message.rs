use crate::serialization::{
    deserializable_internal_order::DeserializableInternalOrder,
    error_serialization::ErrorSerialization,
    serializable_internal_order::SerializableInternalOrder,
};

use crate::block_structure::hash::hash256d_reduce;

use super::{
    addr_message::AddrMessage,
    alert_message::AlertMessage,
    block_message::BlockMessage,
    command_name::CommandName,
    fee_filter_message::FeeFilterMessage,
    get_data_message::GetDataMessage,
    get_headers_message::GetHeadersMessage,
    headers_message::HeadersMessage,
    inventory_message::InventoryMessage,
    message_header::{MagicType, MessageHeader},
    ping_message::PingMessage,
    pong_message::PongMessage,
    send_cmpct_message::SendCmpctMessage,
    send_headers_message::SendHeadersMessage,
    tx_message::TxMessage,
    verack_message::VerackMessage,
    version_message::VersionMessage,
};

use std::io::{Read, Write, ErrorKind};

pub const CHECKSUM_EMPTY_PAYLOAD: MagicType = [0x5d, 0xf6, 0xe0, 0xe2];

pub trait Message: SerializableInternalOrder + DeserializableInternalOrder {
    /// Serialize a message with a payload that is serializable
    ///
    ///  * `ErrorSerialization::ErrorSerialization`: It will appear when there is an error in the serialization
    fn serialize_message(
        stream: &mut dyn Write,
        magic_numbers: MagicType,
        payload: &dyn SerializableInternalOrder,
    ) -> Result<(), ErrorSerialization> {
        let mut serialized_payload: Vec<u8> = Vec::new();
        payload.io_serialize(&mut serialized_payload)?;
        let serialized_payload: &[u8] = &serialized_payload;

        let header = MessageHeader {
            magic_numbers,
            command_name: Self::get_command_name(),
            payload_size: serialized_payload.len() as u32,
            checksum: hash256d_reduce(serialized_payload)?,
        };

        header.io_serialize(stream)?;
        serialized_payload.io_serialize(stream)?;

        Ok(())
    }

    /// Deserialize a message given the header of it
    ///
    /// ### Error
    ///  * `ErrorSerialization::ErrorSerialization`: It will appear when there is an error in the serialization
    ///  * `ErrorSerialization::ErrorInDeserialization`: It will appear when there is an error in the deserialization
    ///  * `ErrorSerialization::ErrorWhileReading`: It will appear when there is an error in the reading from a stream
    fn deserialize_message<R : Read>(
        stream: &mut R,
        message_header: MessageHeader,
    ) -> Result<Self, ErrorSerialization> {
        let mut buffer: Vec<u8> = vec![0; message_header.payload_size as usize];
        read_exact(stream, &mut buffer)?;
        let mut buffer: &[u8] = &buffer[..];
        let message = Self::io_deserialize(&mut buffer)?;

        let mut serialized_message: Vec<u8> = Vec::new();
        message.io_serialize(&mut serialized_message)?;

        let length = serialized_message.len();
        if length != message_header.payload_size as usize {
            return Err(ErrorSerialization::ErrorInDeserialization(format!(
                "Payload size {:?} in {:?} isn't the same as receive: {:?}",
                length,
                Self::get_command_name(),
                message_header.payload_size
            )));
        }

        let checksum = Self::calculate_checksum(&serialized_message)?;
        if !checksum.eq(&message_header.checksum) {
            return Err(ErrorSerialization::ErrorInDeserialization(format!(
                "Checksum {:?} in {:?}  isn't the same as receive: {:?}",
                checksum,
                Self::get_command_name(),
                message_header.checksum
            )));
        }

        Ok(message)
    }

    /// Calculate the checksum of a serialized message
    fn calculate_checksum(serialized_message: &[u8]) -> Result<[u8; 4], ErrorSerialization> {
        hash256d_reduce(serialized_message)
    }

    /// Get the command name of the message to know the type of message
    fn get_command_name() -> CommandName;
}

pub fn read_exact<R : Read>(stream: &mut R, buffer: &mut [u8]) -> Result<(), ErrorSerialization> {
    if let Err(error) = stream.read_exact(buffer) {
        let error = match error.kind() {
            ErrorKind::ConnectionAborted => ErrorSerialization::ConnectionAborted,
            ErrorKind::WouldBlock => ErrorSerialization::InformationNotReady,
            _ => ErrorSerialization::ErrorWhileReading,
        };
        return Err(error)
    }
    Ok(())
}

/// Ignores any message that is not the one that is being searched for
///
/// ### Error
///  * `ErrorSerialization::ErrorSerialization`: It will appear when there is an error in the serialization
///  * `ErrorSerialization::ErrorInDeserialization`: It will appear when there is an error in the deserialization
///  * `ErrorSerialization::ErrorWhileReading`: It will appear when there is an error in the reading from a stream
pub fn deserialize_until_found<RW: Read + Write>(
    stream: &mut RW,
    search_name: CommandName,
) -> Result<MessageHeader, ErrorSerialization> {
    loop {
        let header = match MessageHeader::deserialize_header(stream) {
            Ok(header) => header,
            Err(error) => return Err(error),
        };

        if header.command_name == search_name {
            return Ok(header);
        }

        let magic_bytes = header.magic_numbers;

        match header.command_name {
            CommandName::Version => ignore_message::<RW, VersionMessage>(stream, header)?,
            CommandName::Verack => ignore_message::<RW, VerackMessage>(stream, header)?,
            CommandName::GetHeaders => ignore_message::<RW, GetHeadersMessage>(stream, header)?,
            CommandName::Headers => ignore_message::<RW, HeadersMessage>(stream, header)?,
            CommandName::Inventory => ignore_message::<RW, InventoryMessage>(stream, header)?,
            CommandName::Block => ignore_message::<RW, BlockMessage>(stream, header)?,
            CommandName::Ping => {
                let ping = PingMessage::deserialize_message(stream, header)?;

                let pong = PongMessage { nonce: ping.nonce };

                PongMessage::serialize_message(stream, magic_bytes, &pong)?;
            }
            CommandName::Pong => ignore_message::<RW, PongMessage>(stream, header)?,
            CommandName::SendHeaders => ignore_message::<RW, SendHeadersMessage>(stream, header)?,
            CommandName::SendCmpct => ignore_message::<RW, SendCmpctMessage>(stream, header)?,
            CommandName::Addr => ignore_message::<RW, AddrMessage>(stream, header)?,
            CommandName::FeeFilter => ignore_message::<RW, FeeFilterMessage>(stream, header)?,
            CommandName::GetData => ignore_message::<RW, GetDataMessage>(stream, header)?,
            CommandName::Alert => ignore_message::<RW, AlertMessage>(stream, header)?,
            CommandName::Tx => ignore_message::<RW, TxMessage>(stream, header)?,
        }
    }
}

/// Ignores any message of type Message
pub fn ignore_message<R : Read, M: Message>(
    stream: &mut R,
    header: MessageHeader,
) -> Result<(), ErrorSerialization> {
    let _ = M::deserialize_message(stream, header)?;
    Ok(())
}
