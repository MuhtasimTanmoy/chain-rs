use sled::IVec;
use crate::block::Block;
use crate::blockchain_itr::BlockchainIter;

#[derive(Debug, Clone)]
pub struct Blockchain {
    curr_hash: String,
    // curr_height: i32,
    pub(crate) db: sled::Db,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain, failure::Error> {
        let db = sled::open("block_store")?;
        let block_head = db.get("block_head_hash")?;
        let block_height = db.get("block_height")?;
        match block_head {
            Some(hash) => {
                let hash_str = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain{
                    curr_hash: hash_str,
                    // curr_height: block_height as i32,
                    db,
                })
            }
            None => {
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("block_head_hash", block.get_hash().as_bytes())?;
                // db.insert("block_height", IVec::from(0))?;
                let bc = Blockchain {
                    curr_hash: block.get_hash(),
                    // curr_height: 0,
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<(), failure::Error> {
        // self.curr_height += 1;
        let new_block = Block::new(data, self.curr_hash.clone(), 0)?;
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("block_head_hash", new_block.get_hash().as_bytes())?;
        // self.db.insert("block_height", self.curr_height)?;
        self.curr_hash = new_block.get_hash();
        Ok(())
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
        let mut b = Blockchain::new().unwrap();
        b.add_block("block 0".to_string()).ok();
        b.add_block("block 1".to_string()).ok();
        b.add_block("block 2".to_string()).ok();
        dbg!(b);
    }

    #[test]
    fn test_blockchain_DB() {
        let mut b = Blockchain::new().unwrap();
        // b.add_block("data 1".to_string());
        // b.add_block("data 2".to_string());
        // b.add_block("data 3".to_string());

        for item in b.iter() {
            println!("item {:?}",item)
        }
    }
}