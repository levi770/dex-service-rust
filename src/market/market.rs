use std::collections::HashMap;
use std::{env, fs};
use web3::types::Address;

const ASSETS_JSON: &str = include_str!("./json/address_book/assets.json");
const EXCHANGES_JSON: &str = include_str!("./json/address_book/exchanges.json");

pub async fn build_market() -> Market {
    let assets: HashMap<u32, HashMap<String, Address>> = serde_json::from_str(ASSETS_JSON).unwrap();
    let exchanges: HashMap<u32, HashMap<String, Address>> =
        serde_json::from_str(EXCHANGES_JSON).unwrap();
    let mut abis: HashMap<String, Vec<u8>> = HashMap::new();
    let abi_dir = env::current_dir().unwrap().join("src/market/json/abi/");
    for entry in fs::read_dir(abi_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let f_name = path.file_name().unwrap();
        let abi_name = f_name
            .to_string_lossy()
            .trim_end_matches(".json")
            .to_owned();
        let abi_value = fs::read(path).unwrap();
        abis.insert(abi_name, abi_value);
    }
    Market::new(assets, exchanges, abis).clone()
}

/// Wrapper around a hash map that maps a [Chain] to the contract's deployed address on that chain.
#[derive(Clone, Debug)]
pub struct Market {
    pub assets: HashMap<u32, HashMap<String, Address>>,
    pub exchanges: HashMap<u32, HashMap<String, Address>>,
    pub abis: HashMap<String, Vec<u8>>,
}

impl Market {
    pub fn new(
        assets: HashMap<u32, HashMap<String, Address>>,
        exchanges: HashMap<u32, HashMap<String, Address>>,
        abis: HashMap<String, Vec<u8>>,
    ) -> Self {
        Self {
            assets,
            exchanges,
            abis,
        }
    }

    // pub fn try_get_assets(&self, chain: &u32) -> Option<HashMap<String, Address>> {
    //     self.assets.get(chain).cloned()
    // }

    pub fn try_get_asset(&self, chain: &u32, name: &String) -> Option<Address> {
        let assets = self.assets.get(chain).unwrap();
        assets.get(name).cloned()
    }

    // pub fn try_get_exchanges(&self, chain: &u32) -> Option<HashMap<String, Address>> {
    //     self.exchanges.get(chain).cloned()
    // }

    pub fn try_get_exchange(&self, chain: &u32, name: &String) -> Option<Address> {
        let exchange = self.exchanges.get(chain).unwrap();
        exchange.get(name).cloned()
    }

    pub fn try_get_abi(&self, name: &String) -> Option<Vec<u8>> {
        self.abis.get(name).cloned()
    }
}
