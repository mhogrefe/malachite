// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul::karatsuba::mul_to_out_karatsuba;
use crate::integer_polynomial::arithmetic::mul_high::classical::mul_high_to_out_classical;
use crate::integer_vector::arithmetic::add::{vec_add, vec_add_assign};
use crate::integer_vector::arithmetic::sub::vec_sub_assign;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Parity, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

// Multiplication using truncated Karatsuba. Below length 7, classical truncated multiplication is
// always theoretically faster, so it is the basecase. Above that, the ordinary (left/right)
// Karatsuba identity is used, with one full Karatsuba multiplication and two truncated Karatsuba
// multiplications, done recursively.
//
// This is equivalent to `_fmpz_poly_mulhigh_kara_recursive` from `fmpz_poly/mulhigh_karatsuba_n.c`,
// FLINT 3.6.0.
fn mul_high_karatsuba_recursive(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    temp: &mut [Integer],
    length: usize,
) {
    let m1 = length >> 1;
    let m2 = length - m1;
    let odd = length.odd();
    if length <= 6 {
        mul_high_to_out_classical(
            &mut out[..(length << 1) - 1],
            &xs[..length],
            &ys[..length],
            length - 1,
        );
        return;
    }
    let two_m1 = m1 << 1;
    let two_m2 = m2 << 1;
    vec_add(&mut out[..m1], &xs[..m1], &xs[m1..two_m1]);
    if odd {
        out[m1] = xs[two_m1].clone();
    }
    vec_add(&mut out[m2..m2 + m1], &ys[..m1], &ys[m1..two_m1]);
    if odd {
        out[m2 + m1] = ys[two_m1].clone();
    }
    {
        let (high, rest) = temp.split_at_mut(two_m2);
        mul_high_karatsuba_recursive(high, &out[..m2], &out[m2..two_m2], rest, m2);
    }
    mul_to_out_karatsuba(
        &mut out[two_m1..two_m1 + two_m2 - 1],
        &xs[m1..m1 + m2],
        &ys[m1..m1 + m2],
    );
    out[two_m1 - 1] = Integer::ZERO;
    mul_high_karatsuba_recursive(&mut out[..two_m1 - 1], xs, ys, &mut temp[two_m2..], m1);
    vec_sub_assign(&mut temp[m2 - 1..two_m1 - 1], &out[m2 - 1..two_m1 - 1]);
    vec_sub_assign(
        &mut temp[m2 - 1..two_m2 - 1],
        &out[two_m1 + m2 - 1..two_m1 + two_m2 - 1],
    );
    vec_add_assign(
        &mut out[length - 1..length - 1 + m2],
        &temp[m2 - 1..two_m2 - 1],
    );
    out[..length - 1].fill(Integer::ZERO);
}

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// which have the same nonzero length `len`, except that the coefficients of $x^i$ for $i <$ `len -
// 1` are set to zero rather than computed. `out` must have length `2 * len - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the largest number of
// significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mulhigh_karatsuba_n` from `fmpz_poly/mulhigh_karatsuba_n.c`,
// FLINT 3.6.0.
crate_test_fn! {mul_high_to_out_karatsuba_n(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    let len = xs.len();
    assert_ne!(len, 0);
    assert_eq!(ys.len(), len);
    assert_eq!(out.len(), (len << 1) - 1);
    if len == 1 {
        out[0] = &xs[0] * &ys[0];
        return;
    }
    let length = usize::power_of_2(u64::exact_from(len).ceiling_log_base_2());
    let mut temp = vec![Integer::ZERO; length << 1];
    mul_high_karatsuba_recursive(out, xs, ys, &mut temp, len);
}}
