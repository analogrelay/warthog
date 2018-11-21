use std::fmt;

#[derive(PartialEq, Clone)]
pub struct TextRange(pub TextLocation, pub TextLocation);

impl TextRange {
    pub fn empty() -> TextRange {
        TextRange(TextLocation::start(), TextLocation::start())
    }

    pub fn new(start: TextLocation, end: TextLocation) -> TextRange {
        TextRange(start, end)
    }

    pub fn start(&self) -> TextLocation {
        self.0
    }

    pub fn end(&self) -> TextLocation {
        self.1
    }
}

impl<'a> From<&'a TextRange> for TextRange {
    fn from(v: &'a TextRange) -> TextRange {
        v.clone()
    }
}

impl fmt::Display for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.0, self.1)
    }
}

impl fmt::Debug for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.0, self.1)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct TextLocation {
    /// 0-based character offset represented by the position.
    pub offset: usize,

    /// 1-based line number represented by the position.
    pub line: usize,

    /// 1-based column number represented by the position.
    pub column: usize,
}

impl TextLocation {
    pub fn start() -> TextLocation {
        TextLocation {
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn advance(&self, amount: usize) -> TextLocation {
        TextLocation {
            offset: self.offset + amount,
            line: self.line,
            column: self.column + amount,
        }
    }

    pub fn advance_for(&self, character: u8) -> TextLocation {
        match character {
            // Don't advance column at all.
            b'\r' => TextLocation {
                offset: self.offset + 1,
                line: self.line,
                column: self.column,
            },
            // Next line
            b'\n' => TextLocation {
                offset: self.offset + 1,
                line: self.line + 1,
                column: 1,
            },
            // Other characters
            _ => TextLocation {
                offset: self.offset + 1,
                line: self.line,
                column: self.column + 1,
            },
        }
    }
}

impl fmt::Display for TextLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.line, self.column)
    }
}

impl fmt::Debug for TextLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.offset, self.line, self.column)
    }
}
