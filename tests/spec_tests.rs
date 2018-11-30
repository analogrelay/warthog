extern crate warthog;

use std::io::Cursor;

use warthog::{
    hosting::{ExternVal, Host, ModuleAddr},
    interp::Thread,
    module::Module,
    reader::Reader,
    runtime, Trap, Value,
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
        let mut host = Host::new();

        // Install the Env and SpecTest modules
        host.external(runtime::Env::new()).unwrap();
        host.external(runtime::SpecTest::new()).unwrap();

        TestContext {
            current_line: 0,
            active_module: None,
            host,
        }
    }

    pub fn set_line(&mut self, line: usize) {
        self.current_line = line;
    }

    pub fn load_module(&mut self, module_name: &str, module_data: &[u8]) {
        let module = {
            let mut cur = Cursor::new(module_data);
            let mut reader = Reader::new(cur);
            match Module::load(reader) {
                Ok(m) => m,
                Err(e) => self.panic(format!(
                    "Failed to load module: {}. Error: {:?}",
                    module_name, e
                )),
            }
        };
        let addr = match self.host.instantiate(module_name, module) {
            Ok(a) => a,
            Err(e) => self.panic(format!(
                "Failed to instantiate module: {}. Error: {:?}",
                module_name, e
            )),
        };
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

    pub fn assert_return(&self, expected: Vec<Value>, actual: Result<Vec<Value>, Trap>) {
        // Extract the actual value
        let actual = match actual {
            Ok(mut v) => self.unwrap_val(v),
            Err(t) => self.panic(format!("Trapped: {}", t)),
        };

        // Expect up to one item in the return value
        let expected = self.unwrap_val(expected);

        // Match the results
        if expected != actual {
            self.panic(format!("Expected: {}, Actual: {}", expected, actual));
        }
    }

    pub fn assert_nan(&self, actual: Result<Vec<Value>, Trap>) {
        match actual {
            Err(e) => self.panic(format!("Expected: NaN, Actual: <Trap: {}>", e)),
            Ok(vals) => {
                match self.unwrap_val(vals) {
                    Value::F32(f) if f.is_nan() => { /* success */ }
                    Value::F64(f) if f.is_nan() => { /* success */ }
                    v => {
                        self.panic(format!("Expected: NaN, Actual: {}", v));
                    }
                }
            }
        };
    }

    pub fn assert_trap(&self, expected: &str, actual: Result<Vec<Value>, Trap>) {
        match actual {
            Ok(v) => {
                self.panic(format!(
                    "Expected: <Trap: {}>, Actual: {}",
                    expected,
                    self.unwrap_val(v)
                ));
            }
            Err(ref t) if t.cause() != expected => {
                self.panic(format!(
                    "Expected: <Trap: {}>, Actual: <Trap: {}>",
                    expected,
                    t.cause()
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
                    return Err(format!(
                        "Export '{}' from module '{}' is not a function, it's a {:?}",
                        field, module, e
                    )
                    .into())
                }
            }
        };

        thread.call(&mut self.host, module, func_addr, params)
    }

    #[inline]
    fn unwrap_val(&self, mut vals: Vec<Value>) -> Value {
        if vals.len() > 1 {
            self.panic("Multiple return values are not supported.");
        }

        vals.drain(..).next().unwrap_or(Value::Nil)
    }

    #[inline]
    fn panic<S: AsRef<str>>(&self, message: S) -> ! {
        panic!("{} (line {})", message.as_ref(), self.current_line);
    }
}
