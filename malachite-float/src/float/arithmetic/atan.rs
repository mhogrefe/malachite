// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's arctangent (`mpfr_atan`, `atan.c`). An input above 1 in magnitude is inverted and
// its arctangent taken from pi/2. The argument is then reduced by atan(x) = 2 atan((sqrt(1 + x^2)
// - 1)/x) until it is below about 1/sqrt(prec), and what remains is split into binary chunks, each
// of whose arctangent is summed by binary splitting of the series for atan(x)/x (with a table of
// the twenty most common small chunks for low precisions), the pieces being combined through
// atan(a) + atan(b) = atan((a + b)/(1 - ab)). The arctangent is bounded, so it never overflows; it
// underflows only for an input at the very bottom of the exponent range, where it is the input
// itself rounded toward zero, which MPFR's small-input shortcut decides directly.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::float::arithmetic::tan::round_bracket_signed_by;
use crate::{Float, emulate_float_to_float_fn};
use alloc::vec;
use core::cmp::Ordering::{self, Equal, Greater};
use core::cmp::min;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    Abs, Atan, AtanAssign, CeilingLogBase2, Parity, PowerOf2, Square, SquareAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Down, Exact, Floor, Nearest};
use malachite_base::split_into_chunks_mut;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// For each pair (r, p), a 192-bit truncation of atan(x)/x for x = p/2^r, as three 64-bit words with
// the lowest first; these are the chunks that a precision of at most 192 bits meets most often, and
// that are the most expensive to sum. MPFR only compiles this table for 64-bit limbs; the values do
// not depend on the limb width, so it is used unconditionally here.
//
// In Sage: for p in range(1, 2^ceil(r/2)), with x = p/2^r, the words are floor(2^192*n(atan(x)/x,
// 300)).digits(2^64).
const ATAN_TABLE: [[u64; 3]; 20] = [
    [0x6e141587261cdf00, 0x6fe445ecbc3a8d03, 0xed63382b0dda7b45], // (1,1)
    [0xaa7fa90388b3836b, 0x6dc79ef5f7a217e5, 0xfadbafc96406eb15], // (2,1)
    [0x319c12cf59d4b2dc, 0xcb2792dc0e2e0d51, 0xffaaddb967ef4e36], // (4,1)
    [0x8b3957d95d9ad922, 0xc897989f3e888ef7, 0xfeadd4d5617b6e32], // (4,2)
    [0xc4e6abc8af62e439, 0x4eb9bf602625f0b4, 0xfd0fcdd343cac19b], // (4,3)
    [0x7c18baeb9bc95789, 0xb12afb6b6d4f7e16, 0xffffaaaaddddb94b], // (8,1)
    [0x6856a0171a2f001a, 0x62351fbbe60af47, 0xfffeaaadddd4b968],  // (8,2)
    [0x69164c094f49da06, 0xd517294f7373d07a, 0xfffd001032cb1179], // (8,3)
    [0x20ef65c10deef460, 0xe78c564015f76048, 0xfffaaadddb94d5bb], // (8,4)
    [0x3ce233aa002f0344, 0x9dd8ea342a65d4cc, 0xfff7ab27a1f32f95], // (8,5)
    [0xa37f403c7279c5cb, 0x13ab53a1c8db8497, 0xfff40103192ce74d], // (8,6)
    [0xe5a85657103c1aa8, 0xb8409e6c914191d3, 0xffefac8a9c40a26b], // (8,7)
    [0x806d0294c0db8816, 0x779d776dda8c6213, 0xffeaaddd4bb12542], // (8,8)
    [0x5545d1914ef21478, 0x3aea58d6660f5a12, 0xffe5051f0aebf73a], // (8,9)
    [0x6e47a91d015f4133, 0xc085ab6b490b7f02, 0xffdeb2787d4adac1], // (8,10)
    [0x4efc1f931f7ec9b3, 0xb7f43cd16195ef4b, 0xffd7b61702b09aad], // (8,11)
    [0xd27d1dbf55fed60d, 0xd812c11d7d473e5e, 0xffd0102cb3c1bfbe], // (8,12)
    [0xca629e927383fe97, 0x8c61aedf58e42206, 0xffc7c0f05db9d1b6], // (8,13)
    [0x4eff0b53d4e905b7, 0x28ac1e800ca31e9d, 0xffbec89d7dddd7e9], // (8,14)
    [0xb0a7931deec6fe60, 0xb46feea78588554b, 0xffb527743c8cdd8f], // (8,15)
];

