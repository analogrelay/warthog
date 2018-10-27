extern crate warthog;

use std::{borrow::Cow, env, fs, path::Path, process};

use warthog::{
    module::{FuncType, Module, ValType},
    reader::Reader,
    runtime::{FuncImpl, Host, ModuleAddr, ModuleInst},
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
    host.synthesize(env);

    // Instantiate the module
    let entry_point = host.instantiate(module).unwrap();

    // Dump the host
    println!("Host information:");
    dump_funcs(&host);
    dump_instances(entry_point, &host);
}

fn dump_funcs(host: &Host) {
    println!("  Functions:");
    for (i, func_inst) in host.funcs().iter().enumerate() {
        match func_inst.imp() {
            FuncImpl::Local { module: m, .. } => {
                println!("  * {:04} {} {}", i, func_inst.typ(), m);
            }
            FuncImpl::Synthetic(f) => {
                println!("  * {:04} {} <Synthetic: {:p}>", i, func_inst.typ(), f.imp)
            }
        }
    }
}

fn dump_instances(entry_point: ModuleAddr, host: &Host) {
    for (i, module_inst) in host.modules().iter().enumerate() {
        println!("Instance '{}':", module_inst.name());
        if i == entry_point.val() {
            println!("  Entry Point");
        }
        dump_instance_funcs(module_inst);
        dump_instance_exports(module_inst);
    }
}

fn dump_instance_funcs(module_inst: &ModuleInst) {
    println!("  Functions:");
    for (i, func_addr) in module_inst.funcs().iter().enumerate() {
        println!("  * {:04} {}", i, func_addr);
    }
}

fn dump_instance_exports(module_inst: &ModuleInst) {
    println!("  Exports:");
    for (i, export_inst) in module_inst.exports().iter().enumerate() {
        println!(
            "  * {:04} {} {:?}",
            i,
            export_inst.name(),
            export_inst.value()
        );
    }
}
