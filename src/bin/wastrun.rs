#![deny(warnings)]

extern crate warthog;

use std::{env, fs, process};

use warthog::parser;

fn main() {
    // Arg 0 is the executable name
    let arg0 = env::args().nth(0).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() > 0 {
        let script = {
            let mut file = fs::File::open(&args[0]).expect("failed to open file");
            match parser::parse(&mut file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("syntax error: {}{}", args[0], e);
                    process::exit(1);
                }
            }
        };

        let mut exit_code = 0;
        for result in script.run() {
            println!("{}", result);
            if !result.is_success() {
                exit_code = 1;
            }
        }

        process::exit(exit_code);
    } else {
        eprintln!("Usage: {} <wasm file>", arg0);
        process::exit(1);
    }
}
