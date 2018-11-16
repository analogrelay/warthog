use crate::{reader::NameSection, SparseVec};

#[derive(Clone, PartialEq)]
pub struct FuncNames {
    func_name: Option<String>,
    locals: SparseVec<String>,
}

impl FuncNames {
    pub fn new() -> FuncNames {
        FuncNames {
            func_name: None,
            locals: SparseVec::new(),
        }
    }

    pub fn func_name(&self) -> Option<&str> {
        self.func_name.as_ref().map(|x| &**x)
    }

    pub fn locals(&self) -> &SparseVec<String> {
        &self.locals
    }

    pub fn local_name(&self, local_idx: usize) -> Option<&str> {
        self.locals.get(local_idx).map(|x| &**x)
    }
}

#[derive(Clone, PartialEq)]
pub struct ModuleNames {
    module_name: Option<String>,
    funcs: SparseVec<FuncNames>,
}

impl ModuleNames {
    pub fn new() -> ModuleNames {
        ModuleNames {
            module_name: None,
            funcs: SparseVec::new(),
        }
    }

    pub fn load(section: NameSection) -> ModuleNames {
        let mut funcs = SparseVec::new();

        // Load function names
        for name in section.func_names {
            let mut f = funcs.get_or_add(name.index(), |_| FuncNames::new());
            f.func_name = Some(name.name().to_owned());
        }

        // Load local names
        for ind_name in section.local_names {
            let mut f = funcs.get_or_add(ind_name.index(), |_| FuncNames::new());
            for name in ind_name.names() {
                f.locals.set(name.index(), name.name().to_owned());
            }
        }

        ModuleNames {
            module_name: section.module_name,
            funcs,
        }
    }

    pub fn module_name(&self) -> Option<&str> {
        self.module_name.as_ref().map(|x| &**x)
    }

    pub fn funcs(&self) -> &SparseVec<FuncNames> {
        &self.funcs
    }
}
