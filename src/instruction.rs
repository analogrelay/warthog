use std::{fmt, io};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{utils, Error, Value};

#[derive(Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub payload: InstructionPayload,
}

impl Instruction {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<Instruction, Error> {
        // Read the opcode and decode it
        let opcode = reader.read_u8()?;
        let opcode = match Opcode::decode(opcode) {
            Some(o) => o,
            None => return Err(Error::UnknownOpcode(opcode)),
        };

        // Decode the payload
        let payload = InstructionPayload::decode(opcode, reader)?;

        Ok(Instruction { opcode, payload })
    }

    pub fn unwrap_idx(&self) -> u32 {
        match &self.payload {
            InstructionPayload::Idx(x) => *x,
            x => panic!(
                "Attempted to unwrap an 'Idx' payload, but the payload is: '{:?}'",
                x
            ),
        }
    }

    pub fn unwrap_mem_arg(&self) -> (u32, u32) {
        match &self.payload {
            InstructionPayload::MemArg(a, s) => (*a, *s),
            x => panic!(
                "Attempted to unwrap a 'MemArg' payload, but the payload is: '{:?}'",
                x
            ),
        }
    }

    pub fn unwrap_const(&self) -> Value {
        match &self.payload {
            InstructionPayload::Const(v) => *v,
            x => panic!(
                "Attempted to unwrap a 'Const' payload, but the payload is: '{:?}'",
                x
            ),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.payload == InstructionPayload::Empty {
            write!(f, "{}", self.opcode)
        } else {
            write!(f, "{} {}", self.opcode, self.payload)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionPayload {
    Empty,
    Idx(u32),
    MemArg(u32, u32),
    Const(Value),
}

impl fmt::Display for InstructionPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InstructionPayload::*;

        match self {
            Empty => Ok(()),
            Idx(i) => write!(f, "{}", i),
            MemArg(a, s) => write!(f, "{} {}", a, s),
            Const(v) => write!(f, "{}", v),
        }
    }
}

macro_rules! isa {
    ($(
        ($opcode: expr, $text: expr, $read_method: ident) => $name: ident
    ),*,) => {
        #[repr(u8)]
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, PartialEq, Eq)]
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

        impl InstructionPayload {
            pub fn decode<R: ::std::io::Read>(opcode: Opcode, reader: &mut R) -> Result<InstructionPayload, $crate::Error> {
                match opcode {
                    $(
                        Opcode::$name => $read_method(reader),
                    )*
                }
            }
        }

        impl ::std::convert::Into<u8> for Opcode {
            fn into(self) -> u8 {
                unsafe { ::std::mem::transmute(self) }
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

        impl ::std::fmt::Debug for Opcode {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(self, f)
            }
        }
    }
}

isa! {
    // Control Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    (0x00, "unreachable", read_none) => Unreachable,
    (0x01, "nop", read_none) => Nop,
    (0x02, "block", read_none) => Block,
    (0x03, "loop", read_none) => Loop,
    (0x04, "if", read_none) => If,
    (0x0C, "br", read_none) => Br,
    (0x0D, "br_if", read_none) => Br_If,
    (0x0E, "br_table", read_none) => Br_Table,
    (0x0F, "return", read_none) => Return,
    (0x10, "call", read_idx) => Call,
    (0x11, "call_indirect", read_idx) => Call_Indirect,

    // Parametric Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#parametric-instructions
    (0x1A, "drop", read_none) => Drop,
    (0x1B, "select", read_none) => Select,

    // Variable Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#variable-instructions
    (0x20, "get_local", read_idx) => GetLocal,
    (0x21, "set_local", read_idx) => SetLocal,
    (0x22, "tee_local", read_idx) => TeeLocal,
    (0x23, "get_global", read_idx) => GetGlobal,
    (0x24, "set_global", read_idx) => SetGlobal,

    // Memory Instructions,
    // https://webassembly.github.io/spec/core/binary/instructions.html#memory-instructions
    (0x28, "i32.load", read_mem_arg) => I32Load,
    (0x29, "i64.load", read_mem_arg) => I64Load,
    (0x2A, "f32.load", read_mem_arg) => F32Load,
    (0x2B, "f64.load", read_mem_arg) => F64Load,
    (0x2C, "i32.load8_s", read_mem_arg) => I32Load8_S,
    (0x2D, "i32.load8_u", read_mem_arg) => I32Load8_U,
    (0x2E, "i32.load16_s", read_mem_arg) => I32Load16_S,
    (0x2F, "i32.load16_u", read_mem_arg) => I32Load16_U,
    (0x30, "i64.load8_s", read_mem_arg) => I64Load8_S,
    (0x31, "i64.load8_u", read_mem_arg) => I64Load8_U,
    (0x32, "i64.load16_s", read_mem_arg) => I64Load16_S,
    (0x33, "i64.load16_u", read_mem_arg) => I64Load16_U,
    (0x34, "i64.load32_s", read_mem_arg) => I64Load32_S,
    (0x35, "i64.load32_u", read_mem_arg) => I64Load32_U,
    (0x36, "i32.store", read_mem_arg) => I32Store,
    (0x37, "i64.store", read_mem_arg) => I64Store,
    (0x38, "f32.store", read_mem_arg) => F32Store,
    (0x39, "f64.store", read_mem_arg) => F64Store,
    (0x3A, "i32.store8", read_mem_arg) => I32Store8,
    (0x3B, "i32.store16", read_mem_arg) => I32Store16,
    (0x3C, "i64.store8", read_mem_arg) => I64Store8,
    (0x3D, "i64.store16", read_mem_arg) => I64Store16,
    (0x3E, "i64.store32", read_mem_arg) => I64Store32,
    (0x3F, "memory.size", read_idx) => Memory_Size,
    (0x40, "memory.grow", read_idx) => Memory_Grow,

    // Numeric instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#numeric-instructions
    (0x41, "i32.const", read_i32) => I32Const,
    (0x42, "i64.const", read_i64) => I64Const,
    (0x43, "f32.const", read_f32) => F32Const,
    (0x44, "f64.const", read_f64) => F64Const,

    (0x45, "i32.eqz", read_none) => I32Eqz,
    (0x46, "i32.eq", read_none) => I32Eq,
    (0x47, "i32.ne", read_none) => I32Ne,
    (0x48, "i32.lt_s", read_none) => I32Lt_S,
    (0x49, "i32.lt_u", read_none) => I32Lt_U,
    (0x4A, "i32.gt_s", read_none) => I32Gt_S,
    (0x4B, "i32.gt_u", read_none) => I32Gt_U,
    (0x4C, "i32.le_s", read_none) => I32Le_S,
    (0x4D, "i32.le_u", read_none) => I32Le_U,
    (0x4E, "i32.ge_s", read_none) => I32Ge_S,
    (0x4F, "i32.ge_u", read_none) => I32Ge_U,

    (0x50, "i64.eqz", read_none) => I64Eqz,
    (0x51, "i64.eq", read_none) => I64Eq,
    (0x52, "i64.ne", read_none) => I64Ne,
    (0x53, "i64.lt_s", read_none) => I64Lt_S,
    (0x54, "i64.lt_u", read_none) => I64Lt_U,
    (0x55, "i64.gt_s", read_none) => I64Gt_S,
    (0x56, "i64.gt_u", read_none) => I64Gt_U,
    (0x57, "i64.le_s", read_none) => I64Le_S,
    (0x58, "i64.le_u", read_none) => I64Le_U,
    (0x59, "i64.ge_s", read_none) => I64Ge_S,
    (0x5A, "i64.ge_u", read_none) => I64Ge_U,

    (0x5B, "f32.eq", read_none) => F32Eq,
    (0x5C, "f32.ne", read_none) => F32Ne,
    (0x5D, "f32.lt", read_none) => F32Lt,
    (0x5E, "f32.gt", read_none) => F32Gt,
    (0x5F, "f32.le", read_none) => F32Le,
    (0x60, "f32.ge", read_none) => F32Ge,

    (0x61, "f64.eq", read_none) => F64Eq,
    (0x62, "f64.ne", read_none) => F64Ne,
    (0x63, "f64.lt", read_none) => F64Lt,
    (0x64, "f64.gt", read_none) => F64Gt,
    (0x65, "f64.le", read_none) => F64Le,
    (0x66, "f64.ge", read_none) => F64Ge,

    (0x67, "i32.clz", read_none) => I32Clz,
    (0x68, "i32.ctz", read_none) => I32Ctz,
    (0x69, "i32.popcnt", read_none) => I32Popcnt,
    (0x6A, "i32.add", read_none) => I32Add,
    (0x6B, "i32.sub", read_none) => I32Sub,
    (0x6C, "i32.mul", read_none) => I32Mul,
    (0x6D, "i32.div_s", read_none) => I32Div_S,
    (0x6E, "i32.div_u", read_none) => I32Div_U,
    (0x6F, "i32.rem_s", read_none) => I32Rem_S,
    (0x70, "i32.rem_u", read_none) => I32Rem_U,
    (0x71, "i32.and", read_none) => I32And,
    (0x72, "i32.or", read_none) => I32Or,
    (0x73, "i32.xor", read_none) => I32Xor,
    (0x74, "i32.shl", read_none) => I32Shl,
    (0x75, "i32.shr_s", read_none) => I32Shr_S,
    (0x76, "i32.shr_u", read_none) => I32Shr_U,
    (0x77, "i32.rotl", read_none) => I32Rotl,
    (0x78, "i32.rotr", read_none) => I32Rotr,

    (0x79, "i64.clz", read_none) => I64Clz,
    (0x7A, "i64.ctz", read_none) => I64Ctz,
    (0x7B, "i64.popcnt", read_none) => I64Popcnt,
    (0x7C, "i64.add", read_none) => I64Add,
    (0x7D, "i64.sub", read_none) => I64Sub,
    (0x7E, "i64.mul", read_none) => I64Mul,
    (0x7F, "i64.div_s", read_none) => I64Div_S,
    (0x80, "i64.div_u", read_none) => I64Div_U,
    (0x81, "i64.rem_s", read_none) => I64Rem_S,
    (0x82, "i64.rem_u", read_none) => I64Rem_U,
    (0x83, "i64.and", read_none) => I64And,
    (0x84, "i64.or", read_none) => I64Or,
    (0x85, "i64.xor", read_none) => I64Xor,
    (0x86, "i64.shl", read_none) => I64Shl,
    (0x87, "i64.shr_s", read_none) => I64Shr_S,
    (0x88, "i64.shr_u", read_none) => I64Shr_U,
    (0x89, "i64.rotl", read_none) => I64Rotl,
    (0x8A, "i64.rotr", read_none) => I64Rotr,

    (0x8B, "f32.abs", read_none) => F32Abs,
    (0x8C, "f32.neg", read_none) => F32Neg,
    (0x8D, "f32.ceil", read_none) => F32Ceil,
    (0x8E, "f32.floor", read_none) => F32Floor,
    (0x8F, "f32.trunc", read_none) => F32Trunc,
    (0x90, "f32.nearest", read_none) => F32Nearest,
    (0x91, "f32.sqrt", read_none) => F32Sqrt,
    (0x92, "f32.add", read_none) => F32Add,
    (0x93, "f32.sub", read_none) => F32Sub,
    (0x94, "f32.mul", read_none) => F32Mul,
    (0x95, "f32.div", read_none) => F32Div,
    (0x96, "f32.min", read_none) => F32Min,
    (0x97, "f32.max", read_none) => F32Max,
    (0x98, "f32.copysign", read_none) => F32Copysign,

    (0x99, "f64.abs", read_none) => F64Abs,
    (0x9A, "f64.neg", read_none) => F64Neg,
    (0x9B, "f64.ceil", read_none) => F64Ceil,
    (0x9C, "f64.floor", read_none) => F64Floor,
    (0x9D, "f64.trunc", read_none) => F64Trunc,
    (0x9E, "f64.nearest", read_none) => F64Nearest,
    (0x9F, "f64.sqrt", read_none) => F64Sqrt,
    (0xA0, "f64.add", read_none) => F64Add,
    (0xA1, "f64.sub", read_none) => F64Sub,
    (0xA2, "f64.mul", read_none) => F64Mul,
    (0xA3, "f64.div", read_none) => F64Div,
    (0xA4, "f64.min", read_none) => F64Min,
    (0xA5, "f64.max", read_none) => F64Max,
    (0xA6, "f64.copysign", read_none) => F64Copysign,

    (0xA7, "i32.wrap/i64", read_none) => I32Wrap_I64,
    (0xA8, "i32.trunc_s/f32", read_none) => I32Trunc_S_F32,
    (0xA9, "i32.trunc_u/f32", read_none) => I32Trunc_U_F32,
    (0xAA, "i32.trunc_s/f64", read_none) => I32Trunc_S_F64,
    (0xAB, "i32.trunc_u/f64", read_none) => I32Trunc_U_F64,
    (0xAC, "i64.extend_s/i32", read_none) => I64Extend_S_I32,
    (0xAD, "i64.extend_u/i32", read_none) => I64Extend_U_I32,
    (0xAE, "i64.trunc_s/f32", read_none) => I64Trunc_S_F32,
    (0xAF, "i64.trunc_u/f32", read_none) => I64Trunc_U_F32,
    (0xB0, "i64.trunc_s/f64", read_none) => I64Trunc_S_F64,
    (0xB1, "i64.trunc_u/f64", read_none) => I64Trunc_U_F64,
    (0xB2, "f32.convert_s/i32", read_none) => F32Convert_S_I32,
    (0xB3, "f32.convert_u/i32", read_none) => F32Convert_U_I32,
    (0xB4, "f32.convert_s/i64", read_none) => F32Convert_S_I64,
    (0xB5, "f32.convert_u/i64", read_none) => F32Convert_U_I64,
    (0xB6, "f32.demote/f64", read_none) => F32Demote_F64,
    (0xB7, "f64.convert_s/i32", read_none) => F64Convert_S_I32,
    (0xB8, "f64.convert_u/i32", read_none) => F64Convert_U_I32,
    (0xB9, "f64.convert_s/i64", read_none) => F64Convert_S_I64,
    (0xBA, "f64.convert_u/i64", read_none) => F64Convert_U_I64,
    (0xBB, "f64.promote/f32", read_none) => F64Promote_F32,
    (0xBC, "i32.reinterpret/f32", read_none) => I32Reinterpret_F32,
    (0xBD, "i64.reinterpret/f64", read_none) => I64Reinterpret_F64,
    (0xBE, "f32.reinterpret/i32", read_none) => F32Reinterpret_I32,
    (0xBF, "f64.reinterpret/i64", read_none) => F64Reinterpret_I64,

    // Psuedo-Instruction 'end'
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    (0x0B, "end", read_none) => End,

    // Pseudo-Instruction 'else'
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    (0x05, "else", read_none) => Else, // Else is a pseudo instruction
}

#[inline]
fn read_none<R: io::Read>(_: &mut R) -> Result<InstructionPayload, Error> {
    Ok(InstructionPayload::Empty)
}

#[inline]
fn read_idx<R: io::Read>(reader: &mut R) -> Result<InstructionPayload, Error> {
    Ok(InstructionPayload::Idx(utils::read_leb128_u32(reader)?))
}

#[inline]
fn read_mem_arg<R: io::Read>(reader: &mut R) -> Result<InstructionPayload, Error> {
    Ok(InstructionPayload::MemArg(
        utils::read_leb128_u32(reader)?,
        utils::read_leb128_u32(reader)?,
    ))
}

#[inline]
fn read_i32<R: io::Read>(reader: &mut R) -> Result<InstructionPayload, Error> {
    Ok(InstructionPayload::Const(Value::I32(utils::read_leb128_s(
        reader,
    )?)))
}

#[inline]
fn read_i64<R: io::Read>(reader: &mut R) -> Result<InstructionPayload, Error> {
    Ok(InstructionPayload::Const(Value::I64(utils::read_leb128_s(
        reader,
    )?)))
}

#[inline]
fn read_f32<R: io::Read>(reader: &mut R) -> Result<InstructionPayload, Error> {
    let bits = reader.read_u32::<LittleEndian>()?;
    Ok(InstructionPayload::Const(Value::F32(f32::from_bits(bits))))
}

#[inline]
fn read_f64<R: io::Read>(reader: &mut R) -> Result<InstructionPayload, Error> {
    let bits = reader.read_u64::<LittleEndian>()?;
    Ok(InstructionPayload::Const(Value::F64(f64::from_bits(bits))))
}
