mod print_blockchain;
mod mining;
mod blockchain;

use std::io;

fn main() {
    println!("Hello there! What do you want to do?\n1. Print blockchain\n2. Start mining\n3. Reset blockchain\n4. Quit");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                    println!("Printing Blockchain");
                }
            "2" => {
                    println!("Starting mining");
                    blockchain::testing();
                }
            "3" => {
                    println!("Resetting the ledger");
                }
            "4" => {
                    println!("Good Bye!");
                    return;
                }
            _ => {
                    println!("Enter a valid option");
                }
            }
    }

}
