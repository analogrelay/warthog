macro_rules! addr_type {
    ($name: ident) => {
        #[derive(Clone, Copy)]
        pub struct $name(usize);

        impl $name {
            pub fn new(id: usize) -> $name {
                $name(id)
            }

            pub fn val(&self) -> usize {
                self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, concat!("[", stringify!($name), "]0x{:04X}"), self.0)
            }
        }
    };
}

mod func_inst;
mod host;
mod instance;

pub use self::func_inst::{FuncAddr, FuncInst};
pub use self::host::Host;
pub use self::instance::{Instance, InstanceAddr};
