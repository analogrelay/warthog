use std::io;

use crate::{
    builder::{FuncBuilder, ModuleBuilder},
    module::{FuncType, ValType},
    parser::{
        token::{Token, TokenKind, Tokenizer},
        ParserError, ParserErrorKind, ScriptCommand,
    },
};

struct Parser<R: io::Read> {
    tokenizer: std::iter::Peekable<Tokenizer<R>>,
    pos: usize,
}

impl<R: io::Read> Parser<R> {
    pub fn new(reader: R) -> Parser<R> {
        Parser {
            tokenizer: Tokenizer::new(reader).peekable(),
            pos: 0,
        }
    }

    pub fn command(&mut self) -> Result<Option<ScriptCommand>, ParserError> {
        match self.consume()? {
            Some(Token(TokenKind::LParen, _)) => Ok(Some(self.read_command()?)),
            Some(Token(kind, range)) => {
                Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone())))
            }
            None => Ok(None),
        }
    }

    fn read_command(&mut self) -> Result<ScriptCommand, ParserError> {
        match self.expect_atom()?.atom() {
            ("module", _) => self.read_module(),
            (x, range) => Err(err!(range, ParserErrorKind::UnexpectedAtom(x.to_string()))),
        }
    }

    fn read_module(&mut self) -> Result<ScriptCommand, ParserError> {
        let mut module = ModuleBuilder::new();
        loop {
            match self.expect()? {
                Token(TokenKind::RParen, _) => return Ok(ScriptCommand::Module(module.build())),
                Token(TokenKind::LParen, _) => {
                    self.consume()?;
                    match self.expect_atom()?.atom() {
                        ("func", _) => module.func(self.read_func()?),
                        (x, range) => {
                            return Err(err!(range, ParserErrorKind::UnexpectedAtom(x.to_string())))
                        }
                    }
                }
                Token(kind, range) => {
                    return Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone())))
                }
            }
        }
    }

    fn read_func(&mut self) -> Result<FuncBuilder, ParserError> {
        let mut func = FuncBuilder::new();
        let mut typ = FuncType::empty();
        loop {
            match self.expect()? {
                Token(TokenKind::RParen, _) => return Ok(func),
                Token(TokenKind::LParen, _) => {
                    self.consume()?;
                    unimplemented!()
                }
                Token(kind, range) => {
                    return Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone())))
                }
            }
        }
    }

    fn read_valtyp(&mut self) -> Result<ValType, ParserError> {
        match self.expect_atom()?.atom() {
            ("i32", _) => Ok(ValType::Integer32),
            ("i64", _) => Ok(ValType::Integer64),
            ("f32", _) => Ok(ValType::Float32),
            ("f64", _) => Ok(ValType::Float64),
            (x, range) => return Err(err!(range, ParserErrorKind::UnexpectedAtom(x.to_string()))),
        }
    }

    // Inverts the Option and Result returned by `bytes.next` to make `?` work better
    #[inline]
    fn peek(&mut self) -> Result<Option<&Token>, ParserError> {
        match self.tokenizer.peek() {
            Some(Ok(t)) => Ok(Some(t)),
            Some(Err(e)) => Err(e.clone()),
            None => Ok(None),
        }
    }

    #[inline]
    fn expect(&mut self) -> Result<Token, ParserError> {
        match self.consume() {
            Ok(Some(t)) => Ok(t),
            Ok(None) => Err(err!(self.pos, ParserErrorKind::UnexpectedEof)),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn expect_str(&mut self) -> Result<String, ParserError> {
        match self.expect()? {
            Token(TokenKind::Str(s), _) => Ok(s),
            Token(kind, range) => Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone()))),
        }
    }

    #[inline]
    fn expect_atom(&mut self) -> Result<Token, ParserError> {
        match self.expect()? {
            t @ Token(TokenKind::Atom(_), _) => Ok(t),
            Token(kind, range) => Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone()))),
        }
    }

    fn consume(&mut self) -> Result<Option<Token>, ParserError> {
        // Skip comments
        while let Some(Token(TokenKind::Comment, _)) = self.peek()? {
            if let Some(Err(e)) = self.tokenizer.next() {
                return Err(e);
            }
        }

        let res = match self.tokenizer.next() {
            Some(Ok(b)) => Ok(Some(b)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        };
        let new_pos = if let Some(Token(_, (_, end))) = self.peek()? {
            *end
        } else {
            return res;
        };
        self.pos = new_pos;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::ModuleBuilder;

    #[test]
    pub fn simple_func() {
        let expected_module = ModuleBuilder::new().build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            single_command("(module (func (export \"add\") (param i32) (param i32) (result i32) (i32.add (get_local 0) (get_local 1))))")
        );
    }

    #[test]
    pub fn empty_module() {
        let expected_module = ModuleBuilder::new().build();

        assert_eq!(
            ScriptCommand::Module(expected_module),
            single_command("(module)")
        );
    }

    fn single_command(input: &str) -> ScriptCommand {
        let actual = format!(";; Starting Comment\n{}\n;; Ending Comment", input);
        let mut parser = Parser::new(actual.as_bytes());
        let first = parser.command().unwrap().unwrap();

        // We should have no more commands to parse
        assert_eq!(None, parser.command().unwrap());

        first
    }
}
