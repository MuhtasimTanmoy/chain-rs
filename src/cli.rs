use clap::{arg, Command};
use crate::blockchain::Blockchain;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli, failure::Error> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }
    pub fn run(&mut self) -> Result<(), failure::Error> {
        let matches = Command::new("cli")
            .version("0.1")
            .author("test")
            .about("test")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(
                Command::new("addblock")
                    .about("add a block in the blockchain")
                    .arg(arg!(<DATA>" 'the blockchain data'")),
            )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA"){
                self.addblock(String::from(c))?;
            } else {
                println!("Not printing testing lists...");
            }
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            self.print_chain();
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<(), failure::Error> {
        self.bc.add_block(data)
    }

    fn print_chain(&mut self) {
        for b in &mut self.bc.iter() {
            println!("block: {:#?}", b);
        }
    }
}