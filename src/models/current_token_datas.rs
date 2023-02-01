use anyhow::Result;
use bigdecimal::BigDecimal;

use diesel::prelude::*;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::current_token_ownerships::CurrentTokenOwnership;


#[derive(Debug, Queryable, Deserialize, FieldCount, Serialize)]
#[diesel(primary_key(token_data_id_hash))]
#[diesel(table_name = current_token_datas)]
pub struct CurrentTokenData {
    pub token_data_id_hash: String,
    pub creator_address: String,
    pub collection_name: String,
    pub name: String,
    pub maximum: BigDecimal,
    pub supply: BigDecimal,
    pub largest_property_version: BigDecimal,
    pub metadata_uri: String,
    pub payee_address: String,
    pub royalty_points_numerator: BigDecimal,
    pub royalty_points_denominator: BigDecimal,
    pub maximum_mutable: bool,
    pub uri_mutable: bool,
    pub description_mutable: bool,
    pub properties_mutable: bool,
    pub royalty_mutable: bool,
    pub default_properties: Value,
    pub last_transaction_version: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

pub fn query_nfts_by_collection(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
    address: &str,
    c_name: &str,
) -> Result<Vec<CurrentTokenData>> {
    use crate::schema::current_token_datas::dsl::*;
    // collection_data_id_hash
    let results = current_token_datas
        .filter(creator_address.eq(address))
        .filter(collection_name.eq(c_name))
        .limit(20)
        .load::<CurrentTokenData>(&mut *db)?;

    println!("Displaying {} posts", results.len());
    Ok(results)
}

pub fn query_nfts_by_owner(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
    address: &str,
) -> Result<Vec<(CurrentTokenData, CurrentTokenOwnership)>> {
    use crate::schema::*;

    let query = current_token_datas::table.inner_join(current_token_ownerships::table);
    let query = query
        .filter(current_token_ownerships::owner_address.eq(address))
        .filter(current_token_ownerships::amount.gt(BigDecimal::from(0)));

    query
        .load::<(CurrentTokenData, CurrentTokenOwnership)>(&mut *db)
        .map_err(|e| e.into())
}


pub fn query_bigger_then_version(
    mut db:&mut PooledConnection<ConnectionManager<PgConnection>>,
    version: i64,
) -> Result<Vec<CurrentTokenData>> {
    use crate::schema::current_token_datas::dsl::*;
    
    current_token_datas
        .filter(last_transaction_version.gt(version))
        .limit(100)
        .order(last_transaction_version)
        .load::<CurrentTokenData>(db)
        .map_err(|e| e.into())
}
