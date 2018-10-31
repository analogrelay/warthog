use crate::script::ScriptCommand;

#[derive(Debug)]
pub struct Script {
    commands: Vec<ScriptCommand>
}

impl Script {
    pub fn new(commands: Vec<ScriptCommand>) -> Script {
        Script {
            commands
        }
    }

    pub fn commands(&self) -> &Vec<ScriptCommand> {
        &self.commands
    }
}