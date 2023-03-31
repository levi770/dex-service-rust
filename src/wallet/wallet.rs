use rand::thread_rng;
use secp256k1::SecretKey;
use std::{collections::HashMap, env};
use web3::{
    transports::{self, WebSocket}, signing::SecretKeyRef,
    types::{TransactionParameters, TransactionRequest, H160, H256},
    Error as Web3Error, Web3,
};
use web3_keystore::KeyStoreError;

use crate::wallet::keystore::{save_keyfile, Kdf, KeyFile, KeyfileStorage, Keystore};
use crate::{
    database::models::account::Account,
    wallet::core::{Address, PrivateKey},
};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use super::Transaction;

pub async fn build_wallet() -> Wallet {
    dotenv::dotenv().ok();
    let chains: Vec<u32> = vec![1, 5, 137, 80001, 56, 97];
    let mut intstances: HashMap<u32, Web3<transports::WebSocket>> = HashMap::new();

    for chain in chains {
        let url: String;
        match chain {
            1 => url = env::var("ETH_MAINNET").unwrap(),
            5 => url = env::var("ETH_TESTNET").unwrap(),
            137 => url = env::var("MATIC_MAINNET").unwrap(),
            80001 => url = env::var("MATIC_TESTNET").unwrap(),
            56 => url = env::var("BSC_MAINNET").unwrap(),
            97 => url = env::var("BSC_TESTNET").unwrap(),
            x => panic!("Unexpected invalid chain {:?}", x),
        }
        let websocket = transports::WebSocket::new(&url).await.unwrap();
        let web3 = Web3::new(websocket);
        intstances.insert(chain, web3);
    }

    Wallet::new(intstances).clone()
}

#[derive(Clone, Debug)]
pub struct Wallet {
    inst: HashMap<u32, Web3<WebSocket>>,
}

impl Wallet {
    pub fn new(intstances: HashMap<u32, Web3<WebSocket>>) -> Self {
        Self { inst: intstances }
    }

    pub fn try_get_instance(&self, chain: &u32) -> Option<Web3<WebSocket>> {
        self.inst.get(chain).cloned()
    }

    pub fn new_account() -> Result<(Address, KeyFile), KeyStoreError> {
        let (kdf, ks, pk) = create_keystore().unwrap();
        let binding = env::var("KEYSTORE_PATH").unwrap();
        let path = binding.as_str();
        let s = &env::var("SECRET").unwrap();
        let mut rng = thread_rng();
        let keyfile = KeyFile::new_custom(pk, &s, kdf, &mut rng, None, None).unwrap();
        let _ = save_keyfile(keyfile.clone(), path).unwrap();
        let accs = ks.list_accounts(false).unwrap();
        let (addr, _) = unlock_keyfile(&accs[0].filename.to_string()).unwrap();
        Ok((addr, keyfile))
    }

    pub async fn send(
        w3: &Web3<WebSocket>,
        acc: &Account,
        tx: &TransactionParameters,
    ) -> Result<H256, Web3Error> {
        dotenv::dotenv().ok();
        let s = &env::var("DEFAULT_PASS").unwrap();
        let ks = acc.keystore.as_str().unwrap();
        let kf = KeyFile::decode(ks).unwrap();
        let pk = kf.decrypt_key(s).expect("Wrong passphrase");
        // let sk: SecretKey = pk.into();
        // let skr = SecretKeyRef::new(&sk);

        // let signed = w3.accounts().sign_transaction(tx.to_owned(), sk.into());

        Ok(H256::random())
    }
}

fn create_keystore() -> Result<(Kdf, Keystore, PrivateKey), KeyStoreError> {
    let pk = PrivateKey::gen();
    let kdf = Kdf::from((8, 2, 1));
    let binding = env::var("KEYSTORE_PATH").unwrap();
    let path = binding.as_str();
    let ks = Keystore::new(&path);
    Ok((kdf, ks, pk))
}

fn unlock_keyfile(filename: &str) -> Result<(Address, PrivateKey), KeyStoreError> {
    let binding = env::var("SECRET").unwrap();
    let s = binding.as_str();
    let path =
        env::var("KEYSTORE_PATH").unwrap() + &keyfile_path(filename).to_str().unwrap().to_string();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let keyfile = KeyFile::decode(&contents).unwrap();
    let k = keyfile.decrypt_key(s).expect("Wrong passphrase");
    let h = KeyFile::decode(&contents).unwrap().address;
    Ok((h, k))
}

pub fn keyfile_path(name: &str) -> PathBuf {
    let mut path = keystore_path();
    path.push(name);
    path
}

pub fn keystore_path() -> PathBuf {
    let mut buf = PathBuf::from("");
    buf.push("/");
    buf
}
