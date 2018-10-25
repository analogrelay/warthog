extern crate warthog;

use std::{env, process};

fn main() {
    // To start with, we are building a module "dumper" so we can parse the module content

    // Arg 0 is the executable name
    let arg0 = env::args().nth(0).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() > 0 {
        let file = &args[0];
        warthog::run(file).unwrap();
    } else {
        eprintln!("Usage: {} <wasm file>", arg0);
        process::exit(1);
    }
}
