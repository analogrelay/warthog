extern crate byteorder;
extern crate leb128;

mod error;
mod format;
mod utils;

use std::fs;
use std::io;

pub use crate::error::Error;
use crate::format::{
    CodeSection, DataSection, ExportSection, FunctionSection, ImportSection, SectionId, TypeSection,
};

pub fn run(file: &str) -> Result<(), Error> {
    let file = fs::File::open(file)?;
    let mut r = format::Reader::new(io::BufReader::new(file))?;

    while let Some(header) = r.read_section_header()? {
        println!("Section: {} (size: {})", header.id, header.size);
        match header.id {
            SectionId::Type => dump_type_section(&mut r)?,
            SectionId::Import => dump_import_section(&mut r)?,
            SectionId::Function => dump_function_section(&mut r)?,
            SectionId::Export => dump_export_section(&mut r)?,
            SectionId::Data => dump_data_section(&mut r)?,
            SectionId::Code => dump_code_section(&mut r)?,
            _ => r.skip(header.size as usize)?,
        }
    }

    Ok(())
}

fn dump_type_section<R: io::Read>(r: &mut format::Reader<R>) -> Result<(), Error> {
    let section: TypeSection = r.read_section()?;
    for (i, typ) in section.types().iter().enumerate() {
        println!("* {:04} {}", i, typ);
    }
    Ok(())
}

fn dump_import_section<R: io::Read>(r: &mut format::Reader<R>) -> Result<(), Error> {
    let section: ImportSection = r.read_section()?;
    for (i, import) in section.imports().iter().enumerate() {
        println!("* {:04} {}", i, import);
    }
    Ok(())
}

fn dump_function_section<R: io::Read>(r: &mut format::Reader<R>) -> Result<(), Error> {
    let section: FunctionSection = r.read_section()?;
    for (i, func) in section.funcs().iter().enumerate() {
        println!("* {:04} (func {})", i, func);
    }
    Ok(())
}

fn dump_export_section<R: io::Read>(r: &mut format::Reader<R>) -> Result<(), Error> {
    let section: ExportSection = r.read_section()?;
    for (i, export) in section.exports().iter().enumerate() {
        println!("* {:04} {}", i, export);
    }
    Ok(())
}

fn dump_data_section<R: io::Read>(r: &mut format::Reader<R>) -> Result<(), Error> {
    let section: DataSection = r.read_section()?;
    for (i, item) in section.data().iter().enumerate() {
        println!("* {:04} {}", i, item);
    }
    Ok(())
}

fn dump_code_section<R: io::Read>(r: &mut format::Reader<R>) -> Result<(), Error> {
    let section: CodeSection = r.read_section()?;
    for (i, item) in section.code().iter().enumerate() {
        print!("* {:04}", i);
        for local in item.locals.iter() {
            print!(" {}", local);
        }
        println!();
        for inst in item.body.iter() {
            println!("     {}", inst);
        }
    }
    Ok(())
}
