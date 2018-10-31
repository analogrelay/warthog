use std::io;

use crate::script::ScriptError;

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(i64),
    Float(f64),
    Name(String),
    Str(String),
    Atom(String),
}

pub struct Tokenizer<R: io::BufRead> {
    bytes: std::iter::Peekable<io::Bytes<R>>,
}

impl<R: io::BufRead> Iterator for Tokenizer<R> {
    type Item = Result<Token, ScriptError>;

    fn next(&mut self) -> Option<Result<Token, ScriptError>> {
        self.read_token()
    }
}

impl<R: io::BufRead> Tokenizer<R> {
    pub fn new(reader: R) -> Tokenizer<R> {
        Tokenizer {
            bytes: reader.bytes().peekable(),
        }
    }

    #[inline]
    fn consume_as(&mut self, token: Token) -> Result<Token, ScriptError> {
        self.bytes.next();
        Ok(token)
    }

    #[inline]
    fn consume_while<P: FnMut(u8) -> bool>(
        &mut self,
        mut predicate: P,
    ) -> Result<Vec<u8>, ScriptError> {
        let mut vals = Vec::new();
        loop {
            let byt = match self.bytes.peek() {
                Some(Ok(b)) => *b,
                Some(Err(e)) => return Err(e.into()),
                None => break,
            };

            if predicate(byt) {
                self.bytes.next();
                vals.push(byt);
            } else {
                break;
            }
        }
        Ok(vals)
    }

    fn read_token(&mut self) -> Option<Result<Token, ScriptError>> {
        let byt = match self.bytes.peek() {
            Some(Ok(b)) => *b,
            Some(Err(e)) => return Some(Err(e.into())),
            None => return None,
        };

        Some(match byt {
            b'(' => self.consume_as(Token::LParen),
            b')' => self.consume_as(Token::RParen),
            b'a'...b'z' | b'A'...b'Z' | b'_' => self.read_atom(),
            b'+' | b'-' | b'0'...b'9' => self.read_int(),
            x => Err(ScriptError::UnexpectedCharacter(x)),
        })
    }

    fn read_int(&mut self) -> Result<Token, ScriptError> {
        // Read sign
        // Check for hex number
        // Read number
        unimplemented!()
    }

    fn read_atom(&mut self) -> Result<Token, ScriptError> {
        let byts = self.consume_while(is_idchar)?;

        // We can unwrap the from_utf8 result because we validated each char was ASCII
        let s = String::from_utf8(byts).unwrap();

        Ok(Token::Atom(s))
    }
}

fn is_idchar(b: u8) -> bool {
    (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || (b >= b'0' && b <= b'9') || b == b'_'
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
        assert_eq!(
            Some(Token::Atom("h3ll0_h0r1d".to_owned())),
            single_tok("h3ll0_h0r1d")
        );
    }

    #[test]
    pub fn single_token_int() {
        assert_eq!(
            Some(Token::Integer(42)),
            single_tok("42garbage")
        );
        assert_eq!(
            Some(Token::Integer(42)),
            single_tok("+42garbage")
        );
        assert_eq!(
            Some(Token::Integer(-42)),
            single_tok("-42garbage")
        );
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
