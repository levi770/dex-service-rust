//! # Account address (20 bytes)

use super::util::to_arr;
use super::Error;
use hex;
use std::str::FromStr;
use std::{fmt, ops};

/// Fixed bytes number to represent `Address`
pub const ADDRESS_BYTES: usize = 20;

/// Account address (20 bytes)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub [u8; ADDRESS_BYTES]);

impl Address {
    /// Try to convert a byte vector to `Address`.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice with `ADDRESS_BYTES` length
    ///
    /// # Example
    ///
    /// ```
    /// let addr = emerald_rs::Address::try_from(&[0u8; emerald_rs::ADDRESS_BYTES]).unwrap();
    /// assert_eq!(addr.to_string(), "0x0000000000000000000000000000000000000000");
    /// ```
    pub fn try_from(data: &[u8]) -> Result<Self, Error> {
        if data.len() != ADDRESS_BYTES {
            return Err(Error::InvalidLength(data.len()));
        }

        Ok(Address(to_arr(data)))
    }
}

impl ops::Deref for Address {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; ADDRESS_BYTES]> for Address {
    fn from(bytes: [u8; ADDRESS_BYTES]) -> Self {
        Address(bytes)
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != ADDRESS_BYTES * 2 && !s.starts_with("0x") {
            return Err(Error::InvalidHexLength(s.to_string()));
        }

        let value = if s.starts_with("0x") {
            s.split_at(2).1
        } else {
            s
        };

        Address::try_from(hex::decode(&value)?.as_slice())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}
