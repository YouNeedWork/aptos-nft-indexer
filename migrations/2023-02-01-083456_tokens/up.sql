-- Your SQL goes here

CREATE TABLE tokens (
    "id" serial,
    "chain_id" bigint NOT NULL,
    "token_id" varchar NOT NULL,
    "collection_id" varchar NOT NULL,
    "creator_address" varchar NOT NULL,
    "collection_name" varchar NOT NULL,
    "token_name" varchar NOT NULL,
    "attributes" text,
    "supply" bigint NOT NULL,
    "version" bigint NOT NULL,
    "payee_address" varchar NOT NULL,    
    "royalty_points_numerator" bigint NOT NULL,
    "royalty_points_denominator" bigint NOT NULL,
    "onwer_address" varchar,
    "metadata_uri" varchar NOT NULL,
    "metadata_json" varchar,
    "image" varchar,
    "created_at" timestamp DEFAULT now(),
    "updated_at" timestamp DEFAULT now(),    
    PRIMARY KEY ("id")
);
