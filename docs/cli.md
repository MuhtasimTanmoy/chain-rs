# CLI

Run the following command

```
## first user will have the coinbase transaction, and geensis block will store that
cargo run create first_user

cargo run printchain

## should print 100
cargo run getbalance first_user

cargo run addblock tanmoy

cargo run send first_user second_user 10
```