//! # Util functions module
mod rlp;

//pub use self::crypto::{keccak256, KECCAK256_BYTES};
pub use self::rlp::{RLPList, WriteRLP};
use byteorder::{BigEndian, WriteBytesExt};
use chrono::prelude::Utc;
// use hex::FromHex;
// use std::io::Cursor;
use std::mem::transmute;
use std::time::{SystemTime, UNIX_EPOCH};
use web3::types::U256;

static HEX_CHARS: &'static [u8] = b"0123456789abcdef";

// const ETH: &'static str = "eth";
// const MORDEN: &'static str = "morden";
// const ROPSTEN: &'static str = "ropsten";
// const RINKEBY: &'static str = "rinkeby";
// const ROOTSTOCK_MAINNET: &'static str = "rootstock-main";
// const ROOTSTOCK_TESTNET: &'static str = "rootstock-test";
// const KOVAN: &'static str = "kovan";
// const ETC: &'static str = "etc";
// const MAINNET: &'static str = "mainnet";
// const ETC_MORDEN: &'static str = "etc-morden";

/// Convert `self` into hex string
pub trait ToHex {
    /// converts to hex
    fn to_hex(&self) -> String;
}

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        let mut v = Vec::with_capacity(self.len() * 2);
        for &byte in self.iter() {
            v.push(HEX_CHARS[(byte >> 4) as usize]);
            v.push(HEX_CHARS[(byte & 0xf) as usize]);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}

impl ToHex for u64 {
    fn to_hex(&self) -> String {
        let bytes: [u8; 8] = unsafe { transmute(self.to_be()) };
        bytes.to_hex()
    }
}

/// Get chain name by chain id
///
/// # Arguments:
/// * `id` - target chain id
///
// pub fn to_chain_name(id: u8) -> Option<&'static str> {
//     match id {
//         1 => Some(ETH),
//         2 => Some(MORDEN),
//         3 => Some(ROPSTEN),
//         4 => Some(RINKEBY),
//         30 => Some(ROOTSTOCK_MAINNET),
//         31 => Some(ROOTSTOCK_TESTNET),
//         42 => Some(KOVAN),
//         61 => Some(ETC),
//         62 => Some(ETC_MORDEN),
//         _ => None,
//     }
// }

/// Get chain id by chain name
///
/// # Arguments:
/// * `name` - target chain name
///
// pub fn to_chain_id(name: &str) -> Option<u8> {
//     match name.to_lowercase().as_str() {
//         ETH => Some(1),
//         MORDEN => Some(2),
//         ROPSTEN => Some(3),
//         RINKEBY => Some(4),
//         ROOTSTOCK_MAINNET => Some(30),
//         ROOTSTOCK_TESTNET => Some(31),
//         KOVAN => Some(42),
//         ETC | MAINNET => Some(61),
//         ETC_MORDEN => Some(62),
//         _ => None,
//     }
// }

/// Convert byte array into `u64`
///
/// # Arguments
///
/// * `v` - array to be converted
///
// pub fn to_u64(v: &[u8]) -> u64 {
//     let data = align_bytes(v, 8);
//     let mut buf = Cursor::new(&data);

//     buf.read_u64::<BigEndian>().unwrap()
// }

/// Trix hex prefix `0x`
///
/// # Arguments
///
/// * `val` - string to be trimmed
///
// pub fn trim_hex(val: &str) -> &str {
//     if !val.starts_with("0x") {
//         return val;
//     }

//     let (_, s) = val.split_at(2);
//     s
// }

/// Convert a slice into array
///
/// # Arguments
///
/// * `slice` - slice to be converted
///
pub fn to_arr<A, T>(slice: &[T]) -> A
where
    A: AsMut<[T]> + Default,
    T: Clone,
{
    let mut arr = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut arr).clone_from_slice(slice);
    arr
}

