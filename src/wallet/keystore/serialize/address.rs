//! # JSON serialize format for hex encoded account addresses (without '0x' prefix)

use super::core::Address;
//use regex::Regex;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .map(|s| format!("0x{}", s))
            .and_then(|s| Address::from_str(&s).map_err(de::Error::custom))
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string()[2..]) /* cut '0x' prefix */
    }
}

// Try to extract `Address` from JSON formatted text
// pub fn try_extract_address(text: &str) -> Option<Address> {
//     lazy_static! {
//         static ref ADDR_RE: Regex = Regex::new(r#"address.+?([a-fA-F0-9]{40})"#).unwrap();
//     }

//     ADDR_RE
//         .captures(text)
//         .and_then(|g| g.get(1).map(|m| format!("0x{}", m.as_str())))
//         .and_then(|s| s.parse().ok())
// }