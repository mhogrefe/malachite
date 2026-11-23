// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::modular::traits::DivisibleBy;

// The denominator of B_n, by the von Staudt–Clausen theorem with every prime found by trial
// division: the product of the primes p with p - 1 dividing n, for an even n >= 2.
pub fn bernoulli_number_denominator_naive(n: u64) -> Natural {
    if n == 0 {
        return Natural::ONE;
    }
    if n.odd() {
        return Natural::from(if n == 1 { 2u32 } else { 1 });
    }
    let mut den = Natural::ONE;
    for p in 2..=n + 1 {
        if (2..p)
            .take_while(|d| d * d <= p)
            .all(|d| !p.divisible_by(d))
            && n.divisible_by(p - 1)
        {
            den *= Natural::from(p);
        }
    }
    den
}
