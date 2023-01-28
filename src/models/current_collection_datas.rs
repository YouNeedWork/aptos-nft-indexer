use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Deserialize, Serialize};

use log::info;

use crate::schema;
use schema::*;

/// Need a separate struct for queryable because we don't want to define the inserted_at column (letting DB fill)
#[derive(Debug, Identifiable, Queryable, Deserialize, Serialize)]
#[diesel(primary_key(collection_data_id_hash))]
#[diesel(table_name = current_collection_datas)]
pub struct CurrentCollectionDataQuery {
    pub collection_data_id_hash: String,
    pub creator_address: String,
    pub collection_name: String,
    pub description: String,
    pub metadata_uri: String,
    pub supply: BigDecimal,
    pub maximum: BigDecimal,
    pub maximum_mutable: bool,
    pub uri_mutable: bool,
    pub description_mutable: bool,
    pub last_transaction_version: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

pub fn query_info_by_collection_hash(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
    hash: &str,
) -> Result<CurrentCollectionDataQuery> {
    use crate::schema::current_collection_datas::dsl::*;

    info!("Querying nfts by collection");

    current_collection_datas::table()
        .filter(collection_data_id_hash.eq(hash))
        .first(&mut *db)
        .map_err(|e| e.into())
}

pub fn query_bigger_then_version(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
    version: i64,
) -> Result<Vec<CurrentCollectionDataQuery>> {
    use crate::schema::current_collection_datas::dsl::*;

    current_collection_datas::table()
        .filter(last_transaction_version.gt(version))
        .limit(20)
        .load::<CurrentCollectionDataQuery>(&mut *db)
        .map_err(|e| e.into())
}
