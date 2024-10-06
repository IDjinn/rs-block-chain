
use sha2::{Digest, Sha256};
use std::fmt::Debug;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use block_chain::{Block};


#[derive(Clone, Debug)]
pub struct Transaction {
    pub previous_hash: String,
    pub hash: String,

    pub owner_id: usize,
    pub item_id: usize,
    pub origin_id: usize,
    pub quantity: usize,
    pub created_at: Duration,
}


impl Transaction {
    pub fn new(previous_hash: String, item_id: usize, origin_id: usize, owner_id: usize, quantity: usize) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let mut transaction = Transaction {
            previous_hash,
            hash: String::new(),
            owner_id,
            item_id,
            origin_id,
            quantity,
            created_at
        };

        transaction.hash = transaction.calculate_hash();

        transaction
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}{:?}{:?}{:?}{:?}", self.previous_hash, self.owner_id, self.item_id, self.quantity, self.created_at));
        format!("{:x}", hasher.finalize())
    }
}


const GENESIS_BLOCK: &str = "--GENESIS--";
const GENESIS_BLOCK_HASH: &str = "--GENESIS-HASH--";

fn main() {
    let genesis_block: Block<Vec<Transaction>> = Block::new(
        vec![],
        GENESIS_BLOCK_HASH.to_string(),
    );
    println!("Genesis Block: {:?}", genesis_block);

    let drop_white_bag = Transaction::new(
        String::new(),
        131265,
        213123,
        3242343242345,
        1);
    println!("first transaction: {:?}", drop_white_bag.clone());

    let drop_st_bag = Transaction::new(
        drop_white_bag.clone().hash,
        5968,
        213123,
        3242343242345,
        1);
    println!("second transaction: {:?}", drop_st_bag.clone());

    let second_block = Block::new(
        vec![drop_white_bag, drop_st_bag],
        genesis_block.hash.clone(),
    );
    println!("Second Block: {:?}", second_block);
}