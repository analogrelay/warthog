extern crate warthog;

use std::{borrow::Cow, env, fmt, fs, process};

use warthog::{
    interp::{Thread, Trap},
    runtime::{ExternVal, Host, ModuleAddr},
    text::{self, ScriptAction, ScriptCommand},
    Value,
};

macro_rules! spec_test {
    ($name: ident) => {
        #[test]
        pub fn $name() {
            $crate::run_spec_test(stringify!($name));
        }
    };
}

mod spec_tests {
    spec_test!(i32);
    spec_test!(i64);
}

#[derive(PartialEq, Clone)]
enum AssertionResult {
    Success,
    Failure(Cow<'static, str>),
}

impl fmt::Display for AssertionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssertionResult::Success => write!(f, "success"),
            AssertionResult::Failure(s) => write!(f, "failure - {}", s),
        }
    }
}

impl fmt::Debug for AssertionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn run_spec_test(name: &'static str) {
    let mut parent_dir = env::current_dir().unwrap();
    parent_dir.push("vendor");
    parent_dir.push("webassembly");
    parent_dir.push("test");
    parent_dir.push("core");

    if !parent_dir.exists() {
        panic!(
            "Could not find WebAssembly spec tests directory: '{}'",
            parent_dir.to_str().unwrap()
        );
    }

    let mut test_path = parent_dir.clone();
    test_path.push(format!("{}.wast", name));

    if !test_path.exists() {
        panic!(
            "Could not find WebAssembly spec test '{}.wast' in '{}'",
            name,
            parent_dir.to_str().unwrap()
        );
    }

    let commands = {
        let mut file = fs::File::open(test_path).expect("failed to open file");
        text::parse(&mut file).unwrap()
    };

    run(commands);
}

fn run(commands: Vec<ScriptCommand>) {
    // Create a host
    let mut host = Host::new();

    // Set up state
    let mut last_module = None;

    // Run the commands
    for command in commands {
        match command {
            ScriptCommand::Nil => { /* no-op */ }
            ScriptCommand::Module(module) => {
                let module_addr = host.instantiate("current", module).unwrap();
                last_module = Some(module_addr);
                println!("Instatiated Module: {}", module_addr);
            }
            ScriptCommand::AssertReturn(action, expr) => {
                let result = if let Some(module_addr) = last_module {
                    let mut thread = Thread::new();
                    let result = run_action(&mut thread, &action, module_addr, &mut host);
                    let expected = match expr {
                        Some(ref e) => thread.eval(module_addr, e, &mut host),
                        None => Ok(Value::Nil),
                    };

                    evaluate_assertion(expected, result)
                } else {
                    AssertionResult::Failure("no active module".into())
                };

                assert_eq!(AssertionResult::Success, result);
            }
            ScriptCommand::AssertTrap(action, failure) => {
                let result = if let Some(module_addr) = last_module {
                    let mut thread = Thread::new();
                    let result = run_action(&mut thread, &action, module_addr, &mut host);
                    evaluate_assertion(Err(Trap::new(failure.clone())), result)
                } else {
                    AssertionResult::Failure("no active module".into())
                };

                assert_eq!(AssertionResult::Success, result);
            }
        }
    }
}

fn evaluate_assertion(
    expected: Result<Value, Trap>,
    actual: Result<Vec<Value>, Trap>,
) -> AssertionResult {
    match (expected, actual) {
        (Err(ref e), Err(ref a)) if e == a => AssertionResult::Success,
        (Err(_), Ok(ref x)) => {
            AssertionResult::Failure(format!("returned '{}'", format_vals(x)).into())
        }
        (_, Err(ref a)) => AssertionResult::Failure(format!("trapped '{}'", a).into()),
        (Ok(Value::Nil), Ok(ref a)) if a.len() == 0 => AssertionResult::Success,
        (Ok(Value::Nil), Ok(ref a)) if a.len() != 0 => AssertionResult::Failure(
            format!("expected no results, actual: '{}'", format_vals(a)).into(),
        ),
        (Ok(v), Ok(ref a)) if a.len() > 1 => AssertionResult::Failure(
            format!("expected: '{}', actual: '{}'", v, format_vals(a)).into(),
        ),
        (Ok(v), Ok(ref a)) if v == a[0] => AssertionResult::Success,
        (Ok(v), Ok(ref a)) => AssertionResult::Failure(
            format!("expected: '{}', actual: '{}'", v, format_vals(a)).into(),
        ),
    }
}

fn format_vals(vals: &Vec<Value>) -> String {
    let mut s = String::new();
    for val in vals {
        s.push_str(&format!("{}, ", val));
    }
    let target = s.len() - 2;
    s.truncate(target);
    s
}

fn run_action(
    thread: &mut Thread,
    action: &ScriptAction,
    module: ModuleAddr,
    host: &mut Host,
) -> Result<Vec<Value>, Trap> {
    match action {
        ScriptAction::Invoke(name, exprs) => {
            // Resolve the FuncAddr
            let func_addr = {
                let export = host.resolve_import(module, &name).unwrap();
                match export.value() {
                    ExternVal::Func(func_addr) => *func_addr,
                    e => {
                        return Err(Trap::new(format!(
                            "Export '{}' from module '{}' is not a function, it's a {:?}",
                            name, module, e
                        )))
                    }
                }
            };

            // Run the expressions to fill the stack
            for expr in exprs {
                thread.run(host, expr.instructions());
            }

            thread.invoke(host, func_addr)
        }
        ScriptAction::Get(name) => unimplemented!("(get) action"),
    }
}
