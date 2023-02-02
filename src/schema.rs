// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Int4,
        chain_id -> Int8,
        collection_id -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        description -> Varchar,
        supply -> Int8,
        version -> Int8,
        metadata_uri -> Varchar,
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
        supply -> Int8,
        version -> Int8,
        payee_address -> Varchar,
        royalty_points_numerator -> Int8,
        royalty_points_denominator -> Int8,
        metadata_uri -> Varchar,
        metadata_json -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(collections, tokens,);
