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
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, which
// has length at least 2, a nonzero first element, and a nonzero last element, where `e` is at least
// 3. `out` must have length `e * (xs.len() - 1) + 1`.
//
// This uses J. C. P. Miller's recurrence. With $q$ the polynomial, $d$ its degree, and $r = q^e$,
// $$
// k q_0 r_k = \sum_{i=1}^{\min(k, d)} (i(e + 1) - k) q_i r_{k - i}.
// $$
// The sum is formed in one of two ways. When the polynomial has at least 8 coefficients and every
// multiple $i(e + 1) q_i$ fits in a limb, the multiples are computed once and the sum is taken as
// $\sum i(e + 1) q_i r_{k - i} - k \sum q_i r_{k - i}$, each term a single fused multiply-add with
// no temporary. Otherwise each product $q_i r_{k - i}$ is formed once and added, scaled by $|i(e +
// 1) - k|$, into one of two sums according to the sign of $i(e + 1) - k$.
//
// This is equivalent to `_fmpz_poly_pow_multinomial` from `fmpz_poly/pow_multinomial.c`, FLINT
// 3.6.0, except for the two forms of the sum, and except that FLINT also accepts zero low
// coefficients, which it strips itself. FLINT adds or subtracts each scaled product into a single
// sum, which can pass through negative values; both forms here also work for `Natural`
// coefficients, where every partial sum is non-negative.
crate_test_fn! {pow_to_out_multinomial<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], e: u64) {
    let len = xs.len();
    let max_multiplier = u64::exact_from(len - 1) * (e + 1);
    if len >= 8 && vec_max_bits(xs).0 + max_multiplier.significant_bits() <= Limb::WIDTH {
        pow_to_out_multinomial_multiples(out, xs, e);
    } else {
        pow_to_out_multinomial_split(out, xs, e);
    }
}}

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, as
// `pow_to_out_multinomial` does, by precomputing the multiples $i(e + 1) q_i$.
crate_test_fn! {pow_to_out_multinomial_multiples<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    e: u64,
) {
    let len = xs.len();
    let e_plus_1 = e + 1;
    let multiples: Vec<C> = xs[1..]
        .iter()
        .zip(1..)
        .map(|(x, i)| {
            let mut c = C::from(i * e_plus_1);
            c *= x;
            c
        })
        .collect();
    out[0] = xs[0].pow_ref(e);
    let mut d = C::ZERO;
    for k in 1..out.len() {
        let mut weighted = C::ZERO;
        let mut plain = C::ZERO;
        for i in 1..=k.min(len - 1) {
            weighted.add_mul_assign(&multiples[i - 1], &out[k - i]);
            plain.add_mul_assign(&xs[i], &out[k - i]);
        }
        plain *= &C::from(u64::exact_from(k));
        weighted -= &plain;
        d += &xs[0];
        weighted.div_exact_assign(&d);
        out[k] = weighted;
    }
}}

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, as
// `pow_to_out_multinomial` does, by summing the positive and negative terms separately.
crate_test_fn! {pow_to_out_multinomial_split<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    e: u64,
) {
    let len = xs.len();
    let mut d = C::ZERO;
    out[0] = xs[0].pow_ref(e);
    for k in 1..out.len() {
        let k_u64 = u64::exact_from(k);
        let mut positive = C::ZERO;
        let mut negative = C::ZERO;
        let mut w = 0;
        for i in 1..=k.min(len - 1) {
            let t = xs[i].mul_ref(&out[k - i]);
            w += e + 1;
            if w >= k_u64 {
                positive.add_mul_assign(&t, &C::from(w - k_u64));
            } else {
                negative.add_mul_assign(&t, &C::from(k_u64 - w));
            }
        }
        positive -= &negative;
        d += &xs[0];
        positive.div_exact_assign(&d);
        out[k] = positive;
    }
}}
