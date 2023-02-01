use anyhow::{anyhow, Result};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::schema::*;
use crate::ChainID;

#[derive(Queryable, Insertable)]
#[diesel(table_name = collections)]
pub struct Collections {
    pub id: i32,
    pub chain_id: i32,
    pub collection_id: String,
    pub creator_address: String,
    pub collection_name: String,
    pub description: String,
    pub supply: i32,
    pub version: i32,
    pub metadata_uri: String,
    pub metadata_json: String,
    pub image: String,
}

pub fn query_collections(mut db: PooledConnection<ConnectionManager<PgConnection>>) -> Result<i32> {
    use crate::schema::collections::dsl::*;

    let a: Collections = collections::table()
        .filter(chain_id.eq(Into::<u8>::into(ChainID::Aptos) as i32))
        .order(version.desc())
        .first(&mut *db)
        .map_err(|e| anyhow!(e))?;

    Ok(a.version)
}

pub fn insert_collection(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
    c: Collections,
) -> Result<()> {
    //use crate::schema::collections::*;
    
    diesel::insert_into(collections::table)
        .values(&c)
        .execute(&mut *db)?;
    
    Ok(())
}
