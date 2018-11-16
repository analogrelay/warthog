use std::{borrow::Cow, fmt};

use crate::interp::StackTrace;

#[derive(Clone, PartialEq)]
pub struct Trap {
    message: Cow<'static, str>,
    stack_trace: Option<StackTrace>,
}

impl Trap {
    pub fn new<S: Into<Cow<'static, str>>>(message: S, stack_trace: Option<StackTrace>) -> Trap {
        Trap {
            message: message.into(),
            stack_trace,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn trace(&self) -> Option<&StackTrace> {
        self.stack_trace.as_ref()
    }
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for Trap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
