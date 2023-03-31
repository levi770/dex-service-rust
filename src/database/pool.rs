use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub async fn build_pool() -> Result<PgPool, PoolError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
