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

    // Inverts the Option and Result returned by `bytes.next` to make `?` work better
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>, ScriptError> {
        match self.bytes.peek() {
            Some(Ok(b)) => Ok(Some(*b)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    #[inline]
    fn consume(&mut self) -> Result<Option<u8>, ScriptError> {
        match self.bytes.next() {
            Some(Ok(b)) => Ok(Some(b)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    #[inline]
    fn expect_any(&mut self) -> Result<u8, ScriptError> {
        match self.consume() {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err(ScriptError::UnexpectedEof),
            Err(e) => Err(e),
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
            if let Some(byt) = self.peek()? {
                if predicate(byt) {
                    self.bytes.next();
                    vals.push(byt);
                } else {
                    break;
                }
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
            b'+' | b'-' | b'0'...b'9' => self.read_num(),
            x => Err(ScriptError::UnexpectedCharacter(x)),
        })
    }

    fn read_num(&mut self) -> Result<Token, ScriptError> {
        // Read the optional sign
        let mut negate = match self.peek()? {
            Some(b'+') => { self.consume()?; false },
            Some(b'-') => { self.consume()?; true }
            _ => false,
        };

        // Read the first digit
        let first_digit = self.expect_any()?;
        if first_digit < b'0' || first_digit > b'9' {
            return Err(ScriptError::UnexpectedCharacter(first_digit));
        }

        // Prepare a place to store the running total.
        let mut val: i64 = 0;

        // Check if we're a hex number
        let hex = if self.peek()? == Some(b'x') {
            // Ignore first_digit and this character
            self.expect_any()?;

            // Mark the number as hex
            true
        } else {
            // Process the first digit
            val = first_digit as i64 - b'0' as i64;
            false
        };

        // Iterate over remaining digits, shifting and adding
        loop {
            let chr = match self.peek()? {
                Some(c) => c,
                None => break
            };
            if chr >= b'0' && chr <= b'9' {
                if hex {
                    val = (val << 4) + (chr - b'0') as i64;
                } else {
                    val = (val << 3) + (val << 1) + (chr - b'0') as i64;
                }
                self.consume()?;
            } else if hex && chr >= b'A' && chr <= b'F' {
                val = (val << 4) + (chr - b'A' + 10) as i64;
                self.consume()?;
            } else if hex && chr >= b'a' && chr <= b'f' {
                val = (val << 4) + (chr - b'a' + 10) as i64;
                self.consume()?;
            } else {
                break;
            }
        }
        Ok(Token::Integer(if negate { -val } else { val }))
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
    pub fn single_token_int_hex() {
        assert_eq!(
            Some(Token::Integer(0xAB)),
            single_tok("0xABgarbage")
        );
        assert_eq!(
            Some(Token::Integer(0xAB)),
            single_tok("+0xABgarbage")
        );
        assert_eq!(
            Some(Token::Integer(-0xAB)),
            single_tok("-0xABgarbage")
        );
        assert_eq!(
            Some(Token::Integer(0xAB)),
            single_tok("0xAbgarbage")
        );
        assert_eq!(
            Some(Token::Integer(0xAB)),
            single_tok("+0xAbgarbage")
        );
        assert_eq!(
            Some(Token::Integer(-0xAB)),
            single_tok("-0xAbgarbage")
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
