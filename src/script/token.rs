use std::io;

use crate::script::ScriptError;

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(u64),
    Float(f64),
    Name(String),
    Str(String),
    Atom(String),
}

pub struct Tokenizer<R: io::BufRead> {
    reader: R,
}

impl<R: io::BufRead> Tokenizer<R> {
    pub fn new(reader: R) -> Tokenizer<R> {
        Tokenizer { reader }
    }
}

impl<R: io::BufRead> Iterator for Tokenizer<R> {
    type Item = Result<Token, ScriptError>;

    fn next(&mut self) -> Option<Result<Token, ScriptError>> {
        // Fill the buffer
        let (tok, consumed) = match self.reader.fill_buf() {
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return None,
            Err(e) => return Some(Err(e.into())),
            Ok(b) => {
                if b.len() == 0 {
                    return None
                } else {
                    // Try to parse the next token from the buffer
                    match read_token(b) {
                        Err(e) => return Some(Err(e.into())),
                        Ok(t) => t,
                    }
                }
            }
        };
        self.reader.consume(consumed);
        Some(Ok(tok))
    }
}

fn read_token(buf: &[u8]) -> Result<(Token, usize), ScriptError> {
    match buf[0] {
        b'(' => Ok((Token::LParen, 1)),
        b')' => Ok((Token::RParen, 1)),
        b'a'...b'z' | b'A'...b'Z' | b'_' => read_atom(buf),
        x => Err(ScriptError::UnexpectedCharacter(x)),
    }
}

fn read_atom(buf: &[u8]) -> Result<(Token, usize), ScriptError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn single_token_lparen() {
        assert_eq!(Some(Token::LParen), single_tok("(garbage"));
    }

    #[test]
    pub fn single_token_rparen() {
        assert_eq!(Some(Token::RParen), single_tok(")garbage"));
    }

    #[test]
    pub fn single_token_atom() {
        assert_eq!(Some(Token::Atom("h3ll0_h0r1d".to_owned())), single_tok("h3ll0_h0r1d"));
    }

    fn single_tok(inp: &str) -> Option<Token> {
        let tok = Tokenizer::new(inp.as_bytes());
        tok.map(|t| t.unwrap()).next()
    }

    // fn parse_str(inp: &str) -> Vec<Token> {
    //     let tok = Tokenizer::new(inp.as_bytes());
    //     tok.map(|t| t.unwrap()).collect()
    // }
}
