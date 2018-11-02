pub struct FuncBuilder {
    export: Option<String>,
}

impl FuncBuilder {
    pub fn new() -> FuncBuilder {
        FuncBuilder { export: None }
    }

    pub fn export_as<S: Into<String>>(&mut self, name: S) {
        self.export = Some(name.into());
    }
}
