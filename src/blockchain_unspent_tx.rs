use log::info;
use std::collections::HashMap;
use failure::format_err;

use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::blockchain_itr::BlockchainIter;
use crate::transaction::Transaction;
use crate::txs::TXOutput;

impl Blockchain {
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
}