/// Padding high bytes with `O` to fit `len` bytes
///
/// # Arguments
///
/// * `data` - data to be aligned
/// * `len` - length of required array
///
// pub fn align_bytes(data: &[u8], len: usize) -> Vec<u8> {
//     if data.len() >= len {
//         return data.to_vec();
//     }

//     let mut v = vec![0u8; len - data.len()];
//     v.extend_from_slice(data);
//     v
// }

/// Padding hex string with `O` to get even length
///
/// # Arguments
///
/// * `data` - data to be aligned
///
// pub fn to_even_str(data: &str) -> String {
//     if data.len() % 2 == 0 {
//         return String::from(data);
//     }

//     let mut v = String::from("0");
//     v.push_str(data);
//     v
// }

/// Trim all high zero bytes
///
/// # Arguments
///
/// * `data` - value to be trimmed
///
pub fn trim_bytes(data: &[u8]) -> &[u8] {
    let mut n = 0;
    for b in data {
        if *b != 0u8 {
            break;
        }
        n += 1;
    }
    &data[n..data.len()]
}

/// Counts bytes required to hold `x` value
///
/// # Arguments
///
/// * `x` - value to find size
///
pub fn bytes_count(x: usize) -> u8 {
    match x {
        _ if x > 0xff => 1 + bytes_count(x >> 8),
        _ if x > 0 => 1,
        _ => 0,
    }
}

/// Converts `unsigned` value to byte array
///
/// # Arguments
///
/// * `x` - a value to be converted into byte vector
/// * `len` - size of value
///
pub fn to_bytes(x: u64, len: u8) -> Vec<u8> {
    let mut buf = vec![];
    match len {
        1 => buf.push(x as u8),
        2 => buf.write_u16::<BigEndian>(x as u16).unwrap(),
        4 => buf.write_u32::<BigEndian>(x as u32).unwrap(),
        8 => buf.write_u64::<BigEndian>(x).unwrap(),
        _ => (),
    }
    buf
}

/// Time stamp in format `yyy-mm-ddThh-mm-ss`
pub fn timestamp() -> String {
    // `2017-05-01T20:21:10.163281100+00:00` -> `2017-05-01T20-21-10`
    str::replace(&Utc::now().to_rfc3339(), ":", "-")
        .split('.')
        .next()
        .unwrap()
        .to_string()
}

///
// pub fn to_16bytes(hex: &str) -> [u8; 16] {
//     to_arr(Vec::from_hex(&hex).unwrap().as_slice())
// }

///
// pub fn to_20bytes(hex: &str) -> [u8; 20] {
//     to_arr(Vec::from_hex(&hex).unwrap().as_slice())
// }

///
// pub fn to_32bytes(hex: &str) -> [u8; 32] {
//     to_arr(Vec::from_hex(&hex).unwrap().as_slice())
// }

// pub fn get_nstime() -> u64 {
//     let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
//     // The correct way to calculate the current time is
//     // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
//     // But this is faster, and the difference in terms of entropy is
//     // negligible (log2(10^9) == 29.9).
//     dur.as_secs() << 30 | dur.subsec_nanos() as u64
// }

pub fn get_valid_timestamp(future_millis: u128) -> u128 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_millis().checked_add(future_millis).unwrap()
}

// pub fn wei_to_eth(wei_val: U256) -> f64 {
//     let res = wei_val.as_u128() as f64;
//     res / 1_000_000_000_000_000_000.0
// }

// pub fn eth_to_wei(eth_val: f64) -> U256 {
//     let result = eth_val * 1_000_000_000_000_000_000.0;
//     let result = result as u128;
//     U256::from(result)
// }

pub fn convert_to_wei(amount: f32, multiplier: f32) -> U256 {
    let ether = format!("{:.0}", amount * multiplier);
    U256::from_dec_str(&ether).unwrap()
}

// pub fn convert_from_wei(wei: U256, multiplier: f32) -> f32 {
//     let wei_u128 = wei.as_u128();
//     (wei_u128 as f32) / multiplier
// }
