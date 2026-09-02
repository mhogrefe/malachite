// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::ops::Deref;

/// Traits for arithmetic.
pub mod arithmetic;
/// Comparison of [`ComparableGaussianInteger`]s and [`ComparableGaussianIntegerRef`]s.
pub mod comparison;
/// Functions for converting a [`GaussianInteger`] to and from other types and strings.
pub mod conversion;
/// Iterators that generate [`GaussianInteger`]s without repetition.
pub mod exhaustive;
/// Traits for logic and bit manipulation.
pub mod logic;
#[cfg(feature = "random")]
/// Iterators that generate [`GaussianInteger`]s randomly.
pub mod random;

use malachite_base::named::Named;
use malachite_base::num::basic::traits::{I, NegativeI, NegativeOne, One, Two, Zero};

/// A Gaussian integer: a complex number whose real and imaginary parts are both integers.
///
/// The fields are public, since every combination of real and imaginary parts is a valid Gaussian
/// integer.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GaussianInteger {
    pub real: Integer,
    pub imaginary: Integer,
}

/// The constant 0.
impl Zero for GaussianInteger {
    const ZERO: Self = Self {
        real: Integer::ZERO,
        imaginary: Integer::ZERO,
    };
}

/// The constant 1.
impl One for GaussianInteger {
    const ONE: Self = Self {
        real: Integer::ONE,
        imaginary: Integer::ZERO,
    };
}

/// The constant 2.
impl Two for GaussianInteger {
    const TWO: Self = Self {
        real: Integer::TWO,
        imaginary: Integer::ZERO,
    };
}

/// The constant -1.
impl NegativeOne for GaussianInteger {
    const NEGATIVE_ONE: Self = Self {
        real: Integer::NEGATIVE_ONE,
        imaginary: Integer::ZERO,
    };
}

/// The constant i.
impl I for GaussianInteger {
    const I: Self = Self {
        real: Integer::ZERO,
        imaginary: Integer::ONE,
    };
}

/// The constant -i.
impl NegativeI for GaussianInteger {
    const NEGATIVE_I: Self = Self {
        real: Integer::ZERO,
        imaginary: Integer::NEGATIVE_ONE,
    };
}

// Implements `Named` for `GaussianInteger`.
impl_named!(GaussianInteger);

/// `ComparableGaussianInteger` is a wrapper around a [`GaussianInteger`], taking the
/// [`GaussianInteger`] by value.
///
/// The complex numbers have no total order compatible with their arithmetic, so [`GaussianInteger`]
/// does not implement [`Ord`]. Sometimes a canonical order is wanted anyway: for sorting a list of
/// Gaussian integers, or for using them as keys in a [`BTreeMap`](alloc::collections::BTreeMap) or
/// a [`BTreeSet`](alloc::collections::BTreeSet). Wrapping a [`GaussianInteger`] in a
/// `ComparableGaussianInteger` provides one: values are compared lexicographically, first by real
/// part and then by imaginary part. This order is total, and equality under it agrees with
/// [`GaussianInteger`] equality; it just isn't arithmetically meaningful.
///
/// The analogous wrapper for [`Float`](https://docs.rs/malachite-float/latest/malachite_float/)s is
/// `ComparableFloat`, although that wrapper also changes equality behavior, something that isn't
/// necessary here.
///
/// `ComparableGaussianInteger` owns its value. This is useful in many cases, for example if you
/// want to use [`GaussianInteger`]s as keys in a map. In other situations, it is better to use
/// [`ComparableGaussianIntegerRef`], which only has a reference to its value.
// Serialized as its inner `GaussianInteger`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ComparableGaussianInteger(pub GaussianInteger);

/// `ComparableGaussianIntegerRef` is a wrapper around a [`GaussianInteger`], taking the
/// [`GaussianInteger`] by reference.
///
/// See the [`ComparableGaussianInteger`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ComparableGaussianIntegerRef<'a>(pub &'a GaussianInteger);

impl ComparableGaussianInteger {
    /// Borrows a [`ComparableGaussianInteger`] as a [`ComparableGaussianIntegerRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_nz::gaussian_integer::{
    ///     ComparableGaussianInteger, ComparableGaussianIntegerRef, GaussianInteger,
    /// };
    ///
    /// let x = ComparableGaussianInteger(GaussianInteger::I);
    /// assert_eq!(
    ///     x.as_ref(),
    ///     ComparableGaussianIntegerRef(&GaussianInteger::I)
    /// );
    /// ```
    pub const fn as_ref(&self) -> ComparableGaussianIntegerRef<'_> {
        ComparableGaussianIntegerRef(&self.0)
    }
}

impl Deref for ComparableGaussianInteger {
    type Target = GaussianInteger;

    /// Allows a [`ComparableGaussianInteger`] to dereference to a [`GaussianInteger`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::gaussian_integer::{ComparableGaussianInteger, GaussianInteger};
    ///
    /// let x = ComparableGaussianInteger(GaussianInteger::ONE);
    /// assert_eq!(*x, GaussianInteger::ONE);
    /// ```
    fn deref(&self) -> &GaussianInteger {
        &self.0
    }
}

impl Deref for ComparableGaussianIntegerRef<'_> {
    type Target = GaussianInteger;

    /// Allows a [`ComparableGaussianIntegerRef`] to dereference to a [`GaussianInteger`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
    ///
    /// let x = GaussianInteger::ONE;
    /// let y = ComparableGaussianIntegerRef(&x);
    /// assert_eq!(*y, GaussianInteger::ONE);
    /// ```
    fn deref(&self) -> &GaussianInteger {
        self.0
    }
}
