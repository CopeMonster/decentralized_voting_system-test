use dvslib::now;

use crate::{block::Block, vote::Vote, hashable::Hashable};

pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, vote: Vote) {
        let index = self.blocks.len() as u32;
        let timestamp = now();

        let prev_block_hash = if self.blocks.is_empty() {
            vec![0; 16]
        } else {
            self.blocks.last().unwrap().hash()
        };

        let mut block = Block::new(index, timestamp, prev_block_hash, vote);
        block.hash = block.hash();
        
        self.blocks.push(block)
    }

    pub fn validate(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.hash != block.hash() {
                return false;
            }

            if i != 0 && block.prev_block_hash != self.blocks[i - 1].hash {
                return false;
            }
        }

        true
    }
}