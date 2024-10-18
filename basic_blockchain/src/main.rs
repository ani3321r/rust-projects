#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut miner_addr).expect("Failed to read input");

    print!("Difficulty: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut difficulty).expect("Failed to read input");

    let diff = difficulty.trim().parse::<u32>().expect("We need an integer value");
    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu:");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("0. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        println!("");

        match choice.trim().parse::<u32>() {
            Ok(0) => {
                println!("Exiting...");
                process::exit(0);
            },
            Ok(1) => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender's address: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut sender).expect("Failed to read input");

                print!("Enter receiver's address: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut receiver).expect("Failed to read input");

                print!("Enter amount: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut amount).expect("Failed to read input");

                let amount_val = amount.trim().parse::<f32>().expect("Failed to parse amount");

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount_val);
                if res {
                    println!("Transaction added");
                } else {
                    println!("Transaction failed");
                }
            },
            Ok(2) => {
                println!("Generating Block");
                let res = chain.generate_new_block();
                if res {
                    println!("Block generated successfully");
                } else {
                    println!("Block generation failed");
                }
            },
            Ok(3) => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut new_diff).expect("Failed to read input");

                let new_diff_val = new_diff.trim().parse::<u32>().expect("Failed to parse new difficulty");
                let res = chain.update_difficulty(new_diff_val);
                if res {
                    println!("Updated difficulty");
                } else {
                    println!("Failed to update difficulty");
                }
            },
            Ok(4) => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut new_reward).expect("Failed to read input");

                let new_reward_val = new_reward.trim().parse::<f32>().expect("Failed to parse new reward");
                let res = chain.update_reward(new_reward_val);
                if res {
                    println!("Updated reward");
                } else {
                    println!("Failed to update reward");
                }
            },
            _ => {
                println!("Please select a valid option");
            }
        }
    }
}
