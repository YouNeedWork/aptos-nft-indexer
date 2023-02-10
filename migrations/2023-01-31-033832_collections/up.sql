-- Your SQL goes here

CREATE TABLE IF NOT EXISTS collections
(
    id serial,
    chain_id bigint NOT NULL,
    slug text,
    collection_id character varying COLLATE pg_catalog."default" NOT NULL,
    creator_address character varying COLLATE pg_catalog."default" NOT NULL,
    collection_name character varying COLLATE pg_catalog."default" NOT NULL,
    description character varying COLLATE pg_catalog."default" NOT NULL,
    supply bigint NOT NULL,
    version bigint NOT NULL,
    metadata_uri character varying COLLATE pg_catalog."default" NOT NULL,
    floor_sell_id int,
    floor_sell_value NUMERIC(78, 0),
    floor_sell_coin_id int,
    best_bid_id int,
    best_bid_value NUMERIC(78,0),
    best_bid_coin_id int,
    verify bool NOT NULL DEFAULT false,
    last_metadata_sync timestamp,
    created_at timestamp DEFAULT now(),
    updated_at timestamp DEFAULT now(),
    CONSTRAINT "PK_ID" PRIMARY KEY (id)
);

CREATE UNIQUE INDEX collection_id_index ON collections USING btree (collection_id)
