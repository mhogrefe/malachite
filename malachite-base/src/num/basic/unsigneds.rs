use num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, CeilingLogBase, CeilingLogBase2,
    CeilingLogBasePowerOf2, CheckedLcm, CheckedLogBase, CheckedLogBase2, CheckedLogBasePowerOf2,
    CheckedNextPowerOf2, CoprimeWith, FloorLogBase, FloorLogBase2, FloorLogBasePowerOf2, Gcd,
    GcdAssign, IsPowerOf2, Lcm, LcmAssign, ModAdd, ModAddAssign, ModIsReduced, ModMul,
    ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModNeg, ModNegAssign, ModPow,
    ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign, ModPowerOf2, ModPowerOf2Add,
    ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
    ModPowerOf2Neg, ModPowerOf2NegAssign, ModPowerOf2Pow, ModPowerOf2PowAssign, ModPowerOf2Shl,
    ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, ModPowerOf2Square,
    ModPowerOf2SquareAssign, ModPowerOf2Sub, ModPowerOf2SubAssign, ModSquare, ModSquareAssign,
    ModSquarePrecomputed, ModSquarePrecomputedAssign, ModSub, ModSubAssign, NegMod, NegModAssign,
    NegModPowerOf2, NegModPowerOf2Assign, NextPowerOf2, NextPowerOf2Assign, RootAssignRem, RootRem,
    SqrtAssignRem, SqrtRem, XMulYIsZZ, XXAddYYIsZZ, XXDivModYIsQR, XXSubYYIsZZ, XXXAddYYYIsZZZ,
    XXXSubYYYIsZZZ, XXXXAddYYYYIsZZZZ,
};
use num::basic::integers::PrimitiveInt;
use num::conversion::traits::{
    Digits, FromOtherTypeSlice, IntegerMantissaAndExponent, PowerOf2DigitIterable, PowerOf2Digits,
    SciMantissaAndExponent, VecFromOtherType, VecFromOtherTypeSlice,
};
use num::logic::traits::HammingDistance;

/// This trait defines functions on primitive unsigned integral types: uxx and usize.
pub trait PrimitiveUnsigned:
    CeilingLogBase<Output = u64>
    + CeilingLogBase2<Output = u64>
    + CeilingLogBasePowerOf2<u64, Output = u64>
    + CeilingDivAssignNegMod<Self, ModOutput = Self>
    + CeilingDivNegMod<Self, DivOutput = Self, ModOutput = Self>
    + CheckedLcm<Self, Output = Self>
    + CheckedLogBase<Output = u64>
    + CheckedLogBase2<Output = u64>
    + CheckedLogBasePowerOf2<u64, Output = u64>
    + CheckedNextPowerOf2<Output = Self>
    + CoprimeWith<Self>
    + Digits<u8>
    + Digits<u16>
    + Digits<u32>
    + Digits<u64>
    + Digits<u128>
    + Digits<usize>
    + FloorLogBase<Output = u64>
    + FloorLogBase2<Output = u64>
    + FloorLogBasePowerOf2<u64, Output = u64>
    + From<u8>
    + FromOtherTypeSlice<u8>
    + FromOtherTypeSlice<u16>
    + FromOtherTypeSlice<u32>
    + FromOtherTypeSlice<u64>
    + FromOtherTypeSlice<u128>
    + FromOtherTypeSlice<usize>
    + Gcd<Self, Output = Self>
    + GcdAssign<Self>
    + HammingDistance
    + IntegerMantissaAndExponent<Self, u64>
    + IsPowerOf2
    + Lcm<Self, Output = Self>
    + LcmAssign<Self>
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
    + RootRem<RootOutput = Self, RemOutput = Self>
    + RootAssignRem<RemOutput = Self>
    + SciMantissaAndExponent<f32, u64>
    + SciMantissaAndExponent<f64, u64>
    + SqrtRem<SqrtOutput = Self, RemOutput = Self>
    + SqrtAssignRem<RemOutput = Self>
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
}

macro_rules! impl_basic_traits {
    ($u:ident) => {
        impl PrimitiveUnsigned for $u {}
    };
}
apply_to_unsigneds!(impl_basic_traits);
