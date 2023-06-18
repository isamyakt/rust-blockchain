use crate::Blockhash;
use std::fmt::{ self, Debug, Formatter };
use crate::{
    hash_function::HashFunction, 
    bytes::{
        difficulty_bytes_as_u128, 
        u128_bytes, 
        u64_bytes, 
        u32_bytes
    }
};


pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub prev_block_hash: Blockhash,
    pub hash: Blockhash,
    pub nonce: u64,
    pub data: String,
    pub difficulty: u128
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block[{}]: {} time: {} data: {} nonce: {}", 
            &self.index, 
            &hex::encode(&self.hash), 
            &self.timestamp, 
            &self.data, 
            &self.nonce
        )
    }
}

impl Block {
    pub fn new(index: u32,
        timestamp: u128,
        prev_block_hash: Blockhash,
        nonce: u64,
        data: String,
        difficulty: u128
    ) -> Self {

        Self {
            index, 
            timestamp, 
            prev_block_hash, 
            hash: vec![0; 32], 
            nonce, 
            data, 
            difficulty  
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;

            let hash = self.hash();

            if check_difficult(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl HashFunction for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.data.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficult(hashx: &Blockhash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hashx)
}