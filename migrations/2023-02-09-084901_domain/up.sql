-- Your SQL goes here

CREATE TABLE IF NOT EXISTS domains
(
    id serial,
    chain_id bigint NOT NULL,
    hash_id text,
    domain character varying COLLATE pg_catalog."default" NOT NULL,
    sub_domain character varying COLLATE pg_catalog."default" NOT NULL,
    description character varying COLLATE pg_catalog."default" NOT NULL,
    supply bigint NOT NULL,
    version bigint NOT NULL,
    metadata_uri varchar NOT NULL,
    metadata_json varchar,
    image varchar,
    expired_time timestamp,
    regest_time timestamp,
    onwer_address varchar,
    created_at timestamp DEFAULT now(),
    updated_at timestamp DEFAULT now(),
    PRIMARY KEY (version,id)
);

CREATE UNIQUE INDEX ttyp_index ON domains USING btree (domain, sub_domain)