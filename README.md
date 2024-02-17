# Specification

# API description
## GET methods
* `/get_balance/{account_id}` - Returns balance of an account
* `/get_transaction/{tx_hash}` - Returns transaction information by transaction hash
* `/get_transactions/{account_id}` - Returns list of transactions for an account
* `/get_block_by_hash/{block_hash}` - Returns block information by block hash
* `/get_block_by_id/{block_id}` - Returns block information by block id
* `/get_proof/{tx_hash}` - Returns merkle proof for a transaction
* `/get_target` - Returns current target for the next block
* `/block_height` - Returns current block height

## POST methods
* `/transfer` - Transfers coins from one account to another.

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
* `/mint` - Mints new coins to the account. Only accessible if `test` mode is enabled.

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


