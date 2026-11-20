// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use malachite_base::num::basic::traits::{One, OneHalf, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::tangent_numbers::tangent_numbers_prefix;

// B_2, B_4, ..., B_{2 len}, from the tangent numbers: B_{2j} = (-1)^(j - 1) 2j T_j / (4^(2j) -
// 2^(2j)), reduced. This is an algorithm independent of the ones of `bernoulli_numbers_prefix`.
//
// This is equivalent to `bernoulliEvens` and `bernoulliOfTangent` from `AzRat/Bernoulli.lean`,
// Azurite.
pub fn even_bernoulli_numbers_prefix_tangent(len: u64) -> Vec<Rational> {
    tangent_numbers_prefix(len)
        .into_iter()
        .zip(1u64..)
        .map(|(t, j)| {
            let numerator = Integer::from(t * Natural::from(j << 1));
            let denominator =
                Integer::from(Natural::power_of_2(j << 2) - Natural::power_of_2(j << 1));
            let b = Rational::from_integers(numerator, denominator);
            if j.even() { -b } else { b }
        })
        .collect()
}

// B_n from the tangent numbers.
//
// This is equivalent to `bernoulli` from `AzRat/Bernoulli.lean`, Azurite.
pub fn bernoulli_number_tangent(n: u64) -> Rational {
    match n {
        0 => Rational::ONE,
        1 => -Rational::ONE_HALF,
        _ if n.odd() => Rational::ZERO,
        _ => even_bernoulli_numbers_prefix_tangent(n >> 1).pop().unwrap(),
    }
}

// B_0, ..., B_{len - 1}, from the tangent numbers.
pub fn bernoulli_numbers_prefix_tangent(len: u64) -> Vec<Rational> {
    let mut evens = even_bernoulli_numbers_prefix_tangent(len.saturating_sub(1) >> 1).into_iter();
    (0..len)
        .map(|n| match n {
            0 => Rational::ONE,
            1 => -Rational::ONE_HALF,
            _ if n.odd() => Rational::ZERO,
            _ => evens.next().unwrap(),
        })
        .collect()
}
