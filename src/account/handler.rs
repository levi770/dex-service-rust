mod account_rpc {
    tonic::include_proto!("account");
}

use crate::database::models::account::{Account, NewAccount};
use crate::database::pool::PgPool;
use crate::wallet::wallet::Wallet;
use account_rpc::account_service_server::{AccountService, AccountServiceServer};
use account_rpc::{
    ByIdRequest, ByUserIdRequest, CreateAccountRequest, CreateAccountResponse, EmptyRequest,
    FindAllAccountsResponse, FindOneAccountResponse,
};
use tonic::{Request, Response, Status};

pub struct AccountHandler {
    pool: PgPool,
}

impl AccountHandler {
    pub fn new_grpc_service(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub fn new_grpc_service(conn: PgPool) -> AccountServiceServer<AccountHandler> {
    let handler = AccountHandler::new_grpc_service(conn);
    AccountServiceServer::new(handler)
}

fn into_response(account: &Account) -> FindOneAccountResponse {
    FindOneAccountResponse {
        id: account.id,
        user_id: account.user_id.to_owned(),
        address: account.address.to_owned(),
    }
}

#[tonic::async_trait]
impl AccountService for AccountHandler {
    async fn create(
        &self,
        req: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        let mut conn = self.pool.get().expect("Failed to get connection from pool");
        let create_req = req.into_inner();
        let (addr, kf) = Wallet::new_account().unwrap();
        let new_account = NewAccount {
            user_id: create_req.user_id,
            address: addr.to_string(),
            keystore: serde_json::to_value(&kf).unwrap(),
        };
        let account = new_account.create(&mut conn).await;

        Ok(Response::new(CreateAccountResponse {
            id: account.id,
            address: addr.to_string(),
        }))
    }

    async fn list(
        &self,
        _req: Request<EmptyRequest>,
    ) -> Result<Response<FindAllAccountsResponse>, Status> {
        let mut conn = self.pool.get().expect("Failed to get connection from pool");
        let accounts = match Account::list(&mut conn).await {
            Ok(acc) => {
                let accounts_response = acc.iter().map(|t| into_response(t)).collect::<Vec<_>>();
                FindAllAccountsResponse {
                    accounts: accounts_response,
                }
            }
            Err(_e) => return Err(Status::not_found("Not Found")),
        };
        Ok(Response::new(accounts))
    }

    async fn by_id(
        &self,
        req: Request<ByIdRequest>,
    ) -> Result<Response<FindOneAccountResponse>, Status> {
        let mut conn = self.pool.get().expect("Failed to get connection from pool");
        let account_id: i32 = req.into_inner().id;
        let account = match Account::find_by_id(&account_id, &mut conn).await {
            Ok(acc) => into_response(&acc),
            Err(_e) => return Err(Status::not_found("Not Found")),
        };
        Ok(Response::new(account))
    }

    async fn by_user_id(
        &self,
        req: Request<ByUserIdRequest>,
    ) -> Result<Response<FindOneAccountResponse>, Status> {
        let mut conn = self.pool.get().expect("Failed to get connection from pool");
        let user_id: String = req.into_inner().user_id;
        let account = match Account::find_by_user_id(&user_id, &mut conn).await {
            Ok(acc) => into_response(&acc),
            Err(_e) => return Err(Status::not_found("Not Found")),
        };
        Ok(Response::new(account))
    }
}

// pub fn destroy(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
//     let pg_pool = pg_pool_handler(pool)?;
//     Product::destroy(&id, &pg_pool)
//         .map(|_| HttpResponse::Ok().json(()))
//         .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
// }

// pub fn update(
//     id: web::Path<i32>,
//     new_product: web::Json<NewProduct>,
//     pool: web::Data<PgPool>,
// ) -> Result<HttpResponse, HttpResponse> {
//     let pg_pool = pg_pool_handler(pool)?;
//     Product::update(&id, &new_product, &pg_pool)
//         .map(|_| HttpResponse::Ok().json(()))
//         .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
// }
