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

// The APR-CL primality test with Jacobi sums, for Naturals, following FLINT's aprcl module. See the
// u128 version in malachite-base for the references and the standard variables.
//
// FLINT does the arithmetic in Z[zeta_{p^k}] / (n) in two ways: with its mpn_mod-based unity_zp_mpn
// code when n has at most 16 limbs, and with its fmpz_mod_poly-based unity_zp code otherwise. Only
// the first is here, as MpnRing, and it is used for every n: it was faster than a port of unity_zp
// at every size tried, up to 1500 bits.

use crate::natural::Natural;
use crate::natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::div_mod::{
    limbs_div_mod_by_two_limb_normalized_preinverted, limbs_div_mod_schoolbook,
    limbs_two_limb_inverse_helper,
};
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::{
    limbs_mul_same_length_to_out, limbs_mul_same_length_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
};
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::platform::{DoubleLimb, LIMB_WIDTH_USIZE, Limb};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{DivMod, FloorSqrt, Parity, Pow, PowerOf2, Square};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::factorization::is_prime_aprcl::{
    JACOBI_COST_EXPONENT, MAX_Q, jacobi_sum_2q_one, jacobi_sum_2q_two, jacobi_sum_pq, p_power_in_q,
    pow_select_k,
};
use malachite_base::num::factorization::traits::{Factor, IsPower, IsPrime};
use malachite_base::num::logic::traits::{BitAccess, LeadingZeros, SignificantBits};
use malachite_base::num::modular::traits::{
    DivisibleBy, ModMul, ModMulAssign, ModPow, ModPowerOf2,
};
use malachite_base::num::number_theory::traits::CoprimeWith;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_test_zero;

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

fn mod_small(n: &Natural, m: u64) -> u64 {
    u64::exact_from(&(n % Natural::from(m)))
}

// Arithmetic in Z[zeta_{p^k}] / (n), with elements reduced mod Phi_{p^k}. An element is a flat
// vector of the d coefficients, each a fully reduced residue of `nl` limbs.
//
// Products are computed with delayed reduction: each coefficient of the integer product of two
// elements, a sum of at most d products of residues and so less than d n^2, is accumulated exactly
// in `sl` limbs; the cyclotomic reduction is applied to the unreduced accumulators, and then each
// output coefficient is reduced mod n once.
//
// This is unity_zp_mpn from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev, with plain limb arithmetic in
// place of mpn_mod, and with squaring computing each cross product once.
struct MpnRing<'a> {
    ms: &'a [Limb],
    // n shifted left so that its highest bit is set, the shift, and the inverse used for division
    ms_norm: Vec<Limb>,
    shift: u64,
    d_inv: Limb,
    neg_one: Vec<Limb>,
    // the padding constant n 2^t = pad B^pad_offset, with B^sl <= n 2^t < 2 B^sl
    pad: Vec<Limb>,
    pad_offset: usize,
    p: u64,
    pk: u64,
    pk1: u64,
    d: usize,
    nl: usize,
    sl: usize,
    // the 2 d - 1 unreduced product coefficients, sl limbs each
    c: Vec<Limb>,
    prod: Vec<Limb>,
    mul_scratch: Vec<Limb>,
    // a padded accumulator (sl + 1 limbs), its shifted copy (sl + 2 limbs), and quotient space
    u: Vec<Limb>,
    un: Vec<Limb>,
    qs: Vec<Limb>,
}

fn sub_mod_in_place(xs: &mut [Limb], ys: &[Limb], ms: &[Limb]) {
    if limbs_sub_same_length_in_place_left(xs, ys) {
        limbs_slice_add_same_length_in_place_left(xs, ms);
    }
}

