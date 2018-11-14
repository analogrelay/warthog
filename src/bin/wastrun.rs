#![deny(warnings)]

extern crate warthog;

use std::{env, process};

fn main() {
    // Arg 0 is the executable name
    let arg0 = env::args().nth(0).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() > 0 {
        panic!("share code with spec_tests!");
    } else {
        eprintln!("Usage: {} <wasm file>", arg0);
        process::exit(1);
    }
}
