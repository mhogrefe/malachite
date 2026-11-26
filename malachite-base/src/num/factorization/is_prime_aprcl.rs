// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 Vladimir Glazachev
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// The APR-CL primality test with Jacobi sums, for u128s, following FLINT's aprcl module. The steps
// are described in
//
// - [1] H. Cohen, "A Course in Computational Algebraic Number Theory", algorithm 9.1.28;
// - [2] H. Cohen and A. K. Lenstra, "Implementation of a New Primality Test".
//
// Standard variables, as in FLINT:
//
// - n: the number to test;
// - R, s: configuration parameters, with s^2 > n and a^R = 1 mod s for every a coprime to s;
// - q: a prime dividing s;
// - p: a prime dividing q - 1;
// - r = p^k: the largest power of p dividing q - 1;
// - v = n mod r, u = n / r.
//
// FLINT runs the arithmetic in Z[zeta_{p^k}] / (n) with its mpn_mod-based unity_zp_mpn code for
// two-limb moduli; this is that code with u128 Montgomery arithmetic. An element is a vector of the
// d = (p - 1) p^(k - 1) coefficients of a polynomial reduced modulo the cyclotomic polynomial
// Phi_{p^k}, each in Montgomery form and reduced mod n.

use crate::fail_on_untested_path;
use crate::num::arithmetic::traits::{DivMod, FloorSqrt, Parity, PowerOf2};
use crate::num::conversion::traits::ExactFrom;
use crate::num::factorization::is_prime::{
    RedcContext128, redc_context_128, redc_mul_128, redc_pow_128, to_redc_128,
};
use crate::num::factorization::traits::{Factor, IsPower, IsPrime, PrimitiveRootPrime};
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::num::modular::traits::{
    DivisibleBy, ModAdd, ModAddAssign, ModMul, ModMulAssign, ModPow, ModPowerOf2, ModSub,
    ModSubAssign,
};
use crate::num::number_theory::traits::CoprimeWith;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::swap;

#[cfg(feature = "std")]
#[inline]
fn ln_f64(x: f64) -> f64 {
    x.ln()
}

#[cfg(not(feature = "std"))]
#[inline]
fn ln_f64(x: f64) -> f64 {
    libm::log(x)
}

// Returns k such that p^k | q and p^(k + 1) does not divide q.
//
// This is aprcl_p_power_in_q from aprcl/utility.c, FLINT 3.7.0-dev.
#[doc(hidden)]
pub const fn p_power_in_q(mut q: u64, p: u64) -> u64 {
    let mut k = 0;
    while q != 0 && q.is_multiple_of(p) {
        k += 1;
        q /= p;
    }
    k
}

// Arithmetic in Z[zeta_{p^k}] / (n), with elements reduced mod Phi_{p^k}.
struct UnityZp<'a> {
    ctx: &'a RedcContext128,
    p: u64,
    // p^k
    pk: u64,
    // p^(k - 1)
    pk1: u64,
    // (p - 1) p^(k - 1), the degree of Phi_{p^k}
    d: usize,
    // the 2 d - 1 coefficients of an unreduced product
    c: Vec<u128>,
}

impl<'a> UnityZp<'a> {
    fn new(ctx: &'a RedcContext128, p: u64, exp: u64) -> Self {
        let pk1 = p.pow(u32::exact_from(exp - 1));
        Self {
            ctx,
            p,
            pk: pk1 * p,
            pk1,
            d: usize::exact_from((p - 1) * pk1),
            c: vec![0; usize::exact_from(((p - 1) * pk1) << 1) - 1],
        }
    }

    fn zero(&self) -> Vec<u128> {
        vec![0; self.d]
    }

    fn one(&self) -> Vec<u128> {
        let mut f = self.zero();
        f[0] = self.ctx.one;
        f
    }

    // Converts a vector of small integer coefficients, already reduced mod Phi_{p^k}.
    fn element_from_small(&self, cs: &[i64]) -> Vec<u128> {
        let n = self.ctx.n;
        cs.iter()
            .map(|&c| {
                let r = u128::from(c.unsigned_abs()) % n;
                let r = if c < 0 && r != 0 { n - r } else { r };
                to_redc_128(r, self.ctx)
            })
            .collect()
    }

