use anyhow::{anyhow, Result};
use bigdecimal::ToPrimitive;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use super::current_token_datas::CurrentTokenData;
use crate::schema::*;

use crate::ChainID;

#[derive(Queryable)]
#[diesel(table_name = tokens)]
pub struct TokenQuery {
    pub id: i32,
    pub chain_id: i64,
    pub token_id: String,
    pub collection_id: String,
    pub creator_address: String,
    pub collection_name: String,
    pub token_name: String,
    pub supply: i64,
    pub version: i64,
    pub royalty_points_numerator: i64,
    pub royalty_points_denominator: i64,
    pub metadata_uri: String,
    pub metadata_json: Option<String>,
    pub image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct TokenInsert {
    pub chain_id: i64,
    pub token_id: String,
    pub collection_id: String,
    pub creator_address: String,
    pub collection_name: String,
    pub token_name: String,
    pub supply: i64,
    pub version: i64,
    pub royalty_points_numerator: i64,
    pub royalty_points_denominator: i64,
    pub metadata_uri: String,
    pub metadata_json: Option<String>,
    pub image: Option<String>,
}

impl From<CurrentTokenData> for TokenInsert {
    fn from(v: CurrentTokenData) -> Self {
        Self {
            chain_id: Into::<u8>::into(ChainID::Aptos) as i64,
            token_id: v.token_data_id_hash,
            collection_id: String::from(""),
            token_name: v.name,
            creator_address: v.creator_address,
            collection_name: v.collection_name,
            supply: v.supply.to_i64().unwrap(),
            royalty_points_numerator: v.royalty_points_numerator.to_i64().unwrap(),
            royalty_points_denominator: v.royalty_points_denominator.to_i64().unwrap(),
            version: v.last_transaction_version,
            metadata_uri: v.metadata_uri,
            metadata_json: None,
            image: None,
        }
    }
}

pub fn query_collections(
    db: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<i64> {
    use crate::schema::tokens::dsl::*;

    let a: TokenQuery = tokens::table()
        .filter(chain_id.eq(Into::<u8>::into(ChainID::Aptos) as i64))
        .order(version.desc())
        .first(db)
        .map_err(|e| anyhow!(e))?;

    Ok(a.version)
}

pub fn query_collection_by_hash_id(
    db: &mut PooledConnection<ConnectionManager<PgConnection>>,
    hash_id: &str,
) -> Result<TokenQuery> {
    use crate::schema::tokens::dsl::*;

    tokens::table()
        .filter(collection_id.eq(hash_id))
        .first(db)
        .map_err(|e| anyhow!(e))
}

pub fn insert_collection(
    db: &mut PooledConnection<ConnectionManager<PgConnection>>,
    c: TokenInsert,
) -> Result<()> {
    if query_collection_by_hash_id(db, &c.collection_id).is_err() {
        diesel::insert_into(tokens::table)
            .values(&c)
            .execute(db)?;
    } else {
	use crate::schema::tokens::dsl::*;
	
        diesel::update(tokens.filter(token_id.eq(&c.token_id)))
            .set(version.eq(c.version))
            .execute(db)?;
    }
    
    Ok(())
}
