//! # JSON serialize for crypto field (UTC / JSON)

use super::{Cipher, CryptoType, Error, KdfParams, KeyFile, Salt, CIPHER_IV_BYTES};
use hex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::default::Default;

pub const KECCAK256_BYTES: usize = 32;

byte_array_struct!(Mac, KECCAK256_BYTES);
byte_array_struct!(Iv, CIPHER_IV_BYTES);

/// `Keyfile` related crypto attributes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreCrypto {
    /// Cipher
    pub cipher: Cipher,

    /// Cipher text
    pub cipher_text: Vec<u8>,

    /// Params for `Cipher`
    pub cipher_params: CipherParams,

    /// Key derivation funciton
    pub kdf_params: KdfParams,

    /// HMAC authentication code
    pub mac: Mac,
}

/// Serialization representation for `CoreCrypto`
#[derive(Serialize, Deserialize, Debug)]
struct SerCoreCrypto {
    pub cipher: Cipher,

    #[serde(rename = "ciphertext")]
    pub cipher_text: String,

    #[serde(rename = "cipherparams")]
    pub cipher_params: CipherParams,

    pub kdf: String,

    #[serde(rename = "kdfparams")]
    pub kdf_params: KdfParams,

    pub mac: Mac,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CipherParams {
    pub iv: Iv,
}

impl Default for CipherParams {
    fn default() -> Self {
        CipherParams {
            iv: Iv::from([0; CIPHER_IV_BYTES]),
        }
    }
}

impl CoreCrypto {
    /// Try to create crypto attributes from
    /// corresponding Keyfile (simple or HDWallet keyfile)
    pub fn try_from(kf: &KeyFile) -> Result<Self, Error> {
        match kf.crypto {
            CryptoType::Core(ref core) => Ok(CoreCrypto {
                cipher: core.cipher,
                cipher_text: core.cipher_text.clone(),
                cipher_params: core.cipher_params.clone(),
                kdf_params: KdfParams {
                    kdf: core.kdf_params.kdf,
                    dklen: core.kdf_params.dklen,
                    salt: Salt::from(core.kdf_params.salt.0),
                },
                mac: Mac::from(core.mac.0),
            }),
            //_ => Err(Error::NotFound),
        }
    }
}

impl Default for CoreCrypto {
    fn default() -> Self {
        Self {
            cipher: Cipher::default(),
            cipher_text: vec![],
            cipher_params: CipherParams::default(),
            kdf_params: KdfParams::default(),
            mac: Mac::default(),
        }
    }
}

impl Into<KeyFile> for CoreCrypto {
    fn into(self) -> KeyFile {
        KeyFile {
            crypto: CryptoType::Core(self),
            ..Default::default()
        }
    }
}

impl From<SerCoreCrypto> for CoreCrypto {
    fn from(ser: SerCoreCrypto) -> Self {
        CoreCrypto {
            cipher: ser.cipher,
            cipher_text: hex::decode(ser.cipher_text).unwrap(),
            cipher_params: ser.cipher_params,
            kdf_params: ser.kdf_params,
            mac: ser.mac,
        }
    }
}

impl Into<SerCoreCrypto> for CoreCrypto {
    fn into(self) -> SerCoreCrypto {
        SerCoreCrypto {
            cipher: self.cipher,
            cipher_text: hex::encode(self.cipher_text),
            cipher_params: self.cipher_params,
            kdf: self.kdf_params.kdf.to_string(),
            kdf_params: self.kdf_params,
            mac: self.mac,
        }
    }
}

impl<'de> Deserialize<'de> for CoreCrypto {
    fn deserialize<D>(deserializer: D) -> Result<CoreCrypto, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ser: SerCoreCrypto = SerCoreCrypto::deserialize(deserializer)?;
        Ok(ser.into())
    }
}

impl Serialize for CoreCrypto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ser: SerCoreCrypto = self.clone().into();
        ser.serialize(serializer)
    }
}