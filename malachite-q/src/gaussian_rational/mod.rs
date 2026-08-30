// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{I, NegativeI, NegativeOne, One, OneHalf, Two, Zero};

/// Traits for arithmetic.
pub mod arithmetic;
/// Comparison of [`ComparableGaussianRational`]s and [`ComparableGaussianRationalRef`]s.
pub mod comparison;
/// Functions for converting a [`GaussianRational`] to and from other types and strings.
pub mod conversion;
/// Iterators that generate [`GaussianRational`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`GaussianRational`]s randomly.
pub mod random;

/// A Gaussian rational: a complex number whose real and imaginary parts are both rational.
///
/// The fields are public, since every combination of real and imaginary parts is a valid Gaussian
/// rational.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GaussianRational {
    pub real: Rational,
    pub imaginary: Rational,
}

/// The constant 0.
impl Zero for GaussianRational {
    const ZERO: Self = Self {
        real: Rational::ZERO,
        imaginary: Rational::ZERO,
    };
}

/// The constant 1.
impl One for GaussianRational {
    const ONE: Self = Self {
        real: Rational::ONE,
        imaginary: Rational::ZERO,
    };
}

/// The constant 2.
impl Two for GaussianRational {
    const TWO: Self = Self {
        real: Rational::TWO,
        imaginary: Rational::ZERO,
    };
}

/// The constant 1/2.
impl OneHalf for GaussianRational {
    const ONE_HALF: Self = Self {
        real: Rational::ONE_HALF,
        imaginary: Rational::ZERO,
    };
}

/// The constant -1.
impl NegativeOne for GaussianRational {
    const NEGATIVE_ONE: Self = Self {
        real: Rational::NEGATIVE_ONE,
        imaginary: Rational::ZERO,
    };
}

/// The constant i.
impl I for GaussianRational {
    const I: Self = Self {
        real: Rational::ZERO,
        imaginary: Rational::ONE,
    };
}

/// The constant -i.
impl NegativeI for GaussianRational {
    const NEGATIVE_I: Self = Self {
        real: Rational::ZERO,
        imaginary: Rational::NEGATIVE_ONE,
    };
}

// Implements `Named` for `GaussianRational`.
impl_named!(GaussianRational);

/// `ComparableGaussianRational` is a wrapper around a [`GaussianRational`], taking the
/// [`GaussianRational`] by value.
///
/// The complex numbers have no total order compatible with their arithmetic, so
/// [`GaussianRational`] does not implement [`Ord`]. Sometimes a canonical order is wanted anyway:
/// for sorting a list of Gaussian rationals, or for using them as keys in a
/// [`BTreeMap`](alloc::collections::BTreeMap) or a [`BTreeSet`](alloc::collections::BTreeSet).
/// Wrapping a [`GaussianRational`] in a `ComparableGaussianRational` provides one: values are
/// compared lexicographically, first by real part and then by imaginary part. This order is total,
/// and equality under it agrees with [`GaussianRational`] equality; it just isn't arithmetically
/// meaningful.
///
/// The analogous wrapper for [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger)s
/// is [`ComparableGaussianInteger`](malachite_nz::gaussian_integer::ComparableGaussianInteger).
///
/// `ComparableGaussianRational` owns its value. This is useful in many cases, for example if you
/// want to use [`GaussianRational`]s as keys in a map. In other situations, it is better to use
/// [`ComparableGaussianRationalRef`], which only has a reference to its value.
// Serialized as its inner `GaussianRational`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ComparableGaussianRational(pub GaussianRational);

/// `ComparableGaussianRationalRef` is a wrapper around a [`GaussianRational`], taking the
/// [`GaussianRational`] by reference.
///
/// See the [`ComparableGaussianRational`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ComparableGaussianRationalRef<'a>(pub &'a GaussianRational);

impl ComparableGaussianRational {
    /// Borrows a [`ComparableGaussianRational`] as a [`ComparableGaussianRationalRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_q::gaussian_rational::{
    ///     ComparableGaussianRational, ComparableGaussianRationalRef, GaussianRational,
    /// };
    ///
    /// let x = ComparableGaussianRational(GaussianRational::I);
    /// assert_eq!(
    ///     x.as_ref(),
    ///     ComparableGaussianRationalRef(&GaussianRational::I)
    /// );
    /// ```
    pub const fn as_ref(&self) -> ComparableGaussianRationalRef<'_> {
        ComparableGaussianRationalRef(&self.0)
    }
}

impl Deref for ComparableGaussianRational {
    type Target = GaussianRational;

    /// Allows a [`ComparableGaussianRational`] to dereference to a [`GaussianRational`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_q::gaussian_rational::{ComparableGaussianRational, GaussianRational};
    ///
    /// let x = ComparableGaussianRational(GaussianRational::ONE);
    /// assert_eq!(*x, GaussianRational::ONE);
    /// ```
    fn deref(&self) -> &GaussianRational {
        &self.0
    }
}

impl Deref for ComparableGaussianRationalRef<'_> {
    type Target = GaussianRational;

    /// Allows a [`ComparableGaussianRationalRef`] to dereference to a [`GaussianRational`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_q::gaussian_rational::{ComparableGaussianRationalRef, GaussianRational};
    ///
    /// let x = GaussianRational::ONE;
    /// let y = ComparableGaussianRationalRef(&x);
    /// assert_eq!(*y, GaussianRational::ONE);
    /// ```
    fn deref(&self) -> &GaussianRational {
        self.0
    }
}
