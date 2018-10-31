#[derive(Debug)]
pub enum ScriptCommand {
    Nil,
    Module { name: Option<String> }
}

impl ScriptCommand {
    pub fn is_nil(&self) -> bool {
        match self {
            ScriptCommand::Nil => true,
            _ => false
        }
    }
}
