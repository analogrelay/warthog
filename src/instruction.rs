use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{utils, Error, ValType, Value};

macro_rules! drop_tt {
    ($($ts: tt)*) => (_)
}

macro_rules! instruction_format {
    // Finalizer
    (@a $inp: expr; $w: expr; ($($accum:tt)*); ) => {
        match $inp {
            $($accum)*
        }
    };

    // Accumulators
    (@a $inp: expr; $w: expr; ($($accum:tt)*); ($name: ident, $text: expr, $p1: ty, $p2: ty), $($rest: tt)*) => {
        instruction_format!(@a $inp; $w;
            (
                $($accum)*
                Instruction::$name(p1, p2) => write!($w, concat!($text, " {} {}"), p1, p2),
            );
            $($rest)*);
    };

    (@a $inp: expr; $w: expr; ($($accum:tt)*); ($name: ident, $text: expr, $p1: ty), $($rest: tt)*) => {
        instruction_format!(@a $inp; $w;
            (
                $($accum)*
                Instruction::$name(p1) => write!($w, concat!($text, " {}"), p1),
            );
            $($rest)*);
    };

    (@a $inp: expr; $w: expr; ($($accum:tt)*); ($name: ident, $text: expr, ), $($rest: tt)*) => {
        instruction_format!(@a $inp; $w;
            (
                $($accum)*
                Instruction::$name => write!($w, $text),
            );
            $($rest)*);
    };

    // Entry point
    ($inp: expr; $w: expr; $(($name: ident, $text: expr, $($read_method: ident : $param: ty),*),)*) => {
        instruction_format!(@a $inp; $w; ();
            $(
                ($name, $text, $($param),*),
            )*);
    };
}

macro_rules! instruction_get_opcode {
    // Finalizer
    (@a $val: expr; ($($accum: tt)*); ) => {
        match $val {
            $($accum)*
        }
    };

    // Accumulators
    (@a $val: expr; ($($accum: tt)*); ($opcode: expr, $name: ident, $($read_method: ident: $param: ty),+ ), $($rest: tt)*) => {
        instruction_get_opcode!(@a $val;
            (
                Instruction::$name($(drop_tt!($param)),+) => $opcode,
                $($accum)*
            );
            $($rest)*);
    };

    (@a $val: expr; ($($accum: tt)*); ($opcode: expr, $name: ident, ), $($rest: tt)*) => {
        instruction_get_opcode!(@a $val;
            (
                Instruction::$name => $opcode,
                $($accum)*
            );
            $($rest)*);
    };

    // Entry Point
    ($val: expr; (); $(($opcode: expr, $name: ident, $($defn: tt)*),)*) => {
        instruction_get_opcode!(@a $val; (); $(($opcode, $name, $($defn)*),)*);
    };
}

macro_rules! instruction_enum {
    // Finalizer
    (@a ($($accum: tt)*); ) => {
        #[derive(Clone, Copy, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum Instruction { $($accum)* }
    };

    // Accumulators
    (@a ($($accum: tt)*); $name: ident($( $read_method: ident: $param: ty ),+) , $($rest: tt)*) => {
        instruction_enum!(@a
            (
                $name($($param),*),
                $($accum)*
            );
            $($rest)*);
    };

    (@a ($($accum: tt)*); $name: ident() , $($rest: tt)*) => {
        instruction_enum!(@a
            (
                $name,
                $($accum)*
            );
            $($rest)*);
    };

    // Entrypoint
    ($($items: tt)*) => {
        instruction_enum!(@a (); $($items)*);
    };
}

macro_rules! instruction_read {
    ($reader: ident, $name: ident, ) => {
        Instruction::$name
    };
    ($reader: ident, $name: ident, $( $read_method: ident: $param: ty ),+) => {
        Instruction::$name($(
            $read_method($reader)?
        ),+)
    };
}

macro_rules! isa {
    ($(
        ($opcode: expr, $text: expr) => $name:ident ($($defn: tt)*)
    ),*,) => {
        instruction_enum!($(
            $name($($defn)*),
        )*);

        impl Instruction {
            pub fn read<R: ::std::io::Read>(reader: &mut R) -> Result<Instruction, $crate::Error> {
                let opcode = ::byteorder::ReadBytesExt::read_u8(reader)?;
                match opcode {
                    $(
                        $opcode => Ok(instruction_read!(reader, $name, $($defn)*)),
                    )*
                    x => Err($crate::Error::UnknownOpcode(x))
                }
            }

            pub fn read_sequence<R: ::std::io::Read>(reader: &mut R) -> Result<Vec<Instruction>, $crate::Error> {
                let mut insts = Vec::new();
                let mut blocks = 1;
                loop {
                    let inst = Instruction::read(reader)?;
                    if inst.is_block() {
                        blocks += 1;
                    } else if inst == Instruction::End {
                        blocks -= 1;
                        if blocks == 0 {
                            return Ok(insts);
                        }
                    }
                    insts.push(inst);
                }
            }

            pub fn opcode(&self) -> u8 {
                instruction_get_opcode!(self; (); $(
                    ($opcode, $name, $($defn)*),
                )*)
            }

            pub fn is_block(&self) -> bool {
                match self {
                    Instruction::Block(_) => true,
                    Instruction::Loop(_) => true,
                    Instruction::If(_) => true,
                    _ => false,
                }
            }
        }

        impl ::std::fmt::Display for Instruction {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                instruction_format!(self; f; $(
                    ($name, $text, $($defn)*),
                )*)
            }
        }

        impl ::std::fmt::Debug for Instruction {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(self, f)
            }
        }
    };
}

