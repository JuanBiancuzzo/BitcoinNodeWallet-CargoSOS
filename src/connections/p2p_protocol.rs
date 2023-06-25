use super::error_connection::ErrorConnection;

use crate::serialization::{
    deserializable_little_endian::DeserializableLittleEndian,
    error_serialization::ErrorSerialization, serializable_little_endian::SerializableLittleEndian,
};

use crate::configurations::{
    error_configuration::ErrorConfiguration,
    parsable::{value_from_map, KeyValueMap, Parsable},
};

use std::{
    cmp::PartialEq, 
    str::FromStr,
    convert::{TryFrom, Into},
};

/// It's the representation of the P2P protocol version
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ProtocolVersionP2P {
    V70016,
    V70015,
    V70014,
    V70013,
    V70012,
    V70011,
    V70002,
    V70001,
    V60002,
    V60001,
    V60000,
    V31800,
    V31402,
    V311,
    V209,
    V106,
}

impl FromStr for ProtocolVersionP2P {
    type Err = ErrorConfiguration;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "V70016" => Ok(ProtocolVersionP2P::V70016),
            "V70015" => Ok(ProtocolVersionP2P::V70015),
            "V70014" => Ok(ProtocolVersionP2P::V70014),
            "V70013" => Ok(ProtocolVersionP2P::V70013),
            "V70012" => Ok(ProtocolVersionP2P::V70012),
            "V70011" => Ok(ProtocolVersionP2P::V70011),
            "V70002" => Ok(ProtocolVersionP2P::V70002),
            "V70001" => Ok(ProtocolVersionP2P::V70001),
            "V60002" => Ok(ProtocolVersionP2P::V60002),
            "V60001" => Ok(ProtocolVersionP2P::V60001),
            "V60000" => Ok(ProtocolVersionP2P::V60000),
            "V31800" => Ok(ProtocolVersionP2P::V31800),
            "V31402" => Ok(ProtocolVersionP2P::V31402),
            "V311" => Ok(ProtocolVersionP2P::V311),
            "V209" => Ok(ProtocolVersionP2P::V209),
            "V106" => Ok(ProtocolVersionP2P::V106),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "protocol version p2p of {:?}",
                value
            ))),
        }
    }
}

impl Parsable for ProtocolVersionP2P {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        let value = value_from_map(name.to_string(), map)?;
        value.parse::<ProtocolVersionP2P>()
    }
}

impl TryFrom<i32> for ProtocolVersionP2P {
    type Error = ErrorConfiguration;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            70016 => Ok(ProtocolVersionP2P::V70016),
            70015 => Ok(ProtocolVersionP2P::V70015),
            70014 => Ok(ProtocolVersionP2P::V70014),
            70013 => Ok(ProtocolVersionP2P::V70013),
            70012 => Ok(ProtocolVersionP2P::V70012),
            70011 => Ok(ProtocolVersionP2P::V70011),
            70002 => Ok(ProtocolVersionP2P::V70002),
            70001 => Ok(ProtocolVersionP2P::V70001),
            60002 => Ok(ProtocolVersionP2P::V60002),
            60001 => Ok(ProtocolVersionP2P::V60001),
            60000 => Ok(ProtocolVersionP2P::V60000),
            31800 => Ok(ProtocolVersionP2P::V31800),
            31402 => Ok(ProtocolVersionP2P::V31402),
            311 => Ok(ProtocolVersionP2P::V311),
            209 => Ok(ProtocolVersionP2P::V209),
            106 => Ok(ProtocolVersionP2P::V106),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "protocol version p2p of {:?}",
                value
            ))),
        }
    }
}

impl Into<i32> for ProtocolVersionP2P {
    fn into(self) -> i32 {
        match self {
            ProtocolVersionP2P::V70016 => 70016,
            ProtocolVersionP2P::V70015 => 70015,
            ProtocolVersionP2P::V70014 => 70014,
            ProtocolVersionP2P::V70013 => 70013,
            ProtocolVersionP2P::V70012 => 70012,
            ProtocolVersionP2P::V70011 => 70011,
            ProtocolVersionP2P::V70002 => 70002,
            ProtocolVersionP2P::V70001 => 70001,
            ProtocolVersionP2P::V60002 => 60002,
            ProtocolVersionP2P::V60001 => 60001,
            ProtocolVersionP2P::V60000 => 60000,
            ProtocolVersionP2P::V31800 => 31800,
            ProtocolVersionP2P::V31402 => 31402,
            ProtocolVersionP2P::V311 => 311,
            ProtocolVersionP2P::V209 => 209,
            ProtocolVersionP2P::V106 => 106,
        }
    }
}

impl SerializableLittleEndian for ProtocolVersionP2P {
    fn le_serialize(&self, stream: &mut dyn std::io::Write) -> Result<(), ErrorSerialization> {
        let version: i32 = match (*self).try_into() {
            Ok(version) => version,
            _ => {
                return Err(ErrorSerialization::ErrorInSerialization(format!(
                    "While serializing p2p protocol version {:?}",
                    self
                )))
            }
        };

        version.le_serialize(stream)
    }
}

impl DeserializableLittleEndian for ProtocolVersionP2P {
    fn le_deserialize(stream: &mut dyn std::io::Read) -> Result<Self, ErrorSerialization> {
        let version_int = i32::le_deserialize(stream)?;
        match version_int.try_into() {
            Ok(version) => Ok(version),
            _ => Err(ErrorSerialization::ErrorInDeserialization(format!(
                "While deserializing p2p protocol version {:?}",
                version_int
            ))),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::configurations::parsable::parse_structure;

    #[test]
    fn test01_serialize_correctly_protocol_version_p2p() -> Result<(), ErrorSerialization> {
        let expected_stream: Vec<u8> = vec![0xAA, 0x7A, 0x00, 0x00];

        let mut stream: Vec<u8> = Vec::new();
        let protocol: ProtocolVersionP2P = ProtocolVersionP2P::V31402;

        protocol.le_serialize(&mut stream)?;

        assert_eq!(expected_stream, stream);

        Ok(())
    }

    #[test]
    fn test02_deserialize_correctly_protocol_version_p2p() -> Result<(), ErrorSerialization> {
        let stream: Vec<u8> = vec![0xAA, 0x7A, 0x00, 0x00];
        let mut stream: &[u8] = &stream;
        let protocol: ProtocolVersionP2P = ProtocolVersionP2P::V31402;

        let expected_protocol = ProtocolVersionP2P::le_deserialize(&mut stream)?;

        assert_eq!(expected_protocol, protocol);

        Ok(())
    }

    #[test]
    fn test03_accept_valid_input() {
        let configuration = "p2p_protocol = V70002";

        let name = "p2p_protocol";
        let map = parse_structure(configuration.to_string()).unwrap();

        let p2p_protocol_result = ProtocolVersionP2P::parse(name, &map);

        let expected_p2p_protocol = ProtocolVersionP2P::V70002;

        assert_eq!(Ok(expected_p2p_protocol), p2p_protocol_result);
    }
}
