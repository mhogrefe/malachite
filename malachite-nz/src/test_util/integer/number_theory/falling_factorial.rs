// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.

use crate::integer::Integer;
use malachite_base::num::basic::traits::One;

// The product x(x - 1)...(x - n + 1), one factor at a time.
pub fn falling_factorial_naive(x: &Integer, n: u64) -> Integer {
    let mut f = Integer::ONE;
    let mut factor = x.clone();
    for _ in 0..n {
        f *= &factor;
        factor -= Integer::ONE;
    }
    f
}
