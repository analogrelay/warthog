use std::io;

use crate::parser::{ParserError, ParserErrorKind};

#[derive(Debug, Clone, PartialEq)]
pub struct SExpr(pub SVal, pub usize, pub usize);

impl SExpr {
    pub fn new(kind: SVal, start: usize, end: usize) -> SExpr {
        SExpr(kind, start, end)
    }

    pub fn val(&self) -> &SVal {
        &self.0
    }

    pub fn start(&self) -> usize {
        self.1
    }

    pub fn end(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SVal {
    Integer(i64),
    Float(f64),
    Identifier(String),
    Str(String),
    Atom(String),
    Expr(Vec<SExpr>),
}

pub struct SExprParser<R: io::Read> {
    bytes: std::iter::Peekable<io::Bytes<R>>,
    pos: usize,
    token_start: usize,
}

impl<R: io::Read> SExprParser<R> {
    pub fn new(reader: R) -> SExprParser<R> {
        SExprParser {
            bytes: reader.bytes().peekable(),
            pos: 0,
            token_start: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Option<SExpr>, ParserError> {
        // Skip whitespace
        self.skip_to_token()?;

        while let Some(byt) = self.peek()? {
            match byt {
                b'(' => self.read_block_comment(),
                b'a'...b'z' | b'A'...b'Z' | b'_' => return self.read_atom().map(|x| Some(x)),
                b'+' | b'-' | b'0'...b'9' => return self.read_num().map(|x| Some(x)),
                b'$' => return self.read_identifier().map(|x| Some(x)),
                b'"' => return self.read_string().map(|x| Some(x)),
                x => return Err(err!(
                    self.pos,
                    ParserErrorKind::UnexpectedCharacter(x as char)
                )),
            }
        }

        Ok(None)
    }

    fn read_atom(&mut self) -> Result<SExpr, ParserError> {
        let start = self.pos;
        let byts = self.consume_while(is_atomchar)?;

        // We can unwrap the from_utf8 result because we validated each char was ASCII
        let s = String::from_utf8(byts).unwrap();

        Ok(SExpr::new(SVal::Atom(s), start, self.pos))
    }

    fn read_identifier(&mut self) -> Result<SExpr, ParserError> {
        let start = self.pos;
        let byts = self.consume_while(is_idchar)?;

        // We can unwrap the from_utf8 result because we validated each char was ASCII
        let s = String::from_utf8(byts).unwrap();

        Ok(SExpr::new(SVal::Identifier(s), start, self.pos))
    }

    fn read_string(&mut self) -> Result<SExpr, ParserError> {
        let start = self.pos;
        self.consume()?;

        let mut byts = Vec::new();
        loop {
            let chr = self.expect()?;
            if chr == b'\"' {
                break;
            } else if chr == b'\\' {
                match self.expect()? {
                    b't' => byts.push(b'\t'),
                    b'n' => byts.push(b'\n'),
                    b'r' => byts.push(b'\r'),
                    b'"' => byts.push(b'"'),
                    b'\'' => byts.push(b'\''),
                    b'\\' => byts.push(b'\\'),
                    b'u' => self.read_unicode_escape(&mut byts)?,
                    x @ b'0'...b'9' | x @ b'a'...b'f' | x @ b'A'...b'F' => {
                        let y = self.expect()?;
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

        Ok(SExpr::new(SVal::Str(s), start, self.pos))
    }

    fn read_block_comment(&mut self) -> Result<Option<SExpr>, ParserError> {
        self.consume()?; // Consume the '('

        if self.consume_if(b';')? {
            // It's a block comment
            let mut nesting_level = 1;
            while nesting_level > 0 {
                let chr = self.expect()?;

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

            Ok(None)
        } else {
            unimplemented!();
        }
    }

    fn read_line_comment(&mut self) -> Result<(), ParserError> {
        self.consume()?;

        let next = self.expect()?;
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
        Ok(())
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
    fn expect(&mut self) -> Result<u8, ParserError> {
        match self.consume() {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err(err!(self.pos, ParserErrorKind::UnexpectedEof)),
            Err(e) => Err(e),
        }
    }

    fn read_unicode_escape(&mut self, v: &mut Vec<u8>) -> Result<(), ParserError> {
        let c = self.expect()?;
        if c != b'{' {
            return Err(err!(
                self.pos,
                ParserErrorKind::UnexpectedCharacter(c as char)
            ));
        }

        let mut val: u32 = 0;
        loop {
            let c = self.expect()?;
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

    fn read_num(&mut self) -> Result<SExpr, ParserError> {
        let start = self.pos;

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
        let first_digit = self.expect()?;
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
            self.expect()?;

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
                Ok(SExpr::new(SVal::Float(self.read_float(negate, nat, hex)?), start, self.pos))
            }
            _ => Ok(SExpr::new(SVal::Integer(if negate { -nat } else { nat }), start, self.pos))
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

    #[inline]
    fn consume_if(&mut self, expected: u8) -> Result<bool, ParserError> {
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
    pub fn single_token_atom() {
        assert_eq!(
            SExpr::new(SVal::Atom("h3ll0_w0r1d".to_owned()), 0, 11),
            single_expr("h3ll0_w0r1d")
        );
    }

    #[test]
    pub fn single_token_string() {
        assert_eq!(
            SExpr::new(SVal::Str("this is a string".to_owned()), 0, 18),
            single_expr("\"this is a string\"")
        );
        assert_eq!(
            SExpr::new(
                SVal::Str("some escape sequences: \t\n\r\"\'\\".to_owned()),
                0,
                37
            ),
            single_expr("\"some escape sequences: \\t\\n\\r\\\"\\\'\\\\\"")
        );
        assert_eq!(
            SExpr::new(
                SVal::Str("some escape sequences: +\u{d2}".to_owned()),
                0,
                31
            ),
            single_expr("\"some escape sequences: \\2B\\d2\"")
        );
        assert_eq!(
            SExpr::new(
                SVal::Str("some escape sequences: +\u{d2}ᚠ".to_owned()),
                0,
                45
            ),
            single_expr("\"some escape sequences: \\u{2B}\\u{d2}\\u{16A0}\"")
        );
    }

    #[test]
    pub fn single_token_identifier() {
        assert_eq!(
            SExpr::new(SVal::Identifier("$007".to_owned()), 0, 4),
            single_expr("$007")
        );

        assert_eq!(
            SExpr::new(SVal::Identifier("$h3ll0_w0rld".to_owned()), 0, 12),
            single_expr("$h3ll0_w0rld")
        );

        assert_eq!(
            SExpr::new(
                SVal::Identifier("$!#$%&`*+-./:<=>?@\\^_'|~wow".to_owned()),
                0,
                27
            ),
            single_expr("$!#$%&`*+-./:<=>?@\\^_'|~wow")
        );
    }

        #[test]
        pub fn single_token_int_hex() {
            assert_eq!(SExpr::new(SVal::Integer(0xA), 0, 3), single_expr("0xA"));
            assert_eq!(SExpr::new(SVal::Integer(0xA), 0, 4), single_expr("+0xA"));
            assert_eq!(SExpr::new(SVal::Integer(-0xA), 0, 4), single_expr("-0xA"));
            assert_eq!(SExpr::new(SVal::Integer(0xA), 0, 3), single_expr("0xa"));
            assert_eq!(SExpr::new(SVal::Integer(0xA), 0, 4), single_expr("+0xa"));
            assert_eq!(SExpr::new(SVal::Integer(-0xA), 0, 4), single_expr("-0xa"));
            assert_eq!(SExpr::new(SVal::Integer(0xAB), 0, 4), single_expr("0xAB"));
            assert_eq!(SExpr::new(SVal::Integer(0xAB), 0, 5), single_expr("+0xAB"));
            assert_eq!(SExpr::new(SVal::Integer(-0xAB), 0, 5), single_expr("-0xAB"));
            assert_eq!(SExpr::new(SVal::Integer(0xAB), 0, 4), single_expr("0xAb"));
            assert_eq!(SExpr::new(SVal::Integer(0xAB), 0, 5), single_expr("+0xAb"));
            assert_eq!(SExpr::new(SVal::Integer(-0xAB), 0, 5), single_expr("-0xAb"));
            assert_eq!(SExpr::new(SVal::Integer(0xAB), 0, 5), single_expr("0xA_B"));
            assert_eq!(SExpr::new(SVal::Integer(0xAB), 0, 6), single_expr("+0xA_B"));
            assert_eq!(SExpr::new(SVal::Integer(-0xAB), 0, 6), single_expr("-0xA_B"));
        }

        #[test]
        pub fn single_expren_int() {
            assert_eq!(SExpr::new(SVal::Integer(42), 0, 3), single_expr("4_2"));
            assert_eq!(SExpr::new(SVal::Integer(42), 0, 4), single_expr("+4_2"));
            assert_eq!(SExpr::new(SVal::Integer(-42), 0, 4), single_expr("-4_2"));
            assert_eq!(SExpr::new(SVal::Integer(42), 0, 2), single_expr("42"));
            assert_eq!(SExpr::new(SVal::Integer(42), 0, 3), single_expr("+42"));
            assert_eq!(SExpr::new(SVal::Integer(-42), 0, 3), single_expr("-42"));
            assert_eq!(SExpr::new(SVal::Integer(4), 0, 1), single_expr("4"));
            assert_eq!(SExpr::new(SVal::Integer(4), 0, 2), single_expr("+4"));
            assert_eq!(SExpr::new(SVal::Integer(-4), 0, 2), single_expr("-4"));
        }

        #[test]
        pub fn single_token_float() {
            // Hex float to decimal float is weird,
            // this value is "0xAB.CD" in float
            let ab = 0xAB as f64;
            let cd_frac = (0xC as f64 / 0x10 as f64) + (0xD as f64 / 0x10 as f64 / 0x10 as f64);
            let ab_div_cd = ab + cd_frac;
            let p5 = (2f64).powi(5);

            assert_eq!(SExpr::new(SVal::Float(4.2), 0, 3), single_expr("4.2"));
            assert_eq!(SExpr::new(SVal::Float(-4.2), 0, 4), single_expr("-4.2"));
            assert_eq!(SExpr::new(SVal::Float(4.2e5), 0, 5), single_expr("4.2e5"));
            assert_eq!(SExpr::new(SVal::Float(-4.2e5), 0, 6), single_expr("-4.2e5"));
            assert_eq!(SExpr::new(SVal::Float(4.2e5), 0, 5), single_expr("4.2E5"));
            assert_eq!(SExpr::new(SVal::Float(-4.2e5), 0, 6), single_expr("-4.2E5"));
            assert_eq!(SExpr::new(SVal::Float(ab_div_cd), 0, 7), single_expr("0xAB.CD"));
            assert_eq!(SExpr::new(SVal::Float(-ab_div_cd), 0, 8), single_expr("-0xAB.CD"));
            assert_eq!(SExpr::new(SVal::Float(ab_div_cd * p5), 0, 9), single_expr("0xAB.CDp5"));
            assert_eq!(SExpr::new(SVal::Float(-ab_div_cd * p5), 0, 10), single_expr("-0xAB.CDp5"));
        }

    //     #[test]
    //     pub fn single_token_block_comment() {
    //         assert_eq!(TokenKind::Comment, single_tok("(;Hello, World!;)"));
    //         assert_eq!(TokenKind::Comment, single_tok("(;Hello( World!;)"));
    //         assert_eq!(TokenKind::Comment, single_tok("(;Hello; World!;)"));
    //         assert_eq!(TokenKind::Comment, single_tok("(;Hello(;,;)World!;)"));

    //         assert_eq!(
    //             Err(ParserError::new(ParserErrorKind::UnexpectedEof, (9, 9))),
    //             tokenize_str("(; Hello")
    //         );
    //     }

    //     #[test]
    //     pub fn simple_doc() {
    //         let doc = ";; This is a simple WAST document

    // (atom 1.45 \"f\\too\" (; goop ;) $id -24)";
    //         let toks = tokenize_str(doc).unwrap();

    //         assert_eq!(
    //             vec![
    //                 TokenKind::Comment,
    //                 TokenKind::LParen,
    //                 TokenKind::Atom("atom".to_owned()),
    //                 TokenKind::Float(1.45),
    //                 TokenKind::Str("f\too".to_owned()),
    //                 TokenKind::Comment,
    //                 TokenKind::Identifier("$id".to_owned()),
    //                 TokenKind::Integer(-24),
    //                 TokenKind::RParen,
    //             ],
    //             toks
    //         );
    //     }

    fn single_expr(inp: &str) -> SExpr {
        let actual = format!(";; Line comment\n   (; block comment;)  \n   {}", inp);
        let size_diff = actual.len() - inp.len();
        let mut tokens = parse_to_end(&actual).unwrap();
        let mut iter = tokens.drain(..);
        let first = iter.next().unwrap();

        assert_eq!(None, iter.next());

        // Adjust the start and end to make the test easier
        let SExpr(val, start, end) = first;
        SExpr::new(val, start - size_diff, end - size_diff)
    }

    fn parse_to_end(inp: &str) -> Result<Vec<SExpr>, ParserError> {
        let mut tok = SExprParser::new(inp.as_bytes());
        let mut tokens = Vec::new();
        while let Some(expr) = tok.parse()? {
            tokens.push(expr);
        }
        Ok(tokens)
    }
}
