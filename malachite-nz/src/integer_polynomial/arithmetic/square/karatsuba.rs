// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul::karatsuba::{
    add_shifted_rev, revbin_in, revbin_out,
};
use crate::integer_polynomial::arithmetic::vec::{vec_add, vec_sub_assign};
use alloc::vec;
use core::borrow::Borrow;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, PowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;

// Karatsuba squaring of a polynomial in bit-reversed order: `xs` has length $2^b$, `out` has length
// $2^{b+1}$, and `temp` has length at least $2^{b+1}$. See `mul/karatsuba.rs`.
//
// This is equivalent to `_fmpz_poly_sqr_kara_recursive` from `fmpz_poly/sqr_karatsuba.c`, FLINT
// 3.6.0.
fn square_karatsuba_recursive<C: PolynomialCoefficient, T: Borrow<C>>(
    out: &mut [C],
    xs: &[T],
    temp: &mut [C],
    bits: u64,
) {
    let length = usize::power_of_2(bits);
    let m = length >> 1;
    if length == 1 {
        out[0] = xs[0].borrow().square_ref();
        out[1] = C::ZERO;
        return;
    }
    let (sums, temp) = temp.split_at_mut(length);
    vec_add(&mut sums[..m], &xs[..m], &xs[m..length]);
    let (out_lo, out_hi) = out.split_at_mut(length);
    square_karatsuba_recursive(out_lo, &xs[..m], temp, bits - 1);
    square_karatsuba_recursive(out_hi, &sums[..m], temp, bits - 1);
    square_karatsuba_recursive(sums, &xs[m..], temp, bits - 1);
    vec_sub_assign(&mut out_hi[..length], out_lo);
    vec_sub_assign(&mut out_hi[..length], sums);
    add_shifted_rev(out_lo, sums, bits);
}

// Sets `out` to the coefficients of the square of the polynomial with coefficients `xs`, which is
// nonempty. `out` must have length `2 * xs.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the largest number of
// significant bits of any element of `xs`.
//
// This is equivalent to `_fmpz_poly_sqr_karatsuba` from `fmpz_poly/sqr_karatsuba.c`, FLINT 3.6.0.
crate_test_fn! {square_to_out_karatsuba<C: PolynomialCoefficient>(out: &mut [C], xs: &[C]) {
    let len = xs.len();
    assert_ne!(len, 0);
    if len == 1 {
        out[0] = xs[0].square_ref();
        return;
    }
    let loglen = u64::exact_from(len).ceiling_log_base_2();
    let length = usize::power_of_2(loglen);
    let zero = C::ZERO;
    let mut rev = vec![&zero; length];
    let mut scratch = vec![C::ZERO; length << 2];
    let (rev_out, temp) = scratch.split_at_mut(length << 1);
    revbin_in(&mut rev, xs, loglen);
    square_karatsuba_recursive(rev_out, &rev, temp, loglen);
    revbin_out(out, rev_out, loglen + 1);
}}
