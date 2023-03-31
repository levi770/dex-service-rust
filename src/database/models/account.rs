use crate::database::schema::accounts;
use crate::database::schema::accounts::dsl::*;
use crate::diesel::ExpressionMethods;
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::RunQueryDsl;
use diesel::{delete, insert_into, update, QueryDsl};

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub user_id: String,
    pub address: String,
    pub keystore: serde_json::Value,
}

impl NewAccount {
    pub async fn create(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Account {
        insert_into(accounts)
            .values(self)
            .get_result(conn)
            .expect("Error saving new account")
    }
}

#[derive(Queryable, AsChangeset, Debug)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub user_id: String,
    pub address: String,
    pub keystore: serde_json::Value,
}

impl Account {
    pub async fn find_by_id(
        account_id: &i32,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Account, diesel::result::Error> {
        accounts::table.find(account_id).first(conn)
    }

    pub async fn find_by_user_id(
        account_user_id: &String,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Account, diesel::result::Error> {
        accounts.filter(user_id.eq(account_user_id)).first(conn)
    }

    pub async fn destroy(
        account_id: &i32,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), diesel::result::Error> {
        delete(accounts::table.find(account_id)).execute(conn)?;
        Ok(())
    }

    pub async fn update(
        account_id: &i32,
        account: &Account,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), diesel::result::Error> {
        update(accounts::table.find(account_id))
            .set(account)
            .execute(conn)?;
        Ok(())
    }

    pub async fn list(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<Account>, diesel::result::Error> {
        accounts.load::<Account>(conn)
    }
}
