use std::io;

use crate::parser::{ParserError, ParserErrorKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Token(pub TokenKind, pub (usize, usize));

impl Token {
    pub fn new(kind: TokenKind, range: (usize, usize)) -> Token {
        Token(kind, range)
    }

    pub fn kind(&self) -> &TokenKind {
        &self.0
    }

    pub fn range(&self) -> (usize, usize) {
        self.1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    Integer(i64),
    Float(f64),
    Identifier(String),
    Str(String),
    Atom(String),
    Comment,
}

pub struct Tokenizer<R: io::Read> {
    bytes: std::iter::Peekable<io::Bytes<R>>,
    pos: usize,
    token_start: usize,
}

impl<R: io::Read> Iterator for Tokenizer<R> {
    type Item = Result<Token, ParserError>;

    fn next(&mut self) -> Option<Result<Token, ParserError>> {
        self.read_token()
    }
}

impl<R: io::Read> Tokenizer<R> {
    pub fn new(reader: R) -> Tokenizer<R> {
        Tokenizer {
            bytes: reader.bytes().peekable(),
            pos: 0,
            token_start: 0,
        }
    }

    // Inverts the Option and Result returned by `bytes.next` to make `?` work better
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>, ParserError> {
        match self.bytes.peek() {
            Some(Ok(b)) => Ok(Some(*b)),
            Some(Err(e)) => Err(err!(self.pos, e)),
            None => Ok(None),
        }
    }

    #[inline]
    pub fn consume_if(&mut self, expected: u8) -> Result<bool, ParserError> {
        let res = match self.bytes.peek() {
            Some(Ok(b)) if *b == expected => true,
            Some(Err(e)) => return Err(err!(self.pos, e)),
            _ => false,
        };

        if res {
            self.consume()?;
        }
        Ok(res)
    }

    #[inline]
    fn consume(&mut self) -> Result<Option<u8>, ParserError> {
        self.pos += 1;
        match self.bytes.next() {
            Some(Ok(b)) => Ok(Some(b)),
            Some(Err(e)) => Err(err!(self.pos, e)),
            None => Ok(None),
        }
    }

    #[inline]
    fn expect_any(&mut self) -> Result<u8, ParserError> {
        match self.consume() {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err(err!(self.pos, ParserErrorKind::UnexpectedEof)),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn skip_while<P: FnMut(u8) -> bool>(&mut self, mut predicate: P) -> Result<(), ParserError> {
        loop {
            if let Some(byt) = self.peek()? {
                if predicate(byt) {
                    self.consume()?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    #[inline]
    fn consume_while<P: FnMut(u8) -> bool>(
        &mut self,
        mut predicate: P,
    ) -> Result<Vec<u8>, ParserError> {
        let mut vals = Vec::new();
        loop {
            if let Some(byt) = self.peek()? {
                if predicate(byt) {
                    self.consume()?;
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

    fn read_token(&mut self) -> Option<Result<Token, ParserError>> {
        // Skip whitespace
        match self.skip_while(is_whitespace) {
            Err(e) => return Some(Err(e)),
            Ok(()) => {}
        }

        self.token_start = self.pos;

        let byt = match self.bytes.peek() {
            Some(Ok(b)) => *b,
            Some(Err(e)) => return Some(Err(err!(self.pos, e))),
            None => return None,
        };

        let kind = match byt {
            b'(' => self.read_block_comment(),
            b')' => {
                if let Err(e) = self.consume() {
                    return Some(Err(e.into()));
                }
                Ok(TokenKind::RParen)
            }
            b'a'...b'z' | b'A'...b'Z' | b'_' => self.read_atom(),
            b'+' | b'-' | b'0'...b'9' => self.read_num(),
            b'$' => self.read_identifier(),
            b'"' => self.read_string(),
            b';' => self.read_line_comment(),
            x => Err(err!(
                self.pos,
                ParserErrorKind::UnexpectedCharacter(x as char)
            )),
        };

        match kind {
            Ok(k) => Some(Ok(Token::new(k, (self.token_start, self.pos)))),
            Err(e) => Some(Err(e)),
        }
    }

    fn read_block_comment(&mut self) -> Result<TokenKind, ParserError> {
        self.consume()?; // Consume the '('

        if self.consume_if(b';')? {
            // It's a block comment
            let mut nesting_level = 1;
            while nesting_level > 0 {
                let chr = self.expect_any()?;

                match (chr, self.peek()?) {
                    (b'(', Some(b';')) => {
                        self.consume()?;
                        nesting_level += 1;
                    }
                    (b';', Some(b')')) => {
                        self.consume()?;
                        nesting_level -= 1;
                    }

                    // Skip everything else.
                    _ => {}
                }
            }

            Ok(TokenKind::Comment)
        } else {
            // It's just an LParen
            Ok(TokenKind::LParen)
        }
    }

    fn read_line_comment(&mut self) -> Result<TokenKind, ParserError> {
        self.consume()?;

        let next = self.expect_any()?;
        if next != b';' {
            return Err(err!(
                self.pos,
                ParserErrorKind::UnexpectedCharacter(next as char)
            ));
        }

        // Both kinds of newline sequence end \n
        self.skip_while(|c| c != b'\n')?;

        // Eat the newline itself
        self.consume()?;
        Ok(TokenKind::Comment)
    }

    fn read_string(&mut self) -> Result<TokenKind, ParserError> {
        self.consume()?;

        let mut byts = Vec::new();
        loop {
            let chr = self.expect_any()?;
            if chr == b'\"' {
                break;
            } else if chr == b'\\' {
                match self.expect_any()? {
                    b't' => byts.push(b'\t'),
                    b'n' => byts.push(b'\n'),
                    b'r' => byts.push(b'\r'),
                    b'"' => byts.push(b'"'),
                    b'\'' => byts.push(b'\''),
                    b'\\' => byts.push(b'\\'),
                    b'u' => self.read_unicode_escape(&mut byts)?,
                    x @ b'0'...b'9' | x @ b'a'...b'f' | x @ b'A'...b'F' => {
                        let y = self.expect_any()?;
                        if !is_hex_digit(y) {
                            return Err(err!(
                                self.pos,
                                ParserErrorKind::UnexpectedCharacter(y as char),
                                "Unexpected character in hex escape",
                            ));
                        }

                        let val = (get_digit(x) << 4) + get_digit(y);
                        if let Some(c) = std::char::from_u32(val as u32) {
                            let mut buf = [0u8; 4];
                            for b in c.encode_utf8(&mut buf).as_bytes().iter() {
                                byts.push(*b);
                            }
                        } else {
                            return Err(err!(
                                self.pos,
                                ParserErrorKind::InvalidEscape,
                                "Hex escape is not a valid character",
                            ));
                        }
                    }
                    _ => {
                        return Err(err!(
                            self.pos,
                            ParserErrorKind::InvalidEscape,
                            "Unrecognized escape sequence"
                        ))
                    }
                };
            } else {
                byts.push(chr);
            }
        }

        // We don't validate that we have ASCII strings here, so we can't unwrap
        let s = match String::from_utf8(byts) {
            Ok(s) => s,
            Err(e) => return Err(err!(self.pos, e)),
        };

        Ok(TokenKind::Str(s))
    }

    fn read_identifier(&mut self) -> Result<TokenKind, ParserError> {
        let byts = self.consume_while(is_idchar)?;

        // We can unwrap the from_utf8 result because we validated each char was ASCII
        let s = String::from_utf8(byts).unwrap();

        Ok(TokenKind::Identifier(s))
    }

    fn read_atom(&mut self) -> Result<TokenKind, ParserError> {
        let byts = self.consume_while(is_atomchar)?;

        // We can unwrap the from_utf8 result because we validated each char was ASCII
        let s = String::from_utf8(byts).unwrap();

        Ok(TokenKind::Atom(s))
    }

    fn read_num(&mut self) -> Result<TokenKind, ParserError> {
        // Read the optional sign
        let negate = match self.peek()? {
            Some(b'+') => {
                self.consume()?;
                false
            }
            Some(b'-') => {
                self.consume()?;
                true
            }
            _ => false,
        };

        // Read the first digit
        let first_digit = self.expect_any()?;
        if first_digit < b'0' || first_digit > b'9' {
            return Err(err!(
                self.pos,
                ParserErrorKind::UnexpectedCharacter(first_digit as char)
            ));
        }

        // Check if we're a hex number
        let mut nat = 0;
        let hex = if self.peek()? == Some(b'x') {
            // Ignore first_digit and this character
            self.expect_any()?;

            // Mark the number as hex
            true
        } else {
            // That was the first digit of the number
            nat = (first_digit - b'0') as i64;
            false
        };

        // Read the rest of the number
        let (nat, _) = self.read_digits(nat, hex)?;

        // Check if this is a float
        match self.peek()? {
            Some(b'.') | Some(b'E') | Some(b'e') => {
                Ok(TokenKind::Float(self.read_float(negate, nat, hex)?))
            }
            _ => Ok(TokenKind::Integer(if negate { -nat } else { nat })),
        }
    }

    fn read_float(&mut self, negate: bool, nat: i64, hex: bool) -> Result<f64, ParserError> {
        // Convert the natural number portion to a float
        let mut val = nat as f64;

        // Check for a dot segment
        if self.consume_if(b'.')? {
            let (frac, count) = self.read_digits(0, hex)?;
            let divisor = (if hex { 16f64 } else { 10f64 }).powi(count);
            let addend = frac as f64 / divisor;
            val += addend
        }

        // Check for an exponent segment
        match (hex, self.peek()?) {
            (true, Some(b'p')) | (true, Some(b'P')) => {
                self.consume()?;
                let (exp, _) = self.read_digits(0, hex)?;
                let multiplier = 2f64.powi(exp as i32);
                val *= multiplier;
            }
            (false, Some(b'e')) | (false, Some(b'E')) => {
                self.consume()?;
                let (exp, _) = self.read_digits(0, hex)?;
                let multiplier = 10f64.powi(exp as i32);
                val *= multiplier;
            }
            _ => {}
        }

        Ok(if negate { -val } else { val })
    }

    fn read_digits(&mut self, mut start_val: i64, hex: bool) -> Result<(i64, i32), ParserError> {
        // Iterate over remaining digits, shifting and adding
        let mut count = 0;
        loop {
            let chr = match self.peek()? {
                Some(c) => c,
                None => break,
            };
            if is_digit(chr) {
                count += 1;
                self.consume()?;
                start_val *= 10;
                start_val += get_digit(chr) as i64;
            } else if hex && is_hex_digit(chr) {
                count += 1;
                self.consume()?;
                start_val *= 16;
                start_val += get_digit(chr) as i64;
            } else if chr == b'_' {
                self.consume()?;
            } else {
                break;
            }
        }
        Ok((start_val, count))
    }

    fn read_unicode_escape(&mut self, v: &mut Vec<u8>) -> Result<(), ParserError> {
        let c = self.expect_any()?;
        if c != b'{' {
            return Err(err!(
                self.pos,
                ParserErrorKind::UnexpectedCharacter(c as char)
            ));
        }

        let mut val: u32 = 0;
        loop {
            let c = self.expect_any()?;
            if c == b'}' {
                break;
            } else if !is_hex_digit(c) {
                return Err(err!(
                    self.pos,
                    ParserErrorKind::UnexpectedCharacter(c as char)
                ));
            }
            val = (val * 16) + get_digit(c) as u32;
        }

        let mut buf = [0u8; 4];
        if let Some(c) = std::char::from_u32(val) {
            for b in c.encode_utf8(&mut buf).as_bytes().iter() {
                v.push(*b);
            }

            Ok(())
        } else {
            Err(err!(
                self.pos,
                ParserErrorKind::InvalidEscape,
                "Unicode escape is not a valid character",
            ))
        }
    }
}

#[inline]
fn is_hex_digit(b: u8) -> bool {
    is_digit(b) || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')
}

#[inline]
fn is_digit(b: u8) -> bool {
    b >= b'0' && b <= b'9'
}

#[inline]
fn get_digit(b: u8) -> usize {
    if b >= b'0' && b <= b'9' {
        (b - b'0') as usize
    } else if b >= b'a' && b <= b'f' {
        (b - b'a' + 0xA) as usize
    } else if b >= b'A' && b <= b'F' {
        (b - b'A' + 0xA) as usize
    } else {
        panic!("character is not a digit: {}", b);
    }
}

#[inline]
fn is_idchar(b: u8) -> bool {
    is_atomchar(b)
        || b == b'!'
        || b == b'#'
        || b == b'$'
        || b == b'%'
        || b == b'&'
        || b == b'`'
        || b == b'*'
        || b == b'+'
        || b == b'-'
        || b == b'.'
        || b == b'/'
        || b == b':'
        || b == b'<'
        || b == b'='
        || b == b'>'
        || b == b'?'
        || b == b'@'
        || b == b'\\'
        || b == b'^'
        || b == b'_'
        || b == b'\''
        || b == b'|'
        || b == b'~'
}

#[inline]
fn is_atomchar(b: u8) -> bool {
    (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || (b >= b'0' && b <= b'9') || b == b'_'
}

#[inline]
fn is_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\r' || b == b'\n'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn single_token_lparen() {
        assert_eq!(TokenKind::LParen, single_tok("("));
    }

    #[test]
    pub fn single_token_rparen() {
        assert_eq!(TokenKind::RParen, single_tok(")"));
    }

    #[test]
    pub fn single_token_atom() {
        assert_eq!(
            TokenKind::Atom("h3ll0_w0r1d".to_owned()),
            single_tok("h3ll0_w0r1d")
        );
    }

    #[test]
    pub fn single_token_string() {
        assert_eq!(
            TokenKind::Str("this is a string".to_owned()),
            single_tok("\"this is a string\"")
        );
        assert_eq!(
            TokenKind::Str("some escape sequences: \t\n\r\"\'\\".to_owned()),
            single_tok("\"some escape sequences: \\t\\n\\r\\\"\\\'\\\\\"")
        );
        assert_eq!(
            TokenKind::Str("some escape sequences: +\u{d2}".to_owned()),
            single_tok("\"some escape sequences: \\2B\\d2\"")
        );
        assert_eq!(
            TokenKind::Str("some escape sequences: +\u{d2}ᚠ".to_owned()),
            single_tok("\"some escape sequences: \\u{2B}\\u{d2}\\u{16A0}\"")
        );
    }

    #[test]
    pub fn single_token_identifier() {
        assert_eq!(TokenKind::Identifier("$007".to_owned()), single_tok("$007"));
        assert_eq!(
            TokenKind::Identifier("$h3ll0_w0rld".to_owned()),
            single_tok("$h3ll0_w0rld")
        );
        assert_eq!(
            TokenKind::Identifier("$!#$%&`*+-./:<=>?@\\^_'|~wow".to_owned()),
            single_tok("$!#$%&`*+-./:<=>?@\\^_'|~wow")
        );
    }

    #[test]
    pub fn single_token_int_hex() {
        assert_eq!(TokenKind::Integer(0xA), single_tok("0xA"));
        assert_eq!(TokenKind::Integer(0xA), single_tok("+0xA"));
        assert_eq!(TokenKind::Integer(-0xA), single_tok("-0xA"));
        assert_eq!(TokenKind::Integer(0xA), single_tok("0xa"));
        assert_eq!(TokenKind::Integer(0xA), single_tok("+0xa"));
        assert_eq!(TokenKind::Integer(-0xA), single_tok("-0xa"));
        assert_eq!(TokenKind::Integer(0xAB), single_tok("0xAB"));
        assert_eq!(TokenKind::Integer(0xAB), single_tok("+0xAB"));
        assert_eq!(TokenKind::Integer(-0xAB), single_tok("-0xAB"));
        assert_eq!(TokenKind::Integer(0xAB), single_tok("0xAb"));
        assert_eq!(TokenKind::Integer(0xAB), single_tok("+0xAb"));
        assert_eq!(TokenKind::Integer(-0xAB), single_tok("-0xAb"));
        assert_eq!(TokenKind::Integer(0xAB), single_tok("0xA_B"));
        assert_eq!(TokenKind::Integer(0xAB), single_tok("+0xA_B"));
        assert_eq!(TokenKind::Integer(-0xAB), single_tok("-0xA_B"));
    }

    #[test]
    pub fn single_token_int() {
        assert_eq!(TokenKind::Integer(42), single_tok("4_2"));
        assert_eq!(TokenKind::Integer(42), single_tok("+4_2"));
        assert_eq!(TokenKind::Integer(-42), single_tok("-4_2"));
        assert_eq!(TokenKind::Integer(42), single_tok("42"));
        assert_eq!(TokenKind::Integer(42), single_tok("+42"));
        assert_eq!(TokenKind::Integer(-42), single_tok("-42"));
        assert_eq!(TokenKind::Integer(4), single_tok("4"));
        assert_eq!(TokenKind::Integer(4), single_tok("+4"));
        assert_eq!(TokenKind::Integer(-4), single_tok("-4"));
    }

    #[test]
    pub fn single_token_float() {
        // Hex float to decimal float is weird,
        // this value is "0xAB.CD" in float
        let ab = 0xAB as f64;
        let cd_frac = (0xC as f64 / 0x10 as f64) + (0xD as f64 / 0x10 as f64 / 0x10 as f64);
        let ab_div_cd = ab + cd_frac;
        let p5 = (2f64).powi(5);

        assert_eq!(TokenKind::Float(4.2), single_tok("4.2"));
        assert_eq!(TokenKind::Float(-4.2), single_tok("-4.2"));
        assert_eq!(TokenKind::Float(4.2e5), single_tok("4.2e5"));
        assert_eq!(TokenKind::Float(-4.2e5), single_tok("-4.2e5"));
        assert_eq!(TokenKind::Float(4.2e5), single_tok("4.2E5"));
        assert_eq!(TokenKind::Float(-4.2e5), single_tok("-4.2E5"));
        assert_eq!(TokenKind::Float(ab_div_cd), single_tok("0xAB.CD"));
        assert_eq!(TokenKind::Float(-ab_div_cd), single_tok("-0xAB.CD"));
        assert_eq!(TokenKind::Float(ab_div_cd * p5), single_tok("0xAB.CDp5"));
        assert_eq!(TokenKind::Float(-ab_div_cd * p5), single_tok("-0xAB.CDp5"));
    }

    #[test]
    pub fn single_token_line_comment() {
        assert_eq!(TokenKind::Comment, single_tok(";; Hello, World!\n"));
        assert_eq!(TokenKind::Comment, single_tok(";; Hello, World!\r\n"));

        assert_eq!(vec![TokenKind::Comment], tokenize_str(";; Hello").unwrap());
    }

    #[test]
    pub fn single_token_block_comment() {
        assert_eq!(TokenKind::Comment, single_tok("(;Hello, World!;)"));
        assert_eq!(TokenKind::Comment, single_tok("(;Hello( World!;)"));
        assert_eq!(TokenKind::Comment, single_tok("(;Hello; World!;)"));
        assert_eq!(TokenKind::Comment, single_tok("(;Hello(;,;)World!;)"));

        assert_eq!(
            Err(ParserError::new(ParserErrorKind::UnexpectedEof, (9, 9))),
            tokenize_str("(; Hello")
        );
    }

    #[test]
    pub fn simple_doc() {
        let doc = ";; This is a simple WAST document

(atom 1.45 \"f\\too\" (; goop ;) $id -24)";
        let toks = tokenize_str(doc).unwrap();

        assert_eq!(
            vec![
                TokenKind::Comment,
                TokenKind::LParen,
                TokenKind::Atom("atom".to_owned()),
                TokenKind::Float(1.45),
                TokenKind::Str("f\too".to_owned()),
                TokenKind::Comment,
                TokenKind::Identifier("$id".to_owned()),
                TokenKind::Integer(-24),
                TokenKind::RParen,
            ],
            toks
        );
    }

    fn single_tok(inp: &str) -> TokenKind {
        let actual_str = format!("{} next_token", inp);
        let tok = Tokenizer::new(actual_str.as_bytes());
        let mut iter = tok.map(|t| t.unwrap());
        let first = iter.next().unwrap().kind().clone();

        // Check the second token to make sure the parser didn't get greedy
        assert_eq!(
            Some(TokenKind::Atom("next_token".to_owned())),
            iter.next().map(|t| t.kind().clone())
        );
        assert_eq!(None, iter.next());

        first
    }

    fn tokenize_str(inp: &str) -> Result<Vec<TokenKind>, ParserError> {
        let tok = Tokenizer::new(inp.as_bytes());
        let mut tokens = Vec::new();
        for res in tok {
            tokens.push(res?.kind().clone());
        }
        Ok(tokens)
    }
}
