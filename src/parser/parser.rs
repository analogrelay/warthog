use std::io;

use crate::{
    module::ModuleBuilder,
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
        // Skip comments
        self.skip_comments()?;

        match self.consume()? {
            Some(Token(TokenKind::LParen, _)) => self.read_command(),
            Some(Token(kind, range)) => {
                Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone())))
            }
            None => Ok(None),
        }
    }

    fn read_command(&mut self) -> Result<Option<ScriptCommand>, ParserError> {
        match self.consume()? {
            Some(Token(TokenKind::Atom(ref x), _)) if x == "module" => self.read_module(),
            Some(Token(TokenKind::Atom(x), range)) => {
                Err(err!(range, ParserErrorKind::UnknownAtom(x.clone())))
            }
            Some(Token(kind, range)) => {
                Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone())))
            }
            None => Err(err!(self.pos, ParserErrorKind::UnexpectedEof)),
        }
    }

    fn read_module(&mut self) -> Result<Option<ScriptCommand>, ParserError> {
        let mut module = ModuleBuilder::new();
        match self.consume()? {
            Some(Token(TokenKind::RParen, _)) => Ok(Some(ScriptCommand::Module(module.build()))),
            Some(Token(kind, range)) => {
                Err(err!(range, ParserErrorKind::UnexpectedToken(kind.clone())))
            }
            None => Err(err!(self.pos, ParserErrorKind::UnexpectedEof)),
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

    fn skip_comments(&mut self) -> Result<(), ParserError> {
        while let Some(Token(TokenKind::Comment, _)) = self.peek()? {
            self.consume()?;
        }
        Ok(())
    }

    fn consume(&mut self) -> Result<Option<Token>, ParserError> {
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
    use crate::module::ModuleBuilder;

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
