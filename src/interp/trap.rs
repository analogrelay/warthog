use std::{borrow::Cow, fmt, io};

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

    pub fn stack_trace(&self) -> Option<&StackTrace> {
        self.stack_trace.as_ref()
    }

    /// Pretty-prints the stack trace.
    ///
    /// The implementation of [`std::fmt::Display`] for [`Trap`] does not include the
    /// stack trace by default because it generally needs to be formatted across multiple
    /// lines. Pretty printing prints the stack trace over multiple lines.
    pub fn pretty_print<W: io::Write>(&self, f: &mut W) -> io::Result<()> {
        writeln!(f, "trap occurred: {}", self.message)?;
        if let Some(ref trace) = self.stack_trace {
            for frame in trace.frames() {
                writeln!(f, "  at {}", frame)?;
            }
        }

        Ok(())
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
