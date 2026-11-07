// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::arithmetic::mul::multi_mod::primes_above;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::modular::crt_comb::CrtComb;
use crate::natural::number_theory::bell_number::divisor_table;
use crate::natural::{Natural, WIDTH_MINUS_1};
use crate::natural_polynomial::NaturalPolynomial;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExact, Parity, Pow, PowerOf2, Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::{
    ModInverse, ModMulPrecomputed, ModMulPrecomputedAssign, ModNegAssign, ModPowPrecomputed,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::series::traits::{ModMulTruncated, MulTruncated};
use malachite_base::polynomial::special::traits::{
    TouchardPolynomial, TouchardPolynomialTruncated,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Upper bounds for the binary logarithms of S(n, k) for k < len, written to `bound`, and their
// maximum: (1/2) binomial(n, k) k^(n - k) bounds S(n, k).
//
// This is `stirling_2_bound_2exp_vec` from `arith/stirling2.c`, FLINT 3.6.0.
fn stirling_2_bound_2exp_vec(bound: &mut [i64], n: u64, len: usize) -> i64 {
    let kmax = len - 1;
    bound[0] = 0;
    let mut max = 0;
    // binomial coefficients
    let mut bnk = 1.0;
    let mut bnk_exp = 0i64;
    for (k, b) in bound[..=kmax].iter_mut().enumerate().skip(1) {
        // binomial recurrence
        bnk = (bnk * (n + 1 - k as u64) as f64) / k as f64;
        let (mantissa, exp) = libm::frexp(bnk);
        bnk = mantissa;
        bnk_exp += i64::from(exp);
        // (1/2) * binomial(n,k) * k^(n-k)
        *b = bnk_exp
            + ((n - k as u64) as f64 * libm::log(k as f64) * core::f64::consts::LOG2_E) as i64
            + 1;
        max = max.max(*b);
    }
    max
}

// The largest n for which every S(n, k) fits in one word, and in two.
pub(crate) const MAX_N_1LIMB: usize = 26;
pub(crate) const MAX_N_2LIMB: usize = 43;

// The bit size of the primes used by the multimodular algorithms.
pub(crate) const PRIME_BITS: i64 = WIDTH_MINUS_1 as i64;

// Advances c from row `from` of the triangle to row n: afterwards c[k] = S(n, k) for k < klen.
fn triangular_step<T: PrimitiveUnsigned>(c: &mut [T], from: usize, n: usize, klen: usize) {
    for m in from + 1..=n {
        if m < klen {
            c[m] = T::ONE;
        }
        for k in (2..m.min(klen)).rev() {
            c[k] = c[k] * T::wrapping_from(k) + c[k - 1];
        }
    }
}

// c[k] = S(n, k) for k < klen, with n at least 1, by the triangular recurrence, starting from row
// 3.
//
// This is `triangular_1` from `arith/stirling2.c`, FLINT 3.6.0.
pub(crate) fn triangular_1(c: &mut [u64], n: usize, klen: usize) {
    c[..4].copy_from_slice(&[0, 1, 3, 1]);
    triangular_step(c, 3, n, klen);
}

// As `triangular_1`, with double-word entries: the rows up to `MAX_N_1LIMB` are computed with
// single words and then widened.
//
// This is `triangular_2` from `arith/stirling2.c`, FLINT 3.6.0, with a `u128` for each pair of
// words.
pub(crate) fn triangular_2(c: &mut [u128], n: usize, klen: usize) {
    let mut c1 = [0u64; MAX_N_2LIMB + 1];
    triangular_1(&mut c1, MAX_N_1LIMB, klen);
    for (x, &y) in c.iter_mut().zip(&c1[..=MAX_N_1LIMB.min(klen - 1)]) {
        *x = u128::from(y);
    }
    triangular_step(c, MAX_N_1LIMB, n, klen);
}

crate_test_fn! {stirling_2_vec_triangular(row: &mut [Natural], n: u64, klen: usize) {
    if klen == 0 {
        return;
    }
    let un = usize::exact_from(n);
    if n >= 1 {
        if un <= MAX_N_1LIMB {
            let mut c = [0u64; MAX_N_2LIMB + 1];
            triangular_1(&mut c, un, klen);
            for k in 0..=(klen - 1).min(un) {
                row[k] = Natural::from(c[k]);
            }
        } else {
            let m = un.min(MAX_N_2LIMB);
            let mut c = [0u128; MAX_N_2LIMB + 1];
            triangular_2(&mut c, m, klen);
            for k in 0..=(klen - 1).min(m) {
                row[k] = Natural::from(c[k]);
            }
            for m in const { MAX_N_2LIMB + 1 }..=un {
                if m < klen {
                    row[m] = Natural::ONE;
                }
                for k in (2..m.min(klen)).rev() {
                    let (lower, upper) = row.split_at_mut(k);
                    upper[0] *= Natural::from(k);
                    upper[0] += &lower[k - 1];
                }
            }
        }
    }
    for (k, x) in row[..klen].iter_mut().enumerate().skip(un) {
        *x = Natural::from(u32::from(k == un));
    }
}}

// res[k] = S(n, k) for k < klen, by the convolution of (-1)^i/i! with i^n/i!, scaled by (len!)^2 to
// stay in the integers.
//
// This is `arith_stirling_number_2_vec_convolution` from `arith/stirling2.c`, FLINT 3.6.0.
crate_test_fn! {stirling_2_vec_convolution(res: &mut [Natural], n: u64, klen: usize) {
    if klen == 0 {
        return;
    }
    let un = usize::exact_from(n);
    let len = (klen - 1).min(un.wrapping_sub(1));
    if n >= 1 && len >= 1 {
        let mut t = vec![Natural::ZERO; len + 1];
        let mut u = vec![Integer::ZERO; len];
        t[len] = Natural::ONE;
        for k in (0..len).rev() {
            t[k] = &t[k + 1] * Natural::from(k + 1);
        }
        let mut kodd = 1;
        while kodd <= len {
            let v = Natural::from(kodd).pow(n);
            let mut k = kodd;
            let mut e = 0;
            while k <= len {
                // k^n / k!
                u[k - 1] = Integer::from((&v * &t[k]) << (e * n));
                k <<= 1;
                e += 1;
            }
            kodd += 2;
        }
        let square = (&t[0]).square();
        t.truncate(len);
        let ts: Vec<Integer> = t
            .into_iter()
            .enumerate()
            .map(|(k, x)| {
                if k.odd() {
                    -Integer::from(x)
                } else {
                    Integer::from(x)
                }
            })
            .collect();
        let v = (&IntegerPolynomial::from_coefficients_asc(ts))
            .mul_truncated(&IntegerPolynomial::from_coefficients_asc(u), u64::exact_from(len));
        let vs = v.into_coefficients_asc();
        // v[k] = (len!)^2 S(n, k + 1), which is positive for k < len, since len < n; so the product
        // has all len coefficients.
        for (r, x) in res[1..=len].iter_mut().zip(&vs) {
            *r = Natural::exact_from(x).div_exact(&square);
        }
    }
    res[0] = Natural::from(u32::from(n == 0));
    for (k, x) in res[..klen].iter_mut().enumerate().skip(un) {
        *x = Natural::from(u32::from(k == un));
    }
}}

// S(n, k) modulo the prime p for k < len, by the convolution of (-1)^i/i! with i^n/i!. p must be
// greater than len - 1, so that the factorials are invertible.
//
// This is `arith_stirling_number_2_nmod_vec` from `arith/stirling2.c`, FLINT 3.6.0. len is
// positive: `stirling_2_vec_multi_mod` gives a prime only to a level that needs at least one
// coefficient.
fn stirling_2_mod_vec(divtab: &[(u32, u32)], n: u64, len: usize, p: Limb) -> Vec<Limb> {
    // The modulus is fixed for the whole computation, so the data for multiplying and
    // exponentiating modulo it is computed once.
    let mul_data = Limb::precompute_mod_mul_data(&p);
    let pow_data = Limb::precompute_mod_pow_data(&p);
    // compute inverse factorials
    let mut t = vec![0; len];
    t[len - 1] = 1;
    for i in (0..len - 1).rev() {
        t[i] = t[i + 1].mod_mul_precomputed(Limb::exact_from(i + 1), p, &mul_data);
    }
    let c = t[0].mod_inverse(p).unwrap();
    t[0] = 1;
    for x in &mut t[1..] {
        *x = x.mod_mul_precomputed(c, p, &mul_data);
    }
    // compute powers
    let mut u = vec![0; len];
    u[0] = Limb::ZERO.mod_pow_precomputed(n, p, &pow_data);
    if len > 1 {
        u[1] = 1;
    }
    for i in 2..len {
        let (a, b) = divtab[i];
        u[i] = if a == 1 {
            Limb::exact_from(i).mod_pow_precomputed(n, p, &pow_data)
        } else {
            u[a as usize].mod_mul_precomputed(u[b as usize], p, &mul_data)
        };
    }
    for i in 1..len {
        u[i].mod_mul_precomputed_assign(t[i], p, &mul_data);
    }
    for i in (1..len).step_by(2) {
        t[i].mod_neg_assign(p);
    }
    let mut res = (&UnsignedPolynomial::from_coefficients_asc(t))
        .mod_mul_truncated(
            &UnsignedPolynomial::from_coefficients_asc(u),
            u64::exact_from(len),
            p,
        )
        .into_coefficients_asc();
    res.resize(len, 0);
    res
}

const CRT_MAX_RESOLUTION: usize = 16;

// res[k] = S(n, k) for k < klen, from their residues modulo enough primes. The entries are
// recovered with as few primes as their bounds allow, using combinations of the residues prepared
// for up to `CRT_MAX_RESOLUTION` numbers of primes.
//
// This is `arith_stirling_number_2_vec_multi_mod` from `arith/stirling2.c`, FLINT 3.6.0.
crate_test_fn! {stirling_2_vec_multi_mod(res: &mut [Natural], n: u64, mut klen: usize) {
    if klen == 0 {
        return;
    }
    if n <= 2 {
        stirling_2_vec_triangular(res, n, klen);
        return;
    }
    let un = usize::exact_from(n);
    if klen > un + 1 {
        for x in &mut res[un + 1..klen] {
            *x = Natural::ZERO;
        }
        klen = un + 1;
    }
    let len = klen;
    let mut bounds = vec![0i64; len];
    let need_bits = stirling_2_bound_2exp_vec(&mut bounds, n, len).max(1);
    // make bounds nonincreasing
    for k in (0..len - 1).rev() {
        bounds[k] = bounds[k].max(bounds[k + 1]);
    }
    let resolution = (un >> 4).clamp(1, CRT_MAX_RESOLUTION);
    let num_primes = usize::exact_from((need_bits + PRIME_BITS - 1) / PRIME_BITS);
    let divtab = divisor_table(len);
    let primes = primes_above(Limb::power_of_2(WIDTH_MINUS_1), num_primes);
    // levels[i] = (the number of primes of level i, the number of coefficients it needs)
    let mut levels = vec![(0, 0); resolution];
    // combs[comb_index[i]] combines the residues of level i; levels with the same number of primes
    // share a combination, and a single prime needs none.
    let mut combs: Vec<Option<CrtComb>> = Vec::new();
    let mut comb_index = vec![0; resolution];
    for i in 0..resolution {
        levels[i].0 = (num_primes * (i + 1) / resolution).max(1);
        if i > 0 && levels[i].0 == levels[i - 1].0 {
            comb_index[i] = comb_index[i - 1];
        } else {
            comb_index[i] = combs.len();
            combs.push(if levels[i].0 > 1 {
                Some(CrtComb::new(&primes[..levels[i].0]).unwrap())
            } else {
                None
            });
        }
        levels[i].1 = len;
        if i > 0 {
            while levels[i].1 > 0
                && bounds[levels[i].1 - 1] < PRIME_BITS * i64::exact_from(levels[i - 1].0)
            {
                levels[i].1 -= 1;
            }
        }
    }
    let polys: Vec<Vec<Limb>> = primes
        .iter()
        .enumerate()
        .map(|(j, &p)| {
            let mut i = resolution - 1;
            while i > 0 && j < levels[i - 1].0 {
                i -= 1;
            }
            stirling_2_mod_vec(&divtab, n, levels[i].1, p)
        })
        .collect();
    let mut residues = vec![0; num_primes];
    for k in 0..len {
        let mut i = 0;
        while i + 1 < resolution && bounds[k] >= i64::exact_from(levels[i].0) * PRIME_BITS {
            i += 1;
        }
        // Use only as large a comb as needed
        let num_primes_k = levels[i].0;
        for (r, poly) in residues[..num_primes_k].iter_mut().zip(&polys) {
            *r = poly[k];
        }
        res[k] = if num_primes_k == 1 {
            Natural::from(residues[0])
        } else {
            combs[comb_index[i]]
                .as_ref()
                .unwrap()
                .combine(&residues[..num_primes_k])
        };
    }
}}

// row[k] = S(n, k) for k < klen: by the triangular recurrence for small rows, by the convolution
// for short prefixes of large rows, and otherwise by the multimodular computation. The thresholds
// were measured with Malachite's own kernels on 2026-11-06; the recurrence stays competitive for
// longer than in FLINT.
//
// This is `arith_stirling_number_2_vec` from `arith/stirling2.c`, FLINT 3.6.0, with different
// thresholds.
crate_test_fn! {stirling_2_vec(row: &mut [Natural], n: u64, klen: usize) {
    let len = klen as u64;
    if n <= 80 {
        stirling_2_vec_triangular(row, n, klen);
    } else if len << 2 <= n {
        stirling_2_vec_convolution(row, n, klen);
    } else if len << 1 <= n {
        if n <= 450 {
            stirling_2_vec_triangular(row, n, klen);
        } else {
            stirling_2_vec_convolution(row, n, klen);
        }
    } else if n <= 750 {
        stirling_2_vec_triangular(row, n, klen);
    } else {
        stirling_2_vec_multi_mod(row, n, klen);
    }
}}

impl TouchardPolynomial for NaturalPolynomial {
    /// Computes the $n$th Touchard polynomial, whose coefficients are the Stirling numbers of the
    /// second kind $S(n, k)$, the numbers of ways to partition a set of $n$ elements into $k$
    /// nonempty subsets.
    ///
    /// $$
    /// T_n(x) = \sum_{k=0}^n S(n, k) x^k.
    /// $$
    ///
    /// $T_n$ has degree $n$ and leading coefficient 1, and $T_n(1)$ is the $n$th Bell number. The
    /// coefficients are found by the recurrence $S(m, k) = kS(m - 1, k) + S(m - 1, k - 1)$, in
    /// words while they fit, unless $n$ is large; then they are found modulo enough word-sized
    /// primes, each of the corresponding convolutions of $(-1)^i/i!$ with $i^n/i!$ taking one
    /// polynomial product, and recombined by the Chinese remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::special::traits::TouchardPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(NaturalPolynomial::touchard_polynomial(0).to_string(), "1");
    /// assert_eq!(NaturalPolynomial::touchard_polynomial(1).to_string(), "x");
    /// // There are 7 ways to partition a set of 4 elements into 2 subsets.
    /// assert_eq!(
    ///     NaturalPolynomial::touchard_polynomial(4).to_string(),
    ///     "x^4+6*x^3+7*x^2+x"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_2_vec` from `arith/stirling2.c`, FLINT 3.6.0,
    /// with a length of $n + 1$, except that the thresholds between its algorithms are different.
    #[inline]
    fn touchard_polynomial(n: u64) -> Self {
        Self::touchard_polynomial_truncated(n, n + 1)
    }
}

impl TouchardPolynomialTruncated for NaturalPolynomial {
    /// Computes the $n$th Touchard polynomial truncated to its first `len` coefficients, which are
    /// the Stirling numbers of the second kind $S(n, k)$ for $k < \text{len}$.
    ///
    /// $$
    /// f(n, l) = \sum_{k=0}^{\min(l, n + 1) - 1} S(n, k) x^k.
    /// $$
    ///
    /// A `len` greater than $n$ gives the whole polynomial. Short prefixes of long rows are found
    /// by a single convolution of $(-1)^i/i!$ with $i^n/i!$, small rows by the recurrence $S(m, k)
    /// = kS(m - 1, k) + S(m - 1, k - 1)$, and long prefixes of long rows modulo enough word-sized
    /// primes, recombined by the Chinese remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(nl (\log n)^3 \log\log n)$
    ///
    /// $M(n, l) = O(nl \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $l$ is `min(len, n + 1)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::special::traits::TouchardPolynomialTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::touchard_polynomial_truncated(5, 3).to_string(),
    ///     "15*x^2+x"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::touchard_polynomial_truncated(2, 10).to_string(),
    ///     "x^2+x"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::touchard_polynomial_truncated(5, 0).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_2_vec` from `arith/stirling2.c`, FLINT 3.6.0,
    /// except that the thresholds between its algorithms are different.
    fn touchard_polynomial_truncated(n: u64, len: u64) -> Self {
        let klen = usize::exact_from(len);
        let mut row = vec![Natural::ZERO; klen];
        stirling_2_vec(&mut row, n, klen);
        Self::from_coefficients_asc(row)
    }
}
