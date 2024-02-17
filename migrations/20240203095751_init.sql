CREATE TABLE transactions (
    hash bytea,
    "from" bytea,
    "to" bytea,
    amount BIGINT,
    block_id BIGINT,
    status cstring,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE blocks (
    id BIGINT,
    hash bytea,
    parent_hash bytea,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE accounts (
    address bytea,
    balance BIGINT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE merkle_tree (
    block_id BIGINT,
    root bytea,
    hashes bytea[],
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);