use std::fmt;

use crate::module::Module;

#[derive(PartialEq)]
pub enum ScriptCommand {
    Nil,
    Module(Module),
}

impl ScriptCommand {
    pub fn is_nil(&self) -> bool {
        match self {
            ScriptCommand::Nil => true,
            _ => false,
        }
    }
}

impl fmt::Display for ScriptCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScriptCommand::Nil => write!(f, "(nil)"),
            ScriptCommand::Module(m) => write!(f, "{}", m),
        }
    }
}

impl fmt::Debug for ScriptCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
