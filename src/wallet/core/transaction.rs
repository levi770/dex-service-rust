//! # Account transaction

use super::util::{trim_bytes, RLPList, WriteRLP};
use super::{Address, Error, PrivateKey, Signature};
use web3::signing::keccak256;

pub const KECCAK256_BYTES: usize = 32;

/// Transaction data
#[derive(Clone, Debug, Default)]
pub struct Transaction {
    /// Nonce
    pub nonce: u64,

    /// Gas Price
    pub gas_price: [u8; 32],

    /// Gas Limit
    pub gas_limit: u64,

    /// Target address, or None to create contract
    pub to: Option<Address>,

    /// Value transferred with transaction
    pub value: [u8; 32],

    /// Data transferred with transaction
    pub data: Vec<u8>,
}

impl Transaction {
    /// Sign transaction data with provided private key
    pub fn to_signed_raw(&self, pk: PrivateKey, chain: u8) -> Result<Vec<u8>, Error> {
        let sig = pk.sign_hash(self.hash(chain))?;
        Ok(self.raw_from_sig(chain, &sig))
    }

    /// RLP packed signed transaction from provided `Signature`
    pub fn raw_from_sig(&self, chain: u8, sig: &Signature) -> Vec<u8> {
        let mut rlp = self.to_rlp_raw(None);

        // [Simple replay attack protection](https://github.com/ethereum/eips/issues/155)
        // Can be already applied by HD wallet.
        // TODO: refactor to avoid this check
        let mut v = u16::from(sig.v);
        let stamp = u16::from(chain * 2 + 35 - 27);
        if v + stamp <= 0xff {
            v += stamp;
        }

        rlp.push(&(v as u8));
        rlp.push(&sig.r[..]);
        rlp.push(&sig.s[..]);

        let mut buf = Vec::new();
        rlp.write_rlp(&mut buf);

        buf
    }

    /// RLP packed transaction
    pub fn to_rlp(&self, chain_id: Option<u8>) -> Vec<u8> {
        let mut buf = Vec::new();
        self.to_rlp_raw(chain_id).write_rlp(&mut buf);

        buf
    }

    fn to_rlp_raw(&self, chain_id: Option<u8>) -> RLPList {
        let mut data = RLPList::default();

        data.push(&self.nonce);
        data.push(trim_bytes(&self.gas_price));
        data.push(&self.gas_limit);

        match self.to {
            Some(addr) => data.push(&Some(&addr[..])),
            _ => data.push::<Option<&[u8]>>(&None),
        };

        data.push(trim_bytes(&self.value));
        data.push(self.data.as_slice());

        if let Some(id) = chain_id {
            data.push(&id);
            data.push(&[][..]);
            data.push(&[][..]);
        }

        data
    }

    fn hash(&self, chain: u8) -> [u8; KECCAK256_BYTES] {
        let rlp = self.to_rlp_raw(Some(chain));
        let mut vec = Vec::new();
        rlp.write_rlp(&mut vec);

        keccak256(&vec)
    }
}