    // Reduces the product coefficients c mod Phi_{p^k}, writing the result to f. Since x^{p^k} = 1
    // and x^e = -(x^{e - d} + x^{e - d + p^(k - 1)} + ... + x^{e - d + (p - 2) p^(k - 1)}) for d <=
    // e < p^k, f_i = c_i + c_{i + p^k} - c_{d + (i mod p^(k - 1))}.
    //
    // This is _unity_zp_mpn_cyclotomic_reduce from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn cyclotomic_reduce(&self, f: &mut [u128]) {
        let n = self.ctx.n;
        let d = self.d;
        let c = &self.c;
        let last = (d << 1) - 2;
        let pk = usize::exact_from(self.pk);
        let pk1 = usize::exact_from(self.pk1);
        for (i, fi) in f.iter_mut().enumerate() {
            let mut x = c[i];
            if i + pk <= last {
                x.mod_add_assign(c[i + pk], n);
            }
            let e = d + i % pk1;
            if e <= last {
                x.mod_sub_assign(c[e], n);
            }
            *fi = x;
        }
    }

    // f = g * h.
    //
    // This is _unity_zp_mpn_mul from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev, with each product
    // reduced mod n at once rather than accumulated exactly.
    fn mul_to(&mut self, f: &mut [u128], g: &[u128], h: &[u128]) {
        let n = self.ctx.n;
        self.c.fill(0);
        for (i, &gi) in g.iter().enumerate() {
            if gi == 0 {
                continue;
            }
            for (j, &hj) in h.iter().enumerate() {
                self.c[i + j].mod_add_assign(redc_mul_128(gi, hj, self.ctx), n);
            }
        }
        self.cyclotomic_reduce(f);
    }

    // f = g^2, computing each cross product g_i g_j with i < j once.
    fn sqr_to(&mut self, f: &mut [u128], g: &[u128]) {
        let n = self.ctx.n;
        self.c.fill(0);
        for (i, &gi) in g.iter().enumerate() {
            if gi == 0 {
                continue;
            }
            for (j, &gj) in g.iter().enumerate().skip(i + 1) {
                let x = redc_mul_128(gi, gj, self.ctx);
                self.c[i + j].mod_add_assign(x.mod_add(x, n), n);
            }
            self.c[i << 1].mod_add_assign(redc_mul_128(gi, gi, self.ctx), n);
        }
        self.cyclotomic_reduce(f);
    }

    fn mul(&mut self, g: &[u128], h: &[u128]) -> Vec<u128> {
        let mut f = self.zero();
        self.mul_to(&mut f, g, h);
        f
    }

    fn sqr(&mut self, g: &[u128]) -> Vec<u128> {
        let mut f = self.zero();
        self.sqr_to(&mut f, g);
        f
    }

    // f *= h, with t as scratch.
    fn mul_assign(&mut self, f: &mut Vec<u128>, h: &[u128], t: &mut Vec<u128>) {
        self.mul_to(t, f, h);
        swap(f, t);
    }

    // f = f^2, with t as scratch.
    fn sqr_assign(&mut self, f: &mut Vec<u128>, t: &mut Vec<u128>) {
        self.sqr_to(t, f);
        swap(f, t);
    }

    // This is unity_zp_mpn_mul_scalar_ui from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn mul_scalar(&self, g: &[u128], s: u64) -> Vec<u128> {
        let s = to_redc_128(u128::from(s) % self.ctx.n, self.ctx);
        g.iter().map(|&x| redc_mul_128(x, s, self.ctx)).collect()
    }

    // Sliding window powering, f = g^pow.
    //
    // This is unity_zp_mpn_pow_sliding_fmpz from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn pow_sliding(&mut self, g: &[u128], pow: u128) -> Vec<u128> {
        // select the optimal k
        let k = pow_select_k(pow.significant_bits());
        let m = usize::power_of_2(k - 1);
        // g_powers[(i + 1) / 2] = g^i, for odd i up to 2^k - 1
        let g2 = self.sqr(g);
        let mut g_powers = Vec::with_capacity(m + 1);
        g_powers.push(self.one());
        g_powers.push(g.to_vec());
        for i in 2..=m {
            let next = self.mul(&g_powers[i - 1], &g2);
            g_powers.push(next);
        }
        let mut f = self.one();
        let mut t = self.zero();
        let mut i = i64::exact_from(pow.significant_bits()) - 1;
        while i >= 0 {
            if pow.get_bit(u64::exact_from(i)) {
                // The chain is the longest bitstring of length at most k ending in 1.
                let mut j = (i - i64::exact_from(k) + 1).max(0);
                while !pow.get_bit(u64::exact_from(j)) && j <= i {
                    j += 1;
                }
                for _ in 0..=i - j {
                    self.sqr_assign(&mut f, &mut t);
                }
                let mut value = 0usize;
                for h in 0..=i - j {
                    if pow.get_bit(u64::exact_from(j + h)) {
                        value.set_bit(u64::exact_from(h));
                    }
                }
                self.mul_assign(&mut f, &g_powers[(value + 1) >> 1], &mut t);
                i = j - 1;
            } else {
                self.sqr_assign(&mut f, &mut t);
                i -= 1;
            }
        }
        f
    }

    // Computes f such that sigma_x(f) = g, where sigma_x(zeta) = zeta^x.
    //
    // This is unity_zp_mpn_aut_inv from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn aut_inv(&self, g: &[u128], x: u64) -> Vec<u128> {
        let n = self.ctx.n;
        let d = u64::exact_from(self.d);
        let mut f = self.zero();
        // for i = 0, 1, ..., d - 1, set f[i] = g[x * i mod p^k]
        for i in 0..d {
            let g_ind = x.mod_mul(i, self.pk);
            if g_ind < d {
                f[usize::exact_from(i)] = g[usize::exact_from(g_ind)];
            }
        }
        // for i = d, ..., p^k - 1 and j = 1, ..., p - 1, set f[i - j * p^(k - 1)] -= g[x * i mod
        // p^k]
        for i in d..self.pk {
            let g_ind = x.mod_mul(i, self.pk);
            if g_ind >= d {
                continue;
            }
            let c = g[usize::exact_from(g_ind)];
            for j in 1..self.p {
                let f_ind = usize::exact_from(i - j * self.pk1);
                f[f_ind].mod_sub_assign(c, n);
            }
        }
        f
    }

    // If f = zeta_{p^k}^h for some h, returns h; otherwise returns -1. For h < d, zeta^h is a basis
    // vector, while for d <= h < p^k the reduced representation of zeta^h has the p - 1
    // coefficients (h - d) + j p^(k - 1), 0 <= j < p - 1, equal to -1 and all others zero.
    //
    // This is unity_zp_mpn_is_unity from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn is_unity(&self, f: &[u128]) -> i64 {
        let one = self.ctx.one;
        let neg_one = self.ctx.n - one;
        let mut ones = 0;
        let mut neg_ones = 0u64;
        let mut one_ind = 0;
        let mut neg_one_ind = None;
        for (i, &c) in f.iter().enumerate() {
            if c == 0 {
                continue;
            }
            if c == one {
                ones += 1;
                one_ind = i;
            } else if c == neg_one {
                neg_ones += 1;
                if neg_one_ind.is_none() {
                    neg_one_ind = Some(i);
                }
            } else {
                return -1;
            }
        }
        if ones == 1 && neg_ones == 0 {
            return i64::exact_from(one_ind);
        }
        if ones == 0 && neg_ones == self.p - 1 {
            let neg_one_ind = neg_one_ind.unwrap();
            let pk1 = usize::exact_from(self.pk1);
            let e0 = neg_one_ind % pk1;
            if neg_one_ind != e0 {
                fail_on_untested_path(
                    "u128_is_prime_aprcl, is_unity, first -1 not in the first block",
                );
                return -1;
            }
            for j in 0..usize::exact_from(self.p - 1) {
                if f[e0 + j * pk1] != neg_one {
                    fail_on_untested_path("u128_is_prime_aprcl, is_unity, -1s not evenly spaced");
                    return -1;
                }
            }
            return i64::exact_from(self.d + e0);
        }
        fail_on_untested_path("u128_is_prime_aprcl, is_unity, coefficients in {0, 1, -1}");
        -1
    }
}

