use anyhow::{anyhow, Result};
use bigdecimal::ToPrimitive;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::schema::*;

#[derive(Insertable)]
#[diesel(table_name = domains)]
pub struct TokenInsert {
    pub chain_id: i64,
    pub hash_id: String,
    pub domain: Option<String>,
    pub sub_domain: Option<String>,
    pub description: Option<String>,
    pub supply:i64,
    pub version:i64,
    pub metadata_uri:String,
    pub metadata_json:Option<String>,
    pub image:Option<String>,
    pub expired_time: chrono::NaiveDateTime,
    pub regest_time: chrono::NaiveDateTime,
    pub onwer_address:String,
}
