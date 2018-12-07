// Compile instructions.csv into Rust instructions files

extern crate csv;

use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

pub enum InstructionType {
    Empty,
    Block,
    Index,
    BranchTable,
    TableIndex,
    MemArg,
}

pub struct InstructionRecord {
    pub opcode: usize,
    pub old_name: String,
    pub new_name: String,
    pub enum_name: String,
    pub typ: InstructionType,
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
                    Some(x) => panic!("Unknown instruction type: {}", x),
                };
                InstructionRecord {
                    opcode: parse_usize(record.get(0).unwrap()),
                    old_name: record.get(1).unwrap().to_owned(),
                    new_name: name.clone(),
                    enum_name: create_enum_name(name),
                    typ,
                }
            })
            .collect()
    };

    generate_instruction_type(instructions).unwrap();
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

fn generate_instruction_type(instructions: Vec<InstructionRecord>) -> Result<(), io::Error> {
    use InstructionType::*;

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("instructions.g.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "// Automatically generated via build.rs")?;
    writeln!(f, "")?;

    writeln!(f, "pub enum Instruction {{")?;
    for record in instructions {
        match record.typ {
            Empty => writeln!(f, "    {},", record.enum_name)?,
            Block => writeln!(f, "    {}(ValType)", record.enum_name)?,
            Index => writeln!(f, "    {}(u32)", record.enum_name)?,
            BranchTable => writeln!(f, "    {}(Box<BranchTable>)", record.enum_name)?,
            TableIndex => writeln!(f, "    {}(u32, u32)", record.enum_name)?,
            MemArg => writeln!(f, "    {}(u32, u32)", record.enum_name)?,
        }
    }
    writeln!(f, "}}")?;
    writeln!(f, "");
    writeln!(f, "impl Instruction {{");
    writeln!(f, "}}");

    Ok(())
}
