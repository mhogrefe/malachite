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

// Generates all purely real Gaussian integers, in order of increasing absolute value of the real
// part.
#[inline]
pub fn exhaustive_real_gaussian_integers() -> ExhaustiveGaussianIntegersFromSingle {
    exhaustive_integers().map(GaussianInteger::from)
}

// Generates all purely imaginary Gaussian integers, in order of increasing absolute value of the
// imaginary part.
#[inline]
pub fn exhaustive_imaginary_gaussian_integers() -> ExhaustiveGaussianIntegersFromSingle {
    exhaustive_integers().map(GaussianInteger::imaginary_from)
}

pub(crate) fn gaussian_integer_from_pair((imaginary, real): (Integer, Integer)) -> GaussianInteger {
    GaussianInteger { real, imaginary }
}

// Generates all Gaussian integers.
#[inline]
pub fn exhaustive_gaussian_integers() -> ExhaustiveGaussianIntegersFromPairs {
    exhaustive_pairs_from_single(exhaustive_integers()).map(gaussian_integer_from_pair)
}
