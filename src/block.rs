use std::fmt::{ self, Debug, Formatter };
use crate::hash_function::HashFunction;
use super::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub prev_block_hash: Blockhash,
    pub hash: Blockhash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}", &self.index, &hex::encode(&self.hash), &self.timestamp, &self.payload)
    }
}

impl Block {
    pub fn new(index: u32,
        timestamp: u128,
        prev_block_hash: Blockhash,
        nonce: u64,
        payload: String
    ) -> Self {

        Self {
            index, 
            timestamp, 
            prev_block_hash, 
            hash: vec![0; 32], 
            nonce, 
            payload  
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
        bytes.extend(self.payload.as_bytes());

        bytes
    }
}