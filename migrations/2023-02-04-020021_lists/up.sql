-- Your SQL goes here

CREATE TABLE IF NOT EXISTS lists
(
    id serial,
    chain_id bigint NOT NULL,
    token_id character varying COLLATE pg_catalog."default" NOT NULL,
    seller_address text NOT NULL,
    saller_value bigint NOT NULL,
    seller_coin_id int NOT NULL,
    seller_end_time timestamp NOT NULL,
    
    -- how to setup mutilt cointype peyment. ?
    created_at timestamp DEFAULT now(),
    updated_at timestamp DEFAULT now(),
    CONSTRAINT "PK_LIST_ID" PRIMARY KEY (id)
)
