use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};
use std::time::{Duration, SystemTime};

pub const MINING_DIFFICULTY:u8 = 2; //two leading zeros. 

#[derive(Hash)]
struct Block {
    id: u64,
    data: String,
    previous_hash: String,
    timestamp: String,
    nonce: u64,

}
impl Block {
    pub fn block_to_string(&self) -> String {
       let mut ans: String = String::new();
       ans.push_str(self.id.to_string().as_str());
       ans.push_str(self.data.as_str());
       ans.push_str(self.previous_hash.as_str());
       ans
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()

}

pub fn testing() {

    let block_1 = Block {
        id: 1,
        data: String::from("First block"),
        timestamp: String::from("0"),
        previous_hash: 0;

    };

    println!("{}", calculate_hash(&block_1))
    
}