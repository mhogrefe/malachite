// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;

// Finds the largest factorial no greater than `x`, then peels off the digits from the top by
// dividing by each factorial in turn, all in `u128`.
pub fn to_factoradic_digits_asc_naive<T: PrimitiveUnsigned>(x: T) -> Vec<u64>
where
    u128: ExactFrom<T>,
{
    let mut x = u128::exact_from(x);
    let mut factorials = vec![1u128];
    let mut k = 1u128;
    while let Some(f) = factorials.last().unwrap().checked_mul(k + 1) {
        if f > x {
            break;
        }
        factorials.push(f);
        k += 1;
    }
    let mut digits = Vec::new();
    for f in factorials.iter().rev() {
        digits.push(u64::exact_from(x / f));
        x %= f;
    }
    // `factorials[0]` is 1!, so the digits are the coefficients of 1!, 2!, ... from the top
    digits.reverse();
    while digits.last() == Some(&0) {
        digits.pop();
    }
    digits
}

// Sums $d_i (i+1)!$ directly, in `u128`, returning `None` for an out-of-range digit or a value that
// doesn't fit in `T`.
pub fn from_factoradic_digits_asc_naive<T: PrimitiveUnsigned>(digits: &[u64]) -> Option<T> {
    let mut x = 0u128;
    let mut factorial = Some(1u128);
    for (i, &digit) in digits.iter().enumerate() {
        let place = u64::exact_from(i) + 1;
        if digit > place {
            return None;
        }
        factorial = factorial.and_then(|f| f.checked_mul(u128::from(place)));
        if digit != 0 {
            x = x.checked_add(factorial?.checked_mul(u128::from(digit))?)?;
        }
    }
    T::try_from(x).ok()
}
