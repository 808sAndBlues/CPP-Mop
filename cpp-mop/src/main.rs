use std::io;
use std::io::Write;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_line_args: Vec<String> = extracted_args(&args, args.len());
}

fn extracted_args(args: &Vec<String>, length: usize) -> Vec<String> {
   return args[..length].to_vec()
}

fn debug_vec(args: &Vec<String>) {
    for i in 0..args.len() {
        println!("{}", args[i]);
    }
}

