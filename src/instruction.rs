use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{utils, Error, ValType, Value};

#[derive(Clone, PartialEq)]
pub enum Signedness {
    Unsigned,
    Signed,
}

impl fmt::Display for Signedness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signedness::Unsigned => write!(f, "u"),
            Signedness::Signed => write!(f, "s"),
        }
    }
}

impl fmt::Debug for Signedness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

macro_rules! isa {
    ($(
        ($text: expr, $opcode: expr) => $name: ident
    ),*,) => {
        #[repr(u8)]
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq)]
        pub enum Opcode {
            $(
                $name = $opcode,
            )*
        }

        impl Opcode {
            pub fn decode(byte: u8) -> Option<Opcode> {
                match byte {
                    $(
                        $opcode => Some(Opcode::$name),
                    )*
                    _ => None
                }
            }
        }

        impl ::std::fmt::Display for Opcode {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let val = match self {
                    $(
                        Opcode::$name => $text,
                    )*
                };
                write!(f, "{}", val)
            }
        }
    }
}

isa! {
    // Control Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    () => Unreachable,
    () => Nop,
    () => Block,
    () => Loop,
    () => If,
    () => IfElse,
    () => Br,
    () => Br_If,
    () => Br_Table,
    () => Return,
    () => Call(usize),
    () => Call_Indirect(usize),

    // Parametric Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#parametric-instructions
    () => Drop,
    () => Select,

    // Variable Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#variable-instructions
    () => GetLocal(usize),
    () => SetLocal(usize),
    () => TeeLocal(usize),
    () => GetGlobal(usize),
    () => SetGlobal(usize),

    // Memory Instructions,
    // https://webassembly.github.io/spec/core/binary/instructions.html#memory-instructions
    () => I32Load(u32, u32),
    () => I64Load(u32, u32),
    () => F32Load(u32, u32),
    () => F64Load(u32, u32),
    () => I32Load8_S(u32, u32),
    () => I32Load8_U(u32, u32),
    () => I32Load16_S(u32, u32),
    () => I32Load16_U(u32, u32),
    () => I64Load8_S(u32, u32),
    () => I64Load8_U(u32, u32),
    () => I64Load16_S(u32, u32),
    () => I64Load16_U(u32, u32),
    () => I64Load32_S(u32, u32),
    () => I64Load32_U(u32, u32),
    () => I32Store(u32, u32),
    () => I64Store(u32, u32),
    () => F32Store(u32, u32),
    () => F64Store(u32, u32),
    () => I32Store8_S(u32, u32),
    () => I32Store8_U(u32, u32),
    () => I32Store16_S(u32, u32),
    () => I32Store16_U(u32, u32),
    () => I64Store8_S(u32, u32),
    () => I64Store8_U(u32, u32),
    () => I64Store16_S(u32, u32),
    () => I64Store16_U(u32, u32),
    () => I64Store32_S(u32, u32),
    () => I64Store32_U(u32, u32),
    () => Memory_Size(u32),
    () => Memory_Grow(u32),

    // Numeric instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#numeric-instructions
    () => I32Const(u32),
    () => I64Const(u64),
    () => F32Const(f32),
    () => F64Const(f64),

    () => I32Eqz,
    () => I32Eq,
    () => I32Ne,
    () => I32Lt_S,
    () => I32Lt_U,
    () => I32Gt_S,
    () => I32Gt_U,
    () => I32Le_S,
    () => I32Le_U,
    () => I32Ge_S,
    () => I32Ge_U,

    () => I64Eqz,
    () => I64Eq,
    () => I64Ne,
    () => I64Lt_S,
    () => I64Lt_U,
    () => I64Gt_S,
    () => I64Gt_U,
    () => I64Le_S,
    () => I64Le_U,
    () => I64Ge_S,
    () => I64Ge_U,

    () => F32Eq,
    () => F32Ne,
    () => F32Lt,
    () => F32Gt,
    () => F32Le,
    () => F32Ge,

    () => F64Eq,
    () => F64Ne,
    () => F64Lt,
    () => F64Gt,
    () => F64Le,
    () => F64Ge,

    () => I32Clz,
    () => I32Ctz,
    () => I32Popcnt,
    () => I32Add,
    () => I32Sub,
    () => I32Mul,
    () => I32Div_S,
    () => I32Div_U,
    () => I32Rem_S,
    () => I32Rem_U,
    () => I32And,
    () => I32Or,
    () => I32Xor,
    () => I32Shl,
    () => I32Shr_S,
    () => I32Shr_U,
    () => I32Rotl,
    () => I32Rotr,

    () => I64Clz,
    () => I64Ctz,
    () => I64Popcnt,
    () => I64Add,
    () => I64Sub,
    () => I64Mul,
    () => I64Div_S,
    () => I64Div_U,
    () => I64Rem_S,
    () => I64Rem_U,
    () => I64And,
    () => I64Or,
    () => I64Xor,
    () => I64Shl,
    () => I64Shr_S,
    () => I64Shr_U,
    () => I64Rotl,
    () => I64Rotr,

    () => F32Abs,
    () => F32Neg,
    () => F32Ceil,
    () => F32Floor,
    () => F32Trunc,
    () => F32Nearest,
    () => F32Sqrt,
    () => F32Add,
    () => F32Sub,
    () => F32Mul,
    () => F32Div,
    () => F32Min,
    () => F32Max,
    () => F32Copysign,

    () => F64Abs,
    () => F64Neg,
    () => F64Ceil,
    () => F64Floor,
    () => F64Trunc,
    () => F64Nearest,
    () => F64Sqrt,
    () => F64Add,
    () => F64Sub,
    () => F64Mul,
    () => F64Div,
    () => F64Min,
    () => F64Max,
    () => F64Copysign,

    () => I32Wrap_I64,
    () => I32Trunc_S_F32,
    () => I32Trunc_U_F32,
    () => I32Trunc_S_F64,
    () => I32Trunc_U_F64,
    () => I64Extend_S_I32,
    () => I64Extend_U_I32,
    () => I64Trunc_S_F32,
    () => I64Trunc_U_F32,
    () => I64Trunc_S_F64,
    () => I64Trunc_U_F64,
    () => F32Convert_S_I32,
    () => F32Convert_U_I32,
    () => F32Convert_S_I64,
    () => F32Convert_U_I64,
    () => F32Demote_F64,
    () => F64Convert_S_I32,
    () => F64Convert_U_I32,
    () => F64Convert_S_I64,
    () => F64Convert_U_I64,
    () => F64Promote_F32,
    () => I32Reinterpret_F32,
    () => I64Reinterpret_F64,
    () => F32Reinterpret_I32,
    () => F64Reinterpret_I64,

    // Psuedo-Instruction 'end'
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    () => End,
}

