use rust_blockchain::{block::Block, hash_function::HashFunction};

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, "genesis block!".to_string());

    println!("{:?}", &block);

    let hash = block.hash();

    println!("{:?}", &hash);

    block.hash = hash;

    println!("{:?}", block);
}