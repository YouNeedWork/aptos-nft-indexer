use diesel::prelude::*;
use anyhow::Result;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::schema::*;

#[derive(Queryable,Insertable)]
pub struct Collections {
    pub id: i64,
    pub chain_id: u8,
    pub collection_id: String,
    pub creator_address: String,
    pub collection_name: String,
    pub description: String,
    pub supply: i64,
    pub version: i64,
    pub metadata_uri: String,
}


pub fn query_collections(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<i64> {
    use crate::schema::collections::*;
    
    let a = collections::table()
	.filter(chain_id.eq(CHAIN_ID::APTOS.into()))
	.order_by(last_transaction_version.desc())
        .first(db)
        .map_err(|e| e.into())?;
    
    a.version
}
