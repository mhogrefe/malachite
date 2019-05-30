use std::ops::Neg;

use num::arithmetic::traits::{
    Abs, CeilingMod, CeilingModAssign, CheckedAbs, NegAssign, OverflowingAbs, Sign, UnsignedAbs,
    WrappingAbs,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::NegativeOne;
use num::basic::unsigneds::PrimitiveUnsigned;

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

macro_rules! impl_basic_traits {
    ($t:ident, $ut: ident, $width:expr) => {
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

impl_basic_traits!(i8, u8, 8);
impl_basic_traits!(i16, u16, 16);
impl_basic_traits!(i32, u32, 32);
impl_basic_traits!(i64, u64, 64);
impl_basic_traits!(i128, u128, 128);
impl_basic_traits!(isize, usize, 0usize.trailing_zeros());
