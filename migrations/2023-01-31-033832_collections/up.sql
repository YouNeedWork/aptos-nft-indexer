-- Your SQL goes here
CREATE TABLE IF NOT EXISTS collections
(
    id integer NOT NULL DEFAULT 0,
    chain_id integer NOT NULL,
    collection_id character varying COLLATE pg_catalog."default" NOT NULL,
    creator_address character varying COLLATE pg_catalog."default" NOT NULL,
    collection_name character varying COLLATE pg_catalog."default" NOT NULL,
    description character varying COLLATE pg_catalog."default" NOT NULL,
    supply integer NOT NULL,
    version integer NOT NULL,
    metadata_uri character varying COLLATE pg_catalog."default" NOT NULL,
    metadata_json character varying COLLATE pg_catalog."default" NOT NULL,
    image character varying COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT "PK_ID" PRIMARY KEY (id)
)
