//! # Advanced encryption standard (AES) cipher

use super::Error;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use aes::Aes128;
use std::fmt;
use std::str::FromStr;

type Aes128CfbEnc = cfb_mode::Encryptor<Aes128>;
//type Aes128CfbDec = cfb_mode::Decryptor<Aes128>;

/// `AES128_CRT` cipher name
pub const AES128_CIPHER_NAME: &str = "aes-128";

/// Cipher type
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cipher {
    /// AES-CTR (specified in (RFC 3686)[https://tools.ietf.org/html/rfc3686])
    #[serde(rename = "aes-128")]
    Aes128,
}

impl Cipher {
    /// Encrypt given text with provided key and initial vector
    /// using AES-128-CTR algorithm
    pub fn encrypt(&self, data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut buf = data.to_vec();
        Aes128CfbEnc::new(key, iv)
            .encrypt_b2b(&data, &mut buf)
            .unwrap();
        buf
    }
}

impl Default for Cipher {
    fn default() -> Self {
        Cipher::Aes128
    }
}

impl FromStr for Cipher {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s == AES128_CIPHER_NAME => Ok(Cipher::Aes128),
            _ => Err(Error::UnsupportedCipher(s.to_string())),
        }
    }
}

impl fmt::Display for Cipher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cipher::Aes128 => f.write_str(AES128_CIPHER_NAME),
        }
    }
}
