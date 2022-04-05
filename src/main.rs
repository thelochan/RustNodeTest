use blockchainlib::*;

fn main () {
    let difficulty = 0x005fffffffffffffffffffffffffffff;

    let mut block = Block::new(0, now(), vec![0; 32], 0, "Rust block".to_owned(), difficulty);

    block.mine();
    println!("Mined Rust Block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=20 {
        let mut block = Block::new(i, now(), last_hash, 0, "New block".to_owned(), difficulty);

    block.mine();
    println!("Mined Hash {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.blocks.push(block);

    }
}