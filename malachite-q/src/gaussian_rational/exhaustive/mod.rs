// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use crate::rational::exhaustive::{ExhaustiveNonzeroRationals, exhaustive_rationals};
use core::iter::{Chain, Map, Once};
use malachite_base::num::conversion::traits::ImaginaryFrom;
use malachite_base::tuples::exhaustive::{ExhaustivePairs1Input, exhaustive_pairs_from_single};

#[doc(hidden)]
pub type ExhaustiveRationals = Chain<Once<Rational>, ExhaustiveNonzeroRationals>;

#[doc(hidden)]
pub type ExhaustiveGaussianRationalsFromSingle =
    Map<ExhaustiveRationals, fn(Rational) -> GaussianRational>;

#[doc(hidden)]
pub type ExhaustiveGaussianRationalsFromPairs =
    Map<ExhaustivePairs1Input<ExhaustiveRationals>, fn((Rational, Rational)) -> GaussianRational>;

/// Generates all purely real [`GaussianRational`]s, in the order of [`exhaustive_rationals`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// Same as the complexity of [`exhaustive_rationals`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::gaussian_rational::exhaustive::exhaustive_real_gaussian_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_real_gaussian_rationals(), 10),
///     "[0, 1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_real_gaussian_rationals() -> ExhaustiveGaussianRationalsFromSingle {
    exhaustive_rationals().map(GaussianRational::from)
}

/// Generates all purely imaginary [`GaussianRational`]s, in the order of [`exhaustive_rationals`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// Same as the complexity of [`exhaustive_rationals`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::gaussian_rational::exhaustive::exhaustive_imaginary_gaussian_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_imaginary_gaussian_rationals(), 10),
///     "[0, i, -i, i/2, -i/2, 2i, -2i, i/3, -i/3, 3i/2, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_imaginary_gaussian_rationals() -> ExhaustiveGaussianRationalsFromSingle {
    exhaustive_rationals().map(GaussianRational::imaginary_from)
}

// The pair generators vary their second component fastest, so drawing the real part from the
// second component puts 1 before i in the exhaustive order.
pub(crate) fn gaussian_rational_from_pair(
    (imaginary, real): (Rational, Rational),
) -> GaussianRational {
    GaussianRational { real, imaginary }
}

/// Generates all [`GaussianRational`]s, the real and imaginary parts interleaved fairly by a bit
/// distributor.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// Same as the complexity of [`exhaustive_rationals`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::gaussian_rational::exhaustive::exhaustive_gaussian_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_gaussian_rationals(), 10),
///     "[0, 1, i, 1+i, -1, 1/2, -1+i, 1/2+i, -i, 1-i, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_gaussian_rationals() -> ExhaustiveGaussianRationalsFromPairs {
    exhaustive_pairs_from_single(exhaustive_rationals()).map(gaussian_rational_from_pair)
}
