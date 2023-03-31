//! # Serialize keystore files (UTC / JSON) encrypted with a passphrase module

mod address;
#[macro_use]
pub mod byte_array;
mod crypto;
mod error;

//pub use self::address::try_extract_address;
pub use self::crypto::{CoreCrypto, Iv, Mac};
pub use self::error::Error;
use super::core::{self, Address};
use super::{Cipher, CryptoType, KdfParams, KeyFile, Salt, CIPHER_IV_BYTES};
use serde::ser;
use serde::{Serialize, Serializer};
use serde_json;
use uuid::Uuid;

/// Keystore file current version used for serializing
pub const CURRENT_VERSION: u8 = 3;

/// Supported keystore file versions (only current V3 now)
pub const SUPPORTED_VERSIONS: &[u8] = &[CURRENT_VERSION];

/// A serializable keystore file (UTC / JSON format)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableKeyFileCore {
    version: u8,
    id: Uuid,
    address: Address,
    name: Option<String>,
    description: Option<String>,
    visible: Option<bool>,
    crypto: CoreCrypto,
}

impl SerializableKeyFileCore {
    fn try_from(kf: KeyFile) -> Result<Self, Error> {
        let cr = CoreCrypto::try_from(&kf)?;

        Ok(SerializableKeyFileCore {
            version: CURRENT_VERSION,
            id: kf.uuid,
            address: kf.address,
            name: kf.name.clone(),
            description: kf.description.clone(),
            visible: kf.visible,
            crypto: cr,
        })
    }
}

impl Into<KeyFile> for SerializableKeyFileCore {
    fn into(self) -> KeyFile {
        KeyFile {
            name: self.name,
            description: self.description,
            address: self.address,
            visible: self.visible,
            uuid: self.id,
            crypto: CryptoType::Core(self.crypto),
        }
    }
}




impl KeyFile {
    /// Decode `Keyfile` from JSON
    /// Handles different variants of `crypto` section
    ///
    pub fn decode(f: &str) -> Result<KeyFile, Error> {
        let buf = f.to_string().to_lowercase();
        let mut ver = 0;

        let kf = serde_json::from_str::<SerializableKeyFileCore>(&buf)
            .and_then(|core| {
                ver = core.version;
                Ok(core.into())
            })
            .map_err(Error::from)?;

        if !SUPPORTED_VERSIONS.contains(&ver) {
            return Err(Error::UnsupportedVersion(ver));
        }

        Ok(kf)
    }
}

impl Serialize for KeyFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match SerializableKeyFileCore::try_from(self.clone()) {
            Ok(sf) => sf.serialize(serializer),
            Err(e) => Err(ser::Error::custom(e))
        }
    }
}