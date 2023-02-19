use std::ptr::hash;
use std::time::SystemTime;
use bincode::serialize;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
use crate::utils::{DIFFICULTY, VERSION};

// Specs available at https://twohop.ventures/wp-content/uploads/2019/12/BSVSpec-Blocks-V1.0.pdf
#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    hash: String,
    hash_prev_block: String,
    transactions: String, // to be list of transactions
    nonce: i32,
    height: i32,
    version: i8,
    difficulty: u32,
}

impl Block {
    pub fn new(transactions: String, hash_prev_block: String, height: i32) -> Result<Block, failure::Error> {

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block = Block {
            timestamp,
            hash: "".to_string(),
            hash_prev_block,
            transactions,
            nonce: 0,
            height,
            version: VERSION,
            difficulty: DIFFICULTY,
        };

        Ok(block)
    }

    pub fn get_hash(&self) -> String {
        return self.hash.clone();
    }

    pub fn new_genesis_block() -> Block {
        Block::new(String::from("Gensis Block"), String::new(), 0).unwrap()
    }

    fn mine(&mut self) -> Result<(), failure::Error> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>, failure::Error> {
        let content = (
            self.hash_prev_block.clone(),
            self.transactions.clone(),
            self.timestamp,
            DIFFICULTY,
            self.nonce
        );
        let bytes = serialize(&content)?;
        Ok(bytes)
    }

    fn validate(&self) -> Result<bool, failure::Error> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(DIFFICULTY as usize, '0' as u8);
        Ok(&hasher.result_str()[0..DIFFICULTY as usize] == String::from_utf8(vec1)?)
    }

}