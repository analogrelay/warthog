use crate::{
    hosting::Host,
    interp::{Thread, Trap},
    Value,
};

pub type HostFunc = fn(&mut Host, &mut Thread, &[Value]) -> Result<Vec<Value>, Trap>;