impl<'a> MpnRing<'a> {
    fn new(ms: &'a [Limb], p: u64, exp: u64) -> Self {
        let nl = ms.len();
        let shift = LeadingZeros::leading_zeros(ms[nl - 1]);
        let mut ms_norm = ms.to_vec();
        if shift != 0 {
            limbs_slice_shl_in_place(&mut ms_norm, shift);
        }
        let d_inv = limbs_two_limb_inverse_helper(ms_norm[nl - 1], ms_norm[nl - 2]);
        let mut neg_one = ms.to_vec();
        neg_one[0] -= 1;
        let pk1 = p.pow(u32::exact_from(exp - 1));
        let d = usize::exact_from((p - 1) * pk1);
        // The accumulators are less than d n^2 < 2^(2 bits(n) + bits(d)).
        let n_bits = limbs_significant_bits(ms);
        let sl = usize::exact_from((n_bits << 1) + d.significant_bits()).div_ceil(LIMB_WIDTH_USIZE);
        let t = (u64::exact_from(sl) << Limb::LOG_WIDTH) - n_bits + 1;
        let pad_offset = usize::exact_from(t >> Limb::LOG_WIDTH);
        let mut pad = ms.to_vec();
        let pad_shift = t & Limb::WIDTH_MASK;
        if pad_shift != 0 {
            let carry = limbs_slice_shl_in_place(&mut pad, pad_shift);
            pad.push(carry);
        }
        Self {
            ms,
            ms_norm,
            shift,
            d_inv,
            neg_one,
            pad,
            pad_offset,
            p,
            pk: pk1 * p,
            pk1,
            d,
            nl,
            sl,
            c: vec![0; ((d << 1) - 1) * sl],
            prod: vec![0; nl << 1],
            mul_scratch: vec![
                0;
                max(
                    limbs_mul_same_length_to_out_scratch_len(nl),
                    limbs_square_to_out_scratch_len(nl)
                )
            ],
            u: vec![0; sl + 1],
            un: vec![0; sl + 2],
            qs: vec![0; sl + 2 - nl],
        }
    }

    fn zero(&self) -> Vec<Limb> {
        vec![0; self.d * self.nl]
    }

    fn one(&self) -> Vec<Limb> {
        let mut f = self.zero();
        f[0] = 1;
        f
    }

    // Converts a vector of small integer coefficients, already reduced mod Phi_{p^k}. Each
    // coefficient is less than n in absolute value.
    fn element_from_small(&self, cs: &[i64]) -> Vec<Limb> {
        let nl = self.nl;
        let mut f = self.zero();
        for (fc, &c) in f.chunks_exact_mut(nl).zip(cs) {
            let a = Limb::exact_from(c.unsigned_abs());
            if c >= 0 {
                fc[0] = a;
            } else {
                fc.copy_from_slice(self.ms);
                limbs_sub_limb_in_place(fc, a);
            }
        }
        f
    }

    // Reduces the accumulator u mod n, writing the result to out.
    fn reduce_u(&mut self, out: &mut [Limb]) {
        let len = self.u.len();
        self.un[len] = if self.shift == 0 {
            self.un[..len].copy_from_slice(&self.u);
            0
        } else {
            limbs_shl_to_out(&mut self.un, &self.u, self.shift)
        };
        let nl = self.nl;
        if nl == 2 {
            limbs_div_mod_by_two_limb_normalized_preinverted(
                &mut self.qs,
                &mut self.un,
                &self.ms_norm,
                self.d_inv,
            );
        } else {
            limbs_div_mod_schoolbook(&mut self.qs, &mut self.un, &self.ms_norm, self.d_inv);
        }
        if self.shift == 0 {
            out.copy_from_slice(&self.un[..nl]);
        } else {
            limbs_shr_to_out(out, &self.un[..nl], self.shift);
        }
    }

