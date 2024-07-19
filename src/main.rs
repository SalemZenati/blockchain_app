extern crate sha2;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use std::fmt::{self, Formatter, Display};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u32,
    timestamp: i64,
    previous_hash: String,
    hash: String,
    data: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &data);
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    fn calculate_hash(index: u32, timestamp: i64, previous_hash: &str, data: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, previous_hash, data);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

    fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().map_or("0".to_string(), |block| block.hash.clone());
        let block = Block::new(self.chain.len() as u32, previous_hash, data);
        self.chain.push(block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let previous_block = &self.chain[i - 1];
            let current_block = &self.chain[i];

            if current_block.hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.previous_hash,
                &current_block.data,
            ) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for block in &self.chain {
            writeln!(f, "{:?}", block)?;
        }
        Ok(())
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Block 1 Data".to_string());
    blockchain.add_block("Block 2 Data".to_string());

    println!("Blockchain valid: {}", blockchain.is_valid());
    println!("{}", blockchain);
}
