use num::arithmetic::traits::{
    Abs, AbsAssign, CeilingDivAssignMod, CeilingDivMod, CeilingMod, CeilingModAssign,
    CeilingModPowerOf2, CeilingModPowerOf2Assign, CheckedAbs, ExtendedGcd, NegAssign,
    OverflowingAbs, OverflowingAbsAssign, SaturatingAbs, SaturatingAbsAssign, SaturatingNeg,
    SaturatingNegAssign, UnsignedAbs, WrappingAbs, WrappingAbsAssign,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::NegativeOne;
use num::logic::traits::CheckedHammingDistance;
use num::random::{HasRandomSignedRange, RandomSignedChunkable};
use std::ops::Neg;

/// Defines functions on primitive signed integer types: ixx and isize.
pub trait PrimitiveSigned:
    Abs<Output = Self>
    + AbsAssign
    + CeilingDivAssignMod<Self, ModOutput = Self>
    + CeilingDivMod<Self, DivOutput = Self, ModOutput = Self>
    + CeilingMod<Self, Output = Self>
    + CeilingModAssign<Self>
    + CeilingModPowerOf2<Output = Self>
    + CeilingModPowerOf2Assign
    + CheckedAbs<Output = Self>
    + CheckedHammingDistance
    + ExtendedGcd<Self, Cofactor = Self>
    + From<i8>
    + HasRandomSignedRange
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OverflowingAbs<Output = Self>
    + OverflowingAbsAssign
    + PrimitiveInt
    + RandomSignedChunkable
    + SaturatingAbs<Output = Self>
    + SaturatingAbsAssign
    + SaturatingNeg<Output = Self>
    + SaturatingNegAssign
    + UnsignedAbs
    + WrappingAbs<Output = Self>
    + WrappingAbsAssign
{
}

/// Defines basic trait implementations for signed types.
macro_rules! impl_basic_traits {
    ($s: ident) => {
        impl PrimitiveSigned for $s {}

        /// The constant -1.
        ///
        /// # Examples
        /// See [here](self).
        impl NegativeOne for $s {
            const NEGATIVE_ONE: $s = -1;
        }
    };
}
apply_to_signeds!(impl_basic_traits);
