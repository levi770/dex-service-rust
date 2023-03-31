//! # Account ECDSA signatures using the SECG curve secp256k1

use super::util::to_arr;
use super::Address;
use super::Error;
use hex;
use rand::rngs::OsRng;
use rand::Rng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, SignOnly};
use std::{fmt, ops, str};
use web3::signing::keccak256;

pub const KECCAK256_BYTES: usize = 32;
pub const PRIVATE_KEY_BYTES: usize = 32;
pub const ECDSA_SIGNATURE_BYTES: usize = 64;

// Create a new Secp256k1 context with the specified capabilities

lazy_static! {
    static ref ECDSA: Secp256k1<SignOnly> = Secp256k1::signing_only();
}

/// Transaction sign data (see Appendix F. "Signing Transactions" from Yellow Paper)
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Signature {
    /// ‘recovery id’, a 1 byte value specifying the sign and finiteness of the curve point
    pub v: u8,

    /// ECDSA signature first point (0 < r < secp256k1n)
    pub r: [u8; 32],

    /// ECDSA signature second point (0 < s < secp256k1n ÷ 2 + 1)
    pub s: [u8; 32],
}

impl From<[u8; ECDSA_SIGNATURE_BYTES]> for Signature {
    fn from(data: [u8; ECDSA_SIGNATURE_BYTES]) -> Self {
        let mut sign = Signature::default();

        sign.v = data[0];
        sign.r.copy_from_slice(&data[1..(1 + 32)]);
        sign.s.copy_from_slice(&data[(1 + 32)..(1 + 32 + 32)]);

        sign
    }
}

impl Into<(u8, [u8; 32], [u8; 32])> for Signature {
    fn into(self) -> (u8, [u8; 32], [u8; 32]) {
        (self.v, self.r, self.s)
    }
}

impl Into<String> for Signature {
    fn into(self) -> String {
        format!(
            "0x{:X}{}{}",
            self.v,
            hex::encode(self.r),
            hex::encode(self.s)
        )
    }
}

/// Private key used as x in an ECDSA signature
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrivateKey(pub [u8; PRIVATE_KEY_BYTES]);

impl PrivateKey {
    /// Generate a new `PrivateKey` at random (`rand::OsRng`)
    pub fn gen() -> Self {
        Self::gen_custom(&mut OsRng)
    }

    /// Generate a new `PrivateKey` with given custom random generator
    pub fn gen_custom<R: Rng>(rng: &mut R) -> Self {
        PrivateKey::from(SecretKey::new(rng))
    }

    /// Try to convert a byte slice into `PrivateKey`.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice with `PRIVATE_KEY_BYTES` length
    ///
    /// # Example
    ///
    /// ```
    /// const PKB: usize = emerald_rs::PRIVATE_KEY_BYTES;
    /// let pk = emerald_rs::PrivateKey::try_from(&[0u8; PKB]).unwrap();
    /// assert_eq!(pk.to_string(),
    ///            "0x0000000000000000000000000000000000000000000000000000000000000000");
    /// ```
    pub fn try_from(data: &[u8]) -> Result<Self, Error> {
        if data.len() != PRIVATE_KEY_BYTES {
            return Err(Error::InvalidLength(data.len()));
        }

        Ok(PrivateKey(to_arr(data)))
    }

    /// Extract `Address` from current private key.
    pub fn to_address(self) -> Result<Address, Error> {
        let key = PublicKey::from_secret_key(&ECDSA, &self.into());
        let hash = keccak256(&key.serialize_uncompressed()[1..] /* cut '04' */);
        Ok(Address(to_arr(&hash[12..])))
    }

    /// Sign message
    pub fn sign_message(&self, msg: &str) -> Result<Signature, Error> {
        self.sign_hash(message_hash(msg))
    }

    /// Sign a slice of bytes
    pub fn sign_bytes(&self, data: &[u8]) -> Result<Signature, Error> {
        self.sign_hash(bytes_hash(data))
    }

    /// Sign hash from message (Keccak-256)
    pub fn sign_hash(&self, hash: [u8; KECCAK256_BYTES]) -> Result<Signature, Error> {
        let msg = Message::from_slice(&hash)?;
        let key = SecretKey::from_slice(&self)?;

        let s = ECDSA.sign_ecdsa(&msg, &key);
        let ret = s.serialize_compact();

        // let mut buf = [0u8; ECDSA_SIGNATURE_BYTES];
        // buf[0] = (rid.to_i32() + 27) as u8;
        // buf[1..65].copy_from_slice(&sig[0..64]);

        Ok(Signature::from(ret))
    }
}

impl ops::Deref for PrivateKey {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; PRIVATE_KEY_BYTES]> for PrivateKey {
    fn from(bytes: [u8; PRIVATE_KEY_BYTES]) -> Self {
        PrivateKey(bytes)
    }
}

impl From<SecretKey> for PrivateKey {
    fn from(key: SecretKey) -> Self {
        PrivateKey(to_arr(&key[0..PRIVATE_KEY_BYTES]))
    }
}

impl Into<SecretKey> for PrivateKey {
    fn into(self) -> SecretKey {
        SecretKey::from_slice(&self).expect("Expect secret key")
    }
}

impl str::FromStr for PrivateKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != PRIVATE_KEY_BYTES * 2 && !s.starts_with("0x") {
            return Err(Error::InvalidHexLength(s.to_string()));
        }

        let value = if s.starts_with("0x") {
            s.split_at(2).1
        } else {
            s
        };

        PrivateKey::try_from(hex::decode(&value)?.as_slice())
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

// pub fn os_random() -> ThreadRng {
//     rand::thread_rng()
// }

fn message_hash(msg: &str) -> [u8; KECCAK256_BYTES] {
    bytes_hash(msg.as_bytes())
}

fn bytes_hash(data: &[u8]) -> [u8; KECCAK256_BYTES] {
    let mut v = prefix(data).into_bytes();
    v.extend_from_slice(data);
    keccak256(&v)
}

/// [internal/ethapi: add personal sign method](https://github.com/ethereum/go-ethereum/pull/2940)
fn prefix(data: &[u8]) -> String {
    format!("\x19Ethereum Signed Message:\x0a{}", data.len())
}
