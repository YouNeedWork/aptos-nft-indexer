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
    verify bool NOT NULL DEFAULT false,
    last_metadata_sync timestamp,
    created_at timestamp DEFAULT now(),
    updated_at timestamp DEFAULT now(),
    CONSTRAINT "PK_ID" PRIMARY KEY (id)
)
