// Compile instructions.csv into Rust instructions files

extern crate csv;

use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

use self::InstructionType::*;

#[derive(PartialEq, Eq)]
pub enum InstructionType {
    Empty,
    Block,
    Index,
    Const,
    BranchTable,
    TableIndex,
    MemArg,
}

pub struct InstructionRecord {
    pub opcode: usize,
    pub old_name: String,
    pub new_name: String,
    pub enum_name: String,
    pub enum_ref: String,
    pub typ: InstructionType,
}

pub struct IndentingWriter<W: Write> {
    writer: W,
    indent_level: usize,
    start_of_line: bool,
}

static INDENT: &'static [u8] = b"    ";
static NEWLINE: &'static [u8] = b"\n";

impl<W: Write> IndentingWriter<W> {
    pub fn new(writer: W) -> IndentingWriter<W> {
        IndentingWriter {
            writer,
            indent_level: 0,
            start_of_line: true,
        }
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        let new_level = if self.indent_level > 0 {
            self.indent_level - 1
        } else {
            self.indent_level
        };
        self.indent_level = new_level;
    }

    pub fn start_block(&mut self, text: &str) -> io::Result<usize> {
        let count = self.writeln(text)?;
        self.indent();
        Ok(count)
    }

    pub fn end_block(&mut self, text: &str) -> io::Result<usize> {
        self.dedent();
        self.writeln(text)
    }

    pub fn block<F: FnOnce(&mut IndentingWriter<W>) -> io::Result<()>>(&mut self, start: &str, b: F) -> io::Result<()> {
        self.start_block(start)?;
        b(self)?;
        self.end_block("}")?;
        Ok(())
    }

    fn write_indent(&mut self) -> io::Result<usize> {
        let mut count = 0;
        if self.start_of_line {
            for _ in 0..self.indent_level {
                count += self.writer.write(INDENT)?;
            }
            self.start_of_line = false;
        }

        Ok(count)
    }

    pub fn writeln(&mut self, str: &str) -> io::Result<usize> {
        let count = self.write(str.as_bytes())?;
        Ok(count + self.write(NEWLINE)?)
    }
}

impl<W: Write> Write for IndentingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut count = 0;

        let lines: Vec<_> = buf.split(|x| *x == b'\n').collect();
        for (i, line) in lines.iter().enumerate() {
            if line.len() > 0 {
                // Don't add indent data to the count
                self.write_indent()?;
                self.writer.write_all(line)?;
                count += line.len();
            }

            // For all segments except the last one, reset the
            // start-of-line flag and add one to the count
            // to include the newline character itself.
            if i < (lines.len() - 1) {
                self.writer.write_all(NEWLINE)?;
                self.start_of_line = true;
                count += NEWLINE.len();
            }
        }

        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

fn main() {
    let instructions: Vec<_> = {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let input_path = Path::new(&manifest_dir).join("instructions.csv");;
        let input_file = File::open(&input_path).unwrap();
        let mut reader = csv::Reader::from_reader(input_file);

        reader
            .records()
            .map(|record| {
                let record = record.unwrap();
                let name = record.get(2).unwrap().to_owned();
                let typ = match record.get(3) {
                    None | Some("") => InstructionType::Empty,
                    Some("block") => InstructionType::Block,
                    Some("index") => InstructionType::Index,
                    Some("branch-table") => InstructionType::BranchTable,
                    Some("table-index") => InstructionType::TableIndex,
                    Some("memarg") => InstructionType::MemArg,
                    Some("const") => InstructionType::Const,
                    Some(x) => panic!("Unknown instruction type: {}", x),
                };
                let enum_name = create_enum_name(name.clone());
                InstructionRecord {
                    opcode: parse_usize(record.get(0).unwrap()),
                    old_name: record.get(1).unwrap().to_owned(),
                    new_name: name,
                    enum_name: enum_name.clone(),
                    enum_ref: format!("Instruction::{}", enum_name),
                    typ,
                }
            })
            .collect()
    };

    generate_instruction_type(&instructions).unwrap();
}

fn parse_usize(inp: &str) -> usize {
    let s: String = inp.chars().skip(2).collect();
    usize::from_str_radix(&s, 16).unwrap()
}

fn create_enum_name(name: String) -> String {
    let mut new_str = String::new();
    for segment in name.replace('.', "_").split('_') {
        let mut chars: Vec<char> = segment.chars().collect();

        // We know these are ascii chars...
        chars[0] = chars[0].to_uppercase().next().unwrap();

        let new_segment: String = chars.into_iter().collect();
        new_str.push_str(&new_segment);
    }
    new_str
}

fn generate_instruction_type(instructions: &Vec<InstructionRecord>) -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("instructions.g.rs");
    let f = File::create(&dest_path).unwrap();
    let mut w = IndentingWriter::new(f);

    w.writeln("// Automatically generated via build.rs")?;
    w.writeln("// Included via 'include!' in src/instruction.rs")?;

    w.writeln("#[derive(PartialEq, Clone, Debug)]")?;
    w.block("pub enum Instruction {", |w| generate_enum_members(w, &instructions))?;
    w.writeln("")?;
    w.block("impl Instruction {", |w| generate_instruction_methods(w, &instructions))?;

    w.writeln("")?;
    generate_fmt(&mut w, &instructions)?;

    Ok(())
}

