-- Your SQL goes here

CREATE TABLE IF NOT EXISTS coins
(
    id serial,
    chain_id bigint NOT NULL,
    symbol character varying COLLATE pg_catalog."default" NOT NULL,
    decimal_point character varying COLLATE pg_catalog."default" NOT NULL,
    logo character varying COLLATE pg_catalog."default" NOT NULL,
    PRIMARY KEY (chain_id, id)
)