// A table entry, in [1/2, 1), truncated to `prec` bits.
//
// This is `set_table` from atan.c, MPFR 4.2.2.
fn set_table(prec: u64, x: &[u64; 3]) -> Float {
    let n = (Natural::from(x[2]) << 128u32) | (Natural::from(x[1]) << 64u32) | Natural::from(x[0]);
    Float::from_rational_prec_round(Rational::from(n) >> 192u32, prec, Down).0
}

// Multiplies `xs[k - 1]` by `xs[k]` in place.
fn mul_prev(xs: &mut [Integer], k: usize) {
    let (lo, hi) = xs.split_at_mut(k);
    lo[k - 1] *= &hi[0];
}

// If x = p/2^r, computes an approximation to atan(x)/x using 2^m terms of the series, with an error
// of at most 1 ulp at precision `precy`. Assumes 0 < x < 1, so 1 <= p < 2^r. More precisely, p
// consists of the floor(r/2) bits of the binary expansion of a number 0 < s < 1: the bit of weight
// 2^-1 is for r = 1, so p <= 1; the bit of weight 2^-2 is for r = 2, so p <= 1; the two bits of
// weight 2^-3 and 2^-4 are for r = 4, so p <= 3; and in general p < 2^(r/2).
//
// With X = x^2 = p/2^r the series is 1 - X/3 + X^2/5 - ... + (-1)^k X^k/(2k+1) + ..., summed by
// binary splitting: P(a,b) = p if a+1 = b, else P(a,c) P(c,b); Q(a,b) = (2a+1) 2^r if a+1 = b
// (except Q(0,1) = 1), else Q(a,c) Q(c,b); S(a,b) = p (2a+1) if a+1 = b, else Q(c,b) S(a,c) +
// Q(a,c) P(a,c) S(c,b). Then atan(x)/x ~ S(0,i)/Q(0,i) for an i making (p/2^r)^i/i small enough.
// The factor 2^(r(b-a)) in Q(a,b) is implicit, and is applied when Q is used.
//
// This is `mpfr_atan_aux` from atan.c, MPFR 4.2.2.
fn atan_aux(mut p: Integer, mut r: u64, m: usize, precy: u64) -> Float {
    debug_assert!(p > 0u32);
    debug_assert!(m > 0);
    // tabulate values for small precision and small r, which are the most expensive to compute
    if precy <= 192 {
        let index = match r {
            // p has 1 bit: necessarily p = 1
            1 => Some(0),
            2 => Some(1),
            // p has at most 2 bits: 1 <= p <= 3
            4 => Some(1 + usize::exact_from(&p)),
            // p has at most 4 bits: 1 <= p <= 15
            8 => Some(4 + usize::exact_from(&p)),
            _ => None,
        };
        if let Some(index) = index {
            return set_table(precy, &ATAN_TABLE[index]);
        }
    }
    // From p to p^2, and r to 2r
    p.square_assign();
    r <<= 1;
    // Normalize p
    let n = p.trailing_zeros().unwrap();
    if n > 0 {
        p >>= n;
        r -= n;
    }
    // Since |p/2^r| < 1, and p is a nonzero integer, necessarily r > 0.
    debug_assert!(r > 0);
    // MPFR lays the three tables out in one array of 3(m+1) entries; the p = 1 loop can run one
    // step past 2^m terms, so S and Q get a spare slot each here
    let len = m + 2;
    let mut scratch = vec![Integer::ZERO; (len << 1) + m + 1];
    // ptoj[j] = p^(2^j)
    split_into_chunks_mut!(scratch, len, [s, q], ptoj);
    let mut log2_nb_terms = vec![0u64; len + 1];
    // accu[k] = mult[0] + ... + mult[k], where mult[j] is the number of bits of the corresponding
    // term S[j]/Q[j]
    let mut accu = vec![0i64; len + 1];
    let ri = i64::exact_from(r);
    let mut i = 0u64;
    let mut k = 0usize;
    let p_is_1 = p == 1u32;
    if p_is_1 {
        // special case p = 1: the i-th term being X^i/(2i+1) with X = 1/2^r, we can stop when r i >
        // precy, i.e. i > precy/r
        let mut n = u64::power_of_2(u64::exact_from(m));
        if precy / r <= n {
            n = precy / r + 1;
        }
        while i < n {
            q[k + 1] = Integer::from((i << 1) + 3);
            s[k] = (&q[k + 1] << r) - Integer::from((i << 1) + 1);
            q[k] = &q[k + 1] * Integer::from((i << 1) + 1);
            log2_nb_terms[k] = 1; // S[k]/Q[k] corresponds to 2 terms
            let mut j = (i + 2) >> 1;
            let mut l = 1u64;
            while j.even() {
                debug_assert!(k > 0);
                s[k] *= &q[k - 1];
                let mut t = &s[k - 1] * &q[k];
                t <<= r << l;
                t += &s[k];
                s[k - 1] = t;
                mul_prev(q, k);
                log2_nb_terms[k - 1] = l + 1;
                l += 1;
                j >>= 1;
                k -= 1;
            }
            i += 2;
            k += 1;
        }
    } else {
        // p != 1: precompute the ptoj table
        ptoj[0] = p.clone();
        for im in 1..=m {
            ptoj[im] = (&ptoj[im - 1]).square();
        }
        // main loop: the i-th term being X^i/(2i+1) with X = p/2^r, we can stop when p^i/2^(ri) <
        // 2^-precy, i.e. r i > precy + log2(p^i)
        let n = u64::power_of_2(u64::exact_from(m));
        let mut done = false;
        while i < n && !done {
            // initialize both S[k], Q[k] and S[k+1], Q[k+1]
            q[k + 1] = Integer::from((i << 1) + 3); // Q(i+1,i+2)
            s[k + 1] = &p * Integer::from((i << 1) + 1); // S(i+1,i+2)
            s[k] = (&q[k + 1] << r) - &s[k + 1]; // S(i,i+2)
            q[k] = &q[k + 1] * Integer::from((i << 1) + 1); // Q(i,i+2)
            log2_nb_terms[k] = 1; // S[k]/Q[k] corresponds to 2 terms
            let mut j = (i + 2) >> 1;
            let mut l = 1u64;
            while j.even() {
                // invariant: S[k-1]/Q[k-1] and S[k]/Q[k] correspond to 2^l terms each; combine them
                // into S[k-1]/Q[k-1]
                debug_assert!(k > 0);
                s[k] *= &q[k - 1];
                s[k] *= &ptoj[usize::exact_from(l)];
                let mut t = &s[k - 1] * &q[k];
                t <<= r << l;
                t += &s[k];
                s[k - 1] = t;
                mul_prev(q, k);
                log2_nb_terms[k - 1] = l + 1;
                // now S[k-1]/Q[k-1] corresponds to 2^(l+1) terms
                let mult = (ri << (l + 1))
                    - i64::exact_from(ptoj[usize::exact_from(l + 1)].significant_bits())
                    - 1;
                accu[k - 1] = if k == 1 { mult } else { accu[k - 2] + mult };
                if accu[k - 1] > i64::exact_from(precy) {
                    done = true;
                }
                l += 1;
                j >>= 1;
                k -= 1;
            }
            i += 2;
            k += 1;
        }
    }
    // we need to combine S[0]/Q[0] ... S[k-1]/Q[k-1]
    let mut h = 0u64; // number of terms accumulated in S[k]/Q[k]
    while k > 1 {
        k -= 1;
        // combine S[k-1]/Q[k-1] and S[k]/Q[k]
        s[k] *= &q[k - 1];
        if !p_is_1 {
            s[k] *= &ptoj[usize::exact_from(log2_nb_terms[k - 1])];
        }
        let mut t = &s[k - 1] * &q[k];
        h += u64::power_of_2(log2_nb_terms[k]);
        t <<= r * h;
        t += &s[k];
        s[k - 1] = t;
        mul_prev(q, k);
    }
    let mut s0 = take(&mut s[0]);
    let mut q0 = take(&mut q[0]);
    let precy_i = i64::exact_from(precy);
    let mut diff = i64::exact_from(s0.significant_bits()) - (precy_i << 1);
    let mut expo = diff;
    // a negative shift is a left shift, covering MPFR's mul_2exp branch
    s0 >>= diff;
    diff = i64::exact_from(q0.significant_bits()) - precy_i;
    expo -= diff;
    q0 >>= diff;
    s0 /= q0; // truncating division (both positive)
    // y = (S[0] rounded down to precy) * 2^(expo - r(i-1)); MPFR sets the mantissa via set_z then
    // overrides the exponent, which is exactly this scaling. The scaled value is atan(x)/x, of
    // ordinary size, so attaching the scaling before the conversion keeps every exponent in range.
    Float::from_rational_prec_round(
        Rational::from(s0) << (expo - ri * (i64::exact_from(i) - 1)),
        precy,
        Floor,
    )
    .0
}

