use crate::{hosting::Host, Trap, Value};

pub type HostFunc = fn(&mut Host, &[Value]) -> Result<Vec<Value>, Trap>;
