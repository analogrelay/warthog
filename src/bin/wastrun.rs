extern crate warthog;

use std::{env, fs, process};

use warthog::{
    interp::{InvokeResult, Thread},
    runtime::{ExternVal, Host, ModuleAddr},
    text::{self, ScriptAction, ScriptCommand},
    module::Expr,
    Value
};

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
                let result = run_action(action, last_module, &mut host);
                let expected = match expr {
                    Some(e) => evaluate_expr(e, &mut host),
                    None => InvokeResult::Empty,
                };
                println!(" {}", evaluate_assertion(expected, result));
            }
            ScriptCommand::AssertTrap(action, failure) => {
                print!("* {} trap '{}' ...", action, failure);
                let result = run_action(action, last_module, &mut host);
                match result {
                    InvokeResult::Trap(ref f) if f == &failure => {
                        println!(" succeeded!");
                    }
                    InvokeResult::Trap(f) => {
                        println!(" trapped: '{}'", f);
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}

fn evaluate_expr(e: Expr, host: &mut Host) -> InvokeResult {
    InvokeResult::Return(vec![host.eval_expr(&e).unwrap()])
}

fn evaluate_assertion(
    expected: InvokeResult,
    actual: InvokeResult
) -> String {
    match (expected, actual) {
        (InvokeResult::Trap(ref exp), InvokeResult::Trap(ref act)) if exp == act => "succeeded!".to_owned(),
        (InvokeResult::Trap(_), InvokeResult::Trap(ref act)) => format!("failed - unexpected trap: '{}'", act),
        (InvokeResult::Trap(_), _) => "failed - did not trap".to_owned(),
        (_, InvokeResult::Trap(ref act)) => format!("failed - trapped: '{}'", act),
        (InvokeResult::Return(r), InvokeResult::Empty) => format!("failed - expected {}, returned nothing", format_vals(r)),
        (InvokeResult::Empty, InvokeResult::Return(r)) => format!("failed - expected nothing, returned {}", format_vals(r)),
        (InvokeResult::Empty, InvokeResult::Empty) => "succeeded!".to_owned(),
        (InvokeResult::Return(exp), InvokeResult::Return(act)) => {
            for x in 0..exp.len() {
                if x > act.len() || exp[x] != act[x] {
                    return format!("failed - expected {}, returned {}", format_vals(exp), format_vals(act))
                }
            }

            "succeeded!".to_owned()
        },
    }
}

fn format_vals(vals: Vec<Value>) -> String {
    let mut s = String::new();
    for val in vals {
        s.push_str(&format!("{}, ", val));
    }
    let target = s.len() - 2;
    s.truncate(target);
    s
}

fn run_action(
    action: ScriptAction,
    last_module: Option<ModuleAddr>,
    host: &mut Host,
) -> InvokeResult {
    match action {
        ScriptAction::Invoke(name, exprs) => {
            // Resolve the FuncAddr
            let func_addr = match last_module {
                Some(module_addr) => {
                    let export = host.resolve_import(module_addr, &name).unwrap();
                    match export.value() {
                        ExternVal::Func(func_addr) => *func_addr,
                        e => panic!(
                            "Export '{}' from module '{}' is not a function, it's a {:?}",
                            name, module_addr, e
                        ),
                    }
                }
                None => panic!("Cannot evaluate action, a module has not been instantiated yet!"),
            };

            // Create a thread to run in
            let mut thread = Thread::new();

            // Run the expressions to fill the stack
            for expr in exprs {
                thread.run(host, expr.instructions());
            }

            thread.invoke(host, func_addr)
        }
        ScriptAction::Get(name) => unimplemented!("(get) action"),
    }
}