// atan x for a tiny nonzero x, bracketed by consecutive partial sums of its alternating series: x -
// x^3/3 < atan x < x - x^3/3 + x^5/5 < x, and so on, a bracket that narrows without bound; the
// arctangent is transcendental, so it eventually rounds unambiguously. This is the fallback for the
// tiny inputs that MPFR's small-input shortcut declines, which are those at the very bottom of the
// exponent range whose result underflows; the general algorithm would otherwise work at a precision
// of about 2^30 bits for them.
fn atan_tiny(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let negative = *x < 0u32;
    let ax = Rational::exact_from(x).abs();
    let x2 = (&ax).square();
    // hi and lo are the partial sums with an odd and an even number of terms
    let mut term = ax.clone();
    let mut hi = ax;
    let mut lo = Rational::ZERO;
    let mut k = 1u64;
    loop {
        term *= &x2;
        let t = &term / Rational::from((k << 1) + 1);
        if k.odd() {
            lo = &hi - t;
        } else {
            hi = &lo + t;
        }
        if let Some(result) = round_bracket_signed_by(negative, lo.clone(), hi.clone(), prec, rm) {
            return result;
        }
        k += 1;
    }
}

// This is mpfr_atan from atan.c, MPFR 4.2.2, for a finite nonzero input, with a series bracket for
// the tiny inputs whose result underflows.
fn atan_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // atan(x) = x - x^3/3 + x^5/5 ..., so the error is < 2^(3 EXP(x) - 1), and `EXP(x) - (3 EXP(x)
    // - 1)` = -2 EXP(x) + 1
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT (atan, x, -2 * MPFR_GET_EXP (x), 1, 0, rnd_mode, {});
    let err1 = -(exp_x << 1);
    if err1 > 0 {
        let err = u64::exact_from(err1) + 1;
        // The error bound only has to clear prec + 1; passing an enormous err (a tiny x has one
        // around 2^31) would make float_round_near_x do work proportional to it.
        if err > prec + 1 {
            if let Some(result) = float_round_near_x(x, min(err, prec + 2), false, prec, rm) {
                return result;
            }
            return atan_tiny(x, prec, rm);
        }
    }
    let negative = *x < 0u32;
    let xp = x.clone().abs();
    // Other simple case: atan(±1) = ±pi/4
    let comparison = xp.partial_cmp(&1u32).unwrap();
    if comparison == Equal {
        let (pi, o) = Float::pi_prec_round(prec, if negative { -rm } else { rm });
        // exact
        let quarter = pi >> 2u32;
        return if negative {
            (-quarter, o.reverse())
        } else {
            (quarter, o)
        };
    }
    let mut realprec = prec + prec.ceiling_log_base_2() + 4;
    let mut increment = Limb::WIDTH;
    // If |x| < 1, we need more precision to be able to round
    let sup = if exp_x < 0 {
        u64::exact_from(2 - exp_x)
    } else {
        1
    };
    loop {
        // n0 = ceil(log(prec_requested + 2 + 1 + ln(2.4)/ln(2))/log(2))
        let n0 = (realprec + sup + 3).ceiling_log_base_2();
        // since realprec >= 4, n0 >= ceil(log2(8)) >= 3, so 3 n0 > 2
        let mut wp = realprec + sup + 1 + (3 * n0 - 2).ceiling_log_base_2();
        // The number of lost bits due to argument reduction is 9 - 2 EXP(sk), estimated by 9 + 2
        // ceil(log2(p)), since we manage that sk < 1/p. Below 100 bits the argument is not reduced.
        let (log2p, est_lost) = if prec > 100 {
            let log2p = (wp.ceiling_log_base_2() >> 1) - 3;
            (log2p, 9 + (log2p << 1))
        } else {
            (0, 0)
        };
        wp += est_lost;
        // use atan(xp) = pi/2 - atan(1/xp) for xp > 1; now 0 < sk <= 1
        let mut sk = if comparison == Greater {
            xp.reciprocal_prec_ref(wp).0
        } else {
            Float::from_float_prec_ref(&xp, wp).0
        };
        // Argument reduction: atan(x) = 2 atan((sqrt(1 + x^2) - 1)/x), applied until |sk| <
        // k/sqrt(p), where p is the target precision. It keeps 0 < sk <= 1, and is applied at least
        // once when sk = 1, after which sk < 1 (see atan.c for the interval-arithmetic argument).
        let mut lost = 0u64;
        let mut red = 0u64;
        let log2p_i = i64::exact_from(log2p);
        loop {
            let exp_sk = i64::from(sk.get_exponent().unwrap());
            if exp_sk <= -log2p_i {
                break;
            }
            lost = u64::exact_from(9 - (exp_sk << 1));
            let mut tmp = sk.square_prec_ref(wp).0;
            tmp.add_prec_assign_ref(&Float::ONE, wp);
            tmp.sqrt_prec_assign(wp);
            tmp.sub_prec_assign_ref(&Float::ONE, wp);
            sk = if red == 0 && comparison == Greater {
                // use xp = 1/sk
                tmp.mul_prec_val_ref(&xp, wp).0
            } else {
                tmp.div_prec_val_ref(&sk, wp).0
            };
            red += 1;
        }
        debug_assert!(sk < 1u32);
        // Assignation
        let mut arctgt = Float::ZERO;
        let mut twopoweri = 1u64;
        for i in 0..n0 {
            if sk == 0u32 {
                break;
            }
            // trunc(sk 2^twopoweri) as an integer; since the s_k are decreasing (see
            // algorithms.tex) and s_0 = min(|x|, 1/|x|) < 1, sk < 1
            let ukz = Integer::rounding_from(&(&sk << twopoweri), Down).0;
            if ukz != 0u32 {
                // tmp = ukz / 2^twopoweri
                let tmp = Float::from_integer_prec_ref(&ukz, wp).0 >> twopoweri;
                // atan(A_k)
                let tmp2 = atan_aux(ukz, twopoweri, usize::exact_from(n0 - i), wp)
                    .mul_prec_val_ref(&tmp, wp)
                    .0;
                // Addition
                arctgt.add_prec_assign(tmp2, wp);
                // Next iteration
                let tmp2 = sk.sub_prec_ref_ref(&tmp, wp).0;
                sk.mul_prec_assign_ref(&tmp, wp);
                sk.add_prec_assign_ref(&Float::ONE, wp);
                sk = tmp2.div_prec(sk, wp).0;
            }
            twopoweri <<= 1;
        }
        // Add the last step: atan(sk) ~= sk
        arctgt.add_prec_assign(sk, wp);
        // undo the argument reduction
        arctgt <<= red;
        if comparison == Greater {
            // atan(x) = pi/2 - atan(1/x) for x > 0
            arctgt = (Float::pi_prec(wp).0 >> 1u32).sub_prec(arctgt, wp).0;
        }
        debug_assert!(arctgt > 0u32);
        let err = i64::exact_from(realprec + est_lost) - i64::exact_from(lost);
        if err > 0
            && float_can_round(
                arctgt.significand_ref().unwrap(),
                u64::exact_from(err),
                prec,
                rm,
            )
        {
            return Float::from_float_prec_round(if negative { -arctgt } else { arctgt }, prec, rm);
        }
        realprec += increment;
        increment = realprec >> 1;
    }
}

