macro_rules! err {
    ($p: expr, $e: expr) => {
        ParserError::new($e, crate::text::ToRange::into_range($p))
    };
    ($p: expr, $e: expr,) => {
        ParserError::new($e, crate::text::ToRange::into_range($p))
    };
    ($p: expr, $e: expr, $det: expr) => {
        ParserError::detailed($e, crate::text::ToRange::into_range($p), $det)
    };
    ($p: expr, $e: expr, $det: expr,) => {
        ParserError::detailed($e, crate::text::ToRange::into_range($p), $det)
    };
}

mod parser;
mod parser_error;
mod script_command;
mod sexpr;

pub use self::parser::parse;
pub use self::parser_error::{ParserError, ParserErrorKind};
pub use self::script_command::{ScriptAction, ScriptCommand};

trait ToRange {
    fn into_range(self) -> (usize, usize);
}

impl ToRange for usize {
    fn into_range(self) -> (usize, usize) {
        (self, self)
    }
}

impl ToRange for (usize, usize) {
    fn into_range(self) -> (usize, usize) {
        self
    }
}
