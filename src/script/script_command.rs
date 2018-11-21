use std::fmt;

use crate::module::{Expr, Module};

#[derive(Clone, PartialEq)]
pub enum ScriptCommand {
    Nil,
    Module(Module),
    AssertReturn(ScriptAction, Option<Expr>),
    AssertTrap(ScriptAction, String),
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
            ScriptCommand::AssertReturn(action, Some(e)) => {
                write!(f, "(assert_return {} {})", action, e)
            }
            ScriptCommand::AssertReturn(action, None) => write!(f, "(assert_return {})", action),
            ScriptCommand::AssertTrap(action, failure) => {
                write!(f, "(assert_trap {} \"{}\")", action, failure)
            }
        }
    }
}

impl fmt::Debug for ScriptCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Clone, PartialEq)]
pub enum ScriptAction {
    Get(String),
    Invoke(String, Vec<Expr>),
}

impl fmt::Display for ScriptAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScriptAction::Get(s) => write!(f, "(get \"{}\")", s),
            ScriptAction::Invoke(s, exprs) => {
                write!(f, "(invoke \"{}\"", s)?;
                for expr in exprs {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Debug for ScriptAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
