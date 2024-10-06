use std::fmt::Debug;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};


#[derive(Clone, Debug)]
pub struct Block<TData: Debug> {
    pub hash: String,
    pub created_at: Duration,
    pub data: TData,
    pub previous_hash: String,
}

impl<TData: Debug > Block<TData> {
    pub fn new(data: TData, previous_hash: String) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let mut block = Block {
            hash: String::new(),
            created_at,
            data,
            previous_hash,
        };

        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(
            format!("{:?}{}{}{}", self.data, self.created_at.as_millis(), self.previous_hash, self.created_at.as_millis())
        );
        format!("{:x}", hasher.finalize())
    }
}