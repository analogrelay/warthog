use std::{io, sync::Arc};

use crate::{
    module::{DataItem, Export, FuncBody, FuncType, Import},
    reader::{
        CodeSection, DataSection, ExportSection, FunctionSection, ImportSection, Reader, SectionId,
        TypeSection,
    },
    Error,
};

/// Represents the static information associated with a WebAssembly Module
///
/// The Module object itself holds an [`Arc`] pointer to it's content so it
/// is cheap to clone and all instances of a specific module share the same content.
#[derive(Clone)]
pub struct Module {
    // Hold all our contents via an Arc so we can be Cloned and Sent
    content: Arc<ModuleContent>,
}

struct ModuleContent {
    name: String,
    types: Vec<FuncType>,
    imports: Vec<Import>,
    funcs: Vec<u32>,
    exports: Vec<Export>,
    code: Vec<FuncBody>,
    data: Vec<DataItem>,
}

impl Module {
    /// Loads a module up from the provided reader, consuming the reader in the process
    pub fn load<R: io::Read + io::Seek, S: Into<String>>(
        name: S,
        mut r: Reader<R>,
    ) -> Result<Module, Error> {
        // Read and validate the header
        let header = r.read_module_header()?;

        if header.version != 1 {
            return Err(Error::UnsupportedVersion {
                version: header.version,
            });
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

        let content = ModuleContent {
            name: name.into(),
            types: types.unwrap_or_else(|| Vec::new()),
            imports: imports.unwrap_or_else(|| Vec::new()),
            funcs: funcs.unwrap_or_else(|| Vec::new()),
            exports: exports.unwrap_or_else(|| Vec::new()),
            code: code.unwrap_or_else(|| Vec::new()),
            data: data.unwrap_or_else(|| Vec::new()),
        };

        Ok(Module {
            content: Arc::new(content),
        })
    }

    pub fn name(&self) -> &str {
        &self.content.name
    }

    pub fn types(&self) -> &Vec<FuncType> {
        &self.content.types
    }

    pub fn imports(&self) -> &Vec<Import> {
        &self.content.imports
    }

    pub fn funcs(&self) -> &Vec<u32> {
        &self.content.funcs
    }

    pub fn exports(&self) -> &Vec<Export> {
        &self.content.exports
    }

    pub fn code(&self) -> &Vec<FuncBody> {
        &self.content.code
    }

    pub fn data(&self) -> &Vec<DataItem> {
        &self.content.data
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
