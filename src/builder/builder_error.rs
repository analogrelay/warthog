use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderError {
    IncompleteFunction(Cow<'static, str>),
}
