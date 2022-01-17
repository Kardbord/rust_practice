extern crate rust_practice;

use rust_practice::fizzbuzz;
use std::io::{self, Write};

fn main() {
    println!("Starting the fizzbuzz demo! Press Ctrl+c to exit.");
    loop {
        prompt_and_compute_and_print();
    }
}

fn prompt_and_compute_and_print() {
    println!("======================================================");
    let lower_bound: u32;
    loop {
        print!("Input the inclusive lower bound for the fizzbuzz loop: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        lower_bound = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!(
                    "Error parsing {} as u32, please input a valid number.",
                    input.trim()
                );
                continue;
            }
        };
        break;
    }

    let upper_bound: u32;
    loop {
        print!("Input the inclusive upper bound for the fizzbuzz loop: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        upper_bound = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!(
                    "Error parsing {} as u32, please input a valid number.",
                    input.trim()
                );
                continue;
            }
        };
        break;
    }

    println!("{}", fizzbuzz::fizzbuzz(lower_bound..upper_bound + 1));
}
