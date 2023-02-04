// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Int4,
        chain_id -> Int8,
        slug -> Nullable<Text>,
        collection_id -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        description -> Varchar,
        supply -> Int8,
        version -> Int8,
        metadata_uri -> Varchar,
        floor_sell_id -> Nullable<Int4>,
        floor_sell_value -> Nullable<Numeric>,
        floor_sell_coin_id -> Nullable<Int4>,
        best_bid_id -> Nullable<Int4>,
        best_bid_value -> Nullable<Numeric>,
        best_bid_coin_id -> Nullable<Int4>,
        verify -> Bool,
        last_metadata_sync -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lists (id) {
        id -> Int4,
        chain_id -> Int8,
        token_id -> Varchar,
        seller_address -> Text,
        saller_value -> Int8,
        seller_coin_id -> Int4,
        seller_end_time -> Timestamp,
        top_buyer -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        chain_id -> Int8,
        token_id -> Varchar,
        collection_id -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        token_name -> Varchar,
        attributes -> Nullable<Text>,
        supply -> Int8,
        version -> Int8,
        payee_address -> Varchar,
        royalty_points_numerator -> Int8,
        royalty_points_denominator -> Int8,
        metadata_uri -> Varchar,
        metadata_json -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(collections, lists, tokens,);
