use std::fs::File;
use std::io::prelude::*;

pub fn print_blockchain () {
    let mut file = File::open("blockchain.json").expect("Blockchain corrupted");
    let mut blockchain = String::new();
    file.read_to_string(&mut blockchain).expect("Cannot read Blockchain");
    println!("{}", blockchain);
}