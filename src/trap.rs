use std::{borrow::Cow, fmt};

use crate::interp::StackTrace;

#[derive(Clone, PartialEq)]
pub struct Trap {
    cause: Cow<'static, str>,
    stack_trace: Option<StackTrace>,
}

impl Trap {
    pub fn new<C: Into<Cow<'static, str>>>(cause: C) -> Trap {
        Trap {
            cause: cause.into(),
            stack_trace: None,
        }
    }

    /// Sets the stack trace associated with this trap, but **only** if it hasn't already been set.
    ///
    /// ## Returns
    /// A boolean indicating if the stack trace was set.
    pub fn try_set_stack(&mut self, trace: StackTrace) -> bool {
        if self.stack_trace.is_some() {
            false
        } else {
            self.stack_trace = Some(trace);
            true
        }
    }

    pub fn message(&self) -> &str {
        &self.cause
    }

    pub fn trace(&self) -> Option<&StackTrace> {
        self.stack_trace.as_ref()
    }
}

impl<C: Into<Cow<'static, str>>> From<C> for Trap {
    fn from(cause: C) -> Trap {
        Trap::new(cause)
    }
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl fmt::Debug for Trap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
