# CLI

Run the following command

```
// first user will have the coinbase transaction, and geensis block will store that
cargo run create first_user

cargo run printchain

// should print 100
cargo run getbalance first_user
Balance of '3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS'; 100 


cargo run addblock tanmoy // deprecated


cargo run send 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS  36M6fHwAame68se5hhqG1j2kXRujFCDQhN 10


cargo run createwallet
success: address 3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS


cargo run listaddress
addresses: 
3JGDNu6Pnuench1hoXyibBemfYjGT8RHRS
```