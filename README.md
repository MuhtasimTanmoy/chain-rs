# Experimental Blockchain in Rust

A blockchain implementation written for educational purpose in the Rust programming language that is in the experimental alpha stage.
The project's main objective is to enhance understanding of Rust programming language fundamentals and best practices by 
constructing a system that involves distributed and peer-to-peer networking. The project yet lacks proper 
documentation, and several implementations are inexperienced and flawed.


### Quickstart

```
## First create a wallet
cargo run createwallet

Output:
success: address 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS


## Print the addresses in wallet
cargo run listaddresses

addresses: 
3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS


## Create the chain
## first user will have the coinbase transaction, and genesis block will store that
cargo run create 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS

Output:
Success


## Print the chain
cargo run printchain

Sample output in analysis.md


## Get balance of the user
cargo run getbalance 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS
Balance of '3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS'; 100 


## Send money to from one user to another
cargo run send 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS  36M6fHwAame68se5hhqG1j2kXRujFCDQhN 10
```

### Features

The project's functionality is currently limited, 
and the following tasks are on the `to-do` list to improve it. 
Feel free to give any idea/ suggestions on improvement.

- [x] Create working chain of blocks
- [x] Save blocks on SQLite database
- [x] Set up command line interface
- [x] Set up transaction
- [x] Set up genesis block, coinbase transaction
- [x] Set up wallet
- [x] Proof of work mining
- [x] Send transaction from one account to another with hash of public key as address
- [x] Add signature for transaction input access control
- [x] Sending transaction, signing with private key
- [x] Use merkle hash for block fingerprint
- [x] Optimize transaction access, currently traverses the complete chain
- [x] Add networking layer
- Add caching layer
- Restructure the project to use different crate for each functionality
- Add spec for communication protocol
- Look at transport medium, grpc?
- Add separate client to interact with full node

### Docs
Courtesy to ChatGPT project for assistance with wordings in this documentation.

- [Wiki](./docs/wiki.md)
- [Doc](./docs/doc.md)
- [Messages](./docs/message.md)
- [Networking](./docs/networking.md)
- [Command Line](./docs/cli.md)
- [Analysis](./docs/analysis.md)
- [Todo](./docs/TODO.md)
- [Changelog](./docs/changelog.md)

### Concepts

Two base primitives in the blockchain system
1. Transaction
2. Block

Nodes collect new transactions into a block, hash them into a hash
tree (`merkle root hash`), and scan through `nonce` values to make the block's hash satisfy
proof-of-work requirements.

When a miner solves the `proof-of-work`, it broadcasts the block
to network nodes and if the block is valid it is added to the blockchain. The first transaction in
the block is the `coinbase transaction` that creates a new coin owned by the creator of the
block.

A node is responsible for processing, validating, and relaying the block and its transactions. A
node is distinct on the network from miners and wallets.

There will be three types of node
- Central Node
    - Other nodes connect to it, it sends data to others
- Miner Node
    - Gathers transactions in memory pool and after reaching a threshold mine a new block and broadcast to the network
- Wallet Node
    - Send coins between wallets, like [SPV](https://learn.saylor.org/mod/page/view.php?id=36320) Node

Nodes will communicate with each other with following types of messages
- `BlockMessage`
    - Passing block info from one node another after successful mining
- `GetMessage`
    - One node requesting block info from another node
- `DataRequestMessage`
    - Requests three types of data
        - Block
        - Data
        - Transaction
- `InventoryMessage`
    - Sent in reply to a “getblocks” message or “mempool” message. Refer: [link](https://developer.bitcoin.org/reference/p2p_networking.html)
- `TxMessage`
    - Sent after successful mining
- `VersionMessage`
    - Nodes version transfer message

## References
- Implementation Testing Playground
    - https://github.com/MuhtasimTanmoy/rust-playground
- Wiki
  - https://wiki.bitcoinsv.io
- Bitcoin Book
  - https://github.com/bitcoinbook/bitcoinbook
- https://github.com/rust-in-blockchain/awesome-blockchain-rust
- https://github.com/MitchellTesla/Alfa-ROMio
- https://input-output-hk.github.io/jormungandr
- https://github.com/JoshOrndorff/blockchain-from-scratch
- https://github.com/jean553/rust-blockchain
- https://github.com/cosme12/SimpleCoin
