use crate::{
    hosting::Host,
    interp::{Thread, Trap},
    Value,
};

pub trait HostFunc {
    fn invoke(
        &self,
        host: &mut Host,
        thread: &mut Thread,
        values: &[Value],
    ) -> Result<Vec<Value>, Trap>;
}

macro_rules! wrap_val {
    ($body: expr, ()) => {
        $body;
        vec![]
    };
    ($body: expr, i32) => {
        vec![$crate::Value::Integer32($body as u32)]
    };
    ($body: expr, i64) => {
        vec![$crate::Value::Integer64($body as u64)]
    };
    ($body: expr, u32) => {
        vec![$crate::Value::Integer32($body)]
    };
    ($body: expr, u64) => {
        vec![$crate::Value::Integer64($body)]
    };
    ($body: expr, f32) => {
        vec![$crate::Value::Float32($body)]
    };
    ($body: expr, f64) => {
        vec![$crate::Value::Float64($body)]
    };
    ($body: expr, $other: ty) => {
        compile_error!(concat!(
            "Type '",
            stringify!($other),
            "' is not supported as a return value. Only i32, i64, u32, u64, f32, f64 are supported"
        ));
    };
}

macro_rules! unwrap_val {
    ($val: expr, i32) => {
        match $val {
            $crate::Value::Integer32(x) => x as i32,
            _ => panic!("Type mismatch. TODO: Trap"),
        }
    };
    ($val: expr, i64) => {
        match $val {
            $crate::Value::Integer64(x) => x as i64,
            _ => panic!("Type mismatch. TODO: Trap"),
        }
    };
    ($val: expr, u32) => {
        match $val {
            $crate::Value::Integer32(x) => x,
            _ => panic!("Type mismatch. TODO: Trap"),
        }
    };
    ($val: expr, u64) => {
        match $val {
            $crate::Value::Integer64(x) => x,
            _ => panic!("Type mismatch. TODO: Trap"),
        }
    };
    ($val: expr, f32) => {
        match $val {
            $crate::Value::Float32(x) => x,
            _ => panic!("Type mismatch. TODO: Trap"),
        }
    };
    ($val: expr, f64) => {
        match $val {
            $crate::Value::Float64(x) => x,
            _ => panic!("Type mismatch. TODO: Trap"),
        }
    };
    ($val: expr, $other: ty) => {
        compile_error!(concat!(
            "Type '",
            stringify!($other),
            "' is not supported as a parameter value. Only i32, i64, u32, u64, f32, f64 are supported"
        ));
    };
}

macro_rules! hostfunc {
    // (fn() -> $ret: ident) => {
    //     impl HostFunc for fn(&mut Host) -> $ret {
    //         fn invoke(
    //             &self,
    //             host: &mut Host,
    //             thread: &mut Thread,
    //             values: &[Value],
    //         ) -> Result<Vec<Value>, Trap> {
    //             let val = wrap_val!(self(host), $ret);
    //             Ok(val)
    //         }
    //     }

    //     impl HostFunc for fn(&mut Host, &mut Thread) -> $ret {
    //         fn invoke(
    //             &self,
    //             host: &mut Host,
    //             thread: &mut Thread,
    //             values: &[Value],
    //         ) -> Result<Vec<Value>, Trap> {
    //             let val = wrap_val!(self(host, thread), $ret);
    //             Ok(val)
    //         }
    //     }

    //     impl HostFunc for fn(&mut Thread) -> $ret {
    //         fn invoke(
    //             &self,
    //             host: &mut Host,
    //             thread: &mut Thread,
    //             values: &[Value],
    //         ) -> Result<Vec<Value>, Trap> {
    //             let val = wrap_val!(self(thread), $ret);
    //             Ok(val)
    //         }
    //     }
    // };
    (fn($($name: ident: $t: ident),*) -> $ret: ident) => {
        impl HostFunc for fn(&mut Host, $($t),*) -> $ret {
            fn invoke(
                &self,
                host: &mut Host,
                thread: &mut Thread,
                values: &[Value],
            ) -> Result<Vec<Value>, Trap> {
                $(
                    let $name = unwrap_val!(values[0], $t);
                )*
                let val = wrap_val!(self(host, $($name),*), $ret);
                Ok(val)
            }
        }

        impl HostFunc for fn(&mut Host, &mut Thread, $($t),*) -> $ret {
            fn invoke(
                &self,
                host: &mut Host,
                thread: &mut Thread,
                values: &[Value],
            ) -> Result<Vec<Value>, Trap> {
                $(
                    let $name = unwrap_val!(values[0], $t);
                )*
                let val = wrap_val!(self(host, thread, $($name),*), $ret);
                Ok(val)
            }
        }

        impl HostFunc for fn(&mut Thread, $($t),*) -> $ret {
            fn invoke(
                &self,
                host: &mut Host,
                thread: &mut Thread,
                values: &[Value],
            ) -> Result<Vec<Value>, Trap> {
                $(
                    let $name = unwrap_val!(values[0], $t);
                )*
                let val = wrap_val!(self(thread, $($name),*), $ret);
                Ok(val)
            }
        }
    }
}

impl HostFunc for fn(&mut Host) -> () {
    fn invoke(
        &self,
        host: &mut Host,
        thread: &mut Thread,
        values: &[Value],
    ) -> Result<Vec<Value>, Trap> {
        self(host);
        Ok(vec![])
    }
}

impl HostFunc for fn(&mut Host, &mut Thread) -> () {
    fn invoke(
        &self,
        host: &mut Host,
        thread: &mut Thread,
        values: &[Value],
    ) -> Result<Vec<Value>, Trap> {
        self(host, thread);
        Ok(vec![])
    }
}

impl HostFunc for fn(&mut Thread) -> () {
    fn invoke(
        &self,
        host: &mut Host,
        thread: &mut Thread,
        values: &[Value],
    ) -> Result<Vec<Value>, Trap> {
        self(thread);
        Ok(vec![])
    }
}

hostfunc!(fn() -> i32);
hostfunc!(fn() -> i64);
hostfunc!(fn() -> u32);
hostfunc!(fn() -> u64);
hostfunc!(fn() -> f32);
hostfunc!(fn() -> f64);

hostfunc!(fn(a: i32) -> i32);
hostfunc!(fn(a: i32) -> i64);
hostfunc!(fn(a: i32) -> u32);
hostfunc!(fn(a: i32) -> u64);
hostfunc!(fn(a: i32) -> f32);
hostfunc!(fn(a: i32) -> f64);

hostfunc!(fn(a: u32) -> i32);
hostfunc!(fn(a: u32) -> i64);
hostfunc!(fn(a: u32) -> u32);
hostfunc!(fn(a: u32) -> u64);
hostfunc!(fn(a: u32) -> f32);
hostfunc!(fn(a: u32) -> f64);

hostfunc!(fn(a: i64) -> i32);
hostfunc!(fn(a: i64) -> i64);
hostfunc!(fn(a: i64) -> u32);
hostfunc!(fn(a: i64) -> u64);
hostfunc!(fn(a: i64) -> f32);
hostfunc!(fn(a: i64) -> f64);

hostfunc!(fn(a: u64) -> i32);
hostfunc!(fn(a: u64) -> i64);
hostfunc!(fn(a: u64) -> u32);
hostfunc!(fn(a: u64) -> u64);
hostfunc!(fn(a: u64) -> f32);
hostfunc!(fn(a: u64) -> f64);

Maybe this will work:

impl<A: Into<Value>, R: Into<Value>, F: Fn(A) -> R> HostFunc for F {

}