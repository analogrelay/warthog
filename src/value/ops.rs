use crate::TrapCause;

/// Basic arithmetic operations defined for BOTH integers and floats.
pub trait ArithmeticOps<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
    fn sub(self, rhs: RHS) -> Self::Output;
    fn mul(self, rhs: RHS) -> Self::Output;
    fn div(self, rhs: RHS) -> Result<Self::Output, TrapCause>;
}

macro_rules! impl_arith_for_integer {
    ($t: ty) => {
        impl ArithmeticOps<$t> for $t {
            type Output = $t;

            fn add(self, rhs: $t) -> $t {
                self.wrapping_add(rhs)
            }

            fn sub(self, rhs: $t) -> $t {
                self.wrapping_sub(rhs)
            }

            fn mul(self, rhs: $t) -> $t {
                self.wrapping_mul(rhs)
            }

            fn div(self, rhs: $t) -> Result<$t, TrapCause> {
                if rhs == 0 {
                    Err(TrapCause::IntegerDivideByZero)
                } else {
                    match self.checked_div(rhs) {
                        Some(y) => Ok(y),
                        None => Err(TrapCause::IntegerOverflow)
                    }
                }
            }
        }
    };
}

impl_arith_for_integer!(u32);
impl_arith_for_integer!(u64);
impl_arith_for_integer!(i32);
impl_arith_for_integer!(i64);

/// Operations defined only for Integers
pub trait IntegerOps<RHS = Self> {
    type Output;

    fn clz(self) -> Self::Output;
    fn ctz(self) -> Self::Output;
    fn popcnt(self) -> Self::Output;
    fn shl(self, rhs: RHS) -> Self::Output;
    fn shr(self, rhs: RHS) -> Self::Output;
    fn rotl(self, rhs: RHS) -> Self::Output;
    fn rotr(self, rhs: RHS) -> Self::Output;
    fn rem(self, rhs: RHS) -> Result<Self::Output, TrapCause>;
}

macro_rules! impl_integer {
    ($t: ty) => {
        impl IntegerOps<$t> for $t {
            type Output = $t;

            fn clz(self) -> $t {
                self.leading_zeros() as $t
            }

            fn ctz(self) -> $t {
                self.trailing_zeros() as $t
            }

            fn popcnt(self) -> $t {
                self.count_ones() as $t
            }

            fn shl(self, rhs: $t) -> Self::Output {
                self.wrapping_shl(rhs as u32) as $t
            }

            fn shr(self, rhs: $t) -> Self::Output {
                self.wrapping_shr(rhs as u32) as $t
            }

            fn rotl(self, rhs: $t) -> $t {
                self.rotate_left(rhs as u32)
            }

            fn rotr(self, rhs: $t) -> $t {
                self.rotate_right(rhs as u32)
            }

            fn rem(self, rhs: $t) -> Result<$t, TrapCause> {
                if rhs == 0 {
                    Err(TrapCause::IntegerDivideByZero)
                } else {
                    Ok(self.overflowing_rem(rhs).0)
                }
            }
        }
    };
}

impl_integer!(u32);
impl_integer!(u64);
impl_integer!(i32);
impl_integer!(i64);

pub trait ConvertInto<T> {
    fn convert_into(self) -> T;
}

macro_rules! impl_convert_by_cast {
    ($from: ty, $to: ty) => {
        impl ConvertInto<$to> for $from {
            fn convert_into(self) -> $to {
                self as $to
            }
        }
    };
}

impl_convert_by_cast!(u64, u32);
impl_convert_by_cast!(u32, u64);
impl_convert_by_cast!(i32, i64);
