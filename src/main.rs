use rust_blockchain::{
    block::Block, 
    hash_function::HashFunction, 
    blockchain::Blockchain, 
    time_now
};

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, time_now(), vec![0; 32], 0, "genesis block!".to_string(), difficulty);

    block.mine();

    println!("Genesis block: {:?}", &block);

    let mut last_hash = block.hash().clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(i, time_now(), last_hash, 0, "next block".to_string(), difficulty);

        block.mine();
        println!("Mined block: {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
        
        println!("Verify: {}", &blockchain.verify());
    }

        // ERRORS
    // blockchain.blocks[4].index = 5;   // Index mismatch
    // blockchain.blocks[7].hash[7] += 1;   // Hash mismatch
    // blockchain.blocks[8].data = "data changed".to_string();    // data mismatch
    // blockchain.blocks[9].prev_block_hash[17] = 8;  // difficulty fail
    // println!("Verify: {}", &blockchain.verify());

}