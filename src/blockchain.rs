use crate::{block::{Block, self}, hash_function::HashFunction};

pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", &block.index, &i);
                return false;
            } else if !block::check_difficult(&block.hash(), block.difficulty) {
                println!("Difficulty faied");
                return false;
            } else if i != 0 {
                // check genesis block 
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time didn't increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("hash mismatch");
                    return false;
                }
            } else {
                // check genesis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_hash invalid");
                    return false;
                }
            }
        }
        true
    }
}