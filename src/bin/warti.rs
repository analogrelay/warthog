#![deny(warnings)]

extern crate warthog;

use std::{borrow::Cow, env, fs, path::Path, process};

use warthog::{
    hosting::{ExternVal, Host},
    format::Module,
    format::reader::Reader,
    runtime,
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
        Module::load(reader).unwrap()
    };

    // Synthesize the 'env' module
    host.external(runtime::Env::new()).unwrap();

    // Instantiate the module
    let entry_point = host.instantiate(name, module).unwrap();

    // Look for the main entry point
    let main_func = match host.resolve_import(entry_point, "_main").unwrap().value() {
        ExternVal::Func(f) => *f,
        _ => panic!("'_main' is not a function!"),
    };

    unimplemented!()

    // // Create a thread
    // let mut thread = Thread::new();

    // // Invoke the entry point
    // if let Err(trap) = thread.invoke(&mut host, main_func) {
    //     eprintln!("trap! {}", trap.cause());
    //     if let Some(trace) = trap.trace() {
    //         for frame in trace.frames() {
    //             if let Some(loc) = frame.func().and_then(|f| host.get_location(f, 0)) {
    //                 eprintln!(" at {}", loc);
    //             } else {
    //                 eprintln!(" at {}", frame);
    //             }
    //         }
    //     }
    // }
}
