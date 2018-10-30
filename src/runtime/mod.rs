macro_rules! addr_type {
    ($name: ident) => {
        #[derive(Clone, Copy)]
        pub struct $name(usize);
        
        impl $name {
            pub const NULL: $name = $name(0);
        
            pub fn new(id: usize) -> $name {
                $name(id + 1)
            }
        
            pub fn is_null(&self) -> bool {
                self.0 == 0
            }
        
            pub fn val(&self) -> usize {
                if self.is_null() {
                    panic!("attempted to dereference a null address");
                }
                self.0 - 1
            }
        }
        
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if self.is_null() {
                    write!(f, concat!("[", stringify!($name), "]null"))
                } else {
                    write!(f, concat!("[", stringify!($name), "]0x{:04X}"), self.0)
                }
            }
        }
    };
}

mod export_inst;
mod func_inst;
mod host;
mod mem_inst;
mod module_inst;

pub use self::export_inst::{ExportInst, ExternVal};
pub use self::func_inst::{FuncAddr, FuncImpl, FuncInst};
pub use self::host::Host;
pub use self::mem_inst::{MemAddr, MemInst};
pub use self::module_inst::{ModuleAddr, ModuleInst};
