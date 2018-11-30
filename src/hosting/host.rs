use std::sync::Arc;

use crate::{
    hosting::{
        ExportInst, ExternVal, ExternalModule, FuncAddr, FuncImpl, FuncInst, MemAddr, MemInst,
        ModuleAddr, ModuleInst,
    },
    module::{Export, Expr, MemberDesc, Module},
    Error, Instruction, Location, Value,
};

#[derive(Clone)]
pub struct Host {
    modules: Vec<Arc<ModuleInst>>,
    funcs: Vec<Arc<FuncInst>>,
    mems: Vec<Arc<MemInst>>,
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

    pub fn get_module(&self, addr: ModuleAddr) -> Arc<ModuleInst> {
        self.modules[addr.val()].clone()
    }

    pub fn get_func(&self, addr: FuncAddr) -> Arc<FuncInst> {
        self.funcs[addr.val()].clone()
    }

    pub fn get_mem(&self, addr: MemAddr) -> Arc<MemInst> {
        self.mems[addr.val()].clone()
    }

    pub fn modules<'a>(&'a self) -> impl 'a + Iterator<Item = Arc<ModuleInst>> {
        self.modules.iter().cloned()
    }

    pub fn funcs<'a>(&'a self) -> impl 'a + Iterator<Item = Arc<FuncInst>> {
        self.funcs.iter().cloned()
    }

    pub fn mems<'a>(&'a self) -> impl 'a + Iterator<Item = Arc<MemInst>> {
        self.mems.iter().cloned()
    }

    pub fn find_module(&self, name: &str) -> Option<ModuleAddr> {
        self.modules
            .iter()
            .position(|m| m.name() == name)
            .map(|a| ModuleAddr::new(a + 1).expect("Searched module address should be non-zero!"))
    }

    pub fn resolve_mem(&self, module: ModuleAddr, mem_idx: usize) -> MemAddr {
        let module_inst = &self.modules[module.val()];
        module_inst.get_mem(mem_idx)
    }

    pub fn resolve_func(&self, module: ModuleAddr, func_idx: usize) -> FuncAddr {
        let module_inst = &self.modules[module.val()];
        module_inst.get_func(func_idx)
    }

    pub fn resolve_import(&self, module: ModuleAddr, name: &str) -> Result<&ExportInst, Error> {
        let module_inst = &self.modules[module.val()];
        if let Some(export) = module_inst.find_export(name) {
            Ok(export)
        } else {
            Err(Error::ExportNotFound {
                module: module_inst.name().to_owned(),
                name: name.to_owned(),
            })
        }
    }

    /// Evaluates an expression at the module scope.
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, Error> {
        use crate::InstructionPayload;

        // Offset must be a constant expression
        match expr.instructions() {
            [Instruction {
                payload: InstructionPayload::Const(v),
                ..
            }] => Ok(*v),
            _ => panic!("expr not implemented!"),
        }
    }

    /// Resolves a [`Location`] based on a provided [`FuncAddr`] and offset
    pub fn get_location(&self, addr: FuncAddr, offset: usize) -> Option<Location> {
        if addr.val() < self.funcs.len() {
            let func = &self.funcs[addr.val()];
            let module = &self.modules[func.module().val()];

            let func_name = match func.imp() {
                FuncImpl::External(f) => Some(f.name().to_owned()),
                FuncImpl::Local(_, id) => module
                    .names()
                    .and_then(|n| n.funcs().get(*id))
                    .and_then(|n| n.func_name())
                    .map(|x| x.to_owned()),
            };

            Some(Location::new(
                func.module(),
                addr,
                Some(module.name().to_owned()),
                func_name,
                offset,
            ))
        } else {
            None
        }
    }

    /// Instantiates an external module.
    pub fn external<M: ExternalModule>(&mut self, module: M) -> Result<ModuleAddr, Error> {
        let module_addr = ModuleAddr::new(self.modules.len() + 1)
            .expect("New module address should be non-zero!");

        let mut funcs = Vec::new();
        let mut exports = Vec::new();
        for (idx, func) in module.funcs().iter().enumerate() {
            // Allocate a func in the host
            let func_addr = FuncAddr::new(self.funcs.len() + 1)
                .expect("New function address should be non-zero!");
            let func_inst = FuncInst::external(func.typ().clone(), module_addr, func.clone());
            self.funcs.push(Arc::new(func_inst));
            funcs.push(func_addr);
            exports.push(Export::new(
                func.name().to_owned(),
                MemberDesc::Function(idx),
            ))
        }

        // Export memories
        for mem in module.mems() {
            exports.push(Export::new(
                mem.name().to_owned(),
                MemberDesc::Memory(mem.typ().clone()),
            ));
        }

        // Export the synthetic module
        let exports = self.export_module(&funcs, &exports)?;

        // Register the module and return
        self.modules.push(Arc::new(ModuleInst::new(
            module.name().to_owned(),
            funcs,
            Vec::new(),
            exports,
            None,
        )));
        Ok(module_addr)
    }

    /// Instantiates the provided [`Module`], consuming it in the process.
    pub fn instantiate<S: Into<String>>(
        &mut self,
        name: S,
        module: Module,
    ) -> Result<ModuleAddr, Error> {
        let module_addr = ModuleAddr::new(self.modules.len() + 1)
            .expect("New module address should be non-zero!");

        let mut funcs = Vec::new();
        let mut mems = Vec::new();

        self.resolve_imports(&module, &mut funcs, &mut mems)?;
        self.instantiate_funcs(module_addr, &module, &mut funcs);
        self.instantiate_data(&module, &mems)?;

        let exports = self.export_module(&funcs, module.exports())?;

        self.modules.push(Arc::new(ModuleInst::new(
            name.into(),
            funcs,
            mems,
            exports,
            module.names().cloned(),
        )));
        Ok(module_addr)
    }

    fn export_module(
        &mut self,
        funcs: &[FuncAddr],
        module_exports: &Vec<Export>,
    ) -> Result<Vec<ExportInst>, Error> {
        let mut exports = Vec::new();
        for export in module_exports {
            match export.description() {
                MemberDesc::Function(func_idx) => {
                    let func_addr = funcs[*func_idx as usize];
                    let inst = ExportInst::func(export.name(), func_addr);
                    exports.push(inst);
                }
                MemberDesc::Memory(ref mem_type) => {
                    let mem_addr = MemAddr::new(self.mems.len() + 1)
                        .expect("New memory address should be non-zero!");
                    self.mems.push(Arc::new(MemInst::from_type(mem_type)?));
                    let inst = ExportInst::mem(export.name(), mem_addr);
                    exports.push(inst);
                }
                _ => { /* skip */ }
            }
        }
        Ok(exports)
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
            let func_addr = FuncAddr::new(self.funcs.len() + 1)
                .expect("New function address should be non-zero!");
            funcs.push(func_addr);

            // Get the function body and type
            let typ = module.types()[*type_id as usize].clone();
            let body = module.code()[code_idx].clone();

            // Create the instance and register it in the host
            self.funcs.push(Arc::new(FuncInst::local(
                typ,
                instance_addr,
                code_idx,
                body,
            )));
        }
    }

    fn resolve_imports(
        &mut self,
        module: &Module,
        funcs: &mut Vec<FuncAddr>,
        mems: &mut Vec<MemAddr>,
    ) -> Result<(), Error> {
        for import in module.imports() {
            if let Some(module_addr) = self.find_module(&import.module()) {
                let export = self.resolve_import(module_addr, &import.name())?;
                match export.value() {
                    ExternVal::Func(func_addr) => funcs.push(func_addr.clone()),
                    ExternVal::Mem(mem_addr) => mems.push(mem_addr.clone()),
                }
            } else {
                return Err(Error::ModuleNotFound {
                    module: import.module().to_owned(),
                });
            }
        }
        Ok(())
    }

    fn instantiate_data(&mut self, module: &Module, mems: &Vec<MemAddr>) -> Result<(), Error> {
        for data in module.data() {
            let offset = match self.eval_expr(data.expr())? {
                Value::I32(i) => i as usize,
                _ => return Err(Error::InvalidModule),
            };

            // Find an initialize the memory
            let mem_addr = mems[data.index() as usize];
            let mem_inst = &mut self.mems[mem_addr.val()];
            let mut mem = mem_inst.memory();

            // Bounds check
            let end = offset + data.init().len();
            if (offset + end) > mem.len() {
                return Err(Error::InvalidModule);
            }

            // Safe because instatiation is single-threaded.
            unsafe {
                mem.data()[offset..end].copy_from_slice(data.init());
            }
        }
        Ok(())
    }
}
