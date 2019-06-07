use std::ops::Neg;

use num::arithmetic::traits::{
    Abs, CeilingMod, CeilingModAssign, CheckedAbs, NegAssign, OverflowingAbs, Sign, UnsignedAbs,
    WrappingAbs,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::NegativeOne;
use num::basic::unsigneds::PrimitiveUnsigned;

/// This trait defines functions on primitive unsigned integral types: ixx and isize.
pub trait PrimitiveSigned:
    Abs<Output = Self>
    + CeilingMod
    + CeilingModAssign
    + CheckedAbs<Output = Self>
    + From<i8>
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OverflowingAbs<Output = Self>
    + PrimitiveInteger
    + Sign
    + UnsignedAbs
    + WrappingAbs<Output = Self>
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
