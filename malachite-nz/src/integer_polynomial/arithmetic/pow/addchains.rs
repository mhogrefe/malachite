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
use crate::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use crate::integer_polynomial::arithmetic::square::square_to_out;
use alloc::vec;
use malachite_base::num::arithmetic::traits::FloorLogBase2;
use malachite_base::num::conversion::traits::ExactFrom;

// A tree with shortest addition chains (star chains, in fact) for all integers up to and including
// 148. The entry at 0 is present to provide 1-based indexing. The integer 1 is the root of the
// tree, and the entry at 1 is irrelevant. For integers i >= 2, the entry at i is the parent of i.
//
// This is `shortest_addchains_148` from `fmpz_poly/pow_addchains.c`, FLINT 3.6.0.
const SHORTEST_ADDCHAINS_148: [u8; 149] = [
    0, 0, 1, 2, 2, 3, 3, 5, 4, 8, 5, 10, 6, 9, 7, 12, 8, 9, 16, 18, 10, 15, 11, 20, 12, 17, 13, 24,
    14, 25, 15, 28, 16, 32, 17, 26, 18, 36, 19, 27, 20, 40, 21, 34, 22, 30, 23, 46, 24, 33, 25, 48,
    26, 37, 27, 54, 28, 49, 29, 56, 30, 52, 31, 51, 32, 64, 33, 66, 34, 68, 35, 70, 36, 72, 66, 60,
    38, 43, 39, 78, 40, 65, 41, 80, 42, 80, 43, 86, 44, 88, 45, 90, 46, 92, 47, 92, 48, 96, 49, 96,
    50, 100, 51, 102, 52, 102, 53, 74, 54, 108, 55, 108, 56, 104, 57, 112, 58, 104, 59, 112, 60,
    120, 61, 120, 62, 100, 63, 126, 64, 128, 65, 130, 128, 132, 67, 90, 68, 136, 69, 138, 70, 140,
    71, 117, 72, 144, 73, 99, 74,
];

// Returns the addition chain $1 = a_0 < a_1 < \cdots < a_n = e$ for `e`, which must be at least 2
// and at most 148, in the last $n + 1$ elements of an array, along with the index of $a_0$. No
// chain has more than 11 terms.
//
// This is the chain-copying loop of `fmpz_poly_pow_addchains` from `fmpz_poly/pow_addchains.c`,
// FLINT 3.6.0.
crate_test_fn! {addition_chain(e: u64) -> ([usize; 11], usize) {
    assert!((2..=148).contains(&e));
    let mut a = [0; 11];
    let mut i = 10;
    let mut n = usize::exact_from(e);
    a[i] = n;
    loop {
        n = usize::from(SHORTEST_ADDCHAINS_148[n]);
        if n == 0 {
            break;
        }
        i -= 1;
        a[i] = n;
    }
    (a, i)
}}

// Whether the addition chain for `e`, which must be at least 2 and at most 148, takes fewer steps
// than binary exponentiation, which squares once for each bit of `e` below the most significant and
// multiplies once for each set bit below it.
pub(crate) fn addition_chain_is_shorter(e: u64) -> bool {
    let start = addition_chain(e).1;
    u64::exact_from(10 - start) < e.floor_log_base_2() + u64::from(e.count_ones()) - 1
}

// Sets `out` to the coefficients of the $a_n$th power of the polynomial with coefficients `xs`,
// which has length at least 2, where $1 = a_0 < a_1 < \cdots < a_n$ is the addition chain `a`, with
// $n$ at least 2. `out` must have length $a_n($`xs.len()`$ - 1) + 1$.
//
// The powers $f^{a_1}, \ldots, f^{a_{n-1}}$ are stored one after another in a scratch buffer, the
// one for $f^{a_k}$ starting at $($`xs.len()`$ - 1)b_{k-1} + k - 1$, where $b_k$ is $a_1 + \cdots +
// a_k$.
//
// This is equivalent to `_fmpz_poly_pow_addchains` from `fmpz_poly/pow_addchains.c`, FLINT 3.6.0,
// with a fix. FLINT reads $f^{a_i}$ from $($`len`$ - 1)b_{i-1}$, omitting the $i - 1$, so its
// result is wrong whenever the chain has more than three terms, as for every exponent from 5 on.
// FLINT's tests miss this because the loop comparing against `fmpz_poly_pow` overwrites its input
// with each power, so after the zeroth power every comparison is between constants.
crate_test_fn! {pow_to_out_addchains<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    a: &[usize],
) {
    let n = a.len() - 1;
    let lenm1 = xs.len() - 1;
    // Compute partial sums
    let mut b = vec![0; n];
    for i in 1..n {
        b[i] = b[i - 1] + a[i];
    }
    // Allocate memory for the polynomials f^{a[1]}, ..., f^{a[n-1]}
    let lenv = lenm1 * b[n - 1] + n - 1;
    let mut v = vec![C::ZERO; lenv];
    // Compute f^{a[1]}, ..., f^{a[n-1]}
    square_to_out(&mut v[..(xs.len() << 1) - 1], xs);
    for i in 1..n {
        let d = a[i + 1] - a[i];
        let x_start = lenm1 * b[i - 1] + i - 1;
        let x_end = x_start + lenm1 * a[i] + 1;
        let out_len = lenm1 * a[i + 1] + 1;
        // The final product, i == n - 1, is stored in out
        let (lo, hi) = v.split_at_mut(x_end);
        let (lo, target) = if i == n - 1 {
            (&*lo, &mut out[..out_len])
        } else {
            (&*lo, &mut hi[..out_len])
        };
        let x = &lo[x_start..];
        if d == 1 {
            mul_greater_to_out(target, x, xs);
        } else {
            let mut j = i;
            while a[j] != d {
                j -= 1;
            }
            let y_start = lenm1 * b[j - 1] + j - 1;
            mul_greater_to_out(target, x, &lo[y_start..=y_start + lenm1 * a[j]]);
        }
    }
}}
