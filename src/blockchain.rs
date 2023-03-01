use log::info;
use std::collections::HashMap;
use failure::format_err;

use crate::block::Block;
use crate::blockchain_itr::BlockchainIter;
use crate::transaction::Transaction;
use crate::txs::TXOutput;

const GENESIS_COINBASE_DATA: &str = "Some data for genesis block";

#[derive(Debug, Clone)]
pub struct Blockchain {
    curr_hash: String,
    pub(crate) db: sled::Db,
}

impl Blockchain {

    /// the last block hash is stored in DB
    /// after this value is accessed
    /// we can traverse through all subsequent blocks via hash_prev_block
    pub fn new() -> Result<Blockchain, failure::Error> {
        info!("open blockchain");

        let db = sled::open("data/blocks")?;
        let hash = db
            .get("block_head_hash")?
            .expect("Create chain should have created one genesis block");
        info!("Found block database");
        let last_block_hash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            curr_hash: last_block_hash.clone(),
            db,
        })
    }

    /// creates the genesis block with coinbase transaction
    /// persists in database
    pub fn create_blockchain(address: String) -> Result<Blockchain, failure::Error> {

        if let Err(e) = std::fs::remove_dir_all("data/blocks") {
            info!("bloks not exist to delete")
        }

        let db = sled::open("data/blocks")?;

        let coinbase_transaction = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis: Block = Block::new_genesis_block(coinbase_transaction);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("block_head_hash", genesis.get_hash().as_bytes())?;

        let bc = Blockchain {
            curr_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }

    /// serializes and inserts block into database
    /// updates the head_hash to point ot this latest block
    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(Block), failure::Error> {
        let new_block = Block::new(transactions, self.curr_hash.clone(), 0)?;
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("block_head_hash", new_block.get_hash().as_bytes())?;
        self.curr_hash = new_block.get_hash();
        Ok((new_block))
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.curr_hash.clone(),
            bc: &self,
        }
    }

    /// invoked when sending transaction
    /// gets previous transactions that are present in input of a transaction
    /// suppose some person get a, b, c transaction accumulating 90 token
    /// when sending 90 token to someone else the transaction input will
    /// contain a, b, c
    pub fn sign_transacton(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<(), failure::Error> {
        let prev_txs = self.get_prev_txs(tx)?;
        tx.sign(private_key, prev_txs)?;
        Ok(())
    }

    /// verify_transaction verifies transaction input signatures
    pub fn verify_transaction(&self, tx: &mut Transaction) -> Result<bool, failure::Error> {
        let prev_txs = self.get_prev_txs(tx)?;
        tx.verify(prev_txs)
    }

    fn get_prev_txs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>, failure::Error> {
        let mut prev_txs = HashMap::new();
        for vin in &tx.input {
            let prev_tx = self.find_transaction(&vin.txid)?;
            prev_txs.insert(prev_tx.id.clone(), prev_tx);
        }
        Ok(prev_txs)
    }

    /// traverses entire blockchain to get specific transaction
    pub fn find_transaction(&self, id: &str) -> Result<Transaction, failure::Error> {
        for b in self.iter() {
            for tx in b.get_transaction() {
                if tx.id == id {
                    return Ok(tx.clone());
                }
            }
        }
        // let tx = self.iter()
        //     .flat_map(|b| b.get_transaction().into_iter())
        //     .find(|tx| tx.id == id)
        //     .cloned()
        //     .ok_or_else(|| format_err!("Transaction not found"));

        Err(format_err!("Transaction is not found"))
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::Blockchain;
    use crate::transaction::Transaction;

    #[test]
    fn test_blockchain_in_memory() {
        // let mut b = Blockchain::new().unwrap();
        // b.add_block("block 0".to_string()).ok();
        // b.add_block("block 1".to_string()).ok();
        // b.add_block("block 2".to_string()).ok();
        // dbg!(b);
    }

    #[test]
    fn test_blockchain_db() {
        // let mut b = Blockchain::new().unwrap();
        // b.add_block("data 1".to_string());
        // b.add_block("data 2".to_string());
        // b.add_block("data 3".to_string());

        // for item in b.iter() {
        //     println!("item {:?}",item)
        // }
    }

    #[test]
    fn add_block() {
        // this test case should fail as pub key hash address is hardcoded, opt for something from wallet in future
        // let mut bc = Blockchain::new().unwrap();
        // let tx = Transaction::new( "34KTu4aiqTaJ1vdYzHS3xGXL1eHkAuXred", "35gt2cJbbmLFLqWtEkCU5yMrECUoccNGy4",10, &bc).unwrap();
        // bc.add_block(vec![tx]).unwrap();
        // println!("success!");
    }
}
