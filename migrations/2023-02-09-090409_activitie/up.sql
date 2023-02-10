-- Your SQL goes here

CREATE TABLE IF NOT EXISTS activities
(
    id serial,
    chain_id bigint NOT NULL,
    version BIGINT NOT NULL,
    event_account_address text NOT NULL,
    event_creation_number BIGINT NOT NULL,
    event_sequence_number BIGINT NOT NULL,
    collection_data_id_hash text NOT NULL,
    token_data_id_hash text NOT NULL,
    property_version NUMERIC NOT NULL,
    creator_address text NOT NULL,
    collection_name text NOT NULL,
    name text NOT NULL,
    transfer_type text NOT NULL,
    from_address text,
    to_address text,
    token_amount NUMERIC NOT NULL,
    coin_type TEXT,
    coin_amount NUMERIC,
    transaction_timestamp timestamp NOT NULL,
    created_at timestamp DEFAULT now(),
    updated_at timestamp DEFAULT now(),

    PRIMARY KEY (version, id)
);

CREATE INDEX ta_from_ttyp_index ON activities (from_address, transfer_type);
CREATE INDEX ta_to_ttyp_index ON activities (to_address, transfer_type);
CREATE INDEX ta_addr_coll_name_pv_index ON activities (
  creator_address,
  collection_name,
  name,
  property_version
);
CREATE UNIQUE INDEX event_index ON activities USING btree (version, 
event_account_address,
event_creation_number,
event_sequence_number
);
CREATE INDEX ta_tdih_pv_index ON activities (token_data_id_hash, property_version);
CREATE INDEX ta_version_index ON activities (version);