impl Float {
    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arctangent is less than, equal
    /// to, or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p+1}$.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, rounded
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\arctan x| < \pi/2$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\arctan x| < |x|$ for nonzero $x$, no other input can reach it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atan_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::atan_round`] instead. If both of these things are true, consider using
    /// [`Float::atan`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: an input above 1 in magnitude is first inverted, and the argument
    /// is then halved a logarithmic number of times and split into chunks whose arctangents are
    /// summed by binary splitting, all at a working precision of about $n$; the summation is the
    /// first term, and the inversion of the $m$-bit input the second. The magnitude of the input
    /// does not drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arctangent of a
    /// finite nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$, or if
    /// `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .atan_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .atan_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .atan_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .atan_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.78539753");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .atan_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.78539848");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .atan_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.78539848");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.atan_prec_round_ref(prec, rm)
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arctangent is less than, equal
    /// to, or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p+1}$.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, rounded
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\arctan x| < \pi/2$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\arctan x| < |x|$ for nonzero $x$, no other input can reach it.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atan_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::atan_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).atan()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: an input above 1 in magnitude is first inverted, and the argument
    /// is then halved a logarithmic number of times and split into chunks whose arctangents are
    /// summed by binary splitting, all at a working precision of about $n$; the summation is the
    /// first term, and the inversion of the $m$-bit input the second. The magnitude of the input
    /// does not drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arctangent of a
    /// finite nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$, or if
    /// `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.78539753");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.78539848");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.78539848");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn atan_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // atan(+infinity) = pi/2, atan(-infinity) = -pi/2
            Infinity { sign } => {
                assert_ne!(rm, Exact, "Inexact atan");
                let (pi, o) = Self::pi_prec_round(prec, if *sign { rm } else { -rm });
                // exact
                let half = pi >> 1u32;
                if *sign {
                    (half, o)
                } else {
                    (-half, o.reverse())
                }
            }
            // atan(+0) = +0, atan(-0) = -0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => atan_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arctangent is less than, equal to, or greater than
    /// the exact arctangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arctangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arctan x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, rounded
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\arctan x| < \pi/2$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\arctan x| < |x|$ for nonzero $x$, no other input can reach it.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::atan`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: an input above 1 in magnitude is first inverted, and the argument
    /// is then halved a logarithmic number of times and split into chunks whose arctangents are
    /// summed by binary splitting, all at a working precision of about $n$; the summation is the
    /// first term, and the inversion of the $m$-bit input the second. The magnitude of the input
    /// does not drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.atan_prec(5);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.atan_prec(20);
    /// assert_eq!(c.to_string(), "0.78539848");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_prec(self, prec: u64) -> (Self, Ordering) {
        self.atan_prec_round(prec, Nearest)
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded arctangent is less than, equal to, or greater
    /// than the exact arctangent. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arctangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arctan x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, rounded
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\arctan x| < \pi/2$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\arctan x| < |x|$ for nonzero $x$, no other input can reach it.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).atan()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: an input above 1 in magnitude is first inverted, and the argument
    /// is then halved a logarithmic number of times and split into chunks whose arctangents are
    /// summed by binary splitting, all at a working precision of about $n$; the summation is the
    /// first term, and the inversion of the $m$-bit input the second. The magnitude of the input
    /// does not drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.78539848");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.atan_prec_round_ref(prec, Nearest)
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\pm\pi/2$, rounded
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\arctan x| < \pi/2$, the result never overflows.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\arctan x| < |x|$ for nonzero $x$, no other input can reach it.
    ///
    /// If you want to specify an output precision, consider using [`Float::atan_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::atan`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: an input
    /// above 1 in magnitude is first inverted, and the argument is then halved a logarithmic number
    /// of times and split into chunks whose arctangents are summed by binary splitting, all at a
    /// working precision of about $n$. The magnitude of the input does not drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arctangent of a
    /// finite nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.atan_round(Floor);
    /// assert_eq!(c.to_string(), "0.78539816339744830961566084581983");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.atan_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.78539816339744830961566084582062");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.atan_round(Nearest);
    /// assert_eq!(c.to_string(), "0.78539816339744830961566084581983");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.atan_prec_round(prec, rm)
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arctangent is less than, equal to, or greater than the exact
    /// arctangent. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\pm\pi/2$, rounded
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\arctan x| < \pi/2$, the result never overflows.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an input of magnitude $2^{-2^{30}}$, the smallest positive [`Float`],
    /// rounded toward zero: since $|\arctan x| < |x|$ for nonzero $x$, no other input can reach it.
    ///
    /// If you want to specify an output precision, consider using [`Float::atan_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).atan()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: an input
    /// above 1 in magnitude is first inverted, and the argument is then halved a logarithmic number
    /// of times and split into chunks whose arctangents are summed by binary splitting, all at a
    /// working precision of about $n$. The magnitude of the input does not drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arctangent of a
    /// finite nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.78539816339744830961566084581983");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.78539816339744830961566084582062");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).atan_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.78539816339744830961566084581983");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.atan_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is replaced by the result, and
    /// an [`Ordering`] is returned, indicating whether the rounded arctangent is less than, equal
    /// to, or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p+1}$.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::atan_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atan_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::atan_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::atan_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: an input above 1 in magnitude is first inverted, and the argument
    /// is then halved a logarithmic number of times and split into chunks whose arctangents are
    /// summed by binary splitting, all at a working precision of about $n$; the summation is the
    /// first term, and the inversion of the $m$-bit input the second. The magnitude of the input
    /// does not drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arctangent of a
    /// finite nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$, or if
    /// `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.781");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.812");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.781");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.78539753");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.78539848");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.78539848");
    /// ```
    #[inline]
    pub fn atan_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.atan_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded arctangent is less than, equal to,
    /// or greater than the exact arctangent. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// If the arctangent is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arctan x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::atan_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::atan_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: an input above 1 in magnitude is first inverted, and the argument
    /// is then halved a logarithmic number of times and split into chunks whose arctangents are
    /// summed by binary splitting, all at a working precision of about $n$; the summation is the
    /// first term, and the inversion of the $m$-bit input the second. The magnitude of the input
    /// does not drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.781");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.78539848");
    /// ```
    #[inline]
    pub fn atan_prec_assign(&mut self, prec: u64) -> Ordering {
        self.atan_prec_round_assign(prec, Nearest)
    }

    /// Computes $\arctan x$, the arctangent of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded arctangent is less than, equal to, or greater than the exact
    /// arctangent. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is not NaN and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::atan_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::atan_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::atan_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: an input
    /// above 1 in magnitude is first inverted, and the argument is then halved a logarithmic number
    /// of times and split into chunks whose arctangents are summed by binary splitting, all at a
    /// working precision of about $n$. The magnitude of the input does not drive the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is nonzero and not NaN, since the arctangent of a
    /// finite nonzero [`Float`] is never exactly representable and neither is $\pm\pi/2$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.78539816339744830961566084581983");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.78539816339744830961566084582062");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.atan_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.78539816339744830961566084581983");
    /// ```
    #[inline]
    pub fn atan_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.atan_prec_round_assign(prec, rm)
    }
}

