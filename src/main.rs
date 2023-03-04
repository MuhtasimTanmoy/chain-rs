use crate::cli::Cli;

mod block;
mod blockchain;
mod blockchain_itr;
mod cli;
mod transaction;
mod txs;
mod utils;
mod wallet;
mod wallet_chain;
mod blockchain_unspent_tx;
mod unspent_tx_util;
mod crypto;
mod r#const;
mod mergetx;
mod node;
mod message;
mod parser_util;
mod message_sender;
mod message_parser;
mod node_util;

fn main() -> Result<(), failure::Error> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
