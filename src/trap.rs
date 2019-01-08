use std::{borrow::Cow, cmp, fmt};

use crate::types::ValType;

#[derive(Clone, PartialEq)]
pub enum TrapCause {
    IntegerOverflow,
    IntegerDivideByZero,
    InvalidConversionToInteger,
    StackUnderflow,
    StackNotEmpty,
    TypeMismatch { expected: ValType, actual: ValType },
    Other(Cow<'static, str>),
}

impl TrapCause {
    pub fn message<'a>(&'a self) -> Cow<'a, str> {
        use self::TrapCause::*;

        match self {
            // Well-known traps return static strings
            // These strings are described by the spec tests. Do not modify them.
            IntegerOverflow => "integer overflow".into(),
            IntegerDivideByZero => "integer divide by zero".into(),
            InvalidConversionToInteger => "invalid conversion to integer".into(),
            StackNotEmpty => "stack not empty".into(),

            // These are other well-known traps that we define
            StackUnderflow => "stack underflow".into(),
            TypeMismatch { expected, actual } => {
                format!("type mismatch (expected: {}, actual {})", expected, actual).into()
            }

            // If the content is static, we can just return the reference,
            // because 'static will always outlive any 'a
            Other(Cow::Borrowed(c)) => Cow::Borrowed(c),

            // If the content is owned, we can borrow it for the
            // return value
            Other(Cow::Owned(ref c)) => Cow::Borrowed(c),
        }
    }
}

impl fmt::Display for TrapCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for TrapCause {
    fn from(c: T) -> TrapCause {
        TrapCause::Other(c.into())
    }
}

impl cmp::PartialEq<str> for TrapCause {
    fn eq(&self, other: &str) -> bool {
        self.message() == other
    }
}

#[derive(Clone, PartialEq)]
pub struct Trap {
    cause: TrapCause,
    stack_trace: Option<()>,
}

impl Trap {
    pub fn new<C: Into<TrapCause>>(cause: C) -> Trap {
        Trap {
            cause: cause.into(),
            stack_trace: None,
        }
    }

    // /// Sets the stack trace associated with this trap, but **only** if it hasn't already been set.
    // ///
    // /// ## Returns
    // /// A boolean indicating if the stack trace was set.
    // pub fn try_set_stack(&mut self, trace: StackTrace) -> bool {
    //     if self.stack_trace.is_some() {
    //         false
    //     } else {
    //         self.stack_trace = Some(trace);
    //         true
    //     }
    // }

    pub fn cause(&self) -> &TrapCause {
        &self.cause
    }

    pub fn trace(&self) -> Option<&()> {
        self.stack_trace.as_ref()
    }
}

impl<C: Into<TrapCause>> From<C> for Trap {
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
