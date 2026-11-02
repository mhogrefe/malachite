// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Abs, Floor, UnsignedAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::number_theory::traits::Factorial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::conversion::digits::factoradic_digits::*;

// Expands the fractional part with `Rational` arithmetic: multiply by $k$, take the floor as the
// digit, and keep the rest.
pub fn factoradic_digits_naive(x: &Rational) -> (Vec<u64>, Vec<u64>) {
    let x = x.abs();
    let floor = (&x).floor().unsigned_abs();
    let mut remainder = x - Rational::from(&floor);
    let mut after_point = Vec::new();
    let mut k = 2u64;
    while remainder != 0u32 {
        remainder *= Rational::from(k);
        let digit = (&remainder).floor().unsigned_abs();
        remainder -= Rational::from(&digit);
        after_point.push(u64::exact_from(&digit));
        k += 1;
    }
    (to_factoradic_digits_asc_naive(&floor), after_point)
}

// Sums $e_j / (j+2)!$ directly, returning `None` for an out-of-range digit.
pub fn from_factoradic_digits_naive(before_point: &[u64], after_point: &[u64]) -> Option<Rational> {
    let mut x = Rational::from(from_factoradic_digits_asc_naive(before_point)?);
    for (j, &digit) in after_point.iter().enumerate() {
        let k = u64::exact_from(j) + 2;
        if digit >= k {
            return None;
        }
        x += Rational::from_naturals(Natural::from(digit), Natural::factorial(k));
    }
    Some(x)
}
