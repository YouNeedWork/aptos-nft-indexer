-- Your SQL goes here

-- Your SQL goes here

CREATE TABLE IF NOT EXISTS lists
(
    id serial,
    chain_id bigint NOT NULL,
    symbol character varying COLLATE pg_catalog."default" NOT NULL,
    decimal_point character varying COLLATE pg_catalog."default" NOT NULL,
)
