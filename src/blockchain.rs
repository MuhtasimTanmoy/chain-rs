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

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), failure::Error> {
        let new_block = Block::new(transactions, self.curr_hash.clone(), 0)?;
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("block_head_hash", new_block.get_hash().as_bytes())?;
        self.curr_hash = new_block.get_hash();
        Ok(())
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.curr_hash.clone(),
            bc: &self,
        }
    }

    pub fn sign_transacton(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<(), failure::Error> {
        let prev_TXs = self.get_prev_txs(tx)?;
        tx.sign(private_key, prev_TXs)?;
        Ok(())
    }

    fn get_prev_txs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>, failure::Error> {
        let mut prev_TXs = HashMap::new();
        for vin in &tx.input {
            let prev_TX = self.find_transaction(&vin.txid)?;
            prev_TXs.insert(prev_TX.id.clone(), prev_TX);
        }
        Ok(prev_TXs)
    }

    pub fn find_transaction(&self, id: &str) -> Result<Transaction, failure::Error> {
        for b in self.iter() {
            for tx in b.get_transaction() {
                if tx.id == id {
                    return Ok(tx.clone());
                }
            }
        }
        Err(format_err!("Transaction is not found"))
    }

    /// VerifyTransaction verifies transaction input signatures
    pub fn verify_transacton(&self, tx: &mut Transaction) -> Result<bool, failure::Error> {
        let prev_txs = self.get_prev_txs(tx)?;
        tx.verify(prev_txs)
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::Blockchain;
    use crate::transaction::Transaction;

    #[test]
    fn find_utxos() {
        let mut bc = Blockchain::new().unwrap();
        let tx = Transaction::new( "34KTu4aiqTaJ1vdYzHS3xGXL1eHkAuXred",
                                              "35gt2cJbbmLFLqWtEkCU5yMrECUoccNGy4",
                                              10, &bc).unwrap();
        bc.add_block(vec![tx]).unwrap();
        println!("success!");
    }
}
