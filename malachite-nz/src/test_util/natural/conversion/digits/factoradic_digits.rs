// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AddMulAssign, DivAssignMod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::number_theory::traits::Factorial;

// Finds the largest factorial no greater than `x`, then peels off the digits from the top by
// dividing by each factorial in turn.
pub fn to_factoradic_digits_asc_naive(x: &Natural) -> Vec<u64> {
    let mut factorials = vec![Natural::ONE];
    let mut k = 1u64;
    loop {
        let f = Natural::factorial(k + 1);
        if f > *x {
            break;
        }
        factorials.push(f);
        k += 1;
    }
    let mut x = x.clone();
    let mut digits = Vec::new();
    for f in factorials.iter().rev() {
        let remainder = x.div_assign_mod(f);
        digits.push(u64::exact_from(&x));
        x = remainder;
    }
    digits.reverse();
    while digits.last() == Some(&0) {
        digits.pop();
    }
    digits
}

// Sums $d_i (i+1)!$ directly, returning `None` for an out-of-range digit.
pub fn from_factoradic_digits_asc_naive(digits: &[u64]) -> Option<Natural> {
    let mut x = Natural::ZERO;
    for (i, &digit) in digits.iter().enumerate() {
        let place = u64::exact_from(i) + 1;
        if digit > place {
            return None;
        }
        x.add_mul_assign(Natural::factorial(place), Natural::from(digit));
    }
    Some(x)
}
