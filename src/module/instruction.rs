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

macro_rules! rd_int {
    ($r:expr, $t: ty) => {
        utils::read_leb128_u32($r)? as $t
    };
}

impl Instruction {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Option<Instruction>, Error> {
        let opcode = reader.read_u8()?;
        match opcode {
            0x0B => Ok(None),
            0x10 => Ok(Some(Instruction::Call(rd_int!(reader, usize)))),
            0x1A => Ok(Some(Instruction::Drop)),
            0x20 => Ok(Some(Instruction::GetLocal(rd_int!(reader, usize)))),
            0x21 => Ok(Some(Instruction::SetLocal(rd_int!(reader, usize)))),
            0x41 => Ok(Some(Instruction::Const(Value::Integer32(
                utils::read_leb128_u32(reader)?,
            )))),
            0x6E => Ok(Some(Instruction::Div(
                ValType::Integer32,
                Signedness::Unsigned,
            ))),
            x => panic!("Instruction not implemented: 0x{:X}", x),
        }
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
