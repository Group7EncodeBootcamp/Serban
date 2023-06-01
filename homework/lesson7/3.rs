/*
write a program in Rust that takes 3 strings as command line inputs and then prints the values.
You will need to look at the env module
 */

use std::env;
use std::process;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("This program accepts 3 arguments only");
        process::exit(1);
    }

    args.remove(0);

    println!("Arguments from the command line are: ");
    for argument in args {
        println!("{argument}");
    }
}