    // Applies the cyclotomic reduction f_i = c_i + c_{i + p^k} - c_{d + (i mod p^(k - 1))} to the
    // unreduced product coefficients and reduces each result mod n, writing it to out. To keep the
    // accumulator nonnegative, the padding constant n 2^t >= B^sl > c_e is added before subtracting
    // c_e; the padded accumulator is less than 4 B^sl, so it fits in sl + 1 limbs.
    //
    // This is _unity_zp_mpn_cyclotomic_reduce from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn cyclotomic_reduce(&mut self, out: &mut [Limb]) {
        let (d, nl, sl) = (self.d, self.nl, self.sl);
        let last = (d << 1) - 2;
        let pk = usize::exact_from(self.pk);
        let pk1 = usize::exact_from(self.pk1);
        for i in 0..d {
            self.u[..sl].copy_from_slice(&self.c[i * sl..(i + 1) * sl]);
            self.u[sl] = 0;
            if i + pk <= last {
                let j = i + pk;
                assert!(!limbs_slice_add_greater_in_place_left(
                    &mut self.u,
                    &self.c[j * sl..(j + 1) * sl]
                ));
            }
            let e = d + i % pk1;
            if e <= last {
                assert!(!limbs_slice_add_greater_in_place_left(
                    &mut self.u[self.pad_offset..],
                    &self.pad
                ));
                assert!(!limbs_sub_greater_in_place_left(
                    &mut self.u,
                    &self.c[e * sl..(e + 1) * sl]
                ));
            }
            self.reduce_u(&mut out[i * nl..(i + 1) * nl]);
        }
    }

    // out = g * h.
    //
    // This is _unity_zp_mpn_mul from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev, with the classical
    // product.
    fn mul_to(&mut self, out: &mut [Limb], g: &[Limb], h: &[Limb]) {
        let (nl, sl) = (self.nl, self.sl);
        // a product is less than n^2 < B^sl, so only its low min(sl, 2 nl) limbs can be nonzero
        let pl = min(sl, nl << 1);
        self.c.fill(0);
        for (i, gi) in g.chunks_exact(nl).enumerate() {
            if slice_test_zero(gi) {
                continue;
            }
            for (j, hj) in h.chunks_exact(nl).enumerate() {
                if slice_test_zero(hj) {
                    continue;
                }
                limbs_mul_same_length_to_out(&mut self.prod, gi, hj, &mut self.mul_scratch);
                let k = i + j;
                assert!(!limbs_slice_add_greater_in_place_left(
                    &mut self.c[k * sl..(k + 1) * sl],
                    &self.prod[..pl]
                ));
            }
        }
        self.cyclotomic_reduce(out);
    }

    // out = g^2. The cross products g_i g_j with i < j are accumulated once and doubled, and then
    // the squares g_i^2 are added; every accumulator stays below d B^(2 nl) < B^sl.
    fn sqr_to(&mut self, out: &mut [Limb], g: &[Limb]) {
        let (nl, sl) = (self.nl, self.sl);
        let pl = min(sl, nl << 1);
        self.c.fill(0);
        for (i, gi) in g.chunks_exact(nl).enumerate() {
            if slice_test_zero(gi) {
                continue;
            }
            for (j, gj) in g.chunks_exact(nl).enumerate().skip(i + 1) {
                if slice_test_zero(gj) {
                    continue;
                }
                limbs_mul_same_length_to_out(&mut self.prod, gi, gj, &mut self.mul_scratch);
                let k = i + j;
                assert!(!limbs_slice_add_greater_in_place_left(
                    &mut self.c[k * sl..(k + 1) * sl],
                    &self.prod[..pl]
                ));
            }
        }
        // Each accumulator is below B^sl / 2, so doubling the whole vector moves no bits between
        // accumulators.
        assert_eq!(limbs_slice_shl_in_place(&mut self.c, 1), 0);
        for (i, gi) in g.chunks_exact(nl).enumerate() {
            if slice_test_zero(gi) {
                continue;
            }
            limbs_square_to_out(&mut self.prod, gi, &mut self.mul_scratch);
            let k = i << 1;
            assert!(!limbs_slice_add_greater_in_place_left(
                &mut self.c[k * sl..(k + 1) * sl],
                &self.prod[..pl]
            ));
        }
        self.cyclotomic_reduce(out);
    }

    fn mul(&mut self, g: &[Limb], h: &[Limb]) -> Vec<Limb> {
        let mut out = self.zero();
        self.mul_to(&mut out, g, h);
        out
    }

    fn sqr(&mut self, g: &[Limb]) -> Vec<Limb> {
        let mut out = self.zero();
        self.sqr_to(&mut out, g);
        out
    }

    // f *= h, with t as scratch.
    fn mul_assign(&mut self, f: &mut Vec<Limb>, h: &[Limb], t: &mut Vec<Limb>) {
        self.mul_to(t, f, h);
        swap(f, t);
    }

    // f = f^2, with t as scratch.
    fn sqr_assign(&mut self, f: &mut Vec<Limb>, t: &mut Vec<Limb>) {
        self.sqr_to(t, f);
        swap(f, t);
    }

    // This is unity_zp_mpn_mul_scalar_ui from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn mul_scalar(&mut self, g: &[Limb], s: Limb) -> Vec<Limb> {
        let nl = self.nl;
        let mut f = self.zero();
        for (fc, gc) in f.chunks_exact_mut(nl).zip(g.chunks_exact(nl)) {
            self.u.fill(0);
            self.u[nl] = limbs_mul_limb_to_out::<DoubleLimb, Limb>(&mut self.u, gc, s);
            self.reduce_u(fc);
        }
        f
    }

    // Sliding window powering, f = g^pow.
    //
    // This is unity_zp_mpn_pow_sliding_fmpz from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn pow_sliding(&mut self, g: &[Limb], pow: &Natural) -> Vec<Limb> {
        let k = pow_select_k(pow.significant_bits());
        let m = usize::power_of_2(k - 1);
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

    // Computes f such that sigma_x(f) = g.
    //
    // This is unity_zp_mpn_aut_inv from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn aut_inv(&self, g: &[Limb], x: u64) -> Vec<Limb> {
        let nl = self.nl;
        let d = u64::exact_from(self.d);
        let mut f = self.zero();
        for i in 0..d {
            let g_ind = x.mod_mul(i, self.pk);
            if g_ind < d {
                let (i, g_ind) = (usize::exact_from(i), usize::exact_from(g_ind));
                f[i * nl..(i + 1) * nl].copy_from_slice(&g[g_ind * nl..(g_ind + 1) * nl]);
            }
        }
        for i in d..self.pk {
            let g_ind = x.mod_mul(i, self.pk);
            if g_ind >= d {
                continue;
            }
            let g_ind = usize::exact_from(g_ind);
            let c = &g[g_ind * nl..(g_ind + 1) * nl];
            for j in 1..self.p {
                let f_ind = usize::exact_from(i - j * self.pk1);
                sub_mod_in_place(&mut f[f_ind * nl..(f_ind + 1) * nl], c, self.ms);
            }
        }
        f
    }

    // If f = zeta_{p^k}^h for some h, returns h; otherwise returns -1.
    //
    // This is unity_zp_mpn_is_unity from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
    fn is_unity(&self, f: &[Limb]) -> i64 {
        let nl = self.nl;
        let mut ones = 0;
        let mut neg_ones = 0u64;
        let mut one_ind = 0;
        let mut neg_one_ind = None;
        for (i, c) in f.chunks_exact(nl).enumerate() {
            if slice_test_zero(c) {
                continue;
            }
            if c[0] == 1 && slice_test_zero(&c[1..]) {
                ones += 1;
                one_ind = i;
            } else if *c == *self.neg_one {
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
                    "natural_is_prime_aprcl, is_unity, first -1 not in the first block",
                );
                return -1;
            }
            for j in 0..usize::exact_from(self.p - 1) {
                let k = e0 + j * pk1;
                if f[k * nl..(k + 1) * nl] != *self.neg_one {
                    fail_on_untested_path(
                        "natural_is_prime_aprcl, is_unity, -1s not evenly spaced",
                    );
                    return -1;
                }
            }
            return i64::exact_from(self.d + e0);
        }
        fail_on_untested_path("natural_is_prime_aprcl, is_unity, coefficients in {0, 1, -1}");
        -1
    }
}

