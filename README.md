# Blockchain DEMO

This project is a simple blockchain implementation with REST API. It supports basic operations like transferring tokens, mining blocks,
getting various information about the state of blockchain and verifying transactions.

# General specification

Project was designed to be as simple as possible and to demonstrate the basic principles of blockchain technology. It is not intended to be used in production.
Main features are inspired by Ethereum and Bitcoin blockchains.

## Glossary
* **Hash** - a 32-bytes number that is generated from some data. Hash function is deterministic and works in "one way", which means that you can't get initial data from its hash.
Different data produces different hashes, but the same data always produces the same hash.
* **Private key** - a 32-bytes number that is used to sign transactions and generate public key.
* **Public key** - a 33-bytes number that is used to verify signatures and generate account address.
* **Account address** - same thing as public key, may be used as its compressed version.
* **Nonce** - a number that is used to prevent replay attacks. It is incremented after each transaction. Nonces are stored in the blockchain, user should use the latest nonce for each transaction.
* **Transaction** - a record in the blockchain that represents a transfer of tokens from one account to another. It contains sender, receiver, amount, nonce and signature.
* **Block** - a record in the blockchain that contains a list of transactions, a nonce and a hash of the previous block. It is used to store transactions and to prevent tampering with the blockchain.
* **Merkle tree** - a binary tree that is used to store transactions in a block. It is used to verify transactions and to generate merkle proofs.
* **Merkle proof** - a list of hashes that is used to verify that a transaction is included in a block. It is used to prove that a transaction is valid and was included in a block.
* **Target** - a number that represents the difficulty for mining the block. It is used to adjust the difficulty of mining, prevent DDOS attacks and guarantee blockchain's workability for miners.
* **Mining** - a process of finding a nonce for a block that satisfies the target. It is used to add new blocks to the blockchain and to reward miners with new tokens.
* **Signature** - a 65-bytes number that is used to verify that a transaction was signed by the owner of the account.

## Cryptography
* Project uses secp256k1 elliptic curve for cryptography. It is the same curve that is used in Bitcoin and Ethereum blockchains. Public and private keys are generated over this curve.
* For signing and verifying signatures projects uses ECDSA algorithm. It is a standard algorithm for signing and verifying messages over elliptic curves.
* For hashing project uses `blake2` algorithms.

## Database
Project uses PostgreSQL as a database. It is used to store blocks, transactions and accounts.
Database contains 4 tables:
* `blocks` - contains information about blocks. It includes block number, hash, previous block hash, nonce, miner address, timestamp and merkle root.
* `transactions` - contains information about transactions. It includes transaction hash, sender, receiver, amount, nonce, signature and block number.
* `accounts` - contains information about accounts. It includes account address, nonce and balance.
* `merkle_nodes` - contains information about merkle tree nodes.

## Configuration
While keeping project simple, it is possible to configure some parameters of the blockchain. Configuration is stored in 
`.env` file and can be changed by the user. Configuration includes:
* `SERVER_PORT` - port of the server
* `DB_URL` - URL of the database.
* `NODE_MODE` - mode of the blockchain. Can be `test` or `full`. In `test` mode, blocks are mined automatically. In `full` mode, blocks should be mined manually.
* `MERKLE_TREE_SIZE` - size of the merkle tree. It is used to store transactions in a block. Should be a power of 2.
* `BASE_REWARD` - reward for mining a block. It is a number of tokens that miner receives for mining a block.
* `BLOCK_SIZE` - maximum number of transactions in a block. It is used to prevent DDOS attacks and to keep the blockchain size reasonable.
* `TARGET` - initial target for the first block. It is used to adjust the difficulty of mining. Should be a 256-bit number.

# API description
## GET methods
* `/get_balance/{account_id}` - Returns balance of an account
* `/get_transaction/{tx_hash}` - Returns transaction information by transaction hash
* `/get_transactions/{account_id}` - Returns list of transactions for an account
* `/get_nonce/{account_id}` - Returns current nonce for account
* `/get_block_by_hash/{block_hash}` - Returns block information by block hash
* `/get_block_by_id/{block_id}` - Returns block information by block id
* `/get_proof/{tx_hash}` - Returns merkle proof for a transaction
* `/get_target` - Returns current target for the next block
* `/block_height` - Returns current block height

## POST methods
* `/transfer` - Transfers tokens from one account to another.

Method data:
```json
{
    "from": "0x123",
    "to": "0x456",
    "amount": 100,
    "nonce": 1,
    "signature": "0x123"
}
```
* `/try_mine` - Tries to mine the latest block. If `test` mode is enabled, it will mine the block automatically.
Otherwise, it will return `Success` if the block was mined successfully, and `Fail` otherwise. If block was mined successfully, 
it will be added to the blockchain, miner will receive a reward and all transactions in the block will be executed.

Method data:
```json
{
    "miner": "0x123",
    "block_number": 1,
    "block_nonce": 1,
    "nonce": 1,
    "signature": "0x123"
}
```
* `/set_target` - Sets target for the next block. Only accessible if `test` mode is enabled. Target is a 256-bit number that represents the difficulty of the block.
* `/mint` - Mints new coins to the account **without making a transaction**. Only accessible if `test` mode is enabled.

Method data:
```json
{
    "to": "0x123",
    "amount": 100
}
```

## Test methods
* `/get_mode` - Returns current mode of the blockchain. Can be `test` or `full`.
* `/set_mode` - Enables or disables test mode. In test mode, blocks are mined automatically.
Accepts variants `test` and `full`.

Method data:
```json
{
    "mode": "test"
}
```
* `/generate_sig` - Generates signature for a given message and private key.
Message should be a hash of the transaction encoded in hex format.
Returns signature in hex format. Generates ECDSA signature over secp256k1 curve.

Method data:
```json
{
    "message": "0x123",
    "private_key": "0x123"
}
```
* `/verify_sig` - Verifies signature for a given message, public key and signature.
Returns `Success` if the signature is valid, and `Fail` otherwise.

Method data:
```json
{
  "message": "0x123",
  "public_key": "0x123",
  "signature": "0x123"
}
```

* `/get_pub_key/{private_key}` - Returns public key for a given private key.
* `/verify_proof` - Verifies merkle proof for a given transaction.
Returns `Success` if the proof is valid, and `Fail` otherwise.

Method data:
```json
{
    "tx_hash": "0x123",
    "proof": [
      {
        "hash": "0x123",
        "parent_direction": "left"
      },
      {
        "hash": "0x123",
        "parent_direction": null
      }
    ],
    "root": "0x123"
}
```


