// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::basic::traits::{One, Two, Zero};

pub fn fibonacci_naive(n: u64) -> Natural {
    let (mut a, mut b) = (Natural::ZERO, Natural::ONE);
    for _ in 0..n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    a
}

pub fn fibonacci_pair_naive(n: u64) -> (Natural, Natural) {
    if n == 0 {
        // F(-1) = 1
        return (Natural::ZERO, Natural::ONE);
    }
    let (mut a, mut b) = (Natural::ZERO, Natural::ONE);
    for _ in 1..n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    (b, a)
}

pub fn lucas_number_naive(n: u64) -> Natural {
    let (mut a, mut b) = (Natural::TWO, Natural::ONE);
    for _ in 0..n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    a
}

// The Lucas number pair (L(n), L(n - 1)) is not defined for n == 0, since L(-1) = -1.
pub fn lucas_number_pair_naive(n: u64) -> (Natural, Natural) {
    assert_ne!(n, 0);
    let (mut a, mut b) = (Natural::TWO, Natural::ONE);
    for _ in 1..n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    (b, a)
}
