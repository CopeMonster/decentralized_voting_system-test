use dvslib::{BlockHash, u32_bytes, u128_bytes};
use crate::{vote::Vote, hashable::Hashable};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub vote: Vote,
}

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash, vote: Vote) -> Block {
        Block {
            index,
            timestamp,
            hash: vec![0; 16],
            prev_block_hash,
            vote
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(u32_bytes(self.index));
        bytes.extend(u128_bytes(self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(self.vote.bytes());

        bytes
    }
}