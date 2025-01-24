// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    Abs, AbsAssign, CeilingDivAssignMod, CeilingDivMod, CeilingMod, CeilingModAssign,
    CeilingModPowerOf2, CeilingModPowerOf2Assign, CheckedAbs, ExtendedGcd, NegAssign,
    OverflowingAbs, OverflowingAbsAssign, SaturatingAbs, SaturatingAbsAssign, SaturatingNeg,
    SaturatingNegAssign, UnsignedAbs, WrappingAbs, WrappingAbsAssign,
};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::traits::NegativeOne;
use crate::num::logic::traits::CheckedHammingDistance;
#[cfg(feature = "random")]
use crate::num::random::{HasRandomSignedRange, RandomSignedChunkable};
use core::ops::Neg;

// When the `random` feature is enabled, the HasRandomSignedRange and RandomSignedChunkable bounds
// are included.

#[cfg(feature = "random")]
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

#[cfg(not(feature = "random"))]
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
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OverflowingAbs<Output = Self>
    + OverflowingAbsAssign
    + PrimitiveInt
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