// We want to use WASM-like names for the enum variants
#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum Instruction {}

impl Instruction {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Option<Instruction>, Error> {
        unimplemented!();
        // use self::{Instruction::*, Signedness::*};
        // use Value::*;

        // let opcode = reader.read_u8()?;
        // if opcode == 0x0B {
        //     return Ok(None);
        // }

        // Ok(Some(match opcode {
        //     0x10 => Call(ri!(reader)),
        //     0x1A => Drop,
        //     0x20 => GetLocal(ri!(reader)),
        //     0x21 => SetLocal(ri!(reader)),

        //     0x41 => Const(I32(ri!(reader))),
        //     0x42 => Const(I64(ri!(reader))),
        //     0x43 => unimplemented!("f32.const"),
        //     0x44 => unimplemented!("f64.const"),

        //     0x45 => Eqz(vt!(i32)),
        //     0x46 => Eq(vt!(i32)),
        //     0x47 => Ne(vt!(i32)),
        //     0x48 => Lt(vt!(i32), Signed),
        //     0x49 => Lt(vt!(i32), Unsigned),
        //     0x4A => Gt(vt!(i32), Signed),
        //     0x4B => Gt(vt!(i32), Unsigned),
        //     0x4C => Le(vt!(i32), Signed),
        //     0x4D => Le(vt!(i32), Unsigned),
        //     0x4E => Ge(vt!(i32), Signed),
        //     0x4F => Ge(vt!(i32), Unsigned),

        //     0x50 => Eqz(vt!(i64)),
        //     0x51 => Eq(vt!(i64)),
        //     0x52 => Ne(vt!(i64)),
        //     0x53 => Lt(vt!(i64), Signed),
        //     0x54 => Lt(vt!(i64), Unsigned),
        //     0x55 => Gt(vt!(i64), Signed),
        //     0x56 => Gt(vt!(i64), Unsigned),
        //     0x57 => Le(vt!(i64), Signed),
        //     0x58 => Le(vt!(i64), Unsigned),
        //     0x59 => Ge(vt!(i64), Signed),
        //     0x5A => Ge(vt!(i64), Unsigned),

        //     0x5B => Eq(vt!(f32)),
        //     0x5C => Ne(vt!(f32)),
        //     0x5D => FLt(vt!(f32)),
        //     0x5E => FGt(vt!(f32)),
        //     0x5F => FLe(vt!(f32)),
        //     0x60 => FGe(vt!(f32)),

        //     0x61 => Eq(vt!(f64)),
        //     0x62 => Ne(vt!(f64)),
        //     0x63 => FLt(vt!(f64)),
        //     0x64 => FGt(vt!(f64)),
        //     0x65 => FLe(vt!(f64)),
        //     0x66 => FGe(vt!(f64)),

