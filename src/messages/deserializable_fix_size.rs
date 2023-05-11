use super::error_message::ErrorMessage;
use std::io::Read;

pub trait DeserializableFixSize : Sized
{
    fn deserialize_fix_size(stream: &mut dyn Read, sizes: usize) -> Result<Self, ErrorMessage>;
}

impl DeserializableFixSize for String {

    fn deserialize_fix_size(stream: &mut dyn Read, sizes: usize) -> Result<Self, ErrorMessage> {
        let mut buffer: Vec<u8> = vec![0; sizes];
        if stream.read_exact(&mut buffer).is_err() {
            return Err(ErrorMessage::ErrorInDeserialization);
        }

        match String::from_utf8(buffer.to_vec()) {
            Ok(user_agent) => Ok(user_agent),
            Err(_) => Err(ErrorMessage::ErrorInDeserialization),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{
        DeserializableFixSize,
        ErrorMessage,
    };

    #[test]
    fn test01_serialize_correctly_string() -> Result<(), ErrorMessage> {
        
        let stream: Vec<u8> = vec![0x62, 0x75, 0x75];
        let mut stream: &[u8] = &stream;        
        
        let expected_string: String = "buu".to_string();

        let string = String::deserialize_fix_size(&mut stream, 3)?;

        assert_eq!(expected_string, string);

        Ok(())
    }
}