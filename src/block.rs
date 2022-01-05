use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};

#[derive(Hash)]
struct Block {
    block_number: u64,
    block_data: String,
    previous_block_hash: String,
}
impl Block {
    pub fn block_to_string(&self) -> String {
       let mut ans: String = String::new();
       ans.push_str(self.block_number.to_string().as_str());
       ans.push_str(self.block_data.as_str());
       ans.push_str(self.previous_block_hash.as_str());
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
        block_number: 1,
        block_data: String::from("First block"),
        previous_block_hash: String::from("0")
    };

    println!("{}", calculate_hash(&block_1))
    
}