extern crate warthog;

use std::{borrow::Cow, env, fs, path::Path, process};

use warthog::{
    interp::{Thread, Trap},
    module::{FuncType, Module, ValType},
    reader::Reader,
    runtime::{ExternVal, Host},
    synth::SyntheticModule,
    Value,
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
    let env = SyntheticModule::new()
        .func(
            "print",
            FuncType::new(vec![ValType::Integer32, ValType::Integer32], vec![]),
            |host, thread, values| print(host, thread, values),
        ).mem("memory", 256, Some(256));
    host.synthesize("env", env).unwrap();

    // Instantiate the module
    let entry_point = host.instantiate(name, module).unwrap();

    // Look for the main entry point
    let main_func = match host.resolve_import(entry_point, "_main").unwrap().value() {
        ExternVal::Func(f) => *f,
        _ => panic!("'_main' is not a function!"),
    };

    // Create a thread
    let mut thread = Thread::new();

    // Invoke the entry point
    thread.invoke(&mut host, main_func);
}

fn print(host: &mut Host, thread: &mut Thread, values: &[Value]) -> Result<Vec<Value>, Trap> {
    let (count, start) = (
        values[0].unwrap_i32() as usize,
        values[1].unwrap_i32() as usize,
    );
    let module = thread.stack_mut().module();
    let end = start + count;

    // Get memory 0 for the current frame
    let mem_addr = host.resolve_mem(module, 0);
    let mem_inst = host.get_mem(mem_addr);
    let mem = mem_inst.memory();

    // Safe as long as other threads (which don't even exist in WASM yet)
    // aren't accessing memory. When threading exists, WASM will provide
    // it's own synchronization primitives.
    unsafe {
        // Read the memory sequence in as a UTF-8 string
        let s = std::str::from_utf8(&mem.data()[start..end]).unwrap();
        println!("{}", s);
    }

    Ok(Vec::new())
}
