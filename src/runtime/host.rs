use crate::{
    module::{Export, Import, MemberDesc, Module},
    runtime::{
        ExportInst, ExternVal, FuncAddr, FuncInst, MemAddr, MemInst, ModuleAddr, ModuleInst,
    },
    synth::ModuleBuilder,
    Error,
};

pub struct Host {
    modules: Vec<ModuleInst>,
    funcs: Vec<FuncInst>,
    mems: Vec<MemInst>,
}

// TODO: Consider if this type needs to be thread-safe
impl Host {
    pub fn new() -> Host {
        Host {
            modules: Vec::new(),
            funcs: Vec::new(),
            mems: Vec::new(),
        }
    }

    /// Synthesizes a module from the provided [`ModuleBuilder`], consuming it in the process.
    pub fn synthesize(&mut self, module: ModuleBuilder) -> ModuleAddr {
        let module_addr = ModuleAddr::new(self.modules.len());

        let mut funcs = Vec::new();
        for func in module.funcs {
            // Allocate a func in the host
            let func_addr = FuncAddr::new(self.funcs.len());
            let func_inst = FuncInst::synthetic(func);
            self.funcs.push(func_inst);
            funcs.push(func_addr);
        }

        // Export the synthetic module
        let exports = self.export_module(&funcs, &module.exports);

        // Register the module and return
        self.modules
            .push(ModuleInst::new(module.name, funcs, Vec::new(), exports));
        module_addr
    }

    /// Instantiates the provided [`Module`], consuming it in the process.
    pub fn instantiate(&mut self, module: Module) -> Result<ModuleAddr, Error> {
        let module_addr = ModuleAddr::new(self.modules.len());

        let mut funcs = Vec::new();
        let mut mems = Vec::new();
        self.resolve_imports(&module, &mut funcs, &mut mems)?;
        self.instantiate_funcs(module_addr, &module, &mut funcs);
        let exports = self.export_module(&funcs, module.exports());

        self.modules
            .push(ModuleInst::new(module.name(), funcs, mems, exports));
        Ok(module_addr)
    }

    pub fn modules(&self) -> &[ModuleInst] {
        &self.modules
    }

    pub fn funcs(&self) -> &[FuncInst] {
        &self.funcs
    }

    pub fn find_module(&self, name: &str) -> Option<&ModuleInst> {
        self.modules.iter().find(|m| m.name() == name)
    }

    pub fn resolve_import(&self, import: &Import) -> Result<&ExportInst, Error> {
        if let Some(module) = self.find_module(&import.module) {
            if let Some(export) = module.find_export(&import.name) {
                Ok(export)
            } else {
                Err(Error::ExportNotFound {
                    module: import.module.clone(),
                    name: import.name.clone(),
                })
            }
        } else {
            Err(Error::ModuleNotFound {
                module: import.module.clone(),
            })
        }
    }

    fn export_module(
        &mut self,
        funcs: &[FuncAddr],
        module_exports: &Vec<Export>,
    ) -> Vec<ExportInst> {
        let mut exports = Vec::new();
        for export in module_exports {
            match export.description {
                MemberDesc::Function(func_idx) => {
                    let func_addr = funcs[func_idx as usize];
                    let inst = ExportInst::func(export.name.as_str(), func_addr);
                    exports.push(inst);
                }
                MemberDesc::Memory(ref mem_type) => {
                    let mem_addr = MemAddr::new(self.mems.len());
                    self.mems.push(MemInst::from_type(mem_type));
                    let inst = ExportInst::mem(export.name.as_str(), mem_addr);
                    exports.push(inst);
                }
                _ => { /* skip */ }
            }
        }
        exports
    }

    fn instantiate_funcs(
        &mut self,
        instance_addr: ModuleAddr,
        module: &Module,
        funcs: &mut Vec<FuncAddr>,
    ) {
        // Instantiate functions
        for (code_idx, type_id) in module.funcs().iter().enumerate() {
            // Assign an address
            let func_addr = FuncAddr::new(self.funcs.len());
            funcs.push(func_addr);

            // Get the function body and type
            let typ = module.types()[*type_id as usize].clone();
            let body = module.code()[code_idx].clone();

            // Create the instance and register it in the host
            self.funcs.push(FuncInst::local(typ, instance_addr, body));
        }
    }

    fn resolve_imports(
        &mut self,
        module: &Module,
        funcs: &mut Vec<FuncAddr>,
        mems: &mut Vec<MemAddr>,
    ) -> Result<(), Error> {
        for import in module.imports() {
            let export = self.resolve_import(import)?;
            match export.value() {
                ExternVal::Func(func_addr) => funcs.push(func_addr.clone()),
                ExternVal::Mem(mem_addr) => mems.push(mem_addr.clone()),
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}
