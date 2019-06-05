use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, CheckedNextPowerOfTwo, FloorLogTwo,
    IsPowerOfTwo, ModPowerOfTwo, ModPowerOfTwoAssign, NegMod, NegModAssign, NextPowerOfTwo,
    NextPowerOfTwoAssign, RemPowerOfTwo, RemPowerOfTwoAssign,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::FromU32Slice;

/// This trait defines functions on primitive unsigned integral types: uxx and usize.
pub trait PrimitiveUnsigned:
    CeilingLogTwo
    + CeilingDivNegMod
    + CeilingDivAssignNegMod
    + CheckedNextPowerOfTwo<Output = Self>
    + FloorLogTwo
    + From<u8>
    + FromU32Slice
    + IsPowerOfTwo
    + ModPowerOfTwo<Output = Self>
    + ModPowerOfTwoAssign
    + NegMod
    + NegModAssign
    + NextPowerOfTwo<Output = Self>
    + NextPowerOfTwoAssign
    + PrimitiveInteger
    + RemPowerOfTwo<Output = Self>
    + RemPowerOfTwoAssign
{
    type SignedOfEqualWidth: PrimitiveSigned;
}

/// This macro defines basic trait implementations for unsigned types.
macro_rules! impl_basic_traits {
    ($t:ident, $st: ident) => {
        impl PrimitiveUnsigned for $t {
            type SignedOfEqualWidth = $st;
        }
    };
}

impl_basic_traits!(u8, i8);
impl_basic_traits!(u16, i16);
impl_basic_traits!(u32, i32);
impl_basic_traits!(u64, i64);
impl_basic_traits!(u128, i128);
impl_basic_traits!(usize, isize);