// Multiplies the factors sigma_i^{-1}(x^i) into j0 and sigma_i^{-1}(x^{(v * i) / r}) into jv, where
// xi = x^i and xe = x^e with e = (v * i) / r.
fn mpn_mul_aut_terms(
    ring: &mut MpnRing,
    j0: &mut Vec<Limb>,
    jv: &mut Vec<Limb>,
    xi: &[Limb],
    xe: &[Limb],
    i: u64,
    t: &mut Vec<Limb>,
) {
    let aut = ring.aut_inv(xi, i);
    ring.mul_assign(j0, &aut, t);
    let aut = ring.aut_inv(xe, i);
    ring.mul_assign(jv, &aut, t);
}

// This is _aprcl_is_prime_jacobi_check_pk_mpn from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev, with the
// powers j^i and j^{(v * i) / r} maintained incrementally, as in FLINT's
// _aprcl_is_prime_jacobi_check_pk.
fn check_pk_mpn(ring: &mut MpnRing, j: &[Limb], u: &Natural, v: u64) -> i64 {
    let r = ring.pk;
    let mut j0 = ring.one();
    let mut jv = ring.one();
    let mut t = ring.zero();
    let mut ji = ring.one();
    let mut je = ring.one();
    let mut e = 0;
    for i in 1..=r {
        ring.mul_assign(&mut ji, j, &mut t);
        while e < v * i / r {
            ring.mul_assign(&mut je, j, &mut t);
            e += 1;
        }
        if i.divisible_by(ring.p) {
            continue;
        }
        mpn_mul_aut_terms(ring, &mut j0, &mut jv, &ji, &je, i, &mut t);
    }
    // j0 = j0^u * jv
    let j0 = ring.pow_sliding(&j0, u);
    ring.mul_to(&mut t, &jv, &j0);
    ring.is_unity(&t)
}