isa! {
    // Control Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    (0x00, "unreachable") => Unreachable(),
    (0x01, "nop") => Nop(),
    (0x02, "block") => Block(read_val_type: ValType),
    (0x03, "loop") => Loop(read_val_type: ValType),
    (0x04, "if") => If(read_val_type: ValType),
    (0x0C, "br") => Br(),
    (0x0D, "br_if") => Br_If(),
    (0x0E, "br_table") => Br_Table(),
    (0x0F, "return") => Return(),
    (0x10, "call") => Call(read_raw_u32: u32),
    (0x11, "call_indirect") => Call_Indirect(read_raw_u32: u32),

    // Parametric Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#parametric-instructions
    (0x1A, "drop") => Drop(),
    (0x1B, "select") => Select(),

    // Variable Instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#variable-instructions
    (0x20, "get_local") => GetLocal(read_raw_u32: u32),
    (0x21, "set_local") => SetLocal(read_raw_u32: u32),
    (0x22, "tee_local") => TeeLocal(read_raw_u32: u32),
    (0x23, "get_global") => GetGlobal(read_raw_u32: u32),
    (0x24, "set_global") => SetGlobal(read_raw_u32: u32),

    // Memory Instructions,
    // https://webassembly.github.io/spec/core/binary/instructions.html#memory-instructions
    (0x28, "i32.load") => I32Load(read_raw_u32: u32, read_raw_u32: u32),
    (0x29, "i64.load") => I64Load(read_raw_u32: u32, read_raw_u32: u32),
    (0x2A, "f32.load") => F32Load(read_raw_u32: u32, read_raw_u32: u32),
    (0x2B, "f64.load") => F64Load(read_raw_u32: u32, read_raw_u32: u32),
    (0x2C, "i32.load8_s") => I32Load8_S(read_raw_u32: u32, read_raw_u32: u32),
    (0x2D, "i32.load8_u") => I32Load8_U(read_raw_u32: u32, read_raw_u32: u32),
    (0x2E, "i32.load16_s") => I32Load16_S(read_raw_u32: u32, read_raw_u32: u32),
    (0x2F, "i32.load16_u") => I32Load16_U(read_raw_u32: u32, read_raw_u32: u32),
    (0x30, "i64.load8_s") => I64Load8_S(read_raw_u32: u32, read_raw_u32: u32),
    (0x31, "i64.load8_u") => I64Load8_U(read_raw_u32: u32, read_raw_u32: u32),
    (0x32, "i64.load16_s") => I64Load16_S(read_raw_u32: u32, read_raw_u32: u32),
    (0x33, "i64.load16_u") => I64Load16_U(read_raw_u32: u32, read_raw_u32: u32),
    (0x34, "i64.load32_s") => I64Load32_S(read_raw_u32: u32, read_raw_u32: u32),
    (0x35, "i64.load32_u") => I64Load32_U(read_raw_u32: u32, read_raw_u32: u32),
    (0x36, "i32.store") => I32Store(read_raw_u32: u32, read_raw_u32: u32),
    (0x37, "i64.store") => I64Store(read_raw_u32: u32, read_raw_u32: u32),
    (0x38, "f32.store") => F32Store(read_raw_u32: u32, read_raw_u32: u32),
    (0x39, "f64.store") => F64Store(read_raw_u32: u32, read_raw_u32: u32),
    (0x3A, "i32.store8") => I32Store8(read_raw_u32: u32, read_raw_u32: u32),
    (0x3B, "i32.store16") => I32Store16(read_raw_u32: u32, read_raw_u32: u32),
    (0x3C, "i64.store8") => I64Store8(read_raw_u32: u32, read_raw_u32: u32),
    (0x3D, "i64.store16") => I64Store16(read_raw_u32: u32, read_raw_u32: u32),
    (0x3E, "i64.store32") => I64Store32(read_raw_u32: u32, read_raw_u32: u32),
    (0x3F, "memory.size") => Memory_Size(read_raw_u32: u32),
    (0x40, "memory.grow") => Memory_Grow(read_raw_u32: u32),

    // Numeric instructions
    // https://webassembly.github.io/spec/core/binary/instructions.html#numeric-instructions
    (0x41, "i32.const") => I32Const(read_i32: Value),
    (0x42, "i64.const") => I64Const(read_i64: Value),
    (0x43, "f32.const") => F32Const(read_f32: Value),
    (0x44, "f64.const") => F64Const(read_f64: Value),

    (0x45, "i32.eqz") => I32Eqz(),
    (0x46, "i32.eq") => I32Eq(),
    (0x47, "i32.ne") => I32Ne(),
    (0x48, "i32.lt_s") => I32Lt_S(),
    (0x49, "i32.lt_u") => I32Lt_U(),
    (0x4A, "i32.gt_s") => I32Gt_S(),
    (0x4B, "i32.gt_u") => I32Gt_U(),
    (0x4C, "i32.le_s") => I32Le_S(),
    (0x4D, "i32.le_u") => I32Le_U(),
    (0x4E, "i32.ge_s") => I32Ge_S(),
    (0x4F, "i32.ge_u") => I32Ge_U(),

    (0x50, "i64.eqz") => I64Eqz(),
    (0x51, "i64.eq") => I64Eq(),
    (0x52, "i64.ne") => I64Ne(),
    (0x53, "i64.lt_s") => I64Lt_S(),
    (0x54, "i64.lt_u") => I64Lt_U(),
    (0x55, "i64.gt_s") => I64Gt_S(),
    (0x56, "i64.gt_u") => I64Gt_U(),
    (0x57, "i64.le_s") => I64Le_S(),
    (0x58, "i64.le_u") => I64Le_U(),
    (0x59, "i64.ge_s") => I64Ge_S(),
    (0x5A, "i64.ge_u") => I64Ge_U(),

    (0x5B, "f32.eq") => F32Eq(),
    (0x5C, "f32.ne") => F32Ne(),
    (0x5D, "f32.lt") => F32Lt(),
    (0x5E, "f32.gt") => F32Gt(),
    (0x5F, "f32.le") => F32Le(),
    (0x60, "f32.ge") => F32Ge(),

    (0x61, "f64.eq") => F64Eq(),
    (0x62, "f64.ne") => F64Ne(),
    (0x63, "f64.lt") => F64Lt(),
    (0x64, "f64.gt") => F64Gt(),
    (0x65, "f64.le") => F64Le(),
    (0x66, "f64.ge") => F64Ge(),

    (0x67, "i32.clz") => I32Clz(),
    (0x68, "i32.ctz") => I32Ctz(),
    (0x69, "i32.popcnt") => I32Popcnt(),
    (0x6A, "i32.add") => I32Add(),
    (0x6B, "i32.sub") => I32Sub(),
    (0x6C, "i32.mul") => I32Mul(),
    (0x6D, "i32.div_s") => I32Div_S(),
    (0x6E, "i32.div_u") => I32Div_U(),
    (0x6F, "i32.rem_s") => I32Rem_S(),
    (0x70, "i32.rem_u") => I32Rem_U(),
    (0x71, "i32.and") => I32And(),
    (0x72, "i32.or") => I32Or(),
    (0x73, "i32.xor") => I32Xor(),
    (0x74, "i32.shl") => I32Shl(),
    (0x75, "i32.shr_s") => I32Shr_S(),
    (0x76, "i32.shr_u") => I32Shr_U(),
    (0x77, "i32.rotl") => I32Rotl(),
    (0x78, "i32.rotr") => I32Rotr(),

    (0x79, "i64.clz") => I64Clz(),
    (0x7A, "i64.ctz") => I64Ctz(),
    (0x7B, "i64.popcnt") => I64Popcnt(),
    (0x7C, "i64.add") => I64Add(),
    (0x7D, "i64.sub") => I64Sub(),
    (0x7E, "i64.mul") => I64Mul(),
    (0x7F, "i64.div_s") => I64Div_S(),
    (0x80, "i64.div_u") => I64Div_U(),
    (0x81, "i64.rem_s") => I64Rem_S(),
    (0x82, "i64.rem_u") => I64Rem_U(),
    (0x83, "i64.and") => I64And(),
    (0x84, "i64.or") => I64Or(),
    (0x85, "i64.xor") => I64Xor(),
    (0x86, "i64.shl") => I64Shl(),
    (0x87, "i64.shr_s") => I64Shr_S(),
    (0x88, "i64.shr_u") => I64Shr_U(),
    (0x89, "i64.rotl") => I64Rotl(),
    (0x8A, "i64.rotr") => I64Rotr(),

    (0x8B, "f32.abs") => F32Abs(),
    (0x8C, "f32.neg") => F32Neg(),
    (0x8D, "f32.ceil") => F32Ceil(),
    (0x8E, "f32.floor") => F32Floor(),
    (0x8F, "f32.trunc") => F32Trunc(),
    (0x90, "f32.nearest") => F32Nearest(),
    (0x91, "f32.sqrt") => F32Sqrt(),
    (0x92, "f32.add") => F32Add(),
    (0x93, "f32.sub") => F32Sub(),
    (0x94, "f32.mul") => F32Mul(),
    (0x95, "f32.div") => F32Div(),
    (0x96, "f32.min") => F32Min(),
    (0x97, "f32.max") => F32Max(),
    (0x98, "f32.copysign") => F32Copysign(),

    (0x99, "f64.abs") => F64Abs(),
    (0x9A, "f64.neg") => F64Neg(),
    (0x9B, "f64.ceil") => F64Ceil(),
    (0x9C, "f64.floor") => F64Floor(),
    (0x9D, "f64.trunc") => F64Trunc(),
    (0x9E, "f64.nearest") => F64Nearest(),
    (0x9F, "f64.sqrt") => F64Sqrt(),
    (0xA0, "f64.add") => F64Add(),
    (0xA1, "f64.sub") => F64Sub(),
    (0xA2, "f64.mul") => F64Mul(),
    (0xA3, "f64.div") => F64Div(),
    (0xA4, "f64.min") => F64Min(),
    (0xA5, "f64.max") => F64Max(),
    (0xA6, "f64.copysign") => F64Copysign(),

    (0xA7, "i32.wrap/i64") => I32Wrap_I64(),
    (0xA8, "i32.trunc_s/f32") => I32Trunc_S_F32(),
    (0xA9, "i32.trunc_u/f32") => I32Trunc_U_F32(),
    (0xAA, "i32.trunc_s/f64") => I32Trunc_S_F64(),
    (0xAB, "i32.trunc_u/f64") => I32Trunc_U_F64(),
    (0xAC, "i64.extend_s/i32") => I64Extend_S_I32(),
    (0xAD, "i64.extend_u/i32") => I64Extend_U_I32(),
    (0xAE, "i64.trunc_s/f32") => I64Trunc_S_F32(),
    (0xAF, "i64.trunc_u/f32") => I64Trunc_U_F32(),
    (0xB0, "i64.trunc_s/f64") => I64Trunc_S_F64(),
    (0xB1, "i64.trunc_u/f64") => I64Trunc_U_F64(),
    (0xB2, "f32.convert_s/i32") => F32Convert_S_I32(),
    (0xB3, "f32.convert_u/i32") => F32Convert_U_I32(),
    (0xB4, "f32.convert_s/i64") => F32Convert_S_I64(),
    (0xB5, "f32.convert_u/i64") => F32Convert_U_I64(),
    (0xB6, "f32.demote/f64") => F32Demote_F64(),
    (0xB7, "f64.convert_s/i32") => F64Convert_S_I32(),
    (0xB8, "f64.convert_u/i32") => F64Convert_U_I32(),
    (0xB9, "f64.convert_s/i64") => F64Convert_S_I64(),
    (0xBA, "f64.convert_u/i64") => F64Convert_U_I64(),
    (0xBB, "f64.promote/f32") => F64Promote_F32(),
    (0xBC, "i32.reinterpret/f32") => I32Reinterpret_F32(),
    (0xBD, "i64.reinterpret/f64") => I64Reinterpret_F64(),
    (0xBE, "f32.reinterpret/i32") => F32Reinterpret_I32(),
    (0xBF, "f64.reinterpret/i64") => F64Reinterpret_I64(),

    // Psuedo-Instruction 'end'
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    (0x0B, "end") => End(),

    // Pseudo-Instruction 'else'
    // https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
    (0x05, "else") => Else(), // Else is a pseudo instruction
}

#[inline]
fn read_val_type<R: io::Read>(reader: &mut R) -> Result<ValType, Error> {
    ValType::read(reader)
}

#[inline]
fn read_raw_u32<R: io::Read>(reader: &mut R) -> Result<u32, Error> {
    Ok(utils::read_leb128_u32(reader)?)
}

#[inline]
fn read_i32<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    Ok(Value::I32(utils::read_leb128_s(reader)?))
}

#[inline]
fn read_i64<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    Ok(Value::I64(utils::read_leb128_s(reader)?))
}

#[inline]
fn read_f32<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    let bits = reader.read_u32::<LittleEndian>()?;
    Ok(Value::F32(f32::from_bits(bits)))
}

#[inline]
fn read_f64<R: io::Read>(reader: &mut R) -> Result<Value, Error> {
    let bits = reader.read_u64::<LittleEndian>()?;
    Ok(Value::F64(f64::from_bits(bits)))
}
