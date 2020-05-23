use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, CheckedLogTwo, CheckedNextPowerOfTwo,
    FloorLogTwo, IsPowerOfTwo, ModAdd, ModAddAssign, ModIsReduced, ModMul, ModMulAssign,
    ModMulPrecomputed, ModMulPrecomputedAssign, ModNeg, ModNegAssign, ModPowerOfTwo,
    ModPowerOfTwoAdd, ModPowerOfTwoAddAssign, ModPowerOfTwoIsReduced, ModPowerOfTwoMul,
    ModPowerOfTwoMulAssign, ModPowerOfTwoNeg, ModPowerOfTwoNegAssign, ModPowerOfTwoShl,
    ModPowerOfTwoShlAssign, ModPowerOfTwoShr, ModPowerOfTwoShrAssign, ModPowerOfTwoSub,
    ModPowerOfTwoSubAssign, ModSub, ModSubAssign, NegMod, NegModAssign, NegModPowerOfTwo,
    NegModPowerOfTwoAssign, NextPowerOfTwo, NextPowerOfTwoAssign, RemPowerOfTwo,
    RemPowerOfTwoAssign, XMulYIsZZ, XXAddYYIsZZ, XXDivModYIsQR, XXSubYYIsZZ, XXXAddYYYIsZZZ,
    XXXSubYYYIsZZZ, XXXXAddYYYYIsZZZZ,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::{FromOtherTypeSlice, VecFromOtherType, VecFromOtherTypeSlice};
use num::logic::traits::{HammingDistance, PowerOfTwoDigitIterable, PowerOfTwoDigits};

/// This trait defines functions on primitive unsigned integral types: uxx and usize.
pub trait PrimitiveUnsigned:
    CeilingLogTwo
    + CeilingDivNegMod
    + CeilingDivAssignNegMod
    + CheckedLogTwo
    + CheckedNextPowerOfTwo<Output = Self>
    + FloorLogTwo
    + From<u8>
    + FromOtherTypeSlice<u8>
    + FromOtherTypeSlice<u16>
    + FromOtherTypeSlice<u32>
    + FromOtherTypeSlice<u64>
    + FromOtherTypeSlice<u128>
    + FromOtherTypeSlice<usize>
    + HammingDistance
    + IsPowerOfTwo
    + ModIsReduced<Self>
    + ModAdd<Self, Self, Output = Self>
    + ModAddAssign<Self, Self>
    + ModMul<Self, Self, Output = Self>
    + ModMulAssign<Self, Self>
    + ModMulPrecomputed<Self, Self, Output = Self>
    + ModMulPrecomputedAssign<Self, Self>
    + ModNeg<Self, Output = Self>
    + ModNegAssign<Self>
    + ModPowerOfTwo<Output = Self>
    + ModPowerOfTwoAdd<Self, Output = Self>
    + ModPowerOfTwoAddAssign<Self>
    + ModPowerOfTwoIsReduced
    + ModPowerOfTwoMul<Self, Output = Self>
    + ModPowerOfTwoMulAssign<Self>
    + ModPowerOfTwoNeg<Output = Self>
    + ModPowerOfTwoNegAssign
    + ModPowerOfTwoShl<i8, Output = Self>
    + ModPowerOfTwoShl<i16, Output = Self>
    + ModPowerOfTwoShl<i32, Output = Self>
    + ModPowerOfTwoShl<i64, Output = Self>
    + ModPowerOfTwoShl<i128, Output = Self>
    + ModPowerOfTwoShl<u8, Output = Self>
    + ModPowerOfTwoShl<u16, Output = Self>
    + ModPowerOfTwoShl<u32, Output = Self>
    + ModPowerOfTwoShl<u64, Output = Self>
    + ModPowerOfTwoShl<u128, Output = Self>
    + ModPowerOfTwoShlAssign<u8>
    + ModPowerOfTwoShlAssign<u16>
    + ModPowerOfTwoShlAssign<u32>
    + ModPowerOfTwoShlAssign<u64>
    + ModPowerOfTwoShlAssign<u128>
    + ModPowerOfTwoShlAssign<usize>
    + ModPowerOfTwoShlAssign<i8>
    + ModPowerOfTwoShlAssign<i16>
    + ModPowerOfTwoShlAssign<i32>
    + ModPowerOfTwoShlAssign<i64>
    + ModPowerOfTwoShlAssign<i128>
    + ModPowerOfTwoShlAssign<isize>
    + ModPowerOfTwoShr<i8, Output = Self>
    + ModPowerOfTwoShr<i16, Output = Self>
    + ModPowerOfTwoShr<i32, Output = Self>
    + ModPowerOfTwoShr<i64, Output = Self>
    + ModPowerOfTwoShr<i128, Output = Self>
    + ModPowerOfTwoShrAssign<i8>
    + ModPowerOfTwoShrAssign<i16>
    + ModPowerOfTwoShrAssign<i32>
    + ModPowerOfTwoShrAssign<i64>
    + ModPowerOfTwoShrAssign<i128>
    + ModPowerOfTwoShrAssign<isize>
    + ModPowerOfTwoSub<Self, Output = Self>
    + ModPowerOfTwoSubAssign<Self>
    + ModSub<Self, Self, Output = Self>
    + ModSubAssign<Self, Self>
    + NegMod
    + NegModAssign
    + NegModPowerOfTwo<Output = Self>
    + NegModPowerOfTwoAssign
    + NextPowerOfTwo<Output = Self>
    + NextPowerOfTwoAssign
    + PowerOfTwoDigits<u8>
    + PowerOfTwoDigits<u16>
    + PowerOfTwoDigits<u32>
    + PowerOfTwoDigits<u64>
    + PowerOfTwoDigits<u128>
    + PowerOfTwoDigits<usize>
    + PowerOfTwoDigitIterable<u8>
    + PowerOfTwoDigitIterable<u16>
    + PowerOfTwoDigitIterable<u32>
    + PowerOfTwoDigitIterable<u64>
    + PowerOfTwoDigitIterable<u128>
    + PowerOfTwoDigitIterable<usize>
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
    + XXAddYYIsZZ
    + XXDivModYIsQR
    + XXSubYYIsZZ
    + XMulYIsZZ
    + XXXAddYYYIsZZZ
    + XXXSubYYYIsZZZ
    + XXXXAddYYYYIsZZZZ
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
