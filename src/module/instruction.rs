use std::{fmt, io};

use byteorder::ReadBytesExt;

use crate::{module::ValType, utils, Error, Value};

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

#[derive(Clone, PartialEq)]
pub enum Instruction {
    Call(usize),
    GetLocal(usize),
    SetLocal(usize),

    Const(Value),
    Clz(ValType),
    Ctz(ValType),
    Popcnt(ValType),
    Add(ValType),
    Mul(ValType),
    Sub(ValType),
    Div(ValType, Signedness),
    Rem(ValType, Signedness),
    And(ValType),
    Or(ValType),
    Xor(ValType),
    Shl(ValType),
    Shr(ValType, Signedness),
    Rotl(ValType),
    Rotr(ValType),

    Abs(ValType),
    Neg(ValType),
    Ceil(ValType),
    Floor(ValType),
    FTrunc(ValType),
    Nearest(ValType),
    Sqrt(ValType),
    FDiv(ValType),
    Min(ValType),
    Max(ValType),
    Copysign(ValType),

    Eqz(ValType),
    Eq(ValType),
    Ne(ValType),
    Lt(ValType, Signedness),
    Gt(ValType, Signedness),
    Le(ValType, Signedness),
    Ge(ValType, Signedness),
    FLt(ValType),
    FGt(ValType),
    FLe(ValType),
    FGe(ValType),

    Wrap,
    Extend(Signedness),
    Demote,
    Promote,
    Reinterpret(ValType),
    Convert(ValType, Signedness, ValType),
    Trunc(ValType, Signedness, ValType),

    Drop,
}

macro_rules! ri {
    ($r:expr) => {
        utils::read_leb128_s($r)?
    };
}

macro_rules! vt {
    (i32) => {
        $crate::module::ValType::Integer32
    };
    (i64) => {
        $crate::module::ValType::Integer64
    };
    (f32) => {
        $crate::module::ValType::Float32
    };
    (f64) => {
        $crate::module::ValType::Float64
    };
}

impl Instruction {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Option<Instruction>, Error> {
        use self::{Instruction::*, Signedness::*};
        use Value::*;

        let opcode = reader.read_u8()?;
        if opcode == 0x0B {
            return Ok(None);
        }

        Ok(Some(match opcode {
            0x10 => Call(ri!(reader)),
            0x1A => Drop,
            0x20 => GetLocal(ri!(reader)),
            0x21 => SetLocal(ri!(reader)),

            0x41 => Const(Integer32(ri!(reader))),
            0x42 => Const(Integer64(ri!(reader))),
            0x43 => unimplemented!("f32.const"),
            0x44 => unimplemented!("f64.const"),

            0x45 => Eqz(vt!(i32)),
            0x46 => Eq(vt!(i32)),
            0x47 => Ne(vt!(i32)),
            0x48 => Lt(vt!(i32), Signed),
            0x49 => Lt(vt!(i32), Unsigned),
            0x4A => Gt(vt!(i32), Signed),
            0x4B => Gt(vt!(i32), Unsigned),
            0x4C => Le(vt!(i32), Signed),
            0x4D => Le(vt!(i32), Unsigned),
            0x4E => Ge(vt!(i32), Signed),
            0x4F => Ge(vt!(i32), Unsigned),

            0x50 => Eqz(vt!(i64)),
            0x51 => Eq(vt!(i64)),
            0x52 => Ne(vt!(i64)),
            0x53 => Lt(vt!(i64), Signed),
            0x54 => Lt(vt!(i64), Unsigned),
            0x55 => Gt(vt!(i64), Signed),
            0x56 => Gt(vt!(i64), Unsigned),
            0x57 => Le(vt!(i64), Signed),
            0x58 => Le(vt!(i64), Unsigned),
            0x59 => Ge(vt!(i64), Signed),
            0x5A => Ge(vt!(i64), Unsigned),

            0x67 => Clz(vt!(i32)),
            0x68 => Ctz(vt!(i32)),
            0x69 => Popcnt(vt!(i32)),
            0x6A => Add(vt!(i32)),
            0x6B => Sub(vt!(i32)),
            0x6C => Mul(vt!(i32)),
            0x6D => Div(vt!(i32), Signed),
            0x6E => Div(vt!(i32), Unsigned),
            0x6F => Rem(vt!(i32), Signed),
            0x70 => Rem(vt!(i32), Unsigned),
            0x71 => And(vt!(i32)),
            0x72 => Or(vt!(i32)),
            0x73 => Xor(vt!(i32)),
            0x74 => Shl(vt!(i32)),
            0x75 => Shr(vt!(i32), Signed),
            0x76 => Shr(vt!(i32), Unsigned),
            0x77 => Rotl(vt!(i32)),
            0x78 => Rotr(vt!(i32)),

            0x79 => Clz(vt!(i64)),
            0x7A => Ctz(vt!(i64)),
            0x7B => Popcnt(vt!(i64)),
            0x7C => Add(vt!(i64)),
            0x7D => Sub(vt!(i64)),
            0x7E => Mul(vt!(i64)),
            0x7F => Div(vt!(i64), Signed),
            0x80 => Div(vt!(i64), Unsigned),
            0x81 => Rem(vt!(i64), Signed),
            0x82 => Rem(vt!(i64), Unsigned),
            0x83 => And(vt!(i64)),
            0x84 => Or(vt!(i64)),
            0x85 => Xor(vt!(i64)),
            0x86 => Shl(vt!(i64)),
            0x87 => Shr(vt!(i64), Signed),
            0x88 => Shr(vt!(i64), Unsigned),
            0x89 => Rotl(vt!(i64)),
            0x8A => Rotr(vt!(i64)),

            0xA7 => Wrap,
            0xA8 => Trunc(vt!(i32), Signed, vt!(f32)),
            0xA9 => Trunc(vt!(i32), Unsigned, vt!(f32)),
            0xAA => Trunc(vt!(i32), Signed, vt!(f64)),
            0xAB => Trunc(vt!(i32), Unsigned, vt!(f64)),
            0xAC => Extend(Signed),
            0xAD => Extend(Unsigned),
            0xAE => Trunc(vt!(i64), Signed, vt!(f32)),
            0xAF => Trunc(vt!(i64), Unsigned, vt!(f32)),
            0xB0 => Trunc(vt!(i64), Signed, vt!(f64)),
            0xB1 => Trunc(vt!(i64), Unsigned, vt!(f64)),
            0xB2 => Convert(vt!(f32), Signed, vt!(i32)),
            0xB3 => Convert(vt!(f32), Unsigned, vt!(i32)),
            0xB4 => Convert(vt!(f32), Signed, vt!(i64)),
            0xB5 => Convert(vt!(f32), Unsigned, vt!(i64)),
            0xB6 => Demote,
            0xB7 => Convert(vt!(f64), Signed, vt!(i32)),
            0xB8 => Convert(vt!(f64), Unsigned, vt!(i32)),
            0xB9 => Convert(vt!(f64), Signed, vt!(i64)),
            0xBA => Convert(vt!(f64), Unsigned, vt!(i64)),
            0xBB => Promote,

            x => panic!("Instruction not implemented: 0x{:X}", x),
        }))
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Call(x) => write!(f, "call {}", x),
            Instruction::GetLocal(x) => write!(f, "get_local {}", x),
            Instruction::SetLocal(x) => write!(f, "set_local {}", x),

