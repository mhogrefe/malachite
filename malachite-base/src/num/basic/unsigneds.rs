use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogBase2, CheckedLogBase2,
    CheckedNextPowerOf2, FloorLogBase2, IsPowerOf2, ModAdd, ModAddAssign, ModIsReduced, ModMul,
    ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModNeg, ModNegAssign, ModPow,
    ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign, ModPowerOf2, ModPowerOf2Add,
    ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
    ModPowerOf2Neg, ModPowerOf2NegAssign, ModPowerOf2Pow, ModPowerOf2PowAssign, ModPowerOf2Shl,
    ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, ModPowerOf2Square,
    ModPowerOf2SquareAssign, ModPowerOf2Sub, ModPowerOf2SubAssign, ModSquare, ModSquareAssign,
    ModSquarePrecomputed, ModSquarePrecomputedAssign, ModSub, ModSubAssign, NegMod, NegModAssign,
    NegModPowerOf2, NegModPowerOf2Assign, NextPowerOf2, NextPowerOf2Assign, XMulYIsZZ, XXAddYYIsZZ,
    XXDivModYIsQR, XXSubYYIsZZ, XXXAddYYYIsZZZ, XXXSubYYYIsZZZ, XXXXAddYYYYIsZZZZ,
};
use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::{
    FromOtherTypeSlice, PowerOf2DigitIterable, PowerOf2Digits, VecFromOtherType,
    VecFromOtherTypeSlice,
};
use num::logic::traits::HammingDistance;

/// This trait defines functions on primitive unsigned integral types: uxx and usize.
pub trait PrimitiveUnsigned:
    CeilingLogBase2
    + CeilingDivAssignNegMod<Self, ModOutput = Self>
    + CeilingDivNegMod<Self, DivOutput = Self, ModOutput = Self>
    + CheckedLogBase2
    + CheckedNextPowerOf2<Output = Self>
    + FloorLogBase2
    + From<u8>
    + FromOtherTypeSlice<u8>
    + FromOtherTypeSlice<u16>
    + FromOtherTypeSlice<u32>
    + FromOtherTypeSlice<u64>
    + FromOtherTypeSlice<u128>
    + FromOtherTypeSlice<usize>
    + HammingDistance
    + IsPowerOf2
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
    + ModPowerOf2<Output = Self>
    + ModPowerOf2Add<Self, Output = Self>
    + ModPowerOf2AddAssign<Self>
    + ModPowerOf2IsReduced
    + ModPowerOf2Mul<Self, Output = Self>
    + ModPowerOf2MulAssign<Self>
    + ModPowerOf2Neg<Output = Self>
    + ModPowerOf2NegAssign
    + ModPowerOf2Pow<u64, Output = Self>
    + ModPowerOf2PowAssign<u64>
    + ModPowerOf2Shl<i8, Output = Self>
    + ModPowerOf2Shl<i16, Output = Self>
    + ModPowerOf2Shl<i32, Output = Self>
    + ModPowerOf2Shl<i64, Output = Self>
    + ModPowerOf2Shl<i128, Output = Self>
    + ModPowerOf2Shl<u8, Output = Self>
    + ModPowerOf2Shl<u16, Output = Self>
    + ModPowerOf2Shl<u32, Output = Self>
    + ModPowerOf2Shl<u64, Output = Self>
    + ModPowerOf2Shl<u128, Output = Self>
    + ModPowerOf2ShlAssign<u8>
    + ModPowerOf2ShlAssign<u16>
    + ModPowerOf2ShlAssign<u32>
    + ModPowerOf2ShlAssign<u64>
    + ModPowerOf2ShlAssign<u128>
    + ModPowerOf2ShlAssign<usize>
    + ModPowerOf2ShlAssign<i8>
    + ModPowerOf2ShlAssign<i16>
    + ModPowerOf2ShlAssign<i32>
    + ModPowerOf2ShlAssign<i64>
    + ModPowerOf2ShlAssign<i128>
    + ModPowerOf2ShlAssign<isize>
    + ModPowerOf2Shr<i8, Output = Self>
    + ModPowerOf2Shr<i16, Output = Self>
    + ModPowerOf2Shr<i32, Output = Self>
    + ModPowerOf2Shr<i64, Output = Self>
    + ModPowerOf2Shr<i128, Output = Self>
    + ModPowerOf2ShrAssign<i8>
    + ModPowerOf2ShrAssign<i16>
    + ModPowerOf2ShrAssign<i32>
    + ModPowerOf2ShrAssign<i64>
    + ModPowerOf2ShrAssign<i128>
    + ModPowerOf2ShrAssign<isize>
    + ModPowerOf2Square<Output = Self>
    + ModPowerOf2SquareAssign
    + ModPowerOf2Sub<Self, Output = Self>
    + ModPowerOf2SubAssign<Self>
    + ModPowPrecomputed<u64, Self, Output = Self>
    + ModPowPrecomputedAssign<u64, Self>
    + ModSquare<Self, Output = Self>
    + ModSquareAssign<Self>
    + ModSquarePrecomputed<u64, Self, Output = Self>
    + ModSquarePrecomputedAssign<u64, Self>
    + ModSub<Self, Self, Output = Self>
    + ModSubAssign<Self, Self>
    + NegMod<Self, Output = Self>
    + NegModAssign<Self>
    + NegModPowerOf2<Output = Self>
    + NegModPowerOf2Assign
    + NextPowerOf2<Output = Self>
    + NextPowerOf2Assign
    + PowerOf2Digits<u8>
    + PowerOf2Digits<u16>
    + PowerOf2Digits<u32>
    + PowerOf2Digits<u64>
    + PowerOf2Digits<u128>
    + PowerOf2Digits<usize>
    + PowerOf2DigitIterable<u8>
    + PowerOf2DigitIterable<u16>
    + PowerOf2DigitIterable<u32>
    + PowerOf2DigitIterable<u64>
    + PowerOf2DigitIterable<u128>
    + PowerOf2DigitIterable<usize>
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
    //TODO remove
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
