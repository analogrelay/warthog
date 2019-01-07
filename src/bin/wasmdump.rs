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

    // while let Some(header) = r.read_section_header().unwrap() {
    //     println!("Section: {} (size: {})", header.id, header.size);
    //     match header.id {
    //         SectionId::Type => dump_type_section(&mut r, header),
    //         SectionId::Import => dump_import_section(&mut r, header),
    //         SectionId::Function => dump_function_section(&mut r, header),
    //         SectionId::Export => dump_export_section(&mut r, header),
    //         SectionId::Data => dump_data_section(&mut r, header),
    //         SectionId::Code => dump_code_section(&mut r, header),
    //         SectionId::Custom => dump_custom_section(&mut r, header),
    //         _ => r.skip(header.size as usize).unwrap(),
    //     }
    // }
}

fn dump_types(m: &Module) {
    println!("Types:");
    for (i, typ) in m.types().iter().enumerate() {
        println!("* {:04} {}", i, typ);
    }
}

// fn dump_custom_section<R: io::Read>(r: &mut Reader<R>, header: SectionHeader) {
//     let section: CustomSection = r.read_section(header).unwrap();
//     println!("  Custom Section Name: {}", section.name);
//     match section.name.as_str() {
//         "name" => dump_name_section(section),
//         _ => {}
//     }
// }

// fn dump_name_section(custom_section: CustomSection) {
//     let section: NameSection = custom_section.read_content().unwrap();
//     if let Some(module_name) = section.module_name {
//         println!("  Module Name: {}", module_name);
//     }
//     println!("  Functions:");
//     for func in section.func_names {
//         println!("  * {:04} {}", func.index(), func.name());
//     }
//     println!("  Locals:");
//     for func in section.local_names {
//         println!("  * Function {:04}", func.index());
//         for local in func.names() {
//             println!("    * {:04} {}", local.index(), local.name());
//         }
//     }
// }

// fn dump_import_section<R: io::Read>(r: &mut Reader<R>, header: SectionHeader) {
//     let section: ImportSection = r.read_section(header).unwrap();
//     for (i, import) in section.imports.iter().enumerate() {
//         println!("* {:04} {}", i, import);
//     }
// }

// fn dump_function_section<R: io::Read>(r: &mut Reader<R>, header: SectionHeader) {
//     let section: FunctionSection = r.read_section(header).unwrap();
//     for (i, func) in section.funcs.iter().enumerate() {
//         println!("* {:04} (type {})", i, func);
//     }
// }

// fn dump_export_section<R: io::Read>(r: &mut Reader<R>, header: SectionHeader) {
//     let section: ExportSection = r.read_section(header).unwrap();
//     for (i, export) in section.exports.iter().enumerate() {
//         println!("* {:04} {}", i, export);
//     }
// }

// fn dump_data_section<R: io::Read>(r: &mut Reader<R>, header: SectionHeader) {
//     let section: DataSection = r.read_section(header).unwrap();
//     for (i, item) in section.data.iter().enumerate() {
//         println!("* {:04} {}", i, item);
//     }
// }

// fn dump_code_section<R: io::Read>(r: &mut Reader<R>, header: SectionHeader) {
//     let section: CodeSection = r.read_section(header).unwrap();
//     for (i, item) in section.code.iter().enumerate() {
//         print!("* {:04}", i);
//         for local in item.locals().iter() {
//             print!(" {}", local);
//         }
//         println!();
//         for inst in item.body().iter() {
//             println!("     {}", inst);
//         }
//     }
// }