            Instruction::Const(x) => write!(f, "{}.const {}", x.typ(), x),
            Instruction::Clz(x) => write!(f, "{}.clz", x),
            Instruction::Ctz(x) => write!(f, "{}.ctz", x),
            Instruction::Popcnt(x) => write!(f, "{}.popcnt", x),
            Instruction::Add(x) => write!(f, "{}.add", x),
            Instruction::Mul(x) => write!(f, "{}.mul", x),
            Instruction::Sub(x) => write!(f, "{}.sub", x),
            Instruction::Div(x, s) => write!(f, "{}.div_{}", x, s),
            Instruction::Rem(x, s) => write!(f, "{}.rem_{}", x, s),
            Instruction::And(x) => write!(f, "{}.and", x),
            Instruction::Or(x) => write!(f, "{}.or", x),
            Instruction::Xor(x) => write!(f, "{}.xor", x),
            Instruction::Shl(x) => write!(f, "{}.shl", x),
            Instruction::Shr(x, s) => write!(f, "{}.shr_{}", x, s),
            Instruction::Rotl(x) => write!(f, "{}.rotl", x),
            Instruction::Rotr(x) => write!(f, "{}.rotr", x),

            Instruction::Abs(x) => write!(f, "{}.abs", x),
            Instruction::Neg(x) => write!(f, "{}.neg", x),
            Instruction::Ceil(x) => write!(f, "{}.ceil", x),
            Instruction::Floor(x) => write!(f, "{}.floor", x),
            Instruction::FTrunc(x) => write!(f, "{}.trunc", x),
            Instruction::Nearest(x) => write!(f, "{}.nearest", x),
            Instruction::Sqrt(x) => write!(f, "{}.sqrt", x),
            Instruction::FDiv(x) => write!(f, "{}.div", x),
            Instruction::Min(x) => write!(f, "{}.min", x),
            Instruction::Max(x) => write!(f, "{}.max", x),
            Instruction::Copysign(x) => write!(f, "{}.copysign", x),

            Instruction::Eqz(x) => write!(f, "{}.eqz", x),
            Instruction::Eq(x) => write!(f, "{}.eq", x),
            Instruction::Ne(x) => write!(f, "{}.ne", x),
            Instruction::Lt(x, s) => write!(f, "{}.lt_{}", x, s),
            Instruction::Gt(x, s) => write!(f, "{}.gt_{}", x, s),
            Instruction::Le(x, s) => write!(f, "{}.le_{}", x, s),
            Instruction::Ge(x, s) => write!(f, "{}.ge_{}", x, s),

            Instruction::FLt(x) => write!(f, "{}.lt", x),
            Instruction::FGt(x) => write!(f, "{}.gt", x),
            Instruction::FLe(x) => write!(f, "{}.le", x),
            Instruction::FGe(x) => write!(f, "{}.ge", x),

            Instruction::Wrap => write!(f, "i32.wrap/i64"),
            Instruction::Extend(s) => write!(f, "i64.extend_{}/i32", s),
            Instruction::Demote => write!(f, "f32.demote/f64"),
            Instruction::Promote => write!(f, "f64.promote/f32"),
            Instruction::Reinterpret(x) => write!(f, "{}.reinterpret/{}", x, reinterpreted(x)),
            Instruction::Convert(x, s, y) => write!(f, "{}.convert_{}/{}", x, s, y),
            Instruction::Trunc(x, s, y) => write!(f, "{}.trunc_{}/{}", x, s, y),

            Instruction::Drop => write!(f, "drop"),
        }
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
        ValType::Integer32 => ValType::Float32,
        ValType::Integer64 => ValType::Float64,
        ValType::Float32 => ValType::Integer32,
        ValType::Float64 => ValType::Integer64,
    }
}
