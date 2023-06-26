use super::{error_wallet::ErrorWallet, public_key::PublicKey};

use crate::serialization::{
    deserializable_fix_size::DeserializableFixSize,
    deserializable_internal_order::DeserializableInternalOrder,
    deserializable_little_endian::DeserializableLittleEndian,
    error_serialization::ErrorSerialization,
    serializable_internal_order::SerializableInternalOrder,
    serializable_little_endian::SerializableLittleEndian,
};

use crate::block_structure::{hash::hash256d_reduce, transaction_output::TransactionOutput};

use std::{
    convert::TryInto,
    fmt::Display,
    io::{Read, Write},
};

use bs58::decode;

pub const ADDRESS_SIZE: usize = 25;
pub const ADDRESS_TESTNET_VERSION_BYTE: u8 = 0x6f;

pub type AddressType = [u8; ADDRESS_SIZE];

/// It's the internal representation of an address in an account
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    address_bytes: AddressType,
    address_string: String,
}

impl Address {
    /// Creates an address object from a string with a Bitcoin address
    ///
    /// ### Error
    ///  * `ErrorWallet::CannotDecodeAddress`: It will appear when address for an account cannot be generated
    pub fn new(address: &str) -> Result<Address, ErrorWallet> {
        if address.len() != 34 {
            return Err(ErrorWallet::CannotDecodeAddress(format!(
                "Invalid address length, expected 34, got {}",
                address.len()
            )));
        }
        let decoded_address = match decode(address).into_vec() {
            Ok(decoded_address) => decoded_address,
            Err(e) => {
                return Err(ErrorWallet::CannotDecodeAddress(format!(
                    "Cannot decode address {}, error : {:?}",
                    address, e
                )))
            }
        };
        let decoded_list: AddressType = match decoded_address.try_into() {
            Ok(decoded_list) => decoded_list,
            Err(e) => {
                return Err(ErrorWallet::CannotDecodeAddress(format!(
                    "Cannot convert decoded address to [u8; 25], error : {:?}",
                    e
                )))
            }
        };
        Ok(Address {
            address_bytes: decoded_list,
            address_string: address.to_string(),
        })
    }

    /// Generates an Address from a public key
    /// ### Error
    ///  * `ErrorWallet::CannotCreateAccount`: It will appear when there was a problem hashing
    pub fn from_public_key(public_key: &PublicKey) -> Result<Address, ErrorWallet> {
        let hashed_pk = match public_key.get_hashed_160() {
            Ok(hashed_pk) => hashed_pk,
            Err(e) => {
                return Err(ErrorWallet::CannotCreateAddress(format!(
                    "Cannot hash public key, error : {:?}",
                    e
                )))
            }
        };
        let mut extended_hashed_pk = Vec::new();
        extended_hashed_pk.push(ADDRESS_TESTNET_VERSION_BYTE);
        extended_hashed_pk.extend_from_slice(&hashed_pk);
        let checksum = match hash256d_reduce(&extended_hashed_pk) {
            Ok(checksum) => checksum,
            Err(e) => {
                return Err(ErrorWallet::CannotCreateAddress(format!(
                    "Cannot hash public key, error : {:?}",
                    e
                )))
            }
        };
        let mut address_bytes = [0; 25];
        address_bytes[..21].clone_from_slice(&extended_hashed_pk);
        address_bytes[21..25].clone_from_slice(&checksum);
        let address_string = bs58::encode(address_bytes.to_vec()).into_string();
        Ok(Address {
            address_bytes,
            address_string,
        })
    }

    /// Extracts the hashed public key from the address
    fn extract_hashed_pk(&self) -> &[u8] {
        let hashed_pk = &self.address_bytes[1..21];
        hashed_pk
    }

    /// Generates the script pubkey for P2PKH from this address
    pub fn generate_script_pubkey_p2pkh(&self) -> Vec<u8> {
        let mut script_pubkey = vec![0x76, 0xa9, 0x14];
        script_pubkey.extend_from_slice(self.extract_hashed_pk());
        script_pubkey.extend_from_slice(&[0x88, 0xac]);
        script_pubkey
    }

    /// Returns true if the address owns the given transaction output (works for P2PKH) and false otherwise.
    pub fn verify_transaction_ownership(&self, txo: &TransactionOutput) -> bool {
        let pk_script = txo.pk_script.clone();
        if pk_script.len() != 25 {
            return false;
        }
        if pk_script[0] != 0x76
            || pk_script[1] != 0xa9
            || pk_script[2] != 0x14
            || pk_script[23] != 0x88
            || pk_script[24] != 0xac
        {
            return false;
        }
        let hashed_pk = &pk_script[3..23];
        hashed_pk == self.extract_hashed_pk()
    }
}

