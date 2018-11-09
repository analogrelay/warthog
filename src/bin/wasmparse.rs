extern crate warthog;

use std::{env, fs, process};

use warthog::parser;

fn main() {
    // Arg 0 is the executable name
    let arg0 = env::args().nth(0).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() > 0 {
        let file = &args[0];
        run(file);
    } else {
        eprintln!("Usage: {} <wasm file>", arg0);
        process::exit(1);
    }
}

pub fn run(file: &str) {
    let mut file = fs::File::open(file).unwrap();
    let commands = parser::parse(&mut file).unwrap();
    println!("{:?}", commands);
}