        //     0x67 => Clz(vt!(i32)),
        //     0x68 => Ctz(vt!(i32)),
        //     0x69 => Popcnt(vt!(i32)),
        //     0x6A => Add(vt!(i32)),
        //     0x6B => Sub(vt!(i32)),
        //     0x6C => Mul(vt!(i32)),
        //     0x6D => Div(vt!(i32), Signed),
        //     0x6E => Div(vt!(i32), Unsigned),
        //     0x6F => Rem(vt!(i32), Signed),
        //     0x70 => Rem(vt!(i32), Unsigned),
        //     0x71 => And(vt!(i32)),
        //     0x72 => Or(vt!(i32)),
        //     0x73 => Xor(vt!(i32)),
        //     0x74 => Shl(vt!(i32)),
        //     0x75 => Shr(vt!(i32), Signed),
        //     0x76 => Shr(vt!(i32), Unsigned),
        //     0x77 => Rotl(vt!(i32)),
        //     0x78 => Rotr(vt!(i32)),

        //     0x79 => Clz(vt!(i64)),
        //     0x7A => Ctz(vt!(i64)),
        //     0x7B => Popcnt(vt!(i64)),
        //     0x7C => Add(vt!(i64)),
        //     0x7D => Sub(vt!(i64)),
        //     0x7E => Mul(vt!(i64)),
        //     0x7F => Div(vt!(i64), Signed),
        //     0x80 => Div(vt!(i64), Unsigned),
        //     0x81 => Rem(vt!(i64), Signed),
        //     0x82 => Rem(vt!(i64), Unsigned),
        //     0x83 => And(vt!(i64)),
        //     0x84 => Or(vt!(i64)),
        //     0x85 => Xor(vt!(i64)),
        //     0x86 => Shl(vt!(i64)),
        //     0x87 => Shr(vt!(i64), Signed),
        //     0x88 => Shr(vt!(i64), Unsigned),
        //     0x89 => Rotl(vt!(i64)),
        //     0x8A => Rotr(vt!(i64)),

        //     0x8B => Abs(vt!(f32)),
        //     0x8C => Neg(vt!(f32)),
        //     0x8D => Ceil(vt!(f32)),
        //     0x8E => Floor(vt!(f32)),
        //     0x8F => FTrunc(vt!(f32)),
        //     0x90 => Nearest(vt!(f32)),
        //     0x91 => Sqrt(vt!(f32)),
        //     0x92 => Add(vt!(f32)),
        //     0x93 => Sub(vt!(f32)),
        //     0x94 => Mul(vt!(f32)),
        //     0x95 => FDiv(vt!(f32)),
        //     0x96 => Min(vt!(f32)),
        //     0x97 => Max(vt!(f32)),
        //     0x98 => Copysign(vt!(f32)),

        //     0x99 => Abs(vt!(f64)),
        //     0x9A => Neg(vt!(f64)),
        //     0x9B => Ceil(vt!(f64)),
        //     0x9C => Floor(vt!(f64)),
        //     0x9D => FTrunc(vt!(f64)),
        //     0x9E => Nearest(vt!(f64)),
        //     0x9F => Sqrt(vt!(f64)),
        //     0xA0 => Add(vt!(f64)),
        //     0xA1 => Sub(vt!(f64)),
        //     0xA2 => Mul(vt!(f64)),
        //     0xA3 => FDiv(vt!(f64)),
        //     0xA4 => Min(vt!(f64)),
        //     0xA5 => Max(vt!(f64)),
        //     0xA6 => Copysign(vt!(f64)),

        //     0xA7 => Wrap,
        //     0xA8 => Trunc(vt!(i32), Signed, vt!(f32)),
        //     0xA9 => Trunc(vt!(i32), Unsigned, vt!(f32)),
        //     0xAA => Trunc(vt!(i32), Signed, vt!(f64)),
        //     0xAB => Trunc(vt!(i32), Unsigned, vt!(f64)),
        //     0xAC => Extend(Signed),
        //     0xAD => Extend(Unsigned),
        //     0xAE => Trunc(vt!(i64), Signed, vt!(f32)),
        //     0xAF => Trunc(vt!(i64), Unsigned, vt!(f32)),
        //     0xB0 => Trunc(vt!(i64), Signed, vt!(f64)),
        //     0xB1 => Trunc(vt!(i64), Unsigned, vt!(f64)),
        //     0xB2 => Convert(vt!(f32), Signed, vt!(i32)),
        //     0xB3 => Convert(vt!(f32), Unsigned, vt!(i32)),
        //     0xB4 => Convert(vt!(f32), Signed, vt!(i64)),
        //     0xB5 => Convert(vt!(f32), Unsigned, vt!(i64)),
        //     0xB6 => Demote,
        //     0xB7 => Convert(vt!(f64), Signed, vt!(i32)),
        //     0xB8 => Convert(vt!(f64), Unsigned, vt!(i32)),
        //     0xB9 => Convert(vt!(f64), Signed, vt!(i64)),
        //     0xBA => Convert(vt!(f64), Unsigned, vt!(i64)),
        //     0xBB => Promote,

