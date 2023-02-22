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

fn main() -> Result<(), failure::Error> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
