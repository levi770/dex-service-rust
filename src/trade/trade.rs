use crate::database::models::account::Account;
use crate::trade::handler::trade_rpc::SwapRequest;
use crate::wallet::util::{convert_to_wei, get_valid_timestamp};
use crate::wallet::wallet::Wallet;
use std::error::Error;
use web3::contract::{tokens::Tokenize, Contract, Options};
use web3::types::{Address, Bytes, TransactionParameters, TransactionRequest, H256, U256};
use web3::{transports::WebSocket, Web3};

pub struct Trade {}

impl Trade {
    pub async fn swap(
        w3: &Web3<WebSocket>,
        acc: &Account,
        r_abi: &[u8],
        t0_abi: &[u8],
        r: &Address,
        t0: &Address,
        t1: &Address,
        p: &SwapRequest,
    ) -> Result<H256, Box<dyn Error>> {
        let addr = Address::from_slice(acc.address.as_bytes());
        let r_cont = Contract::from_json(w3.eth(), r.clone(), r_abi).unwrap();
        let t0_cont = Contract::from_json(w3.eth(), t0.clone(), t0_abi).unwrap();
        let weth: Address = r_cont
            .query("WETH", (), None, Options::default(), None)
            .await
            .unwrap();
        let t0_dec: U256 = t0_cont
            .query("decimals", (), None, Options::default(), None)
            .await
            .unwrap();
        let t0_mult = match t0_dec.as_u32() {
            3 => 1e3,
            6 => 1e6,
            9 => 1e9,
            18 => 1e18,
            _ => 1e18,
        };

        let swap_type: &str;
        let route: Vec<Address>;
        if t0 == &weth {
            swap_type = "swapExactETHForTokens".trim();
            route = vec![weth, t1.to_owned()];
        } else if t1 == &weth {
            swap_type = "swapExactTokensForETH".trim();
            route = vec![t0.to_owned(), weth];
        } else {
            swap_type = "swapExactTokensForTokens".trim();
            route = vec![t0.to_owned(), weth, t1.to_owned()];
        }

        let block = w3.eth().block_number().await.unwrap();
        let amount_in = convert_to_wei(p.amount.to_owned(), t0_mult);
        let amount_out: Vec<U256> = r_cont
            .query(
                "getAmountsOut",
                (amount_in, route.clone()),
                addr,
                Options::default(),
                Some(block.into()),
            )
            .await
            .unwrap();
        let t0_balance: U256 = t0_cont
            .query("balanceOf", addr, None, Options::default(), None)
            .await
            .unwrap();
        if t0_balance < amount_in {
            return Err("Insufficient token0 balance".into());
        }

        let t0_allowance: U256 = t0_cont
            .query(
                "allowance",
                (addr, r.to_owned()),
                None,
                Options::default(),
                None,
            )
            .await
            .unwrap();
        if t0_allowance < amount_in {
            let nonce = w3.eth().transaction_count(addr, None).await.unwrap();
            let gas_price = w3.eth().gas_price().await.unwrap();
            let data = t0_cont
                .abi()
                .function("approve")
                .unwrap()
                .encode_input(&(r.to_owned(), amount_in).into_tokens())
                .unwrap();
            let gas_estimate = t0_cont
                .estimate_gas(
                    "approve",
                    (r.to_owned(), amount_in),
                    addr,
                    Options {
                        ..Default::default()
                    },
                )
                .await
                .unwrap();
            let tx_payload = TransactionParameters {
                nonce: Some(nonce),
                to: Some(t0_cont.address()),
                value: U256::zero(),
                gas_price: Some(gas_price),
                max_priority_fee_per_gas: Some(gas_estimate),
                data: Bytes(data),
                ..Default::default()
            };
            Wallet::send(w3, acc, &tx_payload).await?;
        }

        let valid_timestamp = get_valid_timestamp(300000);
        let deadline = U256::from_dec_str(&valid_timestamp.to_string()).unwrap();

        let nonce = w3.eth().transaction_count(addr, None).await.unwrap();
        let gas_price = w3.eth().gas_price().await.unwrap();
        let tx_payload: TransactionParameters;
        match swap_type {
            "swapExactETHForTokens" => {
                let gas_estimate = r_cont
                    .estimate_gas(
                        swap_type,
                        (amount_out.clone(), route.clone(), addr, deadline),
                        addr,
                        Options {
                            value: Some(amount_in),
                            ..Default::default()
                        },
                    )
                    .await
                    .unwrap();
                let data = r_cont
                    .abi()
                    .function(swap_type)
                    .unwrap()
                    .encode_input(&(amount_out, route, addr, deadline).into_tokens())
                    .unwrap();
                tx_payload = TransactionParameters {
                    nonce: Some(nonce),
                    to: Some(r.to_owned()),
                    value: amount_in,
                    gas_price: Some(gas_price),
                    max_priority_fee_per_gas: Some(gas_estimate),
                    data: Bytes(data),
                    ..Default::default()
                };
            }
            "swapExactTokensForETH" => {
                let in_gas_estimate = r_cont
                    .estimate_gas(
                        swap_type,
                        (amount_in, amount_out.clone(), route.clone(), addr, deadline),
                        addr,
                        Options {
                            value: Some(U256::zero()),
                            gas: Some(500_000.into()),
                            ..Default::default()
                        },
                    )
                    .await
                    .unwrap();
                let data = r_cont
                    .abi()
                    .function(swap_type)
                    .unwrap()
                    .encode_input(&(amount_in, amount_out, route, addr, deadline).into_tokens())
                    .unwrap();
                tx_payload = TransactionParameters {
                    nonce: Some(nonce),
                    to: Some(r.to_owned()),
                    value: U256::zero(),
                    gas_price: Some(gas_price),
                    max_priority_fee_per_gas: Some(in_gas_estimate),
                    data: Bytes(data),
                    ..Default::default()
                };
            }
            "swapExactTokensForTokens" => {
                let in_gas_estimate = r_cont
                    .estimate_gas(
                        swap_type,
                        (amount_in, amount_out.clone(), route.clone(), addr, deadline),
                        addr,
                        Options {
                            value: Some(U256::zero()),
                            gas: Some(500_000.into()),
                            ..Default::default()
                        },
                    )
                    .await
                    .unwrap();
                let data = r_cont
                    .abi()
                    .function(swap_type)
                    .unwrap()
                    .encode_input(&(amount_in, amount_out, route, addr, deadline).into_tokens())
                    .unwrap();
                tx_payload = TransactionParameters {
                    nonce: Some(nonce),
                    to: Some(r.to_owned()),
                    value: U256::zero(),
                    gas_price: Some(gas_price),
                    data: Bytes(data),
                    max_priority_fee_per_gas: Some(in_gas_estimate),
                    ..Default::default()
                };
            }
            _ => {
                return Err("Invalid swap type".into());
            }
        }
        Ok(Wallet::send(w3, acc, &tx_payload).await.unwrap())
    }

    // pub async fn pair() {}
    // pub async fn liquidity() {}
    // pub fn weth() {}
}