impl Atan for Float {
    type Output = Self;

    /// Computes $\arctan x$, the arctangent of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arctangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arctan x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\pm\pi/2$, rounded
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::atan_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::atan_prec`]. If you want both of these things, consider using
    /// [`Float::atan_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `atan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.atan().is_nan());
    /// // an infinity has a precision of 1, so pi/2 rounds to 2
    /// assert_eq!(Float::INFINITY.atan().to_string(), "2.0");
    /// assert_eq!(Float::NEGATIVE_INFINITY.atan().to_string(), "-2.0");
    /// assert_eq!(Float::ZERO.atan().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.atan().to_string(), "-0.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.atan().to_string(),
    ///     "0.78539816339744830961566084581983"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.atan().to_string(),
    ///     "1.5607966601082313810249815754304"
    /// );
    /// ```
    #[inline]
    fn atan(self) -> Self {
        let prec = self.significant_bits();
        self.atan_prec_round(prec, Nearest).0
    }
}

impl Atan for &Float {
    type Output = Float;

    /// Computes $\arctan x$, the arctangent of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the arctangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arctan x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\pm\pi/2$, rounded
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::atan_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::atan_prec_ref`]. If you want both of these things, consider using
    /// [`Float::atan_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `atan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.atan().is_nan());
    /// // an infinity has a precision of 1, so pi/2 rounds to 2
    /// assert_eq!(Float::INFINITY.atan().to_string(), "2.0");
    /// assert_eq!(Float::NEGATIVE_INFINITY.atan().to_string(), "-2.0");
    /// assert_eq!(Float::ZERO.atan().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.atan().to_string(), "-0.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).atan().to_string(),
    ///     "0.78539816339744830961566084581983"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .atan()
    ///         .to_string(),
    ///     "1.5607966601082313810249815754304"
    /// );
    /// ```
    #[inline]
    fn atan(self) -> Float {
        self.atan_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl AtanAssign for Float {
    /// Computes $\arctan x$, the arctangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the arctangent is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \arctan x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arctan x|\rfloor-p}$, where
    ///   $p$ is the precision of the input.
    ///
    /// See the [`Float::atan`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::atan_prec_assign`]. If you want both of these things, consider using
    /// [`Float::atan_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n (\log n)^3 \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$, summed by binary splitting for large $n$, costs the first term, and
    /// for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n +
    /// e$ bits. Unlike most functions, `atan` therefore gets slower as the magnitude of its input
    /// grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AtanAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.atan_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.atan_assign();
    /// assert_eq!(x.to_string(), "2.0");
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.atan_assign();
    /// assert_eq!(x.to_string(), "-2.0");
    ///
    /// let mut x = Float::ZERO;
    /// x.atan_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.atan_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.atan_assign();
    /// assert_eq!(x.to_string(), "0.78539816339744830961566084581983");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.atan_assign();
    /// assert_eq!(x.to_string(), "1.5607966601082313810249815754304");
    /// ```
    #[inline]
    fn atan_assign(&mut self) {
        let prec = self.significant_bits();
        self.atan_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\arctan x$, the arctangent of a primitive float. Using this function is more accurate
/// than using the default `atan` function or the one provided by `libm`.
///
/// $$
/// f(x) = \arctan x+\varepsilon.
/// $$
/// - If $x$ is NaN, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is not NaN, then $|\varepsilon| < 2^{\lfloor\log_2 |\arctan x|\rfloor-p}$, where $p$ is
///   the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\pm\pi/2$, rounded
/// - $f(\pm0.0)=\pm0.0$
///
/// Overflow is not possible, since the result lies in $(-\pi/2, \pi/2)$. The result is subnormal
/// only when $x$ is, and then it is $x$ itself, since $|\arctan x - x| < |x|^3/3$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan::primitive_float_atan;
///
/// assert!(primitive_float_atan(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_atan(f32::INFINITY)),
///     NiceFloat(core::f32::consts::FRAC_PI_2)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan(f32::NEGATIVE_INFINITY)),
///     NiceFloat(-core::f32::consts::FRAC_PI_2)
/// );
/// assert_eq!(NiceFloat(primitive_float_atan(0.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_atan(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_atan(1.0f32)),
///     NiceFloat(core::f32::consts::FRAC_PI_4)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan(1.0f64)),
///     NiceFloat(core::f64::consts::FRAC_PI_4)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::atan_prec, x)
}
