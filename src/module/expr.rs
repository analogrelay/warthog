use std::fmt;

use crate::Instruction;

#[derive(PartialEq, Clone)]
pub struct Expr(Vec<Instruction>);

impl Expr {
    pub fn new(instructions: Vec<Instruction>) -> Expr {
        Expr(instructions)
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Instruction> {
        self.0.iter()
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        for (i, instr) in self.iter().enumerate() {
            if i > 0 {
                write!(f, " {}", instr)?;
            } else {
                write!(f, "{}", instr)?;
            }
        }
        write!(f, ")")
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
