use crate::configurations::{
    error_configuration::ErrorConfiguration,
    parsable::{value_from_map, KeyValueMap, Parsable},
};

use std::{cmp::PartialEq, str::FromStr};

/// It represents the interface that will be used
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interface {
    /// To use the graphical interface
    Gui,

    /// To use the terminal interface
    Tui,
}

///Implementación del trait que permite hacer parse
impl FromStr for Interface {
    type Err = ErrorConfiguration;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GUI" => Ok(Interface::Gui),
            "TUI" => Ok(Interface::Tui),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "interface of {:?}",
                s
            ))),
        }
    }
}

impl Parsable for Interface {
    fn parse(name: &str, map: &KeyValueMap) -> Result<Self, ErrorConfiguration> {
        let value = value_from_map(name.to_string(), map)?;
        match value.parse::<Interface>() {
            Ok(value) => Ok(value),
            _ => Err(ErrorConfiguration::ErrorCantParseValue(format!(
                "interface of {:?}",
                value
            ))),
        }
    }
}
