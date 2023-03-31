use crate::database::models::account::Account;
use crate::database::pool::PgPool;
use crate::market::market::Market;
use crate::trade::trade::Trade;
use crate::wallet::wallet::Wallet;
use tonic::{Request, Response, Status};

pub mod trade_rpc {
    tonic::include_proto!("trade");
}
use trade_rpc::{
    trade_service_server::{TradeService, TradeServiceServer},
    SwapRequest, SwapResponse,
};

pub struct TradeHandler {
    pub pool: PgPool,
    pub market: Market,
    pub web3: Wallet,
}

impl TradeHandler {
    pub fn new_grpc_service(pool: PgPool, market: Market, web3: Wallet) -> Self {
        Self { pool, market, web3 }
    }
}

pub fn new_grpc_service(
    pool: PgPool,
    market: Market,
    web3: Wallet,
) -> TradeServiceServer<TradeHandler> {
    let handler = TradeHandler::new_grpc_service(pool, market, web3);
    TradeServiceServer::new(handler)
}

// fn into_response(hash: &H256) -> SwapResponse {
//     SwapResponse {
//         hash: hash.to_string(),
//     }
// }

#[tonic::async_trait]
impl TradeService for TradeHandler {
    async fn swap(&self, req: Request<SwapRequest>) -> Result<Response<SwapResponse>, Status> {
        let payload = req.into_inner();
        let mut conn = self.pool.get().expect("Failed to get connection from pool");
        let acc = Account::find_by_user_id(&payload.user_id, &mut conn)
            .await
            .expect("Failed to get asset from market");
        let w3 = self
            .web3
            .try_get_instance(&payload.chain_id)
            .expect("Failed to get web3 instance from web3");
        let t0 = self
            .market
            .try_get_asset(&payload.chain_id, &payload.token0)
            .expect("Failed to get asset from market");
        let t1 = self
            .market
            .try_get_asset(&payload.chain_id, &payload.token1)
            .expect("Failed to get asset from market");
        let ex = self
            .market
            .try_get_exchange(&payload.chain_id, &payload.exchange)
            .expect("Failed to get exchange from market");
        let r_abi = self
            .market
            .try_get_abi(&"IUniswapV2Router02".to_string())
            .expect("Failed to get abi from market");
        let t0_abi = self
            .market
            .try_get_abi(&"IERC20".to_string())
            .expect("Failed to get abi from market");

        let tx_hash = Trade::swap(&w3, &acc, &r_abi, &t0_abi, &ex, &t0, &t1, &payload)
            .await
            .unwrap();

        Ok(Response::new(SwapResponse {
            hash: tx_hash.to_string(),
        }))
    }
}
