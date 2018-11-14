use std::{borrow::Cow, fmt};

#[derive(PartialEq, Clone)]
pub struct Trap {
    message: Cow<'static, str>,
}

impl Trap {
    pub fn new<S: Into<Cow<'static, str>>>(message: S) -> Trap {
        Trap {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
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
