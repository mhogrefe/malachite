use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, CheckedNextPowerOfTwo, FloorLogTwo,
    IsPowerOfTwo, ModPowerOfTwo, ModPowerOfTwoAssign, NegMod, NegModAssign, NegModPowerOfTwo,
    NegModPowerOfTwoAssign, NextPowerOfTwo, NextPowerOfTwoAssign, RemPowerOfTwo,
    RemPowerOfTwoAssign,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::{FromOtherTypeSlice, VecFromOtherType, VecFromOtherTypeSlice};

/// This trait defines functions on primitive unsigned integral types: uxx and usize.
pub trait PrimitiveUnsigned:
    CeilingLogTwo
    + CeilingDivNegMod
    + CeilingDivAssignNegMod
    + CheckedNextPowerOfTwo<Output = Self>
    + FloorLogTwo
    + From<u8>
    + FromOtherTypeSlice<u8>
    + FromOtherTypeSlice<u16>
    + FromOtherTypeSlice<u32>
    + FromOtherTypeSlice<u64>
    + FromOtherTypeSlice<u128>
    + FromOtherTypeSlice<usize>
    + IsPowerOfTwo
    + ModPowerOfTwo<Output = Self>
    + ModPowerOfTwoAssign
    + NegMod
    + NegModAssign
    + NegModPowerOfTwo<Output = Self>
    + NegModPowerOfTwoAssign
    + NextPowerOfTwo<Output = Self>
    + NextPowerOfTwoAssign
    + PrimitiveInteger
    + RemPowerOfTwo<Output = Self>
    + RemPowerOfTwoAssign
    + VecFromOtherType<u8>
    + VecFromOtherType<u16>
    + VecFromOtherType<u32>
    + VecFromOtherType<u64>
    + VecFromOtherType<u128>
    + VecFromOtherType<usize>
    + VecFromOtherTypeSlice<u8>
    + VecFromOtherTypeSlice<u16>
    + VecFromOtherTypeSlice<u32>
    + VecFromOtherTypeSlice<u64>
    + VecFromOtherTypeSlice<u128>
    + VecFromOtherTypeSlice<usize>
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
