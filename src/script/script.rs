use crate::{
    interp::{Thread, Trap},
    runtime::{ExternVal, Host, ModuleAddr},
    script::{AssertionResult, ScriptAction, ScriptCommand},
    Value,
};

#[derive(Clone)]
pub struct Script(Vec<ScriptCommand>);

impl Script {
    pub fn new(commands: Vec<ScriptCommand>) -> Script {
        Script(commands)
    }

    pub fn commands(&self) -> &[ScriptCommand] {
        &self.0
    }

    pub fn run(self) -> Vec<AssertionResult> {
        run_commands(self.0)
    }
}

fn run_commands(commands: Vec<ScriptCommand>) -> Vec<AssertionResult> {
    // Create a host
    let mut host = Host::new();

    // Set up state
    let mut last_module = None;

    let mut results = Vec::new();

    // Run the commands
    let mut counter = 0;
    for command in commands {
        match command {
            ScriptCommand::Nil => { /* no-op */ }
            ScriptCommand::Module(module) => {
                let module_addr = host.instantiate("current", module).unwrap();
                last_module = Some(module_addr);
            }
            ScriptCommand::AssertReturn(action, expr) => {
                let assertion = match expr.as_ref() {
                    Some(e) => format!("{} = {}", action, e),
                    None => format!("{}", action),
                };

                if let Some(module_addr) = last_module {
                    let mut thread = Thread::new();
                    let result = run_action(&mut thread, &action, module_addr, &mut host);
                    let expected = match expr.as_ref() {
                        Some(e) => thread.eval(module_addr, e, &mut host),
                        None => Ok(Value::Nil),
                    };

                    results.push(evaluate_assertion(counter, assertion, expected, result));
                } else {
                    results.push(AssertionResult::failure(
                        counter,
                        assertion,
                        "no active module",
                    ))
                };
                counter += 1;
            }
            ScriptCommand::AssertTrap(action, failure) => {
                let assertion = format!("{} trap '{}'", action, failure);
                if let Some(module_addr) = last_module {
                    let mut thread = Thread::new();
                    let result = run_action(&mut thread, &action, module_addr, &mut host);

                    results.push(evaluate_assertion(
                        counter,
                        assertion,
                        Err(Trap::new(failure.clone(), None)),
                        result,
                    ));
                } else {
                    results.push(AssertionResult::failure(
                        counter,
                        assertion,
                        "no active module",
                    ))
                };
                counter += 1;
            }
        }
    }

    results
}

fn evaluate_assertion(
    index: usize,
    assertion: String,
    expected: Result<Value, Trap>,
    actual: Result<Vec<Value>, Trap>,
) -> AssertionResult {
    match (expected, actual) {
        (Err(ref e), Err(ref a)) if e.message() == a.message() => {
            AssertionResult::success(index, assertion)
        }
        (Err(_), Ok(ref x)) => {
            AssertionResult::failure(index, assertion, format!("returned '{}'", format_vals(x)))
        }
        (_, Err(ref a)) => AssertionResult::failure(index, assertion, format!("trapped '{}'", a)),
        (Ok(Value::Nil), Ok(ref a)) if a.len() == 0 => AssertionResult::success(index, assertion),
        (Ok(Value::Nil), Ok(ref a)) if a.len() != 0 => AssertionResult::failure(
            index,
            assertion,
            format!("expected no results, actual: '{}'", format_vals(a)),
        ),
        (Ok(v), Ok(ref a)) if a.len() > 1 => AssertionResult::failure(
            index,
            assertion,
            format!(
                "expected: '{}', actual: '{}'",
                format_val(&v),
                format_vals(a)
            ),
        ),
        (Ok(v), Ok(ref a)) if v == a[0] => AssertionResult::success(index, assertion),
        (Ok(v), Ok(ref a)) => AssertionResult::failure(
            index,
            assertion,
            format!(
                "expected: '{}', actual: '{}'",
                format_val(&v),
                format_vals(a)
            ),
        ),
    }
}

fn format_vals(vals: &Vec<Value>) -> String {
    let mut s = String::new();
    for val in vals {
        s.push_str(&format!("{}, ", format_val(val)));
    }
    let target = s.len() - 2;
    s.truncate(target);
    s
}

fn format_val(val: &Value) -> String {
    match val {
        Value::Nil => "<nil>".to_owned(),
        Value::Integer32(x) => format!("0x{:X}i32", x),
        Value::Integer64(x) => format!("0x{:X}i64", x),
        Value::Float32(x) => format!("{}f32", x),
        Value::Float64(x) => format!("{}f64", x),
    }
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
                        return Err(Trap::new(
                            format!(
                                "Export '{}' from module '{}' is not a function, it's a {:?}",
                                name, module, e
                            ),
                            None,
                        ))
                    }
                }
            };

            thread.stack_mut().enter(module, None, Vec::new());

            // Run the expressions to fill the stack
            for expr in exprs.iter() {
                thread.run(host, expr.instructions())?;
            }

            let res = thread.invoke(host, func_addr);

            thread.stack_mut().exit();

            res
        }
        ScriptAction::Get(_) => unimplemented!("(get) action"),
    }
}
