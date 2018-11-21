use std::fmt;

use crate::hosting::{FuncAddr, ModuleAddr};

pub struct Location {
    module: ModuleAddr,
    func: FuncAddr,
    module_name: Option<String>,
    func_name: Option<String>,
    offset: usize,
}

impl Location {
    pub fn new(
        module: ModuleAddr,
        func: FuncAddr,
        module_name: Option<String>,
        func_name: Option<String>,
        offset: usize,
    ) -> Location {
        Location {
            module,
            func,
            module_name,
            func_name,
            offset,
        }
    }

    pub fn module(&self) -> ModuleAddr {
        self.module
    }

    pub fn func(&self) -> FuncAddr {
        self.func
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn module_name(&self) -> Option<&str> {
        // Convert &String to &str with this one weird trick!
        self.module_name.as_ref().map(|x| &**x)
    }

    pub fn func_name(&self) -> Option<&str> {
        // Convert &String to &str with this one weird trick!
        self.func_name.as_ref().map(|x| &**x)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(module_name) = self.module_name() {
            write!(f, "{}", module_name)?;
        } else {
            write!(f, "0x{:04X}", self.module.val())?;
        }
        write!(f, "!")?;
        if let Some(func_name) = self.func_name() {
            write!(f, "{}", func_name)?;
        } else {
            write!(f, "0x{:04X}", self.func.val())?;
        }
        write!(f, "+{:04}", self.offset)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
