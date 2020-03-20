use std::ops::Neg;

use num::arithmetic::traits::{
    Abs, AbsAssign, CeilingMod, CeilingModAssign, CheckedAbs, NegAssign, OverflowingAbs,
    OverflowingAbsAssign, SaturatingAbs, SaturatingAbsAssign, SaturatingNeg, SaturatingNegAssign,
    UnsignedAbs, WrappingAbs, WrappingAbsAssign,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::NegativeOne;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::logic::traits::CheckedHammingDistance;

/// This trait defines functions on primitive unsigned integral types: ixx and isize.
pub trait PrimitiveSigned:
    Abs<Output = Self>
    + AbsAssign
    + CeilingMod
    + CeilingModAssign
    + CheckedAbs<Output = Self>
    + CheckedHammingDistance
    + From<i8>
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OverflowingAbs<Output = Self>
    + OverflowingAbsAssign
    + PrimitiveInteger
    + SaturatingAbs<Output = Self>
    + SaturatingAbsAssign
    + SaturatingNeg<Output = Self>
    + SaturatingNegAssign
    + UnsignedAbs
    + WrappingAbs<Output = Self>
    + WrappingAbsAssign
{
    type UnsignedOfEqualWidth: PrimitiveUnsigned;
}

/// This macro defines basic trait implementations for signed types.
macro_rules! impl_basic_traits {
    ($t:ident, $ut: ident) => {
        impl PrimitiveSigned for $t {
            type UnsignedOfEqualWidth = $ut;
        }

        /// The constant -1.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1;
        }
    };
}

impl_basic_traits!(i8, u8);
impl_basic_traits!(i16, u16);
impl_basic_traits!(i32, u32);
impl_basic_traits!(i64, u64);
impl_basic_traits!(i128, u128);
impl_basic_traits!(isize, usize);
