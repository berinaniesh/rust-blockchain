use sha2::{Sha256, Digest};
struct Block {
    block_number: u64,
    block_data: String,
    previous_block_hash: String,
}
impl Block {
    pub fn block_to_string() -> String {
       block_number.to_string() + block_data + previous_block_hash
    }
    pub fn hash_block() -> String {
        let mut hash = Sha256::new();
        hash.finalize_fixed(block_to_string())
    }
}

pub fn testing() {
    let block_1 = Block {
        block_number: 1,
        block_data: String::from("First block"),
        previous_block_hash: String::from("0")
    };
    println!()
} 