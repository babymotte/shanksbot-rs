use shanksbot_rs::shanks;
use std::{env, process::exit};

fn main() {
    match env::args().skip(1).next() {
        Some(number) => {
            if let Ok(number) = number.parse() {
                println!("{}", shanks(number));
            } else {
                eprintln!("Not a positive integer.");
                exit(2);
            }
        }
        None => {
            eprintln!("No number specified.");
            exit(1);
        }
    }
}
