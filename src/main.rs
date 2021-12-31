mod print_blockchain;
mod mine;

use std::env;
use std::fs;

fn main() {
    println!("Hello there!\nWhat do you want to do?\n1. Print blockchain\n2. Start mining\n3. Reset blockchain\n4. Quit");
    print_blockchain::print_blockchain();

}