// This is _aprcl_is_prime_jacobi_check_22_mpn from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev.
fn check_22_mpn(ring: &mut MpnRing, j: &[Limb], u: &Natural, v: u64, q: u64) -> i64 {
    // j0 = q * J^2
    let j_pow = ring.sqr(j);
    let j0 = ring.mul_scalar(&j_pow, Limb::exact_from(q));
    // jv = 1 if v == 1; jv = J^2 if v == 3 (v is odd since n is)
    let jv = if v == 1 { ring.one() } else { j_pow };
    // j0 = j0^u * jv
    let t = ring.pow_sliding(&j0, u);
    let j0 = ring.mul(&jv, &t);
    ring.is_unity(&j0)
}

// This is _aprcl_is_prime_jacobi_check_2k_mpn from aprcl/unity_zp_mpn.c, FLINT 3.7.0-dev, with the
// powers of j_j0 maintained incrementally, as in FLINT's _aprcl_is_prime_jacobi_check_2k.
fn check_2k_mpn(
    ring: &mut MpnRing,
    j: &[Limb],
    j2_1: &[Limb],
    j2_2: &[Limb],
    u: &Natural,
    v: u64,
) -> i64 {
    let r = ring.pk;
    let mut j0 = ring.one();
    let mut jv = ring.one();
    let mut t = ring.zero();
    // j_j0 = J(2, q) * J_3(q)
    let j_j0 = ring.mul(j, j2_1);
    // The loop runs over i = 1, 3, 9, 11, 17, 19, ... (i = 1 or 3 mod 8), with ji = j_j0^i and je =
    // j_j0^{(v * i) / r}.
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
        mpn_mul_aut_terms(ring, &mut j0, &mut jv, &ji, &je, i, &mut t);
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

// The Jacobi sums of a check, as small integer coefficient vectors.
enum Sums {
    Pk(Vec<i64>),
    Two2(Vec<i64>),
    Two3(Vec<i64>, Vec<i64>, Vec<i64>),
}

// Runs a check. FLINT uses its mpn_mod-based arithmetic when n has at most 16 limbs, and its
// fmpz_mod-based arithmetic otherwise; here the limb arithmetic is used for every n.
//
// This is the dispatch in _aprcl_is_prime_jacobi_check_pk, _aprcl_is_prime_jacobi_check_22, and
// _aprcl_is_prime_jacobi_check_2k from aprcl/is_prime_jacobi.c, FLINT 3.7.0-dev.
fn check(n: &Natural, p: u64, k: u64, sums: &Sums, u: &Natural, v: u64, q: u64) -> i64 {
    let mut ring = MpnRing::new(n.as_limbs_asc(), p, k);
    match sums {
        Sums::Pk(j) => {
            let j = ring.element_from_small(j);
            check_pk_mpn(&mut ring, &j, u, v)
        }
        Sums::Two2(j) => {
            let j = ring.element_from_small(j);
            check_22_mpn(&mut ring, &j, u, v, q)
        }
        Sums::Two3(j, j2_1, j2_2) => {
            let j = ring.element_from_small(j);
            let j2_1 = ring.element_from_small(j2_1);
            let j2_2 = ring.element_from_small(j2_2);
            check_2k_mpn(&mut ring, &j, &j2_1, &j2_2, u, v)
        }
    }
}

// This is _aprcl_is_prime_jacobi_check_21 from aprcl/is_prime_jacobi.c, FLINT 3.7.0-dev.
fn check_21(n: &Natural, q: u64) -> i64 {
    let n_dec = n - Natural::ONE;
    let q_pow = (n - Natural::from(q)).mod_pow(&n_dec >> 1u32, n);
    if q_pow == 1u32 {
        0
    } else if q_pow == n_dec {
        1
    } else {
        fail_on_untested_path("natural_is_prime_aprcl, check_21, not +-1");
        -1
    }
}

// This is aprcl_R_value from aprcl/config_jacobi.c, FLINT 3.7.0-dev, where FLINT64 is defined.
const fn r_value(bits: u64) -> u64 {
    match bits {
        0..=17 => 6,
        18..=31 => 12,
        32..=54 => 36,
        55..=68 => 72,
        69..=101 => 180,
        102..=127 => 360,
        128..=152 => 720,
        153..=204 => 1260,
        205..=268 => 2520,
        269..=344 => 5040,
        345..=525 => 27720,
        526..=650 => 55440,
        651..=774 => 110880,
        775..=1566 => 720720,
        1567..=1999 => 1441440,
        2000..=2096 => 1663200,
        2097..=2165 => 1965600,
        2166..=2321 => 2162160,
        2322..=2377 => 2827440,
        2378..=2514 => 3326400,
        2515..=2588 => 3341520,
        2589..=2636 => 3603600,
        2637..=3028 => 4324320,
        3029..=3045 => 5654880,
        3046..=3080 => 6652800,
        3081..=3121 => 6683040,
        3122..=3283 => 7207200,
        3284..=3491 => 8648640,
        3492..=3726 => 10810800,
        3727..=3818 => 12972960,
        3819..=3977 => 14414400,
        3978..=4762 => 21621600,
        4763..=5068 => 36756720,
        5069..=5658 => 43243200,
        5659..=5960 => 64864800,
        5961..=6423 => 73513440,
        6424..=6900 => 122522400,
        6901..=9977 => 367567200,
        9978..=12713 => 1396755360,
        _ => 6983776800,
    }
}

struct Config {
    r: u64,
    s: Natural,
    rs: Vec<(u64, u64)>,
    qs: Vec<(u64, u64)>,
    qs_used: Vec<bool>,
}

impl Config {
    // This is aprcl_config_jacobi_init_R, _aprcl_config_jacobi_update, and
    // _aprcl_config_jacobi_reduce_s2 from aprcl/config_jacobi.c, FLINT 3.7.0-dev. FLINT checks that
    // s^2 > n p^2 before removing p = q^k from s; since p | s, that is (s / p)^2 > n.
    fn with_r(n: &Natural, r: u64) -> Self {
        let rs: Vec<(u64, u64)> = r
            .factor()
            .into_iter()
            .map(|(p, e)| (p, u64::from(e)))
            .collect();
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
        let mut s = Natural::ONE;
        let mut qs = Vec::new();
        for d in divisors {
            let q = d + 1;
            if q.is_prime() {
                let k = p_power_in_q(r, q) + 1 + u64::from(q == 2);
                qs.push((q, k));
                s *= Natural::from(q).pow(k);
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
        config
    }

    fn reduce_s2(&mut self, n: &Natural) {
        let w: Vec<f64> = self
            .qs
            .iter()
            .map(|&(q, k)| {
                let mut w = 0.0;
                for (p, e) in (q - 1).factor() {
                    let euler_phi = p.pow(u32::from(e - 1)) * (p - 1);
                    w += f64_from_u64(euler_phi).powf(JACOBI_COST_EXPONENT);
                }
                w /= ln_f64(f64_from_u64(q).powf(f64_from_u64(k)));
                if q > MAX_Q {
                    w += 1e6 * (f64_from_u64(q) / f64_from_u64(MAX_Q));
                }
                w
            })
            .collect();
        loop {
            let mut w_max = -1.0;
            let mut ind = None;
            for (i, &(q, k)) in self.qs.iter().enumerate() {
                if self.qs_used[i]
                    && (&self.s / Natural::from(q).pow(k)).square() > *n
                    && w_max <= w[i]
                {
                    w_max = w[i];
                    ind = Some(i);
                }
            }
            let Some(ind) = ind else {
                break;
            };
            let (q, k) = self.qs[ind];
            self.s /= Natural::from(q).pow(k);
            self.qs_used[ind] = false;
        }
    }

    // The tabulated values of R give s^2 > n for n of up to about 20000 bits; for larger n, R is
    // multiplied by further primes until s^2 > n.
    //
    // This is aprcl_config_jacobi_init from aprcl/config_jacobi.c, FLINT 3.7.0-dev.
    fn new(n: &Natural) -> Self {
        let mut r = r_value(n.significant_bits());
        let mut config = Self::with_r(n, r);
        while (&config.s).square() <= *n {
            let mut p = config.rs.last().unwrap().0 + 1;
            while !p.is_prime() {
                p += 1;
            }
            r = r.checked_mul(p).expect("Config::new: n is too large");
            config = Self::with_r(n, r);
        }
        config
    }
}

fn f64_from_u64(x: u64) -> f64 {
    f64::rounding_from(x, Nearest).0
}

// This is _aprcl_is_prime_jacobi_additional_test from aprcl/is_prime_jacobi.c, FLINT 3.7.0-dev.
fn additional_test(n: &Natural, p: u64) -> u8 {
    let mut p_counter = 50;
    let mut m = 9;
    let mut q = 0;
    while p_counter > 0 {
        q = ((m * p) << 1) + 1;
        if q.is_prime() && mod_small(n, q) != 0 {
            if mod_small(n, q).mod_pow((q - 1) / p, q) != 1 {
                break;
            }
            p_counter -= 1;
        }
        m += 2;
    }
    if p_counter != 0 {
        if mod_small(n, q) == 0 && *n != q {
            fail_on_untested_path("natural_is_prime_aprcl, additional_test, q | n");
            2
        } else {
            let k = p_power_in_q(q - 1, p);
            let (u, v) = n.div_mod(Natural::from(p.pow(u32::exact_from(k))));
            let v = u64::exact_from(&v);
            if p == 2 {
                let h = check(n, p, k, &Sums::Two2(jacobi_sum_pq(q, p)), &u, v, q);
                if h < 0 || h.even() {
                    fail_on_untested_path(
                        "natural_is_prime_aprcl, additional_test, p = 2, h not found or even",
                    );
                    2
                } else {
                    let n_dec = n - Natural::ONE;
                    let q_pow = Natural::from(q).mod_pow(&n_dec >> 1u32, n);
                    if q_pow == n_dec {
                        1
                    } else {
                        fail_on_untested_path(
                            "natural_is_prime_aprcl, additional_test, p = 2, q^((n - 1) / 2) != -1",
                        );
                        2
                    }
                }
            } else {
                let h = check(n, p, k, &Sums::Pk(jacobi_sum_pq(q, p)), &u, v, q);
                if h < 0 || h.divisible_by(i64::exact_from(p)) {
                    fail_on_untested_path(
                        "natural_is_prime_aprcl, additional_test, p odd, h not found or p | h",
                    );
                    2
                } else {
                    1
                }
            }
        }
    } else if mod_small(n, p) == 0 || n.is_power() {
        fail_on_untested_path(
            "natural_is_prime_aprcl, additional_test, no q found, p | n or n is a perfect power",
        );
        2
    } else {
        0
    }
}

struct JacobiTask {
    q: u64,
    p: u64,
    k: u64,
    p_ind: usize,
    h: i64,
    q_pow_is_minus_1: bool,
}

// This is _aprcl_jacobi_run_task from aprcl/is_prime_jacobi.c, FLINT 3.7.0-dev.
fn run_task(n: &Natural, t: &mut JacobiTask) {
    let (p, k, q) = (t.p, t.k, t.q);
    if p == 2 && k == 1 {
        t.h = check_21(n, q);
        return;
    }
    let (u, v) = n.div_mod(Natural::from(p.pow(u32::exact_from(k))));
    let v = u64::exact_from(&v);
    t.h = if p == 2 {
        let n_dec = n - Natural::ONE;
        let q_pow = Natural::from(q).mod_pow(&n_dec >> 1u32, n);
        t.q_pow_is_minus_1 = q_pow == n_dec;
        let sums = if k == 2 {
            Sums::Two2(jacobi_sum_pq(q, p))
        } else {
            Sums::Two3(
                jacobi_sum_pq(q, p),
                jacobi_sum_2q_one(q),
                jacobi_sum_2q_two(q),
            )
        };
        check(n, p, k, &sums, &u, v, q)
    } else {
        check(n, p, k, &Sums::Pk(jacobi_sum_pq(q, p)), &u, v, q)
    };
}

// This is aprcl_is_prime_final_division from aprcl/is_prime_final_division.c, FLINT 3.7.0-dev, as
// its reference walk on one thread.
//
// Since n >= 2^128, FLINT's check for n <= 3 is not needed; since n is odd, only odd residues are
// tested; and since s^2 > n, every residue up to sqrt(n) is below s.
fn final_division(n: &Natural, s: &Natural, r: u64) -> bool {
    let bound = n.floor_sqrt();
    let n_mul = n % s;
    let mut n_pow = n_mul.clone();
    for _ in 1..r {
        if n_pow == 1u32 {
            break;
        }
        if n_pow.odd() && n_pow <= bound && n.divisible_by(&n_pow) {
            fail_on_untested_path("natural_is_prime_aprcl, final_division, divisor found");
            return false;
        }
        n_pow.mod_mul_assign(&n_mul, s);
    }
    true
}

// The APR-CL test with Jacobi sums, for an odd n >= 2^64. Returns whether n is prime, panicking in
// the unlikely case that the test cannot decide.
//
// This is aprcl_is_prime_jacobi and _aprcl_is_prime_jacobi from aprcl/is_prime_jacobi.c, FLINT
// 3.7.0-dev, run on one thread.
crate_test_fn! {natural_is_prime_aprcl(n: &Natural) -> bool {
    // FLINT first checks whether n is one of the primes q, but they are less than 2^64.
    let config = Config::new(n);
    // Check that s * R and n are coprime; if not, n is composite.
    let r = Natural::from(config.r);
    if !(n % &r).coprime_with(&r) || !(n % &config.s).coprime_with(&config.s) {
        return false;
    }
    let mut lambdas: Vec<bool> = config
        .rs
        .iter()
        .map(|&(p, _)| {
            let p2 = p * p;
            p > 2 && mod_small(n, p2).mod_pow(p - 1, p2) != 1
        })
        .collect();
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
    tasks.sort_by(|x, y| {
        let px = x.p.pow(u32::exact_from(x.k - 1)) * (x.p - 1);
        let py = y.p.pow(u32::exact_from(y.k - 1)) * (y.p - 1);
        py.cmp(&px).then(y.q.cmp(&x.q))
    });
    for t in &mut tasks {
        run_task(n, t);
        if t.h < 0 {
            return false;
        }
    }
    let n_mod_4 = mod_small(n, 4);
    for t in &tasks {
        let (p, k, h) = (t.p, t.k, t.h);
        if lambdas[t.p_ind] {
            continue;
        }
        lambdas[t.p_ind] = if p == 2 && k == 1 {
            h == 1 && n_mod_4 == 1
        } else if p == 2 {
            h.odd() && t.q_pow_is_minus_1
        } else {
            !h.divisible_by(i64::exact_from(p))
        };
    }
    let mut unknown = false;
    for (i, &(p, _)) in config.rs.iter().enumerate() {
        if !lambdas[i] {
            match additional_test(n, p) {
                2 => return false,
                1 => lambdas[i] = true,
                _ => unknown = true,
            }
        }
    }
    assert!(!unknown, "natural_is_prime_aprcl: failed to prove n prime for n = {n}");
    final_division(n, &config.s, config.r)
}}
