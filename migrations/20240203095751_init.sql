CREATE TABLE transactions (
    hash bytea,
    "from" bytea,
    "to" bytea,
    amount BIGINT,
    block_id BIGINT,
    nonce BIGINT,
    status VARCHAR(256) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE blocks (
    id BIGINT,
    hash bytea,
    parent_hash bytea,
    merkle_root bytea,
    produced_by bytea,
    nonce BIGINT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE accounts (
    address bytea,
    balance BIGINT,
    nonce BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE merkle_tree (
    block_id BIGINT,
    root bytea,
    hashes bytea[],
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);