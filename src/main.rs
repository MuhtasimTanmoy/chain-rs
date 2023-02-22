use crate::cli::Cli;

mod block;
mod utils;
mod blockchain;
mod blockchain_itr;
mod cli;
mod transaction;

fn main() -> Result<(), failure::Error> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
