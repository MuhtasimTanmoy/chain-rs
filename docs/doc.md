## Blockchain from scratch

Two base primitives in the blockchain system:
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

## Reference
- [Bitcoin 101 - Merkle Roots and Merkle Trees](https://youtu.be/gUwXCt1qkBU)
- [Block Spec for Bitcoin](https://twohop.ventures/wp-content/uploads/2019/12/BSVSpec-Blocks-V1.0.pdf)