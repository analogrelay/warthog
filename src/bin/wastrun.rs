extern crate warthog;

use std::{borrow::Cow, env, fmt, fs, process};

use warthog::{
    interp::{Thread, Trap},
    runtime::{ExternVal, Host, ModuleAddr},
    text::{self, ScriptAction, ScriptCommand},
    Value,
};

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

fn main() {
    // Arg 0 is the executable name
    let arg0 = env::args().nth(0).unwrap();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() > 0 {
        let file = &args[0];
        run(file);
    } else {
        eprintln!("Usage: {} <wasm file>", arg0);
        process::exit(1);
    }
}

pub fn run(file: &str) {
    let mut file = fs::File::open(file).unwrap();
    let commands = text::parse(&mut file).unwrap();

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
                match expr {
                    Some(ref e) => print!("* {} = {} ... ", action, e),
                    None => print!("* {} ...", action),
                }

                if let Some(module_addr) = last_module {
                    let mut thread = Thread::new();
                    let result = run_action(&mut thread, action, module_addr, &mut host);
                    let expected = match expr {
                        Some(ref e) => thread.eval(module_addr, e, &mut host),
                        None => Ok(Value::Nil),
                    };

                    println!(" {}", evaluate_assertion(expected, result));
                } else {
                    println!(" {}", AssertionResult::Failure("no active module".into()));
                };
            }
            ScriptCommand::AssertTrap(action, failure) => {
                print!("* {} trap '{}' ...", action, failure);
                if let Some(module_addr) = last_module {
                    let mut thread = Thread::new();
                    let result = run_action(&mut thread, action, module_addr, &mut host);
                    println!(" {}", evaluate_assertion(Err(Trap::new(failure)), result));
                } else {
                    println!(" {}", AssertionResult::Failure("no active module".into()));
                };
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
    action: ScriptAction,
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
