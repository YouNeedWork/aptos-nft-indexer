use anyhow::Result;
use bigdecimal::BigDecimal;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Queryable, Deserialize, FieldCount, Serialize)]
#[diesel(primary_key(token_data_id_hash, property_version, owner_address))]
#[diesel(table_name = current_token_ownerships)]
pub struct CurrentTokenOwnership {
    pub token_data_id_hash: String,
    pub property_version: BigDecimal,
    pub owner_address: String,
    pub creator_address: String,
    pub collection_name: String,
    pub name: String,
    pub amount: BigDecimal,
    pub token_properties: Value,
    pub last_transaction_version: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

pub fn query_nfts_by_owner(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
    user_wallet: &str,
) -> Result<Vec<CurrentTokenOwnership>> {
    use crate::schema::current_token_ownerships::dsl::*;

    let results = current_token_ownerships
        .filter(owner_address.eq(user_wallet))
        .filter(amount.gt(BigDecimal::from(0)))
        .limit(20)
        .load::<CurrentTokenOwnership>(&mut *db)?;

    println!("Displaying {} posts", results.len());
    Ok(results)
}
