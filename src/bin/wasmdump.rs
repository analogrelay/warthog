#![deny(warnings)]

extern crate warthog;

use std::{env, fs, process};

use warthog::format::Module;

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
    let file = fs::File::open(file).unwrap();
    let m = Module::read(file).unwrap();

    println!("Module Version: {}", m.version());

    dump_types(&m);
    dump_imports(&m);
    dump_functions(&m);
    dump_exports(&m);
    dump_data(&m);
    dump_code(&m);
    dump_names(&m);
}

fn dump_types(m: &Module) {
    println!("Types:");
    for (i, typ) in m.types().iter().enumerate() {
        println!("* {:04} {}", i, typ);
    }
}

fn dump_imports(m: &Module) {
    println!("Imports:");
    for (i, import) in m.imports().iter().enumerate() {
        println!("* {:04} {}", i, import);
    }
}

fn dump_functions(m: &Module) {
    println!("Functions:");
    for (i, func) in m.funcs().iter().enumerate() {
        println!("* {:04} (type {})", i, func);
    }
}

fn dump_exports(m: &Module) {
    println!("Exports:");
    for (i, export) in m.exports().iter().enumerate() {
        println!("* {:04} {}", i, export);
    }
}

fn dump_data(m: &Module) {
    println!("Data:");
    for (i, item) in m.data().iter().enumerate() {
        println!("* {:04} {}", i, item);
    }
}

fn dump_code(m: &Module) {
    println!("Code:");
    for (i, item) in m.code().iter().enumerate() {
        print!("* {:04}", i);
        for local in item.locals().iter() {
            print!(" {}", local);
        }
        println!();
        for inst in item.body().iter() {
            println!("     {}", inst);
        }
    }
}

fn dump_names(m: &Module) {
    if let Some(names) = m.names() {
        println!("Names:");
        if let Some(module_name) = names.module_name() {
            println!("  Module Name: {}", module_name);
        }
        println!("  Functions:");
        for (id, func) in names.funcs().iter() {
            if let Some(name) = func.func_name() {
                println!("  * {:04} {}", id, name);
            } else {
                println!("  * {:04} <unknown>", id);
            }
            if func.locals().len() > 0 {
                for (id, local) in func.locals().iter() {
                    println!("    * {:04} {}", id, local);
                }
            }
        }
    }
}
