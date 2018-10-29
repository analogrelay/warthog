extern crate warthog;

use std::{borrow::Cow, env, fs, path::Path, process};

use warthog::{
    interp::Thread,
    module::{FuncType, Module, ValType},
    reader::Reader,
    runtime::{ExternVal, Host},
    synth::ModuleBuilder,
};

fn main() {
    // Arg 0 is the executable name
    let arg0 = env::args().nth(0).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() > 0 {
        let file = &args[0];
        run(Path::new(file));
    } else {
        eprintln!("Usage: {} <wasm file>", arg0);
        process::exit(1);
    }
}

pub fn run(file: &Path) {
    // Create a host
    let mut host = Host::new();

    // Determine the module name
    let name = match file.file_stem() {
        Some(stem) => stem.to_string_lossy(),
        None => Cow::from("unnamed"),
    };

    // Load the module
    let module = {
        // Close the file once we're done loading
        let file = fs::File::open(file).unwrap();
        let reader = Reader::new(file);
        Module::load(name, reader).unwrap()
    };

    // Synthesize the 'env' module
    let env = ModuleBuilder::new("env")
        .func(
            "print",
            FuncType::new(vec![ValType::Integer32, ValType::Integer32], vec![]),
            || {
                panic!("'print' function not implemented");
            },
        ).mem("memory", 256, Some(256));
    host.synthesize(env).unwrap();

    // Instantiate the module
    let entry_point = host.instantiate(module).unwrap();

    // Look for the main entry point
    let main_func = match host.resolve_import(entry_point, "_main").unwrap().value() {
        ExternVal::Func(f) => *f,
        _ => panic!("'_main' is not a function!"),
    };

    // Create a thread
    let mut thread = Thread::new(host);

    // Invoke the entry point
    thread.invoke(main_func);
}
