// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
//      Copyright © 2020 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::mem::{replace, swap};
use malachite_base::num::arithmetic::traits::{AddMulAssign, DivMod, FloorSqrtAssign, Gcd, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::NotAssign;
use malachite_nz::natural::Natural;

// This is _fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0, using the plain
// Euclidean ("gauss") loop at every size. The small-input kernels and the Lehmer and HGCD
// accelerations are not yet ported.
fn reconstruct_helper(
    a: Natural,
    m: Natural,
    n_bound: &Natural,
    d_bound: &Natural,
) -> Option<Rational> {
    assert!(a < m, "a must be reduced mod m");
    assert_ne!(*n_bound, 0u32, "n_bound must be positive");
    assert_ne!(*d_bound, 0u32, "d_bound must be positive");
    // Quickly identify small integers: n = a and n = a - m, with d = 1.
    if a <= *n_bound {
        return Some(Rational::from(a));
    }
    let diff = &m - &a;
    if diff <= *n_bound {
        return Some(-Rational::from(diff));
    }
    // A > B > N > 0. Accumulate quotients into the first row (m11, m12) of the matrix M until A > N
    // >= B. The sign of det M is tracked separately, and the second row is not needed.
    let mut big_a = m;
    let mut big_b = a;
    let mut m11 = Natural::ONE;
    let mut m12 = Natural::ZERO;
    let mut mdet_pos = true;
    loop {
        let (q, r) = big_a.div_mod(&big_b);
        m12.add_mul_assign(&m11, q);
        swap(&mut m11, &mut m12);
        mdet_pos.not_assign();
        big_a = replace(&mut big_b, r);
        if big_b <= *n_bound {
            break;
        }
    }
    // The candidate is n = ±B and d = m11, with n's sign that of det M.
    if m11 > *d_bound || (&big_b).gcd(&m11) != 1u32 {
        return None;
    }
    // The zero-numerator guard keeps the result canonical if B = 0 ever reaches this point. It is
    // believed unreachable: B = 0 requires m11 = 1 to pass the gcd check, which would mean the loop
    // ran exactly one iteration with quotient 1, and then its remainder m - a would have been at
    // most N, already handled by the second fast path. An instrumented run observed zero hits. It
    // is retained as canonicality insurance, mirroring FLINT's own B == 0 handling.
    Some(Rational {
        sign: mdet_pos || big_b == 0u32,
        numerator: big_b,
        denominator: m11,
    })
}

impl Rational {
    // This is fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0.
    #[inline]
    pub fn reconstruct_with_bounds(
        a: Natural,
        m: Natural,
        n_bound: &Natural,
        d_bound: &Natural,
    ) -> Option<Self> {
        reconstruct_helper(a, m, n_bound, d_bound)
    }

    // This is fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0, where all inputs
    // are taken by reference.
    #[inline]
    pub fn reconstruct_with_bounds_ref(
        a: &Natural,
        m: &Natural,
        n_bound: &Natural,
        d_bound: &Natural,
    ) -> Option<Self> {
        reconstruct_helper(a.clone(), m.clone(), n_bound, d_bound)
    }

    // This is fmpq_reconstruct_fmpz from fmpq/reconstruct_fmpz.c, FLINT 3.6.0.
    pub fn reconstruct(a: Natural, m: Natural) -> Option<Self> {
        assert!(m > 2u32, "m must be greater than 2");
        // The balanced bounds N = D = floor(sqrt((m - 1) / 2)).
        let mut b = &m >> 1u32;
        if m.even() {
            b -= Natural::ONE;
        }
        b.floor_sqrt_assign();
        reconstruct_helper(a, m, &b, &b)
    }

    // This is fmpq_reconstruct_fmpz from fmpq/reconstruct_fmpz.c, FLINT 3.6.0, where all inputs are
    // taken by reference.
    pub fn reconstruct_ref(a: &Natural, m: &Natural) -> Option<Self> {
        assert!(*m > 2u32, "m must be greater than 2");
        let mut b = m >> 1u32;
        if m.even() {
            b -= Natural::ONE;
        }
        b.floor_sqrt_assign();
        reconstruct_helper(a.clone(), m.clone(), &b, &b)
    }
}
