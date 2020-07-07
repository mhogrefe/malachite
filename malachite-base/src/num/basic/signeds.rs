use std::ops::Neg;

use num::arithmetic::traits::{
    Abs, AbsAssign, CeilingDivAssignMod, CeilingDivMod, CeilingMod, CeilingModAssign,
    CeilingModPowerOfTwo, CeilingModPowerOfTwoAssign, CheckedAbs, NegAssign, OverflowingAbs,
    OverflowingAbsAssign, SaturatingAbs, SaturatingAbsAssign, SaturatingNeg, SaturatingNegAssign,
    UnsignedAbs, WrappingAbs, WrappingAbsAssign,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::NegativeOne;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::logic::traits::CheckedHammingDistance;
use num::random::random_bit_chunks::RandomSignedChunkable;

/// This trait defines functions on primitive unsigned integral types: ixx and isize.
pub trait PrimitiveSigned:
    Abs<Output = Self>
    + AbsAssign
    + CeilingDivAssignMod<Self, ModOutput = Self>
    + CeilingDivMod<Self, DivOutput = Self, ModOutput = Self>
    + CeilingMod<Self, Output = Self>
    + CeilingModAssign<Self>
    + CeilingModPowerOfTwo<Output = Self>
    + CeilingModPowerOfTwoAssign
    + CheckedAbs<Output = Self>
    + CheckedHammingDistance
    + From<i8>
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OverflowingAbs<Output = Self>
    + OverflowingAbsAssign
    + PrimitiveInteger
    + RandomSignedChunkable
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
    ($u:ident, $s: ident) => {
        impl PrimitiveSigned for $s {
            type UnsignedOfEqualWidth = $u;
        }

        /// The constant -1.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl NegativeOne for $s {
            const NEGATIVE_ONE: $s = -1;
        }
    };
}
apply_to_unsigned_signed_pair!(impl_basic_traits);
