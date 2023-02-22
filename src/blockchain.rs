use std::collections::HashMap;
use log::info;
use sled::IVec;
use crate::block::Block;
use crate::blockchain_itr::BlockchainIter;
use crate::transaction::{Transaction, TXOutput};

const GENESIS_COINBASE_DATA: &str =
    "Some data for genesis block";

#[derive(Debug, Clone)]
pub struct Blockchain {
    curr_hash: String,
    // curr_height: i32,
    pub(crate) db: sled::Db,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain, failure::Error> {
        info!("open blockchain");

        let db = sled::open("data/blocks")?;
        let hash = db
            .get("block_head_hash")?
            .expect("Create chain should have created one genesis block");
        info!("Found block database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            curr_hash: lasthash.clone(),
            db,
        })
    }

    pub fn create_blockchain(address: String) -> Result<Blockchain, failure::Error> {
        info!("Creating new blockchain");

        let db = sled::open("data/blocks")?;
        info!("Creating new block database");
        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis: Block = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("block_head_hash", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            curr_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), failure::Error> {
        // self.curr_height += 1;
        let new_block = Block::new(transactions, self.curr_hash.clone(), 0)?;
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("block_head_hash", new_block.get_hash().as_bytes())?;
        // self.db.insert("block_height", self.curr_height)?;
        self.curr_hash = new_block.get_hash();
        Ok(())
    }


    /// FindUnspentTransactions returns a list of transactions containing unspent outputs
    /// Very inefficient now
    /// traverses the entire blockchain
    fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_TXs: Vec<Transaction> = Vec::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                // traversing all the output of all transactions
                for index in 0..tx.output.len() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.output[index].can_be_unlock_with(address) {
                        unspend_TXs.push(tx.to_owned())
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.input {
                        if i.can_unlock_output_with(address) {
                            match spent_TXOs.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }

        unspend_TXs
    }

    /// FindUTXO finds and returns all unspent transaction outputs
    pub fn find_UTXO(&self, address: &str) -> Vec<TXOutput> {
        let mut utxos = Vec::<TXOutput>::new();
        let unspend_TXs = self.find_unspent_transactions(address);
        for tx in unspend_TXs {
            for out in &tx.output {
                if out.can_be_unlock_with(&address) {
                    utxos.push(out.clone());
                }
            }
        }
        utxos
    }

    /// find_spendable_outputs will return the spendable amount and their index
    pub fn find_spendable_outputs (
        &self,
        address: &str,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;
        let unspend_TXs = self.find_unspent_transactions(address);

        for tx in unspend_TXs {
            for index in 0..tx.output.len() {
                if tx.output[index].can_be_unlock_with(address) && accumulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![index as i32]);
                        }
                    }
                    accumulated += tx.output[index].value;

                    if accumulated >= amount {
                        return (accumulated, unspent_outputs);
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.curr_hash.clone(),
            bc: &self,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_in_memory() {
        // let mut b = Blockchain::new().unwrap();
        // b.add_block("block 0".to_string()).ok();
        // b.add_block("block 1".to_string()).ok();
        // b.add_block("block 2".to_string()).ok();
        // dbg!(b);
    }

    #[test]
    fn test_blockchain_DB() {
        // let mut b = Blockchain::new().unwrap();
        // b.add_block("data 1".to_string());
        // b.add_block("data 2".to_string());
        // b.add_block("data 3".to_string());

        // for item in b.iter() {
        //     println!("item {:?}",item)
        // }
    }
}