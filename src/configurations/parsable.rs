use super::error_configuration::ErrorConfiguration;

use std::collections::HashMap;

pub type Key = String;
pub type Value = String;

pub type KeyValueMap = HashMap<Key, Value>;

const END_LINE: char = '\n';
const ASSIGNMENT: char = '=';
const OPEN_GROUP: char = '{';
const CLOSE_GROUP: char = '}';

pub trait Parsable: Sized {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration>;
}

pub fn parse_structure(value: Value) -> Result<KeyValueMap, ErrorConfiguration> {
    let mut map: KeyValueMap = HashMap::new();

    let mut group_count: u8 = 0;
    let mut assignment: bool = false;
    let text: Vec<String> = value
        .split(|character| match character {
            OPEN_GROUP => match group_count {
                0 => {
                    group_count += 1;
                    true
                }
                _ => {
                    group_count += 1;
                    false
                }
            },
            CLOSE_GROUP => match group_count {
                0 => true,
                1 => {
                    group_count -= 1;
                    true
                }
                _ => {
                    group_count -= 1;
                    false
                }
            },
            END_LINE => {
                if group_count > 0 {
                    return false;
                }

                match assignment {
                    true => {
                        assignment = false;
                        true
                    }
                    false => false,
                }
            }
            ASSIGNMENT => {
                if group_count > 0 {
                    return false;
                }

                assignment = true;
                true
            }
            _ => false,
        })
        .map(|valor| valor.to_string())
        .collect();

    for (i, key) in text.iter().enumerate().step_by(2) {
        let value = match text.get(i + 1) {
            Some(value) => value,
            None => continue,
        };

        map.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok(map)
}

pub fn value_from_map(key: Key, map: &KeyValueMap) -> Result<Value, ErrorConfiguration> {
    match map.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Err(ErrorConfiguration::ValueNotFound),
    }
}

impl<V: Parsable> Parsable for Option<V> {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        match V::parse(name, map) {
            Ok(value) => Ok(Some(value)),
            Err(ErrorConfiguration::ValueNotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }
}

impl Parsable for i32 {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        let value = value_from_map(name.to_string(), map)?;
        match value.parse::<i32>() {
            Ok(parse_value) => Ok(parse_value),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "i32 of {:?}",
                value
            ))),
        }
    }
}

impl Parsable for u16 {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        let value = value_from_map(name.to_string(), map)?;
        match value.parse::<u16>() {
            Ok(parse_value) => Ok(parse_value),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "u16 of {:?}",
                value
            ))),
        }
    }
}

impl Parsable for u32 {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        let value = value_from_map(name.to_string(), map)?;
        match value.parse::<u32>() {
            Ok(parse_value) => Ok(parse_value),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "u32 of {:?}",
                value
            ))),
        }
    }
}

impl Parsable for usize {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        let value = value_from_map(name.to_string(), map)?;
        match value.parse::<usize>() {
            Ok(parse_value) => Ok(parse_value),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "usize of {:?}",
                value
            ))),
        }
    }
}

impl Parsable for bool {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        let value = value_from_map(name.to_string(), map)?;
        match value.parse::<bool>() {
            Ok(parse_value) => Ok(parse_value),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "bool of {:?}",
                value
            ))),
        }
    }
}

impl Parsable for String {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        value_from_map(name.to_string(), map)
    }
}
