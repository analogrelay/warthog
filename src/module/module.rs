use std::{fmt, io};

use crate::{
    builder::ModuleBuilder,
    module::{DataItem, Export, FuncBody, FuncType, Import, ModuleNames},
    reader::{
        CodeSection, CustomSection, DataSection, ExportSection, FunctionSection, ImportSection,
        Reader, SectionHeader, SectionId, TypeSection,
    },
    Error,
};

/// Represents the static information associated with a WebAssembly Module
#[derive(Clone, PartialEq)]
pub struct Module {
    types: Vec<FuncType>,
    imports: Vec<Import>,
    funcs: Vec<usize>,
    exports: Vec<Export>,
    code: Vec<FuncBody>,
    data: Vec<DataItem>,
    names: Option<ModuleNames>,
}

impl Module {
    pub fn from_builder(builder: ModuleBuilder) -> Module {
        Module {
            types: builder.types,
            imports: builder.imports,
            funcs: builder.funcs,
            exports: builder.exports,
            code: builder.code,
            data: builder.data,
            names: builder.names,
        }
    }

    /// Loads a module up from the provided reader, consuming the reader in the process
    pub fn load<R: io::Read + io::Seek>(mut r: Reader<R>) -> Result<Module, Error> {
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
        let mut names = None;

        // Load all the sections
        while let Some(header) = r.read_section_header()? {
            match header.id {
                SectionId::Type => types = Some(load_types(&mut r, header)?),
                SectionId::Import => imports = Some(load_imports(&mut r, header)?),
                SectionId::Function => funcs = Some(load_functions(&mut r, header)?),
                SectionId::Export => exports = Some(load_exports(&mut r, header)?),
                SectionId::Code => code = Some(load_code(&mut r, header)?),
                SectionId::Data => data = Some(load_data(&mut r, header)?),
                SectionId::Custom => {
                    let section: CustomSection = r.read_section(header)?;
                    match section.name.as_str() {
                        "name" => names = Some(ModuleNames::load(section.read_content()?)),
                        _ => {
                            // Unknown custom section, just ignore it
                        }
                    }
                }
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
            names,
        })
    }

    pub fn types(&self) -> &Vec<FuncType> {
        &self.types
    }

    pub fn imports(&self) -> &Vec<Import> {
        &self.imports
    }

    pub fn funcs(&self) -> &Vec<usize> {
        &self.funcs
    }

    pub fn exports(&self) -> &Vec<Export> {
        &self.exports
    }

    pub fn code(&self) -> &Vec<FuncBody> {
        &self.code
    }

    pub fn data(&self) -> &Vec<DataItem> {
        &self.data
    }

    pub fn names(&self) -> Option<&ModuleNames> {
        self.names.as_ref()
    }
}

fn load_types<R: io::Read>(
    r: &mut Reader<R>,
    header: SectionHeader,
) -> Result<Vec<FuncType>, Error> {
    let section: TypeSection = r.read_section(header)?;
    Ok(section.types)
}

fn load_imports<R: io::Read>(
    r: &mut Reader<R>,
    header: SectionHeader,
) -> Result<Vec<Import>, Error> {
    let section: ImportSection = r.read_section(header)?;
    Ok(section.imports)
}

fn load_functions<R: io::Read>(
    r: &mut Reader<R>,
    header: SectionHeader,
) -> Result<Vec<usize>, Error> {
    let section: FunctionSection = r.read_section(header)?;
    Ok(section.funcs)
}

fn load_exports<R: io::Read>(
    r: &mut Reader<R>,
    header: SectionHeader,
) -> Result<Vec<Export>, Error> {
    let section: ExportSection = r.read_section(header)?;
    Ok(section.exports)
}

fn load_code<R: io::Read>(
    r: &mut Reader<R>,
    header: SectionHeader,
) -> Result<Vec<FuncBody>, Error> {
    let section: CodeSection = r.read_section(header)?;
    Ok(section.code)
}

fn load_data<R: io::Read>(
    r: &mut Reader<R>,
    header: SectionHeader,
) -> Result<Vec<DataItem>, Error> {
    let section: DataSection = r.read_section(header)?;
    Ok(section.data)
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(module")?;
        for typ in self.types().iter() {
            if typ.params().len() > 0 || typ.results().len() > 0 {
                write!(f, " (type {})", typ);
            } else {
                write!(f, " (type)");
            }
        }
        for (func_idx, code) in self.funcs().iter().zip(self.code().iter()) {
            if code.locals().len() > 0 || code.body().len() > 0 {
                write!(f, " (func (type {}) {})", func_idx, code);
            } else {
                write!(f, " (func (type {}))", func_idx);
            }
        }
        for import in self.imports().iter() {
            write!(f, " {}", import);
        }
        for export in self.exports().iter() {
            write!(f, " {}", export);
        }
        for data in self.data().iter() {
            write!(f, " {}", data);
        }
        write!(f, ")")
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
