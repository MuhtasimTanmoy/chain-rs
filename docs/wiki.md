# Wiki

Contains supporting links, docs, concepts

- Wiki
  - https://wiki.bitcoinsv.io
- Merkle Tree Implementation Concept
  - For a merkle tree with n items 
  - The size of the array would be 2n-1
  - The index of item i(start at 0) is i+n-1. 
  - For node at i, the index of its parent is (i-1)/2, the index of its sibling is (i+1)^1-1(^ is xor) and the indexes of its children are [2i+1, 2i+2].
  - https://medium.com/coinmonks/merkle-tree-a-simple-explanation-and-implementation-48903442bc08
- Bitcoin Book
  - https://github.com/bitcoinbook/bitcoinbook