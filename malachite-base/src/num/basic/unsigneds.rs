use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogTwo, CheckedLogTwo, CheckedNextPowerOfTwo,
    FloorLogTwo, IsPowerOfTwo, ModAdd, ModAddAssign, ModIsReduced, ModMul, ModMulAssign,
    ModMulPrecomputed, ModMulPrecomputedAssign, ModNeg, ModNegAssign, ModPow, ModPowAssign,
    ModPowPrecomputed, ModPowPrecomputedAssign, ModPowerOfTwo, ModPowerOfTwoAdd,
    ModPowerOfTwoAddAssign, ModPowerOfTwoIsReduced, ModPowerOfTwoMul, ModPowerOfTwoMulAssign,
    ModPowerOfTwoNeg, ModPowerOfTwoNegAssign, ModPowerOfTwoPow, ModPowerOfTwoPowAssign,
    ModPowerOfTwoShl, ModPowerOfTwoShlAssign, ModPowerOfTwoShr, ModPowerOfTwoShrAssign,
    ModPowerOfTwoSquare, ModPowerOfTwoSquareAssign, ModPowerOfTwoSub, ModPowerOfTwoSubAssign,
    ModSub, ModSubAssign, NegMod, NegModAssign, NegModPowerOfTwo, NegModPowerOfTwoAssign,
    NextPowerOfTwo, NextPowerOfTwoAssign, XMulYIsZZ, XXAddYYIsZZ, XXDivModYIsQR, XXSubYYIsZZ,
    XXXAddYYYIsZZZ, XXXSubYYYIsZZZ, XXXXAddYYYYIsZZZZ,
};
use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::{FromOtherTypeSlice, VecFromOtherType, VecFromOtherTypeSlice};
use num::logic::traits::{HammingDistance, PowerOfTwoDigitIterable, PowerOfTwoDigits};

/// This trait defines functions on primitive unsigned integral types: uxx and usize.
pub trait PrimitiveUnsigned:
    CeilingLogTwo
    + CeilingDivAssignNegMod<Self, ModOutput = Self>
    + CeilingDivNegMod<Self, DivOutput = Self, ModOutput = Self>
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
    + ModPow<u64, Self, Output = Self>
    + ModPowAssign<u64, Self>
    + ModPowerOfTwo<Output = Self>
    + ModPowerOfTwoAdd<Self, Output = Self>
    + ModPowerOfTwoAddAssign<Self>
    + ModPowerOfTwoIsReduced
    + ModPowerOfTwoMul<Self, Output = Self>
    + ModPowerOfTwoMulAssign<Self>
    + ModPowerOfTwoNeg<Output = Self>
    + ModPowerOfTwoNegAssign
    + ModPowerOfTwoPow<u64, Output = Self>
    + ModPowerOfTwoPowAssign<u64>
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
    + ModPowerOfTwoSquare<Output = Self>
    + ModPowerOfTwoSquareAssign
    + ModPowerOfTwoSub<Self, Output = Self>
    + ModPowerOfTwoSubAssign<Self>
    + ModPowPrecomputed<u64, Self, Output = Self>
    + ModPowPrecomputedAssign<u64, Self>
    + ModSub<Self, Self, Output = Self>
    + ModSubAssign<Self, Self>
    + NegMod<Self, Output = Self>
    + NegModAssign<Self>
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
    + PrimitiveInt
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

macro_rules! impl_basic_traits {
    ($u:ident, $s: ident) => {
        impl PrimitiveUnsigned for $u {
            type SignedOfEqualWidth = $s;
        }
    };
}
apply_to_unsigned_signed_pair!(impl_basic_traits);