fn generate_enum_members<W: io::Write>(w: &mut IndentingWriter<W>, instructions: &Vec<InstructionRecord>) -> io::Result<()> {
    for record in instructions {
        match record.typ {
            Empty => writeln!(w, "{},", record.enum_name)?,
            Const => writeln!(w, "{}(crate::Value),", record.enum_name)?,
            Block => writeln!(w, "{}(crate::types::ValType),", record.enum_name)?,
            Index => writeln!(w, "{}(u32),", record.enum_name)?,
            BranchTable => writeln!(w, "{}(crate::format::instruction::BranchTable),", record.enum_name)?,
            TableIndex => writeln!(w, "{}(u32, u32),", record.enum_name)?,
            MemArg => writeln!(w, "{}(u32, u32),", record.enum_name)?,
        }
    }

    Ok(())
}

fn get_value_type(name: &str) -> &'static str {
    match name {
        "i32.const" => "i32",
        "i64.const" => "i64",
        "f32.const" => "f32",
        "f64.const" => "f64",
        x => panic!("Unknown constant instruction: '{}'", x),
    }
}

fn generate_fmt<W: io::Write>(w: &mut IndentingWriter<W>, instructions: &Vec<InstructionRecord>) -> io::Result<()> {
    w.block("impl std::fmt::Display for Instruction {", |w| {
        w.block("fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {", |w| {
            w.block("match self {", |w| {
                for record in instructions {
                    match record.typ {
                        Empty => writeln!(w, "{} => write!(f, \"{}\"),", record.enum_ref, record.new_name)?,
                        Const | Block | Index | BranchTable => writeln!(w, "{}(x) => write!(f, \"{} {{}}\", x),", record.enum_ref, record.new_name)?,
                        TableIndex | MemArg => writeln!(w, "{}(x, y) => write!(f, \"{} {{}} {{}}\", x, y),", record.enum_ref, record.new_name)?,
                    }
                }
                Ok(())
            })
        })
    })
}

fn generate_instruction_methods<W: io::Write>(w: &mut IndentingWriter<W>, instructions: &Vec<InstructionRecord>) -> io::Result<()> {
    w.block("pub fn read<R: std::io::Read>(reader: &mut R) -> Result<Instruction, crate::Error> {", |w| {
        w.writeln("let opcode = byteorder::ReadBytesExt::read_u8(reader)?;")?;
        w.block("match opcode {", |w| {
            for record in instructions {
                let opcode = format!("0x{:02X}", record.opcode);
                match record.typ {
                    Empty => writeln!(w, "{} => Ok({}),", opcode, record.enum_ref)?,
                    Const => writeln!(w, "{} => Ok({}(read_{}(reader)?)),", opcode, record.enum_ref, get_value_type(&record.new_name))?,
                    Block => writeln!(w, "{} => Ok({}(crate::types::ValType::read(reader)?)),", opcode, record.enum_ref)?,
                    Index => writeln!(w, "{} => Ok({}(read_idx(reader)?)),", opcode, record.enum_ref)?,
                    BranchTable => writeln!(w, "{} => Ok({}(BranchTable::read(reader)?)),", opcode, record.enum_ref)?,
                    TableIndex => writeln!(w, "{} => Ok({}(read_idx(reader)?, read_idx(reader)?)),", opcode, record.enum_ref)?,
                    MemArg => writeln!(w, "{} => Ok({}(read_idx(reader)?, read_idx(reader)?)),", opcode, record.enum_ref)?,
                }
            }
            writeln!(w, "x => Err(Error::UnknownOpcode(x)),")?;
            Ok(())
        })?;
        Ok(())
    })?;
    w.writeln("")?;

    w.block("pub fn opcode(&self) -> u8 {", |w| {
        w.block("match self {", |w| {
            for record in instructions {
                let opcode = format!("0x{:02X}", record.opcode);
                match record.typ {
                    Empty => writeln!(w, "{} => {},", record.enum_ref, opcode)?,
                    Const | Block | Index | BranchTable => writeln!(w, "{}(_) => {},", record.enum_ref, opcode)?,
                    TableIndex | MemArg => writeln!(w, "{}(_, _) => {},", record.enum_ref, opcode)?,
                }
            }
            Ok(())
        })
    })?;
    w.writeln("")?;

    w.block("pub fn is_block(&self) -> bool {", |w| {
        w.block("match self {", |w| {
            for record in instructions.iter().filter(|i| i.typ == InstructionType::Block) {
                writeln!(w, "{}(_) => true,", record.enum_ref)?;
            }
            writeln!(w, "_ => false,")?;
            Ok(())
        })
    })?;

    Ok(())
}