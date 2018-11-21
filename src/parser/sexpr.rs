use std::{collections::VecDeque, fmt, io};

use crate::parser::{ParserError, ParserErrorKind, TextLocation, TextRange};

#[derive(Debug, Clone, PartialEq)]
pub struct SExpr(pub SVal, pub TextRange);

impl SExpr {
    pub fn new(kind: SVal, range: TextRange) -> SExpr {
        SExpr(kind, range)
    }

    pub fn range(&self) -> &TextRange {
        &self.1
    }

    pub fn keyword(&self) -> Option<&str> {
        match self {
            SExpr(SVal::Atom(ref s), _) => Some(s),
            _ => None,
        }
    }

    pub fn consume_int(self) -> Result<(u64, TextRange), ParserError> {
        match self {
            SExpr(SVal::Integer(i), range) => Ok((i, range)),
            SExpr(x, range) => Err(err!(
                range,
                ParserErrorKind::UnexpectedToken,
                format!("Expected an Integer, but found: '{:?}'", x)
            )),
        }
    }

    pub fn consume_expr(self) -> Result<(VecDeque<SExpr>, TextRange), ParserError> {
        match self {
            SExpr(SVal::Expr(items), range) => Ok((items, range)),
            SExpr(x, range) => Err(err!(
                range,
                ParserErrorKind::UnexpectedToken,
                format!("Expected an Expr, but found: '{:?}'", x)
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SVal {
    Integer(u64),
    Float(f64),
    Identifier(String),
    Str(String),
    Atom(String),
    Expr(VecDeque<SExpr>),
}

impl SVal {
    #[cfg(test)]
    pub fn new_expr<I: IntoIterator<Item = SExpr>>(content: I) -> SVal {
        SVal::Expr(content.into_iter().collect())
    }
}

impl fmt::Display for SVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SVal::Integer(x) => write!(f, "{}", x),
            SVal::Float(x) => write!(f, "{}", x),
            SVal::Identifier(x) => write!(f, "{}", x),
            SVal::Str(x) => write!(f, "\"{}\"", escape(x)),
            SVal::Atom(x) => write!(f, "{}", x),
            SVal::Expr(_) => write!(f, "<sub-expression>"),
        }
    }
}

fn escape(x: &str) -> String {
    let mut s = String::new();
    for c in x.chars().flat_map(|y| y.escape_default()) {
        s.push(c);
    }
    s
}

pub struct SExprParser<R: io::Read> {
    bytes: std::iter::Peekable<io::Bytes<R>>,
    loc: TextLocation,
}

impl<R: io::Read> SExprParser<R> {
    pub fn new(reader: R) -> SExprParser<R> {
        SExprParser {
            bytes: reader.bytes().peekable(),
            loc: TextLocation::start(),
        }
    }

    pub fn parse(&mut self) -> Result<Option<SExpr>, ParserError> {
        self.parse_expr(false)
    }

    fn parse_expr(&mut self, in_sexpr: bool) -> Result<Option<SExpr>, ParserError> {
        while let Some((byt, start)) = self.skip_to_token()? {
            match byt {
                b'(' => return self.read_expr(start).map(|x| Some(x)),
                b'a'...b'z' => return self.read_atom(start).map(|x| Some(x)),
                b'+' | b'-' | b'0'...b'9' => return self.read_num(start).map(|x| Some(x)),
                b'$' => return self.read_identifier(start).map(|x| Some(x)),
                b'"' => return self.read_string(start).map(|x| Some(x)),
                b')' if in_sexpr => return Ok(None),
                x => {
                    self.consume()?;
                    return Err(err!(
                        TextRange(start, self.loc),
                        ParserErrorKind::UnexpectedCharacter(x as char),
                        format!("Unexpected '{}' in s-expression.", x as char)
                    ));
                }
            }
        }

        if in_sexpr {
            return Err(err!(
                TextRange(self.loc, self.loc),
                ParserErrorKind::UnexpectedEof,
                "Unexpected end-of-file in s-expression."
            ));
        } else {
            Ok(None)
        }
    }

    fn read_expr(&mut self, start: TextLocation) -> Result<SExpr, ParserError> {
        // NOTE: The '(' has already been consumed!
        let mut exprs = VecDeque::new();
        loop {
            if let Some(exp) = self.parse_expr(true)? {
                exprs.push_back(exp);
            } else {
                // End of the expression
                self.consume()?;
                return Ok(SExpr::new(SVal::Expr(exprs), TextRange(start, self.loc)));
            }
        }
    }

    fn read_atom(&mut self, start: TextLocation) -> Result<SExpr, ParserError> {
        let (byts, range) = self.consume_while(is_idchar)?;

        // We can unwrap the from_utf8 result because we validated each char was ASCII
        let s = String::from_utf8(byts).unwrap();

        Ok(SExpr::new(SVal::Atom(s), TextRange(start, range.end())))
    }

    fn read_identifier(&mut self, start: TextLocation) -> Result<SExpr, ParserError> {
        let (byts, range) = self.consume_while(is_idchar)?;

        // We can unwrap the from_utf8 result because we validated each char was ASCII
        let s = String::from_utf8(byts).unwrap();

        Ok(SExpr::new(
            SVal::Identifier(s),
            TextRange(start, range.end()),
        ))
    }

    fn read_string(&mut self, start: TextLocation) -> Result<SExpr, ParserError> {
        self.consume()?;

        let mut byts = Vec::new();
        loop {
            let (chr, _) = self.expect("Unexpected end-of-file in string.")?;
            if chr == b'\"' {
                break;
            } else if chr == b'\\' {
                match self.expect("Unexpected end-of-file in string.")? {
                    (b't', _) => byts.push(b'\t'),
                    (b'n', _) => byts.push(b'\n'),
                    (b'r', _) => byts.push(b'\r'),
                    (b'"', _) => byts.push(b'"'),
                    (b'\'', _) => byts.push(b'\''),
                    (b'\\', _) => byts.push(b'\\'),
                    (b'u', s) => self.read_unicode_escape(&mut byts, s)?,
                    (x, s) if is_hex_digit(x) => {
                        let (y, e) = self.expect("Unexpected end-of-file in hex escape.")?;
                        if !is_hex_digit(y) {
                            return Err(err!(
                                TextRange(s, e),
                                ParserErrorKind::UnexpectedCharacter(y as char),
                                "Unexpected character in hex escape",
                            ));
                        }

                        let val = (get_digit(x) << 4) + get_digit(y);

                        // We know that the hex escape is between 0x00 and 0xFF, so it's legal.
                        let c = std::char::from_u32(val as u32).expect(
                            "Expected that the hex escape would be a legal Unicode character!",
                        );
                        let mut buf = [0u8; 4];
                        for b in c.encode_utf8(&mut buf).as_bytes().iter() {
                            byts.push(*b);
                        }
                    }
                    (x, s) => {
                        return Err(err!(
                            TextRange(s, self.loc),
                            ParserErrorKind::InvalidEscape,
                            format!("Invalid escape sequence: '\\{}'", x as char)
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
            Err(e) => return Err(err!(TextRange(start, self.loc), e)),
        };

        Ok(SExpr::new(SVal::Str(s), TextRange(start, self.loc)))
    }

    fn read_block_comment(&mut self) -> Result<(), ParserError> {
        self.consume()?; // Consume the ';'

        // It's a block comment
        let mut nesting_level = 1;
        while nesting_level > 0 {
            let (chr, _) = self.expect("Unexpected end-of-file in block comment.")?;

            match (chr, self.peek()?) {
                (b'(', Some((b';', _))) => {
                    self.consume()?;
                    nesting_level += 1;
                }
                (b';', Some((b')', _))) => {
                    self.consume()?;
                    nesting_level -= 1;
                }

                // Skip everything else.
                _ => {}
            }
        }
        Ok(())
    }

    // Inverts the Option and Result returned by `bytes.next` to make `?` work better
    #[inline]
    fn peek(&mut self) -> Result<Option<(u8, TextLocation)>, ParserError> {
        match self.bytes.peek() {
            Some(Ok(b)) => Ok(Some((*b, self.loc))),
            Some(Err(e)) => Err(err!(TextRange(self.loc, self.loc), e)),
            None => Ok(None),
        }
    }

    #[inline]
    fn skip_while<P: FnMut(u8) -> bool>(&mut self, mut predicate: P) -> Result<(), ParserError> {
        loop {
            if let Some((byt, _)) = self.peek()? {
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

    fn skip_to_token(&mut self) -> Result<Option<(u8, TextLocation)>, ParserError> {
        loop {
            self.skip_while(is_whitespace)?;
            match self.peek()? {
                Some((b'(', start)) => {
                    self.consume()?;
                    if let Some((b';', _)) = self.peek()? {
                        self.read_block_comment()?;
                    // Keep looping.
                    } else {
                        return Ok(Some((b'(', start)));
                    }
                }
                Some((b';', start)) => {
                    self.consume()?;
                    match self.peek()? {
                        Some((b';', _)) => {
                            // Skip line comment
                            self.consume()?;
                            self.skip_while(|b| b != b'\n')?;
                            self.consume()?;
                            // Keep looping.
                        }
                        Some((x, _)) => {
                            return Err(err!(
                                TextRange(start, self.loc),
                                ParserErrorKind::UnexpectedCharacter(x as char),
                                format!("Unexpected '{}' after ';'.", x as char)
                            ))
                        }
                        None => {
                            return Err(err!(
                                TextRange(start, self.loc),
                                ParserErrorKind::UnexpectedEof,
                                "Unexpected end-of-file after ';'."
                            ))
                        }
                    }
                }
                Some(x) => return Ok(Some(x)),
                None => {
                    return Ok(None);
                }
            }
        }
    }

    #[inline]
    fn consume_while<P: FnMut(u8) -> bool>(
        &mut self,
        mut predicate: P,
    ) -> Result<(Vec<u8>, TextRange), ParserError> {
        let mut vals = Vec::new();
        let start = self.loc;
        loop {
            if let Some((byt, _)) = self.peek()? {
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
        Ok((vals, TextRange(start, self.loc)))
    }

    #[inline]
    fn consume(&mut self) -> Result<Option<(u8, TextLocation)>, ParserError> {
        let loc = self.loc;
        match self.bytes.next() {
            Some(Ok(b)) => {
                self.loc = self.loc.advance_for(b);
                Ok(Some((b, loc)))
            }
            Some(Err(e)) => Err(err!(TextRange(self.loc, self.loc), e)),
            None => Ok(None),
        }
    }

    #[inline]
    fn expect<S: Into<String>>(&mut self, error: S) -> Result<(u8, TextLocation), ParserError> {
        match self.consume() {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err(err!(
                TextRange(self.loc, self.loc.advance(1)),
                ParserErrorKind::UnexpectedEof,
                error.into()
            )),
            Err(e) => Err(e),
        }
    }

    fn read_unicode_escape(
        &mut self,
        v: &mut Vec<u8>,
        start: TextLocation,
    ) -> Result<(), ParserError> {
        let (c, _) = self.expect("Unexpected end-of-file in unicode escape sequence.")?;
        if c != b'{' {
            return Err(err!(
                TextRange(start, self.loc),
                ParserErrorKind::UnexpectedCharacter(c as char),
                format!("Unexpected '{}' in unicode escape.", c as char)
            ));
        }

        let mut val: u32 = 0;
        loop {
            let (c, _) = self.expect("Unexpected end-of-file in unicode escape sequence.")?;
            if c == b'}' {
                break;
            } else if !is_hex_digit(c) {
                return Err(err!(
                    TextRange(start, self.loc),
                    ParserErrorKind::UnexpectedCharacter(c as char),
                    format!("Unexpected '{}' in unicode escape.", c as char)
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
                TextRange(start, self.loc),
                ParserErrorKind::InvalidEscape,
                "Unicode escape is not a valid character.",
            ))
        }
    }

    fn read_num(&mut self, start: TextLocation) -> Result<SExpr, ParserError> {
        // Read the optional sign
        let negate = match self.peek()? {
            Some((b'+', _)) => {
                self.consume()?;
                false
            }
            Some((b'-', _)) => {
                self.consume()?;
                true
            }
            _ => false,
        };

        // Read the first digit
        let (first_digit, s) = self.expect("Unexpected end-of-file in number.")?;
        if first_digit < b'0' || first_digit > b'9' {
            return Err(err!(
                TextRange(s, self.loc),
                ParserErrorKind::UnexpectedCharacter(first_digit as char)
            ));
        }

        // Check if we're a hex number
        let mut nat = 0;
        let hex = if let Some((b'x', _)) = self.peek()? {
            // Ignore first_digit and this character
            self.expect("Unexpected end-of-file in number.")?;

            // Mark the number as hex
            true
        } else {
            // That was the first digit of the number
            nat = (first_digit - b'0') as u64;
            false
        };

        // Read the rest of the number
        let (nat, _) = self.read_digits(nat, hex)?;

        // Check if this is a float
        match self.peek()? {
            Some((b'.', _)) | Some((b'E', _)) | Some((b'e', _)) => Ok(SExpr::new(
                SVal::Float(self.read_float(negate, nat, hex)?),
                TextRange(start, self.loc),
            )),
            _ => Ok(SExpr::new(
                SVal::Integer(if negate { nat.wrapping_neg() } else { nat }),
                TextRange(start, self.loc),
            )),
        }
    }

    fn read_float(&mut self, negate: bool, nat: u64, hex: bool) -> Result<f64, ParserError> {
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
            (true, Some((b'p', _))) | (true, Some((b'P', _))) => {
                self.consume()?;
                let (exp, _) = self.read_digits(0, hex)?;
                let multiplier = 2f64.powi(exp as i32);
                val *= multiplier;
            }
            (false, Some((b'e', _))) | (false, Some((b'E', _))) => {
                self.consume()?;
                let (exp, _) = self.read_digits(0, hex)?;
                let multiplier = 10f64.powi(exp as i32);
                val *= multiplier;
            }
            _ => {}
        }

        Ok(if negate { -val } else { val })
    }

    fn read_digits(&mut self, mut start_val: u64, hex: bool) -> Result<(u64, i32), ParserError> {
        // Iterate over remaining digits, shifting and adding
        let mut count = 0;
        loop {
            let chr = match self.peek()? {
                Some((c, _)) => c,
                None => break,
            };
            if !hex && is_digit(chr) {
                count += 1;
                self.consume()?;
                start_val *= 10;
                start_val += get_digit(chr) as u64;
            } else if hex && is_hex_digit(chr) {
                count += 1;
                self.consume()?;
                start_val *= 16;
                start_val += get_digit(chr) as u64;
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
            Some(Err(e)) => return Err(err!(TextRange(self.loc, self.loc), e)),
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
    is_alphanum(b)
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
fn is_alphanum(b: u8) -> bool {
    (b >= b'a' && b <= b'z')
        || (b >= b'A' && b <= b'Z')
        || (b >= b'0' && b <= b'9')
        || b == b'_'
        || b == b'.'
}

#[inline]
fn is_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\r' || b == b'\n'
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! range {
        ($start:expr, $end: expr) => {
            TextRange(
                TextLocation {
                    offset: $start,
                    line: 1,
                    column: $start + 1,
                },
                TextLocation {
                    offset: $end,
                    line: 1,
                    column: $end + 1,
                },
            )
        };
    }

    #[test]
    pub fn single_token_atom() {
        assert_eq!(
            SExpr::new(SVal::Atom("h3ll0_w0r1d".to_owned()), range!(0, 11)),
            single_expr("h3ll0_w0r1d")
        );
        assert_eq!(
            SExpr::new(SVal::Atom("a.b.c.d".to_owned()), range!(0, 7)),
            single_expr("a.b.c.d")
        );
        assert_eq!(
            SExpr::new(SVal::Atom("ab/trunc_f/2/d$".to_owned()), range!(0, 15)),
            single_expr("ab/trunc_f/2/d$")
        );
    }

    #[test]
    pub fn single_token_string() {
        assert_eq!(
            SExpr::new(SVal::Str("this is a string".to_owned()), range!(0, 18)),
            single_expr("\"this is a string\"")
        );
        assert_eq!(
            SExpr::new(
                SVal::Str("some escape sequences: \t\n\r\"\'\\".to_owned()),
                range!(0, 37)
            ),
            single_expr("\"some escape sequences: \\t\\n\\r\\\"\\\'\\\\\"")
        );
        assert_eq!(
            SExpr::new(
                SVal::Str("some escape sequences: +\u{d2}".to_owned()),
                range!(0, 31)
            ),
            single_expr("\"some escape sequences: \\2B\\d2\"")
        );
        assert_eq!(
            SExpr::new(
                SVal::Str("some escape sequences: +\u{d2}ᚠ".to_owned()),
                range!(0, 45)
            ),
            single_expr("\"some escape sequences: \\u{2B}\\u{d2}\\u{16A0}\"")
        );
    }

    #[test]
    pub fn single_token_identifier() {
        assert_eq!(
            SExpr::new(SVal::Identifier("$007".to_owned()), range!(0, 4)),
            single_expr("$007")
        );

        assert_eq!(
            SExpr::new(SVal::Identifier("$h3ll0_w0rld".to_owned()), range!(0, 12)),
            single_expr("$h3ll0_w0rld")
        );

        assert_eq!(
            SExpr::new(
                SVal::Identifier("$!#$%&`*+-./:<=>?@\\^_'|~wow".to_owned()),
                range!(0, 27)
            ),
            single_expr("$!#$%&`*+-./:<=>?@\\^_'|~wow")
        );
    }

    #[test]
    pub fn single_token_int_hex() {
        // Regression test: Hex digits that look like Dec digits
        assert_eq!(
            SExpr::new(SVal::Integer(0x80000000), range!(0, 10)),
            single_expr("0x80000000")
        );

        assert_eq!(
            SExpr::new(SVal::Integer(0xA), range!(0, 3)),
            single_expr("0xA")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xA), range!(0, 4)),
            single_expr("+0xA")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFFF6), range!(0, 4)),
            single_expr("-0xA")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xA), range!(0, 3)),
            single_expr("0xa")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xA), range!(0, 4)),
            single_expr("+0xa")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFFF6), range!(0, 4)),
            single_expr("-0xa")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xAB), range!(0, 4)),
            single_expr("0xAB")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xAB), range!(0, 5)),
            single_expr("+0xAB")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFF55), range!(0, 5)),
            single_expr("-0xAB")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xAB), range!(0, 4)),
            single_expr("0xAb")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xAB), range!(0, 5)),
            single_expr("+0xAb")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFF55), range!(0, 5)),
            single_expr("-0xAb")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xAB), range!(0, 5)),
            single_expr("0xA_B")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xAB), range!(0, 6)),
            single_expr("+0xA_B")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFF55), range!(0, 6)),
            single_expr("-0xA_B")
        );
    }

    #[test]
    pub fn single_expr_int() {
        assert_eq!(
            SExpr::new(SVal::Integer(42), range!(0, 3)),
            single_expr("4_2")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(42), range!(0, 4)),
            single_expr("+4_2")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFFD6), range!(0, 4)),
            single_expr("-4_2")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(42), range!(0, 2)),
            single_expr("42")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(42), range!(0, 3)),
            single_expr("+42")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFFD6), range!(0, 3)),
            single_expr("-42")
        );
        assert_eq!(SExpr::new(SVal::Integer(4), range!(0, 1)), single_expr("4"));
        assert_eq!(
            SExpr::new(SVal::Integer(4), range!(0, 2)),
            single_expr("+4")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(0xFFFFFFFFFFFFFFFC), range!(0, 2)),
            single_expr("-4")
        );
    }

    #[test]
    pub fn single_token_float() {
        // Hex float to decimal float is weird,
        // this value is "0xAB.CD" in float
        let ab = 0xAB as f64;
        let cd_frac = (0xC as f64 / 0x10 as f64) + (0xD as f64 / 0x10 as f64 / 0x10 as f64);
        let ab_div_cd = ab + cd_frac;
        let p5 = (2f64).powi(5);

        assert_eq!(
            SExpr::new(SVal::Float(4.2), range!(0, 3)),
            single_expr("4.2")
        );
        assert_eq!(
            SExpr::new(SVal::Float(-4.2), range!(0, 4)),
            single_expr("-4.2")
        );
        assert_eq!(
            SExpr::new(SVal::Float(4.2e5), range!(0, 5)),
            single_expr("4.2e5")
        );
        assert_eq!(
            SExpr::new(SVal::Float(-4.2e5), range!(0, 6)),
            single_expr("-4.2e5")
        );
        assert_eq!(
            SExpr::new(SVal::Float(4.2e5), range!(0, 5)),
            single_expr("4.2E5")
        );
        assert_eq!(
            SExpr::new(SVal::Float(-4.2e5), range!(0, 6)),
            single_expr("-4.2E5")
        );
        assert_eq!(
            SExpr::new(SVal::Float(ab_div_cd), range!(0, 7)),
            single_expr("0xAB.CD")
        );
        assert_eq!(
            SExpr::new(SVal::Float(-ab_div_cd), range!(0, 8)),
            single_expr("-0xAB.CD")
        );
        assert_eq!(
            SExpr::new(SVal::Float(ab_div_cd * p5), range!(0, 9)),
            single_expr("0xAB.CDp5")
        );
        assert_eq!(
            SExpr::new(SVal::Float(-ab_div_cd * p5), range!(0, 10)),
            single_expr("-0xAB.CDp5")
        );
    }

    #[test]
    pub fn exprs() {
        assert_eq!(
            SExpr::new(SVal::new_expr(vec![]), range!(0, 2)),
            single_expr("()")
        );
        assert_eq!(
            SExpr::new(
                SVal::new_expr(vec![SExpr::new(SVal::Integer(42), range!(1, 3))]),
                range!(0, 4)
            ),
            single_expr("(42)")
        );
        assert_eq!(
            SExpr::new(
                SVal::new_expr(vec![
                    SExpr::new(SVal::Atom("module".to_owned()), range!(1, 7)),
                    SExpr::new(
                        SVal::new_expr(vec![
                            SExpr::new(SVal::Atom("func".to_owned()), range!(9, 13)),
                            SExpr::new(SVal::Identifier("$abc".to_owned()), range!(43, 47)),
                            SExpr::new(SVal::Float(4.2), range!(48, 51))
                        ]),
                        range!(8, 52)
                    )
                ]),
                range!(0, 53)
            ),
            single_expr("(module (func (;comment(;nested;)comment;) $abc 4.2))")
        );
    }

    #[test]
    pub fn line_comments() {
        assert_eq!(
            SExpr::new(
                SVal::Integer(42),
                TextRange(
                    TextLocation {
                        offset: 11,
                        line: 2,
                        column: 1
                    },
                    TextLocation {
                        offset: 13,
                        line: 2,
                        column: 3
                    },
                )
            ),
            single_expr(";; Comment\n42")
        );
        assert_eq!(
            SExpr::new(SVal::Integer(42), range!(0, 2)),
            single_expr("42;; Comment")
        );
    }

    #[test]
    pub fn error_expr() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedEof,
                range!(1, 1),
                "Unexpected end-of-file in s-expression."
            ),
            parse_err("(")
        );
    }

    #[test]
    pub fn error_string() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::InvalidEscape,
                range!(6, 7),
                "Invalid escape sequence: '\\?'"
            ),
            parse_err("\"foo \\?\"")
        );

        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedEof,
                range!(4, 5),
                "Unexpected end-of-file in string.",
            ),
            parse_err("\"foo")
        );

        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedEof,
                range!(2, 3),
                "Unexpected end-of-file in string.",
            ),
            parse_err("\"\\")
        );

        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::InvalidEscape,
                range!(2, 9),
                "Unicode escape is not a valid character."
            ),
            parse_err("\"\\u{D801}")
        );

        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedCharacter('?'),
                range!(2, 5),
                "Unexpected '?' in unicode escape.",
            ),
            parse_err("\"\\u{?")
        );

        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedCharacter('?'),
                range!(2, 4),
                "Unexpected '?' in unicode escape.",
            ),
            parse_err("\"\\u?")
        );

        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedEof,
                range!(4, 5),
                "Unexpected end-of-file in unicode escape sequence.",
            ),
            parse_err("\"\\u{")
        );
    }

    #[test]
    pub fn error_start_char() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedCharacter(')'),
                range!(0, 1),
                "Unexpected ')' in s-expression."
            ),
            parse_err(")")
        );
    }

    #[test]
    pub fn error_block_comment() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedEof,
                range!(2, 3),
                "Unexpected end-of-file in block comment."
            ),
            parse_err("(;")
        );
    }

    #[test]
    pub fn error_line_comment() {
        assert_eq!(
            ParserError::detailed(
                ParserErrorKind::UnexpectedEof,
                range!(0, 1),
                "Unexpected end-of-file after ';'."
            ),
            parse_err(";")
        );
    }

    fn parse_err(inp: &str) -> ParserError {
        parse_to_end(inp).unwrap_err()
    }

    fn single_expr(inp: &str) -> SExpr {
        let mut tokens = parse_to_end(&inp).unwrap();
        let mut iter = tokens.drain(..);
        let first = iter.next().unwrap();

        assert_eq!(None, iter.next());

        first
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
