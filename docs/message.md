# Message

Nodes will communicate with each other with following types of messages
- Block Message
  - Passing block info from one node another after successful mining
- Get Message
  - One node requesting block info from another node
- Data Request Message
  - Requests three types of data
    - Block
    - Data
    - Transaction
- Inventory message
  - Sent in reply to a “getblocks” message or “mempool” message.
- Transaction Message
  - Sent after successful mining
- Version Message
  - Nodes version transfer message


## Reference
- https://developer.bitcoin.org/reference/p2p_networking.html