// Selects the window size for sliding window powering with an exponent of the given bit length.
//
// This is _unity_zp_pow_select_k from aprcl/unity_zp_pow.c, FLINT 3.7.0-dev.
#[doc(hidden)]
pub const fn pow_select_k(bits: u64) -> u64 {
    match bits {
        0..=8 => 1,
        9..=24 => 2,
        25..=69 => 3,
        70..=196 => 4,
        197..=538 => 5,
        539..=1433 => 6,
        1434..=3714 => 7,
        3715..=9399 => 8,
        9400..=23290 => 9,
        23291..=56651 => 10,
        _ => 11,
    }
}

// Returns a table of the function f defined by 1 - g^x = g^f(x) mod q, where g is a primitive root
// mod q; table[x] = f(x) for x = 1, 2, ..., q - 2.
//
// This is aprcl_f_table from aprcl/f_table.c, FLINT 3.7.0-dev.
fn f_table(q: u64) -> Vec<u64> {
    let g = q.primitive_root_prime();
    let len = usize::exact_from(q);
    let mut tables = vec![0; len << 1];
    let (f_table, g_table) = tables.split_at_mut(len);
    // g_table[g^i mod q] = i
    let mut g_pow = g;
    for i in 1..q {
        g_table[usize::exact_from(g_pow)] = i;
        g_pow.mod_mul_assign(g, q);
    }
    // f_table[i] such that g^f_table[i] = 1 - g^i mod q
    let mut g_pow = g;
    for f in f_table.iter_mut().take(len).skip(1) {
        *f = g_table[usize::exact_from(1u64.mod_sub(g_pow, q))];
        g_pow.mod_mul_assign(g, q);
    }
    tables.truncate(len);
    tables
}

// Computes the sum of zeta_{p^k}^{a * x + b * f(x)} for x = 1, 2, ..., q - 2, reduced mod
// Phi_{p^k}, as a vector of d small integer coefficients.
//
// This is _unity_zp_jacobi_sum_pq_general from aprcl/unity_zp_jacobi_sum.c, FLINT 3.7.0-dev.
fn jacobi_sum_pq_general(table: &[u64], p: u64, q: u64, k: u64, a: u64, b: u64) -> Vec<i64> {
    let pow_dec = p.pow(u32::exact_from(k - 1));
    let size = (p - 1) * pow_dec;
    let pow = pow_dec * p;
    let mut f = vec![0i64; usize::exact_from(size)];
    for i in 1..q - 1 {
        // l = a * i + b * f[i]
        let mut l = (a * i + b * table[usize::exact_from(i)]) % pow;
        if l < size {
            // If l < (p - 1) p^(k - 1), increase f[l] by one.
            f[usize::exact_from(l)] += 1;
        } else {
            // Otherwise, decrease f[l - j p^(k - 1)] by one, for j in [1, p - 1].
            for _ in 0..p - 1 {
                l -= pow_dec;
                f[usize::exact_from(l)] -= 1;
            }
        }
    }
    f
}

// J(p, q): the sum of zeta_{p^k}^{x + f(x)} for x = 1, 2, ..., q - 2.
//
// This is unity_zp_jacobi_sum_pq from aprcl/unity_zp_jacobi_sum.c, FLINT 3.7.0-dev.
#[doc(hidden)]
pub fn jacobi_sum_pq(q: u64, p: u64) -> Vec<i64> {
    let table = f_table(q);
    let k = p_power_in_q(q - 1, p);
    jacobi_sum_pq_general(&table, p, q, k, 1, 1)
}

// J_3(q) / J(2, q): the sum of zeta_{2^k}^{2 * x + f(x)} for x = 1, 2, ..., q - 2.
//
// This is unity_zp_jacobi_sum_2q_one from aprcl/unity_zp_jacobi_sum.c, FLINT 3.7.0-dev.
#[doc(hidden)]
pub fn jacobi_sum_2q_one(q: u64) -> Vec<i64> {
    let table = f_table(q);
    let k = p_power_in_q(q - 1, 2);
    jacobi_sum_pq_general(&table, 2, q, k, 2, 1)
}

// J_2(q): the sum of zeta_{2^k}^{3 * 2^(k - 3) * x + 2^(k - 3) * f(x)} for x = 1, 2, ..., q - 2.
//
// This is unity_zp_jacobi_sum_2q_two from aprcl/unity_zp_jacobi_sum.c, FLINT 3.7.0-dev.
#[doc(hidden)]
pub fn jacobi_sum_2q_two(q: u64) -> Vec<i64> {
    let table = f_table(q);
    let k = p_power_in_q(q - 1, 2);
    let b = u64::power_of_2(k - 3);
    jacobi_sum_pq_general(&table, 2, q, k, 3 * b, b)
}

// Multiplies sigma_i^{-1}(xi) into j0 and sigma_i^{-1}(xe) into jv.
fn mul_aut_terms(
    ring: &mut UnityZp,
    j0: &mut Vec<u128>,
    jv: &mut Vec<u128>,
    xi: &[u128],
    xe: &[u128],
    i: u64,
    t: &mut Vec<u128>,
) {
    let aut = ring.aut_inv(xi, i);
    ring.mul_assign(j0, &aut, t);
    let aut = ring.aut_inv(xe, i);
    ring.mul_assign(jv, &aut, t);
}

// The case p != 2: computes j0 = j_{0, p, q} and jv = j_{v, p, q}, and returns h if j0^u * jv =
// zeta_{p^k}^h, or -1 if there is no such h. See (i1a) in [2], or algorithm 9.1.28, step 4.a, in
// [1].
//
// This is _aprcl_is_prime_jacobi_check_pk_mpn from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev, with the
// powers of j maintained incrementally, as in FLINT's _aprcl_is_prime_jacobi_check_pk.
fn check_pk(ring: &mut UnityZp, j: &[u128], u: u128, v: u64) -> i64 {
    let r = ring.pk;
    let mut j0 = ring.one();
    let mut jv = ring.one();
    let mut t = ring.zero();
    // ji = j^i and je = j^e, with e = (v * i) / r, maintained incrementally; e grows by at most one
    // per step since v < r.
    let mut ji = ring.one();
    let mut je = ring.one();
    let mut e = 0;
    for i in 1..=r {
        ring.mul_assign(&mut ji, j, &mut t);
        while e < v * i / r {
            ring.mul_assign(&mut je, j, &mut t);
            e += 1;
        }
        if i.is_multiple_of(ring.p) {
            continue;
        }
        mul_aut_terms(ring, &mut j0, &mut jv, &ji, &je, i, &mut t);
    }
    // j0 = j0^u * jv
    let j0 = ring.pow_sliding(&j0, u);
    ring.mul_to(&mut t, &jv, &j0);
    ring.is_unity(&t)
}

// The case p = 2, k = 1: j0^u * jv = (-q)^{(n - 1) / 2}. Returns 0 if it is 1, 1 if it is -1, and
// -1 otherwise. See algorithm 9.1.28, step 4.d, in [1].
//
// This is _aprcl_is_prime_jacobi_check_21 from aprcl/is_prime_jacobi.c, FLINT 3.7.0-dev.
fn check_21(ctx: &RedcContext128, q: u64) -> i64 {
    let n = ctx.n;
    let q_pow = redc_pow_128(to_redc_128(n - u128::from(q), ctx), (n - 1) >> 1, ctx);
    if q_pow == ctx.one {
        0
    } else if q_pow == n - ctx.one {
        1
    } else {
        fail_on_untested_path("u128_is_prime_aprcl, check_21, not +-1");
        -1
    }
}

// The case p = 2, k = 2: returns h if j0^u * jv = zeta_4^h, or -1 if there is no such h. See
// algorithm 9.1.28, step 4.c, in [1].
//
// This is _aprcl_is_prime_jacobi_check_22_mpn from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
fn check_22(ring: &mut UnityZp, j: &[u128], u: u128, v: u64, q: u64) -> i64 {
    // j0 = q * J^2
    let j_pow = ring.sqr(j);
    let j0 = ring.mul_scalar(&j_pow, q);
    // jv = 1 if v == 1; jv = J^2 if v == 3 (v is odd since n is)
    let jv = if v == 1 { ring.one() } else { j_pow };
    // j0 = j0^u * jv
    let t = ring.pow_sliding(&j0, u);
    let j0 = ring.mul(&jv, &t);
    ring.is_unity(&j0)
}

// The case p = 2, k >= 3, given J(2, q), J_3(q) / J(2, q), and J_2(q): returns h if j0^u * jv =
// zeta_{2^k}^h, or -1 if there is no such h. See algorithm 9.1.28, step 4.b, in [1].
//
// This is _aprcl_is_prime_jacobi_check_2k_mpn from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev, with the
// powers of j_j0 maintained incrementally, as in FLINT's _aprcl_is_prime_jacobi_check_2k.
fn check_2k(ring: &mut UnityZp, j: &[u128], j2_1: &[u128], j2_2: &[u128], u: u128, v: u64) -> i64 {
    let r = ring.pk;
    let mut j0 = ring.one();
    let mut jv = ring.one();
    let mut t = ring.zero();
    // j_j0 = J(2, q) * J_3(q)
    let j_j0 = ring.mul(j, j2_1);
    // The loop runs over i = 1, 3, 9, 11, 17, 19, ... (i = 1 or 3 mod 8). The powers ji = j_j0^i
    // and je = j_j0^{(v * i) / r} are maintained incrementally: ji is multiplied by jj2 = j_j0^2 or
    // jj6 = j_j0^6 as i steps by 2 or 6, and je picks up one factor of j_j0 each time (v * i) / r
    // increases.
    let jj2 = ring.sqr(&j_j0);
    let jj4 = ring.sqr(&jj2);
    let jj6 = ring.mul(&jj4, &jj2);
    let mut ji = j_j0.clone();
    let mut je = ring.one();
    let mut e = 0;
    let mut i = 1;
    while i < r {
        while e < v * i / r {
            ring.mul_assign(&mut je, &j_j0, &mut t);
            e += 1;
        }
        mul_aut_terms(ring, &mut j0, &mut jv, &ji, &je, i, &mut t);
        let i_mod_8 = i.mod_power_of_2(3);
        ring.mul_assign(&mut ji, if i_mod_8 == 1 { &jj2 } else { &jj6 }, &mut t);
        i += if i_mod_8 == 1 { 2 } else { 6 };
    }
    // If v is not 1 or 3 mod 8, jv *= J_2(q)^2.
    let v_mod_8 = v.mod_power_of_2(3);
    if v_mod_8 != 1 && v_mod_8 != 3 {
        let s = ring.sqr(j2_2);
        ring.mul_assign(&mut jv, &s, &mut t);
    }
    // j0 = j0^u * jv
    let j0 = ring.pow_sliding(&j0, u);
    ring.mul_to(&mut t, &jv, &j0);
    ring.is_unity(&t)
}

// The configuration of the test: R, s, the factorization of R, and the prime powers q^k dividing s.
struct Config {
    r: u64,
    s: u128,
    // the factors of R
    rs: Vec<(u64, u64)>,
    // the primes q with q - 1 | R, and the powers of them in the full s
    qs: Vec<(u64, u64)>,
    qs_used: Vec<bool>,
}

// This is aprcl_R_value from aprcl/config_jacobi.c, FLINT 3.7.0-dev, for n < 2^128.
const fn r_value(bits: u64) -> u64 {
    match bits {
        0..=17 => 6,
        18..=31 => 12,
        32..=54 => 36,
        55..=68 => 72,
        69..=101 => 180,
        102..=127 => 360,
        _ => 720,
    }
}

// The exponent alpha in the cost model cost(p, q) ~ phi(p^k)^alpha used when choosing which primes
// q to keep in s.
//
// This is APRCL_JACOBI_COST_EXPONENT from aprcl/config_jacobi.c, FLINT 3.7.0-dev.
#[doc(hidden)]
pub const JACOBI_COST_EXPONENT: f64 = 2.0;

// Primes q above this bound are only kept in s when they are needed to reach s^2 > n. It is only
// used by the Natural version, since the primes q are at most 721 here.
//
// This is APRCL_MAX_Q from aprcl/config_jacobi.c, FLINT 3.7.0-dev.
#[doc(hidden)]
pub const MAX_Q: u64 = 1 << 22;

impl Config {
    // Sets s to the product of q^(j + 1) over the primes q with q - 1 | R, where q^j || R, with one
    // more power of 2; then removes prime powers from s, most expensive first, as long as s^2 > n.
    //
    // This is aprcl_config_jacobi_init_R, _aprcl_config_jacobi_update, and
    // _aprcl_config_jacobi_reduce_s2 from aprcl/config_jacobi.c, FLINT 3.7.0-dev. FLINT checks that
    // s^2 > n p^2 before removing p = q^k from s; since p | s, that is (s / p)^2 > n, which can be
    // checked without overflow.
    fn new(n: u128) -> Self {
        let r = r_value(n.significant_bits());
        let rs: Vec<(u64, u64)> = r
            .factor()
            .into_iter()
            .map(|(p, e)| (p, u64::from(e)))
            .collect();
        // enumerate the divisors of R, in increasing order
        let mut divisors = vec![1u64];
        for &(p, e) in &rs {
            let len = divisors.len();
            let mut pk = 1;
            for _ in 0..e {
                pk *= p;
                for l in 0..len {
                    divisors.push(divisors[l] * pk);
                }
            }
        }
        divisors.sort_unstable();
        // q = d + 1 prime; q = 2 (from d = 1) gets one extra power of 2
        let mut s = 1u128;
        let mut qs = Vec::new();
        for d in divisors {
            let q = d + 1;
            if q.is_prime() {
                let k = p_power_in_q(r, q) + 1 + u64::from(q == 2);
                qs.push((q, k));
                s *= u128::from(q).pow(u32::exact_from(k));
            }
        }
        let mut config = Self {
            r,
            s,
            rs,
            qs_used: vec![true; qs.len()],
            qs,
        };
        config.reduce_s2(n);
        // The tabulated R makes s^2 > n for every n < 2^128, and the reduction preserves that.
        assert!(s_squared_gt(config.s, n));
        config
    }

    fn reduce_s2(&mut self, n: u128) {
        // w[i] = the estimated cost of the Jacobi sum tests for q per bit contributed to s
        let w: Vec<f64> = self
            .qs
            .iter()
            .map(|&(q, k)| {
                let mut w = 0.0;
                for (p, e) in (q - 1).factor() {
                    let euler_phi = p.pow(u32::from(e - 1)) * (p - 1);
                    w += f64::from(u32::exact_from(euler_phi)).powf(JACOBI_COST_EXPONENT);
                }
                w / ln_f64(f64::from(u32::exact_from(q)).powf(f64::from(u32::exact_from(k))))
            })
            .collect();
        loop {
            let mut w_max = -1.0;
            let mut ind = None;
            for (i, &(q, k)) in self.qs.iter().enumerate() {
                if !self.qs_used[i] {
                    continue;
                }
                // removing q^k from s keeps s^2 > n
                if s_squared_gt(self.s / u128::from(q).pow(u32::exact_from(k)), n) && w_max <= w[i]
                {
                    w_max = w[i];
                    ind = Some(i);
                }
            }
            let Some(ind) = ind else {
                break;
            };
            let (q, k) = self.qs[ind];
            self.s /= u128::from(q).pow(u32::exact_from(k));
            self.qs_used[ind] = false;
        }
    }
}

// Whether s^2 > n.
fn s_squared_gt(s: u128, n: u128) -> bool {
    s.checked_mul(s).is_none_or(|s2| s2 > n)
}

