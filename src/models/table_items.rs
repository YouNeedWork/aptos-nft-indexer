use anyhow::Result;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Deserialize, Serialize};

use crate::schema;
use schema::*;

// key -> Text,
// transaction_version -> Int8,
// write_set_change_index -> Int8,
// transaction_block_height -> Int8,
// table_handle -> Varchar,
// decoded_key -> Jsonb,
// decoded_value -> Nullable<Jsonb>,
// is_deleted -> Bool,
// inserted_at -> Timestamp,
#[derive(Debug, Identifiable, Queryable, Deserialize, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = table_items)]
pub struct TableItem {
    pub key: String,
    pub transaction_version: i64,
    pub write_set_change_index: i64,
    pub transaction_block_height: i64,
    pub table_handle: String,
    pub decoded_key: serde_json::Value,
    pub decoded_value: Option<serde_json::Value>,
    pub is_deleted: bool,
    pub inserted_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(handle))]
#[diesel(table_name = table_metadatas)]
pub struct TableMetadata {
    pub handle: String,
    pub key_type: String,
    pub value_type: String,
    pub inserted_at: chrono::NaiveDateTime,
}

pub fn query_items_by_handle(
    mut db: PooledConnection<ConnectionManager<PgConnection>>,
    hash: &str,
) -> Result<Vec<TableItem>> {
    use crate::schema::table_items::dsl::*;

    table_items::table()
        .filter(table_handle.eq(hash))
        .load(&mut *db)
        .map_err(|e| e.into())
}
