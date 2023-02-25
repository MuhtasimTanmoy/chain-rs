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
        self.db
            .insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db
            .insert("block_head_hash", new_block.get_hash().as_bytes())?;
        // self.db.insert("block_height", self.curr_height)?;
        self.curr_hash = new_block.get_hash();
        Ok(())
    }

    /// FindUnspentTransactions returns a list of transactions containing unspent outputs
    /// Very inefficient now
    /// traverses the entire blockchain
    fn find_unspent_transactions(&self, pub_key_hash: &[u8]) -> Vec<Transaction> {
        let mut spent_txos: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_txs: Vec<Transaction> = Vec::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                // traversing all the output of all transactions
                for index in 0..tx.output.len() {
                    if let Some(ids) = spent_txos.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.output[index].is_locked_with_key(pub_key_hash) {
                        unspend_txs.push(tx.to_owned())
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.input {
                        if i.uses_key(pub_key_hash) {
                            match spent_txos.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spent_txos.insert(i.txid.clone(), vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }

        unspend_txs
    }

    /// FindUTXO finds and returns all unspent transaction outputs
    pub fn find_utxo(&self, address: &[u8]) -> Vec<TXOutput> {
        let mut utxos = Vec::<TXOutput>::new();
        let unspend_txs = self.find_unspent_transactions(address);
        for tx in unspend_txs {
            for out in &tx.output {
                if out.is_locked_with_key(&address) {
                    utxos.push(out.clone());
                }
            }
        }
        utxos
    }

    /// find_spendable_outputs will return the spendable amount and their index
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;
        let unspend_txs = self.find_unspent_transactions(pub_key_hash);

        for tx in unspend_txs {
            for index in 0..tx.output.len() {
                if tx.output[index].is_locked_with_key(pub_key_hash) && accumulated < amount {
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

    pub fn sign_transacton(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<(), failure::Error> {
        let prev_TXs = self.get_prev_txs(tx)?;
        tx.sign(private_key, prev_TXs)?;
        Ok(())
    }

    fn get_prev_txs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>, failure::Error> {
        let mut prev_TXs = HashMap::new();
        for vin in &tx.input {
            let prev_TX = self.find_transacton(&vin.txid)?;
            prev_TXs.insert(prev_TX.id.clone(), prev_TX);
        }
        Ok(prev_TXs)
    }

    pub fn find_transacton(&self, id: &str) -> Result<Transaction, failure::Error> {
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
    fn find_utxos() {
        let mut bc = Blockchain::new().unwrap();
        let tx = Transaction::new( "34KTu4aiqTaJ1vdYzHS3xGXL1eHkAuXred", "35gt2cJbbmLFLqWtEkCU5yMrECUoccNGy4",10, &bc).unwrap();
        bc.add_block(vec![tx]).unwrap();
        println!("success!");
    }
}