// Tries to find a prime q = 2mp + 1, with m odd, such that n^{(q - 1) / p} != 1 mod q, and then
// verifies the condition (L_p) with it. Returns 2 if n is composite, 1 if n can be prime, and 0 if
// no such q was found among the first 50 candidates and n can be prime. See algorithm 9.1.28, step
// 5, in [1].
//
// This is _aprcl_is_prime_jacobi_additional_test from aprcl/is_prime_jacobi.c, FLINT 3.7.0-dev.
fn additional_test(ctx: &RedcContext128, p: u64) -> u8 {
    let n = ctx.n;
    // check only the first 50 primes q = 2mp + 1, starting from m = 9
    let mut p_counter = 50;
    let mut m = 9;
    let mut q = 0;
    while p_counter > 0 {
        // q = 2mp + 1 and m is odd, so if p = 2 then 4 | q - 1 and 8 does not.
        q = ((m * p) << 1) + 1;
        if q.is_prime() && !n.is_multiple_of(u128::from(q)) {
            let n_mod_q = u64::exact_from(n % u128::from(q));
            // if n^{(q - 1) / p} != 1 mod q, we have found q
            if n_mod_q.mod_pow((q - 1) / p, q) != 1 {
                break;
            }
            p_counter -= 1;
        }
        m += 2;
    }
    if p_counter != 0 {
        if n.is_multiple_of(u128::from(q)) && n != u128::from(q) {
            fail_on_untested_path("u128_is_prime_aprcl, additional_test, q | n");
            2
        } else {
            // the largest k with p^k | q - 1; if p = 2, k = 2
            let k = p_power_in_q(q - 1, p);
            let mut ring = UnityZp::new(ctx, p, k);
            let j = ring.element_from_small(&jacobi_sum_pq(q, p));
            let (u, v) = n.div_mod(u128::from(p.pow(u32::exact_from(k))));
            let v = u64::exact_from(v);
            if p == 2 {
                let h = check_22(&mut ring, &j, u, v, q);
                // If h is not found or is not a primitive root, n is composite.
                if h < 0 || h.even() {
                    fail_on_untested_path(
                        "u128_is_prime_aprcl, additional_test, p = 2, h not found or even",
                    );
                    2
                } else {
                    // If q^{(n - 1) / 2} = -1 mod n, n can be prime.
                    let q_pow = redc_pow_128(to_redc_128(u128::from(q), ctx), (n - 1) >> 1, ctx);
                    if q_pow == n - ctx.one {
                        1
                    } else {
                        fail_on_untested_path(
                            "u128_is_prime_aprcl, additional_test, p = 2, q^((n - 1) / 2) != -1",
                        );
                        2
                    }
                }
            } else {
                let h = check_pk(&mut ring, &j, u, v);
                // If h is not found or is not a primitive root, n is composite.
                if h < 0 || h.divisible_by(i64::exact_from(p)) {
                    fail_on_untested_path(
                        "u128_is_prime_aprcl, additional_test, p odd, h not found or p | h",
                    );
                    2
                } else {
                    1
                }
            }
        }
    } else if n.is_multiple_of(u128::from(p)) || n.is_power() {
        // If no q is found: if p | n or n is a perfect power, n is composite.
        fail_on_untested_path(
            "u128_is_prime_aprcl, additional_test, no q found, p | n or n is a perfect power",
        );
        2
    } else {
        0
    }
}

// One (q, p) pair of the Jacobi sum test.
struct JacobiTask {
    q: u64,
    p: u64,
    k: u64,
    // the index of p in the factors of R
    p_ind: usize,
    // h, or -1 if n is composite
    h: i64,
    // whether q^{(n - 1) / 2} = -1 mod n (for p = 2, k >= 2)
    q_pow_is_minus_1: bool,
}

// Runs one task of the Jacobi sum test.
//
// This is _aprcl_jacobi_run_task from aprcl/is_prime_jacobi.c, FLINT 3.7.0-dev.
fn run_task(ctx: &RedcContext128, t: &mut JacobiTask) {
    let n = ctx.n;
    let (p, k, q) = (t.p, t.k, t.q);
    if p == 2 && k == 1 {
        t.h = check_21(ctx, q);
        return;
    }
    let r = p.pow(u32::exact_from(k));
    let (u, v) = n.div_mod(u128::from(r));
    let v = u64::exact_from(v);
    let mut ring = UnityZp::new(ctx, p, k);
    let j = ring.element_from_small(&jacobi_sum_pq(q, p));
    t.h = if p == 2 {
        // q^{(n - 1) / 2} mod n, for the (L_2) check
        let q_pow = redc_pow_128(to_redc_128(u128::from(q), ctx), (n - 1) >> 1, ctx);
        t.q_pow_is_minus_1 = q_pow == n - ctx.one;
        if k == 2 {
            check_22(&mut ring, &j, u, v, q)
        } else {
            // for k >= 3, J_2(q) and J_3(q) are also needed
            let j2_1 = ring.element_from_small(&jacobi_sum_2q_one(q));
            let j2_2 = ring.element_from_small(&jacobi_sum_2q_two(q));
            check_2k(&mut ring, &j, &j2_1, &j2_2, u, v)
        }
    } else {
        check_pk(&mut ring, &j, u, v)
    };
}

