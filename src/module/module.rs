use std::io;

use crate::{
    module::{DataItem, Export, FuncBody, FuncType, Import},
    reader::{
        CodeSection, DataSection, ExportSection, FunctionSection, ImportSection, Reader, SectionId,
        TypeSection,
    },
    Error,
};

pub struct Module {
    types: Vec<FuncType>,
    imports: Vec<Import>,
    funcs: Vec<u32>,
    exports: Vec<Export>,
    code: Vec<FuncBody>,
    data: Vec<DataItem>,
}

impl Module {
    /// Loads a module up from the provided reader, consuming the reader in the process
    pub fn load<R: io::Read + io::Seek>(mut r: Reader<R>) -> Result<Module, Error> {
        // Read and validate the header
        let header = r.read_module_header()?;

        if header.version != 1 {
            return Err(Error::UnsupportedVersion(header.version));
        }

        let mut types = None;
        let mut imports = None;
        let mut funcs = None;
        let mut exports = None;
        let mut code = None;
        let mut data = None;

        // Load all the sections
        while let Some(header) = r.read_section_header()? {
            match header.id {
                SectionId::Type => types = Some(load_types(&mut r)?),
                SectionId::Import => imports = Some(load_imports(&mut r)?),
                SectionId::Function => funcs = Some(load_functions(&mut r)?),
                SectionId::Export => exports = Some(load_exports(&mut r)?),
                SectionId::Code => code = Some(load_code(&mut r)?),
                SectionId::Data => data = Some(load_data(&mut r)?),
                _ => {
                    // Unknown section
                    // TODO: Logging
                    r.skip(header.size as usize)?;
                }
            }
        }

        Ok(Module {
            types: types.unwrap_or_else(|| Vec::new()),
            imports: imports.unwrap_or_else(|| Vec::new()),
            funcs: funcs.unwrap_or_else(|| Vec::new()),
            exports: exports.unwrap_or_else(|| Vec::new()),
            code: code.unwrap_or_else(|| Vec::new()),
            data: data.unwrap_or_else(|| Vec::new()),
        })
    }
}

fn load_types<R: io::Read>(r: &mut Reader<R>) -> Result<Vec<FuncType>, Error> {
    let section: TypeSection = r.read_section()?;
    Ok(section.types)
}

fn load_imports<R: io::Read>(r: &mut Reader<R>) -> Result<Vec<Import>, Error> {
    let section: ImportSection = r.read_section()?;
    Ok(section.imports)
}

fn load_functions<R: io::Read>(r: &mut Reader<R>) -> Result<Vec<u32>, Error> {
    let section: FunctionSection = r.read_section()?;
    Ok(section.funcs)
}

fn load_exports<R: io::Read>(r: &mut Reader<R>) -> Result<Vec<Export>, Error> {
    let section: ExportSection = r.read_section()?;
    Ok(section.exports)
}

fn load_code<R: io::Read>(r: &mut Reader<R>) -> Result<Vec<FuncBody>, Error> {
    let section: CodeSection = r.read_section()?;
    Ok(section.code)
}

fn load_data<R: io::Read>(r: &mut Reader<R>) -> Result<Vec<DataItem>, Error> {
    let section: DataSection = r.read_section()?;
    Ok(section.data)
}
