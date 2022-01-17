extern crate rust_practice;

use rust_practice::fibonacci;
use std::io::{self, Write};

fn main() {
    println!("Starting the fibonacci demo! Press Ctrl+c to exit.");
    loop {
        prompt_and_compute_and_print();
    }
}

fn prompt_and_compute_and_print() {
    println!("======================================================");
    let n: u32;
    loop {
        print!("Input the fibonacci number you would like to compute: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        n = match input.trim().parse() {
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

    let m: fibonacci::Method;
    loop {
        print!(
            "Select the method that should be used to compute the ficonacci({}): ",
            n
        );

        let r_opt = String::from(format!("{}", fibonacci::Method::Recursive as u16));
        let d_opt = String::from(format!("{}", fibonacci::Method::Dynamic as u16));
        print!("\n  ({}) {}", r_opt, fibonacci::Method::Recursive);
        print!("\n  ({}) {}", d_opt, fibonacci::Method::Dynamic);
        print!("\nSelection: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        m = match input.trim() {
            _ if r_opt == input.trim() => fibonacci::Method::Recursive,
            _ if d_opt == input.trim() => fibonacci::Method::Dynamic,
            _ => {
                eprintln!("Please input either \"{}\" or \"{}\"", r_opt, d_opt);
                continue;
            }
        };
        break;
    }

    println!("fibonacci({}) = {}", n, fibonacci::compute(n, m));
}
