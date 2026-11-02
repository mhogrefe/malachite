// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.

use crate::natural::Natural;
use malachite_base::num::basic::traits::{One, Zero};

// The product x(x - 1)...(x - n + 1), one factor at a time.
pub fn falling_factorial_naive(x: &Natural, n: u64) -> Natural {
    let mut f = Natural::ONE;
    let mut factor = x.clone();
    for _ in 0..n {
        if factor == 0u32 {
            return Natural::ZERO;
        }
        f *= &factor;
        factor -= Natural::ONE;
    }
    f
}
