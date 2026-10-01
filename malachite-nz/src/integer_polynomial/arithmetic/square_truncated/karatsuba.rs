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
use crate::integer_polynomial::arithmetic::mul_truncated::karatsuba::{
    combine_truncated_karatsuba, padded,
};
use crate::integer_polynomial::arithmetic::square::karatsuba::square_to_out_karatsuba;
use crate::integer_polynomial::arithmetic::square_truncated::classical::*;
use crate::integer_polynomial::arithmetic::vec::vec_add;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Parity, PowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;

// Squaring using truncated Karatsuba; see `mul_truncated/karatsuba.rs`.
//
// This is equivalent to `_fmpz_poly_sqrlow_kara_recursive` from `fmpz_poly/sqrlow_karatsuba_n.c`,
// FLINT 3.6.0.
fn square_truncated_karatsuba_recursive<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    temp: &mut [C],
    len: usize,
) {
    let m1 = len >> 1;
    let m2 = len - m1;
    let odd = len.odd();
    if len <= 6 {
        square_truncated_to_out_classical(&mut out[..len], &xs[..len]);
        return;
    }
    let two_m1 = m1 << 1;
    vec_add(&mut temp[m2..m2 + m1], &xs[..m1], &xs[m1..two_m1]);
    if odd {
        temp[m2 + m1] = xs[two_m1].clone();
    }
    {
        let (low, rest) = temp.split_at_mut(m2);
        let (sums, rest) = rest.split_at_mut(m2);
        square_truncated_karatsuba_recursive(low, sums, rest, m2);
    }
    {
        let (high, rest) = temp[m2..].split_at_mut(m2);
        square_truncated_karatsuba_recursive(high, &xs[m1..], rest, m2);
    }
    square_to_out_karatsuba(&mut out[..two_m1 - 1], &xs[..m1]);
    out[two_m1 - 1] = C::ZERO;
    combine_truncated_karatsuba(out, temp, m1, m2);
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which has length at least `out.len()`, which must be positive.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs`.
//
// This is equivalent to `_fmpz_poly_sqrlow_karatsuba_n` from `fmpz_poly/sqrlow_karatsuba_n.c`,
// FLINT 3.6.0, where `n` is `out.len()`.
crate_test_fn! {square_truncated_to_out_karatsuba_n<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
) {
    let n = out.len();
    assert_ne!(n, 0);
    assert!(xs.len() >= n);
    if n == 1 {
        out[0] = xs[0].square_ref();
        return;
    }
    // The temporary space is almost 2 * len, but the recursion may need 4 * ceil(len / 2), which
    // exceeds 2 * len by at most 2.
    let len = usize::power_of_2(u64::exact_from(n).ceiling_log_base_2());
    let mut temp = vec![C::ZERO; (len << 1) + 2];
    square_truncated_karatsuba_recursive(out, xs, &mut temp, n);
}}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty. `out.len()` must be positive and at most `2 * xs.len() -
// 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs`.
//
// This is equivalent to `_fmpz_poly_sqrlow_karatsuba` from `fmpz_poly/sqrlow_karatsuba_n.c`, FLINT
// 3.6.0, where `n` is `out.len()`.
crate_test_fn! {square_truncated_to_out_karatsuba<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
) {
    let n = out.len();
    square_truncated_to_out_karatsuba_n(out, &padded(xs, n));
}}