// The final trial division: having shown that every divisor of n is congruent to n^i mod s for some
// 0 <= i < r, tests whether n is divisible by any residue n^i mod s with 1 < n^i mod s <= sqrt(n).
// Returns true if no divisor is found.
//
// This is aprcl_is_prime_final_division from aprcl/is_prime_final_division.c, FLINT 3.7.0-dev, with
// the reference walk (FLINT's _aprcl_is_prime_final_division_fmpz) on u128s, since s < 2^128.
//
// Since n >= 2^64, FLINT's check for n <= 3 is not needed; since n is odd, only odd residues are
// tested; and since s^2 > n, every residue up to sqrt(n) is below s.
fn final_division(n: u128, s: u128, r: u64) -> bool {
    let bound = n.floor_sqrt();
    let n_mul = n % s;
    let mut n_pow = n_mul;
    for _ in 1..r {
        if n_pow == 1 {
            break;
        }
        if n_pow.odd() && n_pow <= bound && n.is_multiple_of(n_pow) {
            fail_on_untested_path("u128_is_prime_aprcl, final_division, divisor found");
            return false;
        }
        n_pow.mod_mul_assign(n_mul, s);
    }
    true
}

// The APR-CL test with Jacobi sums, for an odd n >= 2^64. Returns whether n is prime, panicking in
// the unlikely case that the test cannot decide.
//
// This is aprcl_is_prime_jacobi and _aprcl_is_prime_jacobi from aprcl/is_prime_jacobi.c, FLINT
// 3.7.0-dev, run on one thread.
crate_test_fn! {u128_is_prime_aprcl(n: u128) -> bool {
    // FLINT first checks whether n is one of the primes q, but they are less than 2^64.
    let config = Config::new(n);
    // Check that s * R and n are coprime; if not, n is composite.
    let r = u128::from(config.r);
    if !(n % r).coprime_with(r) || !(n % config.s).coprime_with(config.s) {
        return false;
    }
    let ctx = redc_context_128(n);
    // The condition (L_p) holds iff for each p | R, every prime divisor l of n, and every positive
    // integer a, there is an m with l^(p - 1) = n^(m(p - 1)) mod p^a. lambdas[i] records it for the
    // i-th prime p | R; it holds already if p >= 3 and n^(p - 1) != 1 mod p^2.
    let mut lambdas: Vec<bool> = config
        .rs
        .iter()
        .map(|&(p, _)| {
            let p2 = u128::from(p * p);
            p > 2 && (n % p2).mod_pow(u64::from(u32::exact_from(p - 1)), p2) != 1
        })
        .collect();
    // one task for every prime q | s and prime p | q - 1
    let mut tasks = Vec::new();
    for (i, &(q, _)) in config.qs.iter().enumerate() {
        if !config.qs_used[i] {
            continue;
        }
        for (p, k) in (q - 1).factor() {
            tasks.push(JacobiTask {
                q,
                p,
                k: u64::from(k),
                p_ind: config.rs.iter().position(|&(p_r, _)| p_r == p).unwrap(),
                h: 0,
                q_pow_is_minus_1: false,
            });
        }
    }
    // in order of decreasing phi(p^k), then decreasing q
    tasks.sort_by(|x, y| {
        let px = x.p.pow(u32::exact_from(x.k - 1)) * (x.p - 1);
        let py = y.p.pow(u32::exact_from(y.k - 1)) * (y.p - 1);
        py.cmp(&px).then(y.q.cmp(&x.q))
    });
    // the pseudoprime tests with Jacobi sums
    for t in &mut tasks {
        run_task(&ctx, t);
        if t.h < 0 {
            return false;
        }
    }
    // collect the (L_p) information from the tasks
    let n_mod_4 = n.mod_power_of_2(2);
    for t in &tasks {
        let (p, k, h) = (t.p, t.k, t.h);
        if lambdas[t.p_ind] {
            continue;
        }
        lambdas[t.p_ind] = if p == 2 && k == 1 {
            // If h = 1 (the root of unity is -1) and n = 1 mod 4, (L_2) holds.
            h == 1 && n_mod_4 == 1
        } else if p == 2 {
            // If h is odd (a primitive root of unity) and q^{(n - 1) / 2} = -1 mod n, (L_2) holds.
            h.odd() && t.q_pow_is_minus_1
        } else {
            // If h is not divisible by p (a primitive root of unity), (L_p) holds.
            !h.divisible_by(i64::exact_from(p))
        };
    }
    // the additional tests, for each p | R for which (L_p) has not been shown
    let mut unknown = false;
    for (i, &(p, _)) in config.rs.iter().enumerate() {
        if !lambdas[i] {
            match additional_test(&ctx, p) {
                2 => return false,
                1 => lambdas[i] = true,
                _ => unknown = true,
            }
        }
    }
    assert!(!unknown, "u128_is_prime_aprcl: failed to prove n prime for n = {n}");
    // the final trial division; since s^2 > n, n is prime if no divisor is found
    final_division(n, config.s, config.r)
}}
