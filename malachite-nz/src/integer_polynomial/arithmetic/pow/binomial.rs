// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, which
// has length 2, where `e` is at least 3. `out` must have length `e + 1`.
//
// With `xs` equal to $[a, b]$, the coefficient of $x^i$ is $\binom{e}{i} a^{e - i} b^i$. The
// binomial coefficients are built up from both ends at once, and the powers of $a$ and $b$ are
// multiplied in as they grow.
//
// This is equivalent to `_fmpz_poly_pow_binomial` from `fmpz_poly/pow_binomial.c`, FLINT 3.6.0,
// where `f` is always `e - i`.
crate_test_fn! {pow_to_out_binomial<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], e: u64) {
    let mut a = C::ONE;
    let mut b = C::ONE;
    let mut c = C::ONE;
    out[0] = C::ONE;
    out[usize::exact_from(e)] = C::ONE;
    let mut i = 1;
    while i <= (e - 1) >> 1 {
        let f = e - i;
        a *= &xs[0];
        b *= &xs[1];
        c *= &C::from(f + 1);
        c.div_exact_assign(&C::from(i));
        out[usize::exact_from(i)] = b.mul_ref(&c);
        out[usize::exact_from(f)] = a.mul_ref(&c);
        i += 1;
    }
    if e.even() {
        let f = e - i;
        a *= &xs[0];
        b *= &xs[1];
        c *= &C::from(f + 1);
        c.div_exact_assign(&C::from(i));
        let middle = usize::exact_from(i);
        out[middle] = b.mul_ref(&c);
        out[middle] *= &a;
        i += 1;
    }
    while i <= e {
        let f = e - i;
        a *= &xs[0];
        b *= &xs[1];
        out[usize::exact_from(i)] *= &b;
        out[usize::exact_from(f)] *= &a;
        i += 1;
    }
}}