        //     x => panic!("Instruction not implemented: 0x{:X}", x),
        // }))
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
        // match self {
        //     Instruction::Call(x) => write!(f, "call {}", x),
        //     Instruction::GetLocal(x) => write!(f, "get_local {}", x),
        //     Instruction::SetLocal(x) => write!(f, "set_local {}", x),

        //     Instruction::Const(x) => write!(f, "{}.const {}", x.typ(), x),
        //     Instruction::Clz(x) => write!(f, "{}.clz", x),
        //     Instruction::Ctz(x) => write!(f, "{}.ctz", x),
        //     Instruction::Popcnt(x) => write!(f, "{}.popcnt", x),
        //     Instruction::Add(x) => write!(f, "{}.add", x),
        //     Instruction::Mul(x) => write!(f, "{}.mul", x),
        //     Instruction::Sub(x) => write!(f, "{}.sub", x),
        //     Instruction::Div(x, s) => write!(f, "{}.div_{}", x, s),
        //     Instruction::Rem(x, s) => write!(f, "{}.rem_{}", x, s),
        //     Instruction::And(x) => write!(f, "{}.and", x),
        //     Instruction::Or(x) => write!(f, "{}.or", x),
        //     Instruction::Xor(x) => write!(f, "{}.xor", x),
        //     Instruction::Shl(x) => write!(f, "{}.shl", x),
        //     Instruction::Shr(x, s) => write!(f, "{}.shr_{}", x, s),
        //     Instruction::Rotl(x) => write!(f, "{}.rotl", x),
        //     Instruction::Rotr(x) => write!(f, "{}.rotr", x),

        //     Instruction::Abs(x) => write!(f, "{}.abs", x),
        //     Instruction::Neg(x) => write!(f, "{}.neg", x),
        //     Instruction::Ceil(x) => write!(f, "{}.ceil", x),
        //     Instruction::Floor(x) => write!(f, "{}.floor", x),
        //     Instruction::FTrunc(x) => write!(f, "{}.trunc", x),
        //     Instruction::Nearest(x) => write!(f, "{}.nearest", x),
        //     Instruction::Sqrt(x) => write!(f, "{}.sqrt", x),
        //     Instruction::FDiv(x) => write!(f, "{}.div", x),
        //     Instruction::Min(x) => write!(f, "{}.min", x),
        //     Instruction::Max(x) => write!(f, "{}.max", x),
        //     Instruction::Copysign(x) => write!(f, "{}.copysign", x),

        //     Instruction::Eqz(x) => write!(f, "{}.eqz", x),
        //     Instruction::Eq(x) => write!(f, "{}.eq", x),
        //     Instruction::Ne(x) => write!(f, "{}.ne", x),
        //     Instruction::Lt(x, s) => write!(f, "{}.lt_{}", x, s),
        //     Instruction::Gt(x, s) => write!(f, "{}.gt_{}", x, s),
        //     Instruction::Le(x, s) => write!(f, "{}.le_{}", x, s),
        //     Instruction::Ge(x, s) => write!(f, "{}.ge_{}", x, s),

        //     Instruction::FLt(x) => write!(f, "{}.lt", x),
        //     Instruction::FGt(x) => write!(f, "{}.gt", x),
        //     Instruction::FLe(x) => write!(f, "{}.le", x),
        //     Instruction::FGe(x) => write!(f, "{}.ge", x),

        //     Instruction::Wrap => write!(f, "i32.wrap/i64"),
        //     Instruction::Extend(s) => write!(f, "i64.extend_{}/i32", s),
        //     Instruction::Demote => write!(f, "f32.demote/f64"),
        //     Instruction::Promote => write!(f, "f64.promote/f32"),
        //     Instruction::Reinterpret(x) => write!(f, "{}.reinterpret/{}", x, reinterpreted(x)),
        //     Instruction::Convert(x, s, y) => write!(f, "{}.convert_{}/{}", x, s, y),
        //     Instruction::Trunc(x, s, y) => write!(f, "{}.trunc_{}/{}", x, s, y),

        //     Instruction::Drop => write!(f, "drop"),
        // }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn reinterpreted(v: &ValType) -> ValType {
    match v {
        ValType::Nil => ValType::Nil,
        ValType::I32 => ValType::F32,
        ValType::I64 => ValType::F64,
        ValType::F32 => ValType::I32,
        ValType::F64 => ValType::I64,
    }
}
