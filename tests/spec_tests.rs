extern crate warthog;

use std::io::Cursor;

use warthog::{
    hosting::{ExternVal, Host, ModuleAddr},
    interp::{Thread, Trap},
    module::Module,
    reader::Reader,
    Value,
};

macro_rules! vals {
    ($($v: expr),*) => {
        vec![$(
            ::warthog::Value::from($v)
        ),*]
    };
}

mod spec;

pub struct TestContext {
    current_line: usize,
    active_module: Option<ModuleAddr>,
    host: Host,
}

impl TestContext {
    pub fn new() -> TestContext {
        TestContext {
            current_line: 0,
            active_module: None,
            host: Host::new(),
        }
    }

    pub fn set_line(&mut self, line: usize) {
        self.current_line = line;
    }

    pub fn load_module(&mut self, module_name: &str, module_data: &[u8]) {
        let module = {
            let mut cur = Cursor::new(module_data);
            let mut reader = Reader::new(cur);
            Module::load(reader).unwrap()
        };
        let addr = self.host.instantiate(module_name, module).unwrap();
        self.active_module = Some(addr);
    }

    pub fn invoke(
        &mut self,
        module: Option<&str>,
        field: &str,
        params: Vec<Value>,
    ) -> Result<Vec<Value>, Trap> {
        let module = match (module, self.active_module) {
            (None, None) => self.panic("No active module!"),
            (Some(_name), _) => {
                self.panic("Not yet implemented: Invoke with module.");
            }
            (_, Some(addr)) => addr,
        };

        self.invoke_core(module, field, params)
    }

    pub fn assert_return(&self, mut expected: Vec<Value>, actual: Result<Vec<Value>, Trap>) {
        // Extract the actual value
        let actual = match actual {
            Ok(mut v) => {
                if v.len() > 1 {
                    self.panic("Multiple return values are not supported.");
                }
                v.drain(..).next()
            }
            Err(t) => self.panic(format!("Trapped: {}", t)),
        };

        // Expect up to one item in the return value
        if expected.len() > 1 {
            self.panic("Multiple return values are not supported.");
        }

        let expected = expected.drain(..).next();

        // Match the results
        match (expected, actual) {
            (None, Some(a)) => self.panic(format!("Expected: <nil>, Actual: {}", a)),
            (Some(e), None) => self.panic(format!("Expected: {}, Actual: <nil>", e)),
            (Some(e), Some(a)) if e != a => self.panic(format!("Expected: {}, Actual: {}", e, a)),
            _ => { /* all good! */ }
        }
    }

    pub fn assert_trap(&self, expected: &str, actual: Result<Vec<Value>, Trap>) {
        match actual {
            Ok(mut v) => {
                if v.len() > 1 {
                    self.panic("Multiple return values are not supported.");
                }

                if let Some(v) = v.drain(..).next() {
                    self.panic(format!("Expected: <Trap: {}>, Actual: {}", expected, v));
                } else {
                    self.panic(format!("Expected: <Trap: {}>, Actual: <nil>", expected));
                }
            }
            Err(ref t) if t.message() != expected => {
                self.panic(format!(
                    "Expected: <Trap: {}>, Actual: <Trap: {}>",
                    expected,
                    t.message()
                ));
            }
            _ => { /* all good! */ }
        }
    }

    fn invoke_core(
        &mut self,
        module: ModuleAddr,
        field: &str,
        params: Vec<Value>,
    ) -> Result<Vec<Value>, Trap> {
        // Create a thread
        let mut thread = Thread::new();

        let func_addr = {
            let export = match self.host.resolve_import(module, field) {
                Ok(e) => e,
                Err(e) => {
                    self.panic(format!(
                        "Failed to resolve import: [{}]::[{}]. Error: {:?}",
                        module, field, e
                    ));
                }
            };
            match export.value() {
                ExternVal::Func(func_addr) => *func_addr,
                e => {
                    return Err(Trap::new(
                        format!(
                            "Export '{}' from module '{}' is not a function, it's a {:?}",
                            field, module, e
                        ),
                        None,
                    ))
                }
            }
        };

        thread.call(&mut self.host, module, func_addr, params)
    }

    #[inline]
    fn panic<S: AsRef<str>>(&self, message: S) -> ! {
        panic!("{} (line {})", message.as_ref(), self.current_line);
    }
}
