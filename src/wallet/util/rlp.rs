//! RLP (Recursive Length Prefix) is to encode arbitrarily nested arrays of binary data,
//! RLP is the main encoding method used to serialize objects in Ethereum.
//!
//! See [RLP spec](https://github.com/ethereumproject/wiki/wiki/RLP)

use super::{bytes_count, to_bytes, trim_bytes};

/// The `WriteRLP` trait is used to specify functionality of serializing data to RLP bytes
pub trait WriteRLP {
    /// Writes itself as RLP bytes into specified buffer
    fn write_rlp(&self, buf: &mut Vec<u8>);
}

/// A list serializable to RLP
#[derive(Debug)]
pub struct RLPList {
    tail: Vec<u8>,
}

impl RLPList {
    /// Start with provided vector
    pub fn from_slice<T: WriteRLP>(items: &[T]) -> RLPList {
        let mut start = RLPList { tail: Vec::new() };
        for i in items {
            start.push(i)
        }
        start
    }

    /// Add an item to the list
    pub fn push<T: WriteRLP + ?Sized>(&mut self, item: &T) {
        item.write_rlp(&mut self.tail);
    }
}

impl Default for RLPList {
    fn default() -> RLPList {
        RLPList { tail: Vec::new() }
    }
}

impl Into<Vec<u8>> for RLPList {
    fn into(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        match self.tail.len() {
            s @ 0..=55 => {
                res.push((s + 192) as u8);
                res.extend(self.tail.as_slice());
            }
            v => {
                let sb = to_bytes(v as u64, 8);
                let size_arr = trim_bytes(&sb);
                res.push((size_arr.len() + 247) as u8);
                res.extend(size_arr);
                res.extend(self.tail.as_slice());
            }
        }
        res
    }
}

impl WriteRLP for str {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();

        if self.len() == 1 && bytes[0] <= 0x7f {
            buf.push(bytes[0]);
        } else {
            bytes.write_rlp(buf);
        }
    }
}

impl WriteRLP for String {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();

        if self.len() == 1 && bytes[0] <= 0x7f {
            buf.push(bytes[0]);
        } else {
            bytes.write_rlp(buf);
        }
    }
}

impl WriteRLP for u8 {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        if *self == 0 {
            buf.push(0x80);
        } else if *self <= 0x7f {
            buf.push(*self);
        } else {
            trim_bytes(&to_bytes(u64::from(*self), 1)).write_rlp(buf);
        }
    }
}

impl WriteRLP for u16 {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        if *self == 0 {
            buf.push(0x80);
        } else if *self <= 0x7f {
            buf.push(*self as u8);
        } else {
            trim_bytes(&to_bytes(u64::from(*self), 2)).write_rlp(buf);
        }
    }
}

impl WriteRLP for u32 {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        if *self == 0 {
            buf.push(0x80);
        } else if *self <= 0x7f {
            buf.push(*self as u8);
        } else {
            trim_bytes(&to_bytes(u64::from(*self), 4)).write_rlp(buf);
        }
    }
}

impl WriteRLP for u64 {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        if *self == 0 {
            buf.push(0x80);
        } else if *self <= 0x7f {
            buf.push(*self as u8);
        } else {
            trim_bytes(&to_bytes(*self, 8)).write_rlp(buf);
        }
    }
}

impl<'a, T: WriteRLP + ?Sized> WriteRLP for Option<&'a T> {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        match *self {
            Some(x) => x.write_rlp(buf),
            None => [].write_rlp(buf),
        };
    }
}

impl<T: WriteRLP> WriteRLP for Vec<T> {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        RLPList::from_slice(self).write_rlp(buf);
    }
}

impl WriteRLP for [u8] {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        let len = self.len();
        if len <= 55 {
            // Otherwise, if a string is 0-55 bytes long, the RLP encoding consists of a single byte
            // with value 0x80 plus the length of the string followed by the string. The range of
            // the first byte is thus [0x80, 0xb7].
            buf.push(0x80 + len as u8);
            buf.extend_from_slice(self);
        } else {
            // If a string is more than 55 bytes long, the RLP encoding consists of a single byte
            // with value 0xb7 plus the length in bytes of the length of the string in binary form,
            // followed by the length of the string, followed by the string. For example, a
            // length-1024 string would be encoded as \xb9\x04\x00 followed by the string. The
            // range of the first byte is thus [0xb8, 0xbf].
            let len_bytes = bytes_count(len);
            buf.push(0xb7 + len_bytes);
            buf.extend_from_slice(&to_bytes(len as u64, len_bytes));
            buf.extend_from_slice(self);
        }
    }
}

impl WriteRLP for RLPList {
    fn write_rlp(&self, buf: &mut Vec<u8>) {
        let len = self.tail.len();
        if len <= 55 {
            // If the total payload of a list (i.e. the combined length of all its items) is 0-55
            // bytes long, the RLP encoding consists of a single byte with value 0xc0 plus the
            // length of the list followed by the concatenation of the RLP encodings of the items.
            // The range of the first byte is thus [0xc0, 0xf7].
            buf.push((0xc0 + len) as u8);
        } else {
            // If the total payload of a list is more than 55 bytes long, the RLP encoding consists
            // of a single byte with value 0xf7 plus the length in bytes of the length of the
            // payload in binary form, followed by the length of the payload, followed by the
            // concatenation of the RLP encodings of the items. The range of the first byte is
            // thus [0xf8, 0xff].
            let len_bytes = bytes_count(len);
            buf.push(0xf7 + len_bytes);
            buf.extend_from_slice(&to_bytes(len as u64, len_bytes));
        }
        buf.extend_from_slice(&self.tail);
    }
}
