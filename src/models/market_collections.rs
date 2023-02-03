use anyhow::{anyhow, Result};
use bigdecimal::ToPrimitive;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use super::current_collection_datas::CurrentCollectionDataQuery;
use crate::schema::*;

use crate::ChainID;
// id serial,
// chain_id bigint NOT NULL,
// slug text,
// collection_id character varying COLLATE pg_catalog."default" NOT NULL,
// creator_address character varying COLLATE pg_catalog."default" NOT NULL,
// collection_name character varying COLLATE pg_catalog."default" NOT NULL,
// description character varying COLLATE pg_catalog."default" NOT NULL,
// supply bigint NOT NULL,
// version bigint NOT NULL,
// metadata_uri character varying COLLATE pg_catalog."default" NOT NULL,
// verify bool NOT NULL DEFAULT false,
// last_metadata_sync timestamp,
// created_at timestamp DEFAULT now(),
// updated_at timestamp DEFAULT now(),

#[derive(Queryable)]
#[diesel(table_name = collections)]
pub struct CollectionQuery {
    pub id: i32,
    pub chain_id: i64,
    pub slug: Option<String>,
    pub collection_id: String,
    pub creator_address: String,
    pub collection_name: String,
    pub description: String,
    pub supply: i64,
    pub version: i64,
    pub metadata_uri: String,
    pub verify: bool,
    pub last_metadata_sync: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = collections)]
pub struct CollectionInsert {
    pub chain_id: i64,
    pub slug: Option<String>,
    pub collection_id: String,
    pub creator_address: String,
    pub collection_name: String,
    pub description: String,
    pub supply: i64,
    pub version: i64,
    pub metadata_uri: String,
    pub verify: bool,
    pub last_metadata_sync: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<CurrentCollectionDataQuery> for CollectionInsert {
    fn from(v: CurrentCollectionDataQuery) -> Self {
        Self {
            chain_id: Into::<u8>::into(ChainID::Aptos) as i64,
            slug: None,
            collection_id: v.collection_data_id_hash,
            creator_address: v.creator_address,
            collection_name: v.collection_name,
            description: v.description,
            supply: v.supply.to_i64().unwrap(),
            version: v.last_transaction_version,
            metadata_uri: v.metadata_uri,
            verify: false,
            last_metadata_sync: None,
            created_at: None,
            updated_at: None,
        }
    }
}

pub fn query_collections(
    db: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<i64> {
    use crate::schema::collections::dsl::*;

    let res: CollectionQuery = collections::table()
        .filter(chain_id.eq(Into::<u8>::into(ChainID::Aptos) as i64))
        .order(version.desc())
        .first(db)
        .map_err(|e| anyhow!(e))?;

    Ok(res.version)
}

pub fn query_collection_by_hash_id(
    db: &mut PooledConnection<ConnectionManager<PgConnection>>,
    hash_id: &str,
) -> Result<CollectionQuery> {
    use crate::schema::collections::dsl::*;

    collections::table()
        .filter(collection_id.eq(hash_id))
        .first(db)
        .map_err(|e| anyhow!(e))
}

pub fn insert_collection(
    db: &mut PooledConnection<ConnectionManager<PgConnection>>,
    c: CollectionInsert,
) -> Result<()> {
    if query_collection_by_hash_id(db, &c.collection_id).is_err() {
        diesel::insert_into(collections::table)
            .values(&c)
            .execute(db)?;
    } else {
        use crate::schema::collections::dsl::*;
        diesel::update(collections.filter(collection_id.eq(&c.collection_id)))
            .set(version.eq(c.version))
            .execute(db)?;
    }

    Ok(())
}