impl SerializableInternalOrder for Address {
    fn io_serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        (self.address_string.len() as u64).le_serialize(stream)?;
        self.address_string.le_serialize(stream)?;
        self.address_bytes.io_serialize(stream)?;

        Ok(())
    }
}

impl DeserializableInternalOrder for Address {
    fn io_deserialize(stream: &mut dyn Read) -> Result<Self, ErrorSerialization> {
        let address_string_length = u64::le_deserialize(stream)? as usize;

        Ok(Address {
            address_string: String::deserialize_fix_size(stream, address_string_length)?,
            address_bytes: <[u8; 25] as DeserializableInternalOrder>::io_deserialize(stream)?,
        })
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_structure::transaction::Transaction;

    #[test]
    fn test_01_correct_address_creation() {
        let address = "1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs".to_string();
        let address_bytes = [
            0x00, 0xf5, 0x4a, 0x58, 0x51, 0xe9, 0x37, 0x2b, 0x87, 0x81, 0x0a, 0x8e, 0x60, 0xcd,
            0xd2, 0xe7, 0xcf, 0xd8, 0x0b, 0x6e, 0x31, 0xc7, 0xf1, 0x8f, 0xe8,
        ];
        let address = Address::new(&address).unwrap();
        assert!(address.address_string == "1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs");
        assert!(address.address_bytes == address_bytes);
    }

    #[test]
    fn test_02_correct_extraction_of_hashed_pk() {
        let address = "1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs".to_string();
        let hashed_pk: [u8; 20] = [
            0xf5, 0x4a, 0x58, 0x51, 0xe9, 0x37, 0x2b, 0x87, 0x81, 0x0a, 0x8e, 0x60, 0xcd, 0xd2,
            0xe7, 0xcf, 0xd8, 0x0b, 0x6e, 0x31,
        ];
        let address = Address::new(&address).unwrap();
        assert!(address.extract_hashed_pk() == hashed_pk);
    }

    #[test]
    fn test_03_correct_address_creation_from_pubkey() {
        let pubkey_bytes: [u8; 33] = [
            0x03, 0xBC, 0x6D, 0x45, 0xD2, 0x10, 0x1E, 0x91, 0x28, 0xDE, 0x14, 0xB5, 0xB6, 0x68,
            0x83, 0xD6, 0x9C, 0xF1, 0xC3, 0x1A, 0x50, 0xB9, 0x6F, 0xEA, 0x2D, 0xAD, 0x4E, 0xD2,
            0x35, 0x14, 0x92, 0x4A, 0x22,
        ];
        let pubkey = PublicKey::new(&pubkey_bytes);
        let address = Address::from_public_key(&pubkey).unwrap();
        let actual_address = Address::new("mnQLoVaZ3w1NLVmUhfG8hh6WoG3iu7cnNw").unwrap();
        assert_eq!(address, actual_address);
    }

    #[test]
    fn test_04_correct_verify_transaction_ownership() {
        let address = Address::new("mnQLoVaZ3w1NLVmUhfG8hh6WoG3iu7cnNw").unwrap();

        let transaction_bytes: Vec<u8> = vec![
            0x01, 0x00, 0x00, 0x00, 0x01, 0x20, 0x25, 0xEF, 0x69, 0x2C, 0xA9, 0x87, 0xB3, 0x9A,
            0x81, 0x33, 0x6E, 0xFB, 0x59, 0xB0, 0x56, 0xFB, 0x90, 0xC0, 0x3A, 0x5E, 0xA4, 0xC4,
            0x54, 0x4C, 0xF9, 0x27, 0x57, 0x61, 0x3E, 0x2E, 0xA4, 0x01, 0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0x02, 0xA0, 0x86, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19,
            0x76, 0xA9, 0x14, 0x7A, 0xA8, 0x18, 0x46, 0x85, 0xCA, 0x1F, 0x06, 0xF5, 0x43, 0xB6,
            0x4A, 0x50, 0x2E, 0xB3, 0xB6, 0x13, 0x5D, 0x67, 0x20, 0x88, 0xAC, 0xD0, 0xE6, 0x43,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x76, 0xA9, 0x14, 0x4B, 0x88, 0xC1, 0xD3, 0x87,
            0x49, 0x08, 0x36, 0x57, 0x73, 0xA7, 0x65, 0xCD, 0xB0, 0x52, 0xC9, 0xEF, 0x5F, 0x1A,
            0x80, 0x88, 0xAC, 0x98, 0xFB, 0x95, 0x64,
        ];
        let transaction = Transaction::io_deserialize(&mut transaction_bytes.as_slice()).unwrap();

        let transaction_output = transaction.tx_out[1].clone();

        assert!(address.verify_transaction_ownership(&transaction_output));
    }
}
