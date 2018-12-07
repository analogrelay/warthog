// Compile instructions.csv into Rust instructions files

extern crate csv;

use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

pub struct InstructionRecord {
    pub opcode: usize,
    pub old_name: String,
    pub new_name: String,
    pub enum_name: String,
    pub typ: Option<String>,
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
                InstructionRecord {
                    opcode: parse_usize(record.get(0).unwrap()),
                    old_name: record.get(1).unwrap().to_owned(),
                    new_name: name.clone(),
                    enum_name: create_enum_name(name),
                    typ: record.get(3).map(|x| x.to_string()),
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
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("instructions.g.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "// Automatically generated via build.rs")?;
    writeln!(f, "")?;

    writeln!(f, "pub enum Instruction {{")?;
    for record in instructions {
        writeln!(f, "    {},", record.enum_name)?;
    }
    writeln!(f, "}}")?;

    Ok(())
}
