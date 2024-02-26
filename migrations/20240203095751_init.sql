CREATE TABLE transactions (
    hash bytea,
    "from" bytea,
    "to" bytea,
    amount BIGINT,
    block_id BIGINT,
    nonce BIGINT,
    status VARCHAR(256) DEFAULT 'pending',
    index_in_block BIGINT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE blocks (
    id BIGINT,
    hash bytea,
    parent_hash bytea,
    merkle_root bytea,
    produced_by bytea,
    nonce BIGINT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE accounts (
    address bytea,
    balance BIGINT,
    nonce BIGINT DEFAULT 0,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE merkle_nodes (
    block_id BIGINT,
    root bytea,
    node bytea,
    index BIGINT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);