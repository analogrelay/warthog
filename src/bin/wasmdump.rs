extern crate warthog;

use std::{env, fs, io, process};

use warthog::reader::{
    CodeSection, DataSection, ExportSection, FunctionSection, ImportSection, Reader, SectionId,
    TypeSection,
};

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
    let mut r = Reader::new(io::BufReader::new(file));

    let header = r.read_module_header().unwrap();
    println!("Module Version: {}", header.version);

    while let Some(header) = r.read_section_header().unwrap() {
        println!("Section: {} (size: {})", header.id, header.size);
        match header.id {
            SectionId::Type => dump_type_section(&mut r),
            SectionId::Import => dump_import_section(&mut r),
            SectionId::Function => dump_function_section(&mut r),
            SectionId::Export => dump_export_section(&mut r),
            SectionId::Data => dump_data_section(&mut r),
            SectionId::Code => dump_code_section(&mut r),
            _ => r.skip(header.size as usize).unwrap(),
        }
    }
}

fn dump_type_section<R: io::Read>(r: &mut Reader<R>) {
    let section: TypeSection = r.read_section().unwrap();
    for (i, typ) in section.types.iter().enumerate() {
        println!("* {:04} {}", i, typ);
    }
}

fn dump_import_section<R: io::Read>(r: &mut Reader<R>) {
    let section: ImportSection = r.read_section().unwrap();
    for (i, import) in section.imports.iter().enumerate() {
        println!("* {:04} {}", i, import);
    }
}

fn dump_function_section<R: io::Read>(r: &mut Reader<R>) {
    let section: FunctionSection = r.read_section().unwrap();
    for (i, func) in section.funcs.iter().enumerate() {
        println!("* {:04} (func {})", i, func);
    }
}

fn dump_export_section<R: io::Read>(r: &mut Reader<R>) {
    let section: ExportSection = r.read_section().unwrap();
    for (i, export) in section.exports.iter().enumerate() {
        println!("* {:04} {}", i, export);
    }
}

fn dump_data_section<R: io::Read>(r: &mut Reader<R>) {
    let section: DataSection = r.read_section().unwrap();
    for (i, item) in section.data.iter().enumerate() {
        println!("* {:04} {}", i, item);
    }
}

fn dump_code_section<R: io::Read>(r: &mut Reader<R>) {
    let section: CodeSection = r.read_section().unwrap();
    for (i, item) in section.code.iter().enumerate() {
        print!("* {:04}", i);
        for local in item.locals.iter() {
            print!(" {}", local);
        }
        println!();
        for inst in item.body.iter() {
            println!("     {}", inst);
        }
    }
}
