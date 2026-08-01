// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    Abs, AbsAssign, BalancedMod, BalancedModAssign, CeilingDivAssignMod, CeilingDivMod, CeilingMod,
    CeilingModAssign, CeilingModPowerOf2, CeilingModPowerOf2Assign, CheckedAbs, ExtendedGcd,
    NegAssign, OverflowingAbs, OverflowingAbsAssign, SaturatingAbs, SaturatingAbsAssign,
    SaturatingNeg, SaturatingNegAssign, UnsignedAbs, WrappingAbs, WrappingAbsAssign,
};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::traits::NegativeOne;
use crate::num::comparison::traits::{EqAbs, OrdAbs, OrdAbsDouble, PartialOrdAbs};
use crate::num::logic::traits::CheckedHammingDistance;
#[cfg(feature = "random")]
use crate::num::random::{HasRandomSignedRange, RandomSignedChunkable};
use core::ops::Neg;

/// The bounds that [`PrimitiveSigned`] has only when the `random` feature is enabled.
///
/// With the feature on these are [`HasRandomSignedRange`] and [`RandomSignedChunkable`]; with it
/// off there are none. Every type meeting the bounds implements it automatically, so it never needs
/// to be implemented by hand.
///
/// See [`PrimitiveIntRandomBounds`](crate::num::basic::integers::PrimitiveIntRandomBounds) for why
/// this indirection exists.
#[cfg(feature = "random")]
pub trait PrimitiveSignedRandomBounds: HasRandomSignedRange + RandomSignedChunkable {}

#[cfg(feature = "random")]
impl<T: HasRandomSignedRange + RandomSignedChunkable> PrimitiveSignedRandomBounds for T {}

/// The bounds that [`PrimitiveSigned`] has only when the `random` feature is enabled.
///
/// With the feature off, as here, there are none.
#[cfg(not(feature = "random"))]
pub trait PrimitiveSignedRandomBounds {}

#[cfg(not(feature = "random"))]
impl<T> PrimitiveSignedRandomBounds for T {}

/// Defines functions on primitive signed integer types: ixx and isize.
pub trait PrimitiveSigned:
    Abs<Output = Self>
    + AbsAssign
    + BalancedMod<Self, Output = Self>
    + BalancedModAssign<Self>
    + CeilingDivAssignMod<Self, ModOutput = Self>
    + CeilingDivMod<Self, DivOutput = Self, ModOutput = Self>
    + CeilingMod<Self, Output = Self>
    + CeilingModAssign<Self>
    + CeilingModPowerOf2<Output = Self>
    + CeilingModPowerOf2Assign
    + CheckedAbs<Output = Self>
    + CheckedHammingDistance
    + EqAbs<Self>
    + ExtendedGcd<Self, Cofactor = Self>
    + From<i8>
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OrdAbs
    + OrdAbsDouble<Self>
    + OverflowingAbs<Output = Self>
    + OverflowingAbsAssign
    + PartialOrdAbs<Self>
    + PrimitiveInt
    + PrimitiveSignedRandomBounds
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
