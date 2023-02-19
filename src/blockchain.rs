use crate::block::Block;
use crate::utils::DIFFICULTY;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block()]
        }
    }
    pub fn add_block(&mut self, data: String) -> Result<(), failure::Error> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new(data, prev.get_hash(), DIFFICULTY as i32)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new();
        b.add_block("genesis".to_string());
        b.add_block("block 1".to_string());
        b.add_block("block 2".to_string());
        dbg!(b);
    }
}