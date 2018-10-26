extern crate warthog;

use std::{env, process, fs};

use warthog::loader;

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
    // Load the module
    let module = {
        // Close the file once we're done loading
        let mut file = fs::File::open(file).unwrap();
        loader::load(&mut file).unwrap()
    };
    println!("Module loaded");
}