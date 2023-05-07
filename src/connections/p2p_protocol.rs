use super::connection_error::ConnectionError;

#[derive(Debug, std::cmp::PartialEq)]
///Enum que representa la versión del protocolo P2P que se va a utilizar
pub enum ProtocolVersionP2P {
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
///Implementación del trait que permite hacer parse
impl std::str::FromStr for ProtocolVersionP2P {
    type Err = ConnectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
            _ => Err(ConnectionError::ErrorInvalidInputParse),
        }
    }
}

/// Implementación del trait try_from que permite convertir a i32
impl std::convert::TryFrom<i32> for ProtocolVersionP2P {
    type Error = ConnectionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
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
            _ => return Err(ConnectionError::ErrorInvalidInputParse),
        }
    }
}

/// Implementación del trait que permite convertir a i32
impl std::convert::TryInto<i32> for ProtocolVersionP2P {
    type Error = ConnectionError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            ProtocolVersionP2P::V70015 => Ok(70015),
            ProtocolVersionP2P::V70014 => Ok(70014),
            ProtocolVersionP2P::V70013 => Ok(70013),
            ProtocolVersionP2P::V70012 => Ok(70012),
            ProtocolVersionP2P::V70011 => Ok(70011),
            ProtocolVersionP2P::V70002 => Ok(70002),
            ProtocolVersionP2P::V70001 => Ok(70001),
            ProtocolVersionP2P::V60002 => Ok(60002),
            ProtocolVersionP2P::V60001 => Ok(60001),
            ProtocolVersionP2P::V60000 => Ok(60000),
            ProtocolVersionP2P::V31800 => Ok(31800),
            ProtocolVersionP2P::V31402 => Ok(31402),
            ProtocolVersionP2P::V311 => Ok(311),
            ProtocolVersionP2P::V209 => Ok(209),
            ProtocolVersionP2P::V106 => Ok(106),
        }
    }
}