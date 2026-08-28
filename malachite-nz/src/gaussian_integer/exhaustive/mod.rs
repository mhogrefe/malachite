// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use crate::integer::exhaustive::{IntegerUpDown, exhaustive_integers};
use core::iter::{Chain, Map, Once};
use malachite_base::num::conversion::traits::ImaginaryFrom;
use malachite_base::tuples::exhaustive::{ExhaustivePairs1Input, exhaustive_pairs_from_single};

#[doc(hidden)]
pub type ExhaustiveIntegers = Chain<Once<Integer>, IntegerUpDown>;

#[doc(hidden)]
pub type ExhaustiveGaussianIntegersFromSingle =
    Map<ExhaustiveIntegers, fn(Integer) -> GaussianInteger>;

#[doc(hidden)]
pub type ExhaustiveGaussianIntegersFromPairs =
    Map<ExhaustivePairs1Input<ExhaustiveIntegers>, fn((Integer, Integer)) -> GaussianInteger>;

/// Generates all purely real [`GaussianInteger`]s, in order of increasing absolute value of the
/// real part. When two real parts have the same absolute value, the positive one comes first.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $\ell$ is the
/// number of significant bits of the parts of the $i$th output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::gaussian_integer::exhaustive::exhaustive_real_gaussian_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_real_gaussian_integers(), 10),
///     "[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_real_gaussian_integers() -> ExhaustiveGaussianIntegersFromSingle {
    exhaustive_integers().map(GaussianInteger::from)
}

/// Generates all purely imaginary [`GaussianInteger`]s, in order of increasing absolute value of
/// the imaginary part. When two imaginary parts have the same absolute value, the positive one
/// comes first.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $\ell$ is the
/// number of significant bits of the parts of the $i$th output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::gaussian_integer::exhaustive::exhaustive_imaginary_gaussian_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_imaginary_gaussian_integers(), 10),
///     "[0, i, -i, 2i, -2i, 3i, -3i, 4i, -4i, 5i, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_imaginary_gaussian_integers() -> ExhaustiveGaussianIntegersFromSingle {
    exhaustive_integers().map(GaussianInteger::imaginary_from)
}

pub(crate) fn gaussian_integer_from_pair((imaginary, real): (Integer, Integer)) -> GaussianInteger {
    GaussianInteger { real, imaginary }
}

/// Generates all [`GaussianInteger`]s. The real and imaginary parts are generated roughly in order
/// of increasing absolute value, interleaved fairly by a bit distributor.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $\ell$ is the
/// number of significant bits of the parts of the $i$th output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::gaussian_integer::exhaustive::exhaustive_gaussian_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_gaussian_integers(), 10),
///     "[0, 1, i, 1+i, -1, 2, -1+i, 2+i, -i, 1-i, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_gaussian_integers() -> ExhaustiveGaussianIntegersFromPairs {
    exhaustive_pairs_from_single(exhaustive_integers()).map(gaussian_integer_from_pair)
}
