use rust_blockchain::{block::Block, hash_function::HashFunction};

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, "genesis block!".to_string(), 0x000fffffffffffffffffffffffffffff);

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);

    
}