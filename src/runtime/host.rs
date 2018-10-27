use crate::{
    module::Module,
    runtime::{FuncAddr, FuncInst, Instance, InstanceAddr},
};

pub struct Host {
    modules: Vec<Instance>,
    funcs: Vec<FuncInst>,
}

impl Host {
    pub fn new() -> Host {
        Host {
            modules: Vec::new(),
            funcs: Vec::new(),
        }
    }

    /// Instantiates the provided [`Module`], consuming it in the process.
    pub fn instantiate(&mut self, module: Module) -> Instance {
        // TODO: Consider if this type needs to be thread-safe

        // Allocate an instance id
        let instance_id = InstanceAddr::new(self.modules.len());

        // Information for building the instance
        let mut funcs = Vec::new();

        // Instantiate functions
        for (func_id, type_id) in module.funcs().iter().enumerate() {
            // Assign an address
            let func_addr = FuncAddr::new(self.funcs.len());
            funcs.push(func_addr);

            // Get the function body and type
            let typ = module.types()[*type_id as usize].clone();
            let body = module.code()[func_id].clone();

            // Create the instance and register it in the host
            self.funcs.push(FuncInst::local(typ, instance_id, body));
        }

        Instance::new(funcs)
    }

    pub fn funcs(&self) -> &[FuncInst] {
        &self.funcs
    }
}
