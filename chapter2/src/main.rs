use std::env;

use file_ops::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    for (ix, arg) in args.iter().enumerate() {
        println!("({}) => {}", ix, arg);
    }

    if args.len() < 2 {
        panic!("required file name ommitted!");
    }
    read_file(&args[1]);
}

mod forloops;
mod greplite;
pub mod file_ops;