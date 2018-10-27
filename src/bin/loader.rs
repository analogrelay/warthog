extern crate warthog;

use std::{env, fs, process};

use warthog::{module::Module, reader::Reader, runtime::Host};

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
    // Create a host
    let mut host = Host::new();

    // Load the module
    let module = {
        // Close the file once we're done loading
        let file = fs::File::open(file).unwrap();
        let reader = Reader::new(file);
        Module::load(reader).unwrap()
    };

    // Instantiate the module
    let instance = host.instantiate(module);

    // Dump the host
    println!("Host information:");
    println!("  Functions:");
    for (i, func_inst) in host.funcs().iter().enumerate() {
        if func_inst.is_local() {
            println!("  * {:04} {} {}", i, func_inst.typ(), func_inst.module().unwrap());
        } else {
            println!("  * {:04} {} <Host Function>", i, func_inst.typ())
        }
    }

    println!("Instance information:");
    println!("  Functions:");
    for (i, func_addr) in instance.funcs().iter().enumerate() {
        println!("  * {:04} {}", i, func_addr);
    }
}
