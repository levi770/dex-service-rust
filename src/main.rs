mod account;
mod database;
mod market;
mod trade;
mod wallet;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use database::pool::build_pool;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let pool = build_pool().await.unwrap();
    let market = market::market::build_market().await;
    let web3 = wallet::wallet::build_wallet().await;

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(account::handler::new_grpc_service(pool.clone()))
        .add_service(trade::handler::new_grpc_service(pool.clone(), market, web3))
        .serve(addr)
        .await?;

    Ok(())
}
