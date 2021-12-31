mod print_blockchain;
mod mine;

use std::io;

fn main() {
    println!("Hello there! What do you want to do?\n1. Print blockchain\n2. Start mining\n3. Reset blockchain\n4. Quit");
    
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "1" {
                    println!("Printing Blockchain");
                }
                else if input.trim() == "2" {
                    println!("Starting mining");
                }
                else if input.trim() == "3" {
                    println!("Resetting the ledger");
                }
                else if input.trim() =="4" {
                    println!("Good Bye!");
                    return;
                }
                else {
                    println!("Enter a valid option");
                }
            }
            Err(e) => {
            println!("Something went wrong, here is the error message:\n{}",e);
            return;
            }
        }
    }

}
