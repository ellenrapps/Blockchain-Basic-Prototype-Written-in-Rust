use sha2::{Sha256, Digest};

struct BlockInfo {
    previous_block_hash: Vec<u8>,
    current_block_hash: Vec<u8>,
    message: Vec<u8>,
}

struct Blockchain {
    blocks: Vec<BlockInfo>,
}

impl BlockInfo {
    fn calculate_block_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(&self.previous_block_hash);
        hasher.update(&self.message);
        self.current_block_hash = hasher.finalize().to_vec();
    }
}

fn new_block(data: &str, prev_block_hash: Vec<u8>) -> BlockInfo {
    let mut block_info = BlockInfo {
        previous_block_hash: prev_block_hash,
        current_block_hash: Vec::new(),
        message: data.as_bytes().to_vec(),
    };
    block_info.calculate_block_hash();
    block_info
}

fn genesis_block() -> BlockInfo {
    new_block("Genesis Block", Vec::new())
}

impl Blockchain {
    fn add_block(&mut self, data: &str) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = new_block(data, previous_block.current_block_hash.clone());
        self.blocks.push(new_block);
    }
}

fn new_blockchain() -> Blockchain {
    Blockchain {
        blocks: vec![genesis_block()],
    }
}

fn main() {
    let mut new_blockchain = new_blockchain();
    new_blockchain.add_block("Transaction 1 After Genesis");
    new_blockchain.add_block("Transaction 2 After Genesis");
    new_blockchain.add_block("Transaction 3 After Genesis");

    for (i, block_info) in new_blockchain.blocks.iter().enumerate() {
        println!("Block ID : {}", i);
        println!("Message : {:?}", String::from_utf8_lossy(&block_info.message));
        println!("Previous Block Hash: {:?}", hex::encode(&block_info.previous_block_hash));
        println!("Block Hash : {:?}", hex::encode(&block_info.current_block_hash));
    }
}
