// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2002-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's simultaneous sine and cosine. `mpfr_sin_cos` (`sin_cos.c`) reduces an argument
// with |x| >= 2 modulo 2 pi, takes the cosine of the reduced argument, and derives the sine as
// ±sqrt(1 - cos^2), all inside one Ziv loop that must certify both results. For precisions at or
// above `SINCOS_THRESHOLD`, `sin`, `cos`, and `sin_cos` all use the asymptotically fast tier
// `sin_cos_fast` (`mpfr_sincos_fast`, in the same file): the argument is reduced modulo pi/2 and
// split into chunks of doubling bit length, each chunk's sine and cosine are summed by binary
// splitting of the Taylor series in integer arithmetic, and the chunks are combined by the
// angle-addition formulas.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{
    NEAR_ZERO_MIN_CANCEL, cos_rational_helper, cos_rational_tiny, cos_turns_helper,
    cos_turns_special_case, cos_with_period_prec_round_normal_ref, reduce_huge, round_bracket,
    trig_near_zero, trig_rational_near_zero, trig_turns_near_zero,
};
use crate::float::arithmetic::exp::get_z_2exp;
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::float::arithmetic::sin::{
    SCALED_INPUT_EXPONENT, sin_rational_helper, sin_turns_helper, sin_turns_special_case,
    sin_with_period_prec_round_normal_ref,
};
use crate::{Float, emulate_float_to_float_pair_fn, emulate_rational_to_float_pair_fn};
use alloc::vec;
use core::cmp::Ordering::{self, Equal};
use core::cmp::{max, min};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, FloorSqrt, IsPowerOf2, NegAssign, Parity, PowerOf2, SinCos, SinCosAssign,
    Square, UnsignedAbs,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    NaN as NaNTrait, NegativeZero as NegativeZeroTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Nearest, Up};
use malachite_base::{fail_on_untested_path, split_into_chunks_mut};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The outcome of one iteration of the Ziv loop in `sin_cos_prec_round_normal_ref`.
enum SinCosStep {
    // The working precision could not decide both results; retry at a higher one.
    Retry,
    // The sine and cosine at the working precision, ready for the final rounding.
    Done(Float, Float),
    // The input is within about 2^-cancel of an odd multiple of pi/2, so the cosine is tiny and the
    // sine is within 2^-2cancel of ±1, with the given sign.
    NearZeroCos { cancel: u64, sin_negative: bool },
    // The input is within about 2^-cancel of a nonzero multiple of pi, so the sine is tiny and the
    // cosine is within 2^-2cancel of ±1, with the given sign.
    NearZeroSin { cancel: u64, cos_negative: bool },
}

// One iteration of the Ziv loop at working precision `m`, which the cancellation checks may raise
// for the next iteration (the caller applies the generic increase on `Retry`).
fn sin_cos_ziv_step(
    x: &Float,
    exp_x: i64,
    prec: u64,
    rm: RoundingMode,
    reduce: bool,
    m: &mut u64,
) -> SinCosStep {
    // A cancellation of this many bits sends a result to the near-zero path, and leaves the other
    // one within 2^-(prec + 2) of ±1, so that it rounds from ±1 alone.
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, (prec >> 1) + 1);
    let m_i = i64::exact_from(*m);
    let xr;
    let xx = if reduce {
        // As in `mpfr_sin`: reduce x modulo 2 pi to xr, and check that xr is at least 2^(2-m) away
        // from 0 and from ±pi, which settles the sign of the sine.
        let c_prec = u64::exact_from(exp_x) + *m - 1;
        let pi = Float::pi_prec(c_prec).0;
        xr = x.ieee_remainder_prec_ref_val(&pi << 1u32, *m).0;
        let c = pi.sub_prec_round((&xr).abs(), c_prec, Down).0;
        let threshold = 3 - m_i;
        let xr_small = xr == 0u32 || i64::from(xr.get_exponent().unwrap()) < threshold;
        let c_small = c == 0u32 || i64::from(c.get_exponent().unwrap()) < threshold;
        if xr_small || c_small {
            // x is within 2^(4-m) of a multiple of pi (an even one if xr is small, an odd one if c
            // is small), so |sin(x)| < 2^(5-m); the near-zero path resolves the sine directly, and
            // the cosine is then ±1 to within 2^-2cancel.
            let cancel = *m - 4;
            return if cancel >= near_zero_threshold {
                SinCosStep::NearZeroSin {
                    cancel,
                    cos_negative: c_small,
                }
            } else {
                SinCosStep::Retry
            };
        }
        &xr
    } else {
        x
    };
    // the sign of the sine
    let sign = *xx < 0u32;
    // c = cos(xx) rounded toward zero
    let c = xx.cos_prec_round_ref(*m, Down).0;
    // If no argument reduction was performed, the error is at most ulp(c), otherwise it is at most
    // ulp(c) + 2^(2-m). Since |c| < 1, we have ulp(c) <= 2^(-m), thus the error is bounded by
    // 2^(3-m) in that later case.
    let exp_c = c.get_exponent().map_or(Float::MIN_EXPONENT_I64, i64::from);
    // |cos(x)| < 2^bound_exp
    let bound_exp = if reduce { max(exp_c, 2 - m_i) } else { exp_c } + 1;
    if bound_exp < 0 && exp_x >= 1 {
        let cancel = u64::exact_from(-bound_exp);
        if cancel >= near_zero_threshold {
            return SinCosStep::NearZeroCos {
                cancel,
                sin_negative: sign,
            };
        }
    }
    let err = if reduce { exp_c + m_i - 3 } else { m_i };
    if c == 0u32
        || err <= 0
        || !float_can_round(c.significand_ref().unwrap(), u64::exact_from(err), prec, rm)
    {
        return SinCosStep::Retry;
    }
    // s = sqrt(1 - c^2): the square rounds up, so its absolute error is bounded by 2^(5-m) if
    // reduce, and by 2^(2-m) otherwise; 1 - c^2 rounds to nearest, for 2^(6-m) or 2^(3-m); the
    // square root, also to nearest, has absolute error 2^(6-m-EXP(s)) or 2^(3-m-EXP(s)).
    let mut s = Float::ONE.sub_prec(c.square_round_ref(Ceiling).0, *m).0;
    if s == 0u32 {
        // 1 - c^2 rounded to zero, so sin(xx)^2 is below 2^-(m + 1): x is near a multiple of pi
        let cancel = (*m >> 1).saturating_sub(1);
        if reduce && cancel >= near_zero_threshold {
            return SinCosStep::NearZeroSin {
                cancel,
                cos_negative: c < 0u32,
            };
        }
        fail_on_untested_path("sin_cos_ziv_step, 1 - c^2 rounded to zero");
        *m = max(*m, x.significant_bits()) << 1;
        return SinCosStep::Retry;
    }
    s.sqrt_prec_assign(*m);
    let exp_s = i64::from(s.get_exponent().unwrap());
    // the absolute error on s is at most 2^(err - m)
    let err = 3 + if reduce { 3 } else { 0 } - exp_s;
    if sign {
        s.neg_assign();
    }
    // |sin(x)| < 2^bound_exp
    let bound_exp = max(exp_s, err - m_i) + 1;
    if reduce && bound_exp < 0 {
        let cancel = u64::exact_from(-bound_exp);
        if cancel >= near_zero_threshold {
            return SinCosStep::NearZeroSin {
                cancel,
                cos_negative: c < 0u32,
            };
        }
    }
    // put the error in the form 2^(EXP(s) - err)
    let err = exp_s + m_i - err;
    if err > 0 && float_can_round(s.significand_ref().unwrap(), u64::exact_from(err), prec, rm) {
        return SinCosStep::Done(s, c);
    }
    if err < i64::exact_from(prec) {
        *m += u64::exact_from(i64::exact_from(prec) - err);
    }
    // s is exactly ±1 (its square root rounded to nearest), so the sine is within an ulp of ±1
    // and the working precision is doubled
    if exp_s == 1 && s.eq_abs(&1u32) {
        *m <<= 1;
    }
    SinCosStep::Retry
}

// ±1 rounded to `prec` bits under `rm`, as the value of a function known to lie within 2^(1 - err)
// of ±1 on the side toward zero, with the ternary value; the negative case reuses the positive one
// with the rounding mode mirrored.
fn near_one(err: u64, negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let err = min(err, prec + 2);
    if negative {
        let (r, o) = float_round_near_x(&Float::ONE, err, false, prec, -rm).unwrap();
        (-r, o.reverse())
    } else {
        float_round_near_x(&Float::ONE, err, false, prec, rm).unwrap()
    }
}

// Computes sin(x) and cos(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding
// mode `rm`. (x = 0 is handled by the caller.) Neither result is ever exactly representable, so
// `rm` must not be `Exact`.
//
// This shares the work of `sin_rational_helper` and `cos_rational_helper`: x is rounded once to a
// `Float` y_f at a working precision w, both functions of y_f are taken together, and both are
// bracketed using the Lipschitz bound |f(x) - f(y_f)| <= |x - y_f|, the rounding errors, and, for
// an x too large to be a `Float`, the error of a single `Rational` reduction modulo 2 pi, which for
// such an x is the dominant cost. The brackets are rounded in `Rational` arithmetic, and w is
// raised until both resolve.
pub(crate) fn sin_cos_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    assert_ne!(rm, Exact, "Inexact sin_cos");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // For an x so small that the cosine rounds to 1, both results come cheaply from the separate
    // paths: the sine from its series (or the underflow rule, or, for a precision beyond 2^31 bits,
    // the general path), and the cosine from 1.
    if 1 - (exp_x << 1) > i64::exact_from(prec) {
        let (s, o_s) = sin_rational_helper(x, prec, rm);
        let (c, o_c) = cos_rational_tiny(prec, rm);
        return (s, c, o_s, o_c);
    }
    // an x too small to be a `Float` at a precision that does not round its cosine to 1 needs the
    // series paths of both functions, which is only reachable beyond 2^31 bits of precision
    if exp_x <= Float::MIN_EXPONENT_I64 {
        fail_on_untested_path("sin_cos_rational_helper, series paths");
        let (s, o_s) = sin_rational_helper(x, prec, rm);
        let (c, o_c) = cos_rational_helper(x, prec, rm);
        return (s, c, o_s, o_c);
    }
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, (prec >> 1) + 1);
    let huge = exp_x >= Float::MAX_EXPONENT_I64;
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let reduced;
        let (y, extra) = if huge {
            reduced = reduce_huge(x, exp_x, w);
            (&reduced, Some(2 - i64::exact_from(w)))
        } else {
            (x, None)
        };
        if *y == 0u32 {
            // x is an exact multiple of 2 pi at the working precision; a higher precision breaks
            // the coincidence
            fail_on_untested_path("sin_cos_rational_helper, reduced argument is zero");
        } else {
            let (y_f, y_o) = Float::from_rational_prec_ref(y, w);
            if !huge && y_o == Equal {
                // x is exactly representable at w bits, so its sine and cosine are simply those
                return sin_cos_prec_round_normal_ref(&y_f, prec, rm);
            }
            let (s_f, c_f, _, _) = y_f.sin_cos_round_ref(Nearest);
            // The exponents of y, s_f, and c_f as `Float`s would have them (a zero result means
            // complete cancellation).
            let exp_y = y.floor_log_base_2_abs() + 1;
            let exp_s = s_f
                .get_exponent()
                .map_or(Float::MIN_EXPONENT_I64, i64::from);
            let exp_c = c_f
                .get_exponent()
                .map_or(Float::MIN_EXPONENT_I64, i64::from);
            let w_i = i64::exact_from(w);
            // |f(y) - f_f| <= 2^(exp_f - w) (half an ulp, doubled for safety) + |y - y_f| <=
            // 2^(exp_y - w), plus the reduction error; so |f(y)| < 2^(bound + 2) with bound the
            // largest of those exponents. Heavy cancellation in either function means y is close to
            // one of its zeros, which its near-zero path resolves exactly, while the other function
            // is then within 2^-2cancel of ±1 and rounds from ±1 alone.
            let error_exp = max(exp_y - w_i, extra.unwrap_or(i64::MIN));
            let bound_s = max(exp_s, error_exp) + 2;
            let bound_c = max(exp_c, error_exp) + 2;
            if bound_s < 0 {
                let cancel = u64::exact_from(-bound_s);
                if cancel >= near_zero_threshold {
                    let (s, o_s) = trig_rational_near_zero(y, exp_y, prec, rm, extra, w, false);
                    // 1 - |cos(x)| <= sin(x)^2 / 2 < 2^(2 bound_s - 1)
                    let (c, o_c) = near_one((cancel << 1) + 2, c_f < 0u32, prec, rm);
                    return (s, c, o_s, o_c);
                }
            }
            if bound_c < 0 {
                let cancel = u64::exact_from(-bound_c);
                if cancel >= near_zero_threshold {
                    let (c, o_c) = trig_rational_near_zero(y, exp_y, prec, rm, extra, w, true);
                    // 1 - |sin(x)| <= cos(x)^2 < 2^(2 bound_c)
                    let (s, o_s) = near_one((cancel << 1) + 1, s_f < 0u32, prec, rm);
                    return (s, c, o_s, o_c);
                }
            }
            let mut delta_s = Rational::power_of_2(exp_s - w_i) + Rational::power_of_2(exp_y - w_i);
            let mut delta_c = Rational::power_of_2(exp_c - w_i) + Rational::power_of_2(exp_y - w_i);
            if let Some(extra) = extra {
                let e = Rational::power_of_2(extra);
                delta_s += &e;
                delta_c += e;
            }
            let s = Rational::exact_from(&s_f);
            let c = Rational::exact_from(&c_f);
            if let Some((s, o_s)) = round_bracket(&(&s - &delta_s), &(s + delta_s), prec, rm)
                && let Some((c, o_c)) = round_bracket(&(&c - &delta_c), &(c + delta_c), prec, rm)
            {
                return (s, c, o_s, o_c);
            }
        }
        w += increment;
        increment = w >> 1;
    }
}

// ---------- asymptotically fast implementation below (mpfr_sincos_fast) ----------

// At or above this precision, `sin`, `cos`, and `sin_cos` use the binary-splitting tier
// (`sin_cos_fast`) rather than their basic Ziv loops. Tuned on Apple Silicon with `-g tune_sincos`
// (see `bin_util/tune.rs`), 2026-09-07: the crossover of the two `sin_cos` tiers on inputs in [1/2,
// 1), with `sin` alone crossing at about 21800 bits and `cos` alone at about 27900. MPFR tunes the
// same single threshold on `mpfr_sin_cos` (28990 bits on its arm build, 23323 on x86_64 core2).
// Beyond the crossover the fast tier leads by only a few percent up to about 300000 bits.
pub(crate) const SINCOS_THRESHOLD: u64 = 25285;

// Truncates `r` to at most `prec` bits, returning the truncated integer and the number of bits
// dropped.
//
// This is `reduce` from `sin_cos.c`, MPFR 4.2.2.
fn reduce(r: &Integer, prec: u64) -> (Integer, u64) {
    let l = r.significant_bits().saturating_sub(prec);
    (r >> l, l)
}

// Truncates `s` and `c` by the same number of bits, so that the smaller has at most `prec` bits,
// returning the number of bits dropped.
//
// This is `reduce2` from `sin_cos.c`, MPFR 4.2.2.
fn reduce2(s: &mut Integer, c: &mut Integer, prec: u64) -> u64 {
    let l = min(s.significant_bits(), c.significant_bits()).saturating_sub(prec);
    *s >>= l;
    *c >>= l;
    l
}

const KMAX: usize = 64;
// three arrays of KMAX entries each
const SCRATCH_LEN: usize = 3 * KMAX;

// Returns (Q0, S0, C0, m) such that S0/(Q0 2^m) approximates sin(X) with absolute error at most 9
// 2^-prec, and C0/(Q0 2^m) approximates cos(X) with relative (and so absolute) error at most 9
// 2^-prec, where 0 <= X = p/2^r <= 1/2.
//
// sin(X)/X = sum((-1)^i (p/2^r)^i/(2i+1)!, i = 0..infinity), summed by binary splitting with P(a,b)
// = (-p)^(b-a), Q(a,b) = (2a)(2a+1) 2^r if a+1 = b (except Q(0,1) = 1) and Q(a,c) Q(c,b) otherwise,
// and T(a,b) = 1 if a+1 = b and Q(c,b) T(a,c) + P(a,c) T(c,b) otherwise. Since P(a,b) is only
// needed for b-a = 2^k, only the powers p^(2^k) are computed, and the factor 2^r is not stored in Q
// but tracked as the returned power of two.
//
// This is `sin_bs_aux` from `sin_cos.c`, MPFR 4.2.2. Assumes prec >= 10.
fn sin_bs_aux(p: &Integer, r: u64, prec: u64) -> (Integer, Integer, Integer, u64) {
    if *p == 0u32 {
        // sin(x)/x -> 1
        fail_on_untested_path("sin_bs_aux, p == 0");
        return (Integer::ONE, Integer::ONE, Integer::ONE, 0);
    }
    // check that X = p/2^r <= 1/2 (MPFR's `mpz_sizeinbase (p, 2) - r <= -1` compares in unsigned
    // arithmetic, so it never fails; the first chunk can be exactly 1/2)
    let p_bits = p.significant_bits();
    assert!(p_bits < r || (p_bits == r && p.unsigned_abs_ref().is_power_of_2()));
    let r0 = r;
    // normalize p (non-zero here): p = pp * 2^h, then square
    let h = p.trailing_zeros().unwrap();
    let pp = (p >> h).square();
    // x^2 = (p/2^r0)^2 = pp / 2^r
    let r = (r - h) << 1;
    // now p is odd
    let mut scratch = vec![Integer::ZERO; SCRATCH_LEN];
    split_into_chunks_mut!(scratch, KMAX, [t, q], ptoj); // ptoj[i] = pp^(2^i)
    let mut log2_nb_terms = [0u64; KMAX];
    let mut scratch_i = vec![0i64; SCRATCH_LEN];
    split_into_chunks_mut!(scratch_i, KMAX, [mult, accu], size_ptoj);
    let mut alloc = 2usize;
    // 6*2^r - pp = 6*2^r*(1 - x^2/6)
    t[0] = (const { Integer::const_from_unsigned(6) } << r) - &pp;
    q[0] = const { Integer::const_from_unsigned(6) };
    ptoj[0] = pp.clone();
    ptoj[1] = (&pp).square();
    size_ptoj[1] = i64::exact_from(ptoj[1].significant_bits());
    log2_nb_terms[0] = 1;
    // already take into account the factor x = p/2^r in sin(x) = x * (...): we have x^3 <
    // 1/2^mult[0]
    let pp_s = i64::exact_from(pp.significant_bits());
    let p_s = i64::exact_from(p.significant_bits());
    let r_i = i64::exact_from(r);
    mult[0] = r_i - pp_s + i64::exact_from(r0) - p_s;
    let prec_i = i64::exact_from(prec);
    let mut k = 0usize;
    let mut prec_i_have = mult[0];
    let mut i = 2u64;
    while prec_i_have < prec_i {
        // i is even here. Invariant: Q[0]*Q[1]*...*Q[k] equals (2i-1)!, and we have already summed
        // the terms of index < i in S[0]/Q[0], ..., S[k]/Q[k].
        k += 1;
        if k + 1 >= alloc {
            // necessarily k + 1 == alloc
            assert_eq!(k + 1, alloc);
            alloc += 1;
            assert!(k + 1 < KMAX);
            ptoj[k + 1] = (&ptoj[k]).square(); // pp^(2^(k+1))
            size_ptoj[k + 1] = i64::exact_from(ptoj[k + 1].significant_bits());
        }
        // For i even, we have Q[k] = (2i)(2i+1), T[k] = 1, then Q[k+1] = (2i+2)(2i+3), T[k+1] = 1,
        // which reduces to T[k] = (2i+2)(2i+3) 2^r - pp, Q[k] = (2i)(2i+1)(2i+2)(2i+3).
        assert!(k < KMAX);
        log2_nb_terms[k] = 1;
        let two_i = i << 1;
        q[k] = Integer::from((two_i + 2) * (two_i + 3));
        t[k] = (&q[k] << r) - &pp;
        q[k] *= Integer::from(two_i * (two_i + 1));
        // the next term of the series is divided by Q[k] and multiplied by pp^2/2^(2r), thus the
        // multiplicative factor is < 1/2^mult[k]
        mult[k] = i64::exact_from(q[k].significant_bits()) + (r_i << 1) - size_ptoj[1] - 1;
        // the absolute contribution of the next term is 1/2^accu[k]
        accu[k] = if k == 0 {
            mult[k]
        } else {
            mult[k] + accu[k - 1]
        };
        prec_i_have = accu[k]; // the current term is < 1/2^accu[k]
        let mut j = (i + 2) >> 1;
        let mut l = 1usize;
        while j.even() {
            // combine and reduce
            assert!(k >= 1);
            t[k] *= &ptoj[l];
            let mut tk1 = &t[k - 1] * &q[k];
            tk1 <<= r << l;
            tk1 += &t[k];
            t[k - 1] = tk1;
            let qk = q[k].clone();
            q[k - 1] *= &qk;
            // the number of terms in S[k-1] is a power of 2 by construction
            log2_nb_terms[k - 1] += 1;
            prec_i_have = i64::exact_from(qk.significant_bits());
            mult[k - 1] += prec_i_have + i64::exact_from(r << l) - size_ptoj[l] - 1;
            accu[k - 1] = if k == 1 {
                mult[k - 1]
            } else {
                mult[k - 1] + accu[k - 2]
            };
            prec_i_have = accu[k - 1];
            l += 1;
            j >>= 1;
            k -= 1;
        }
        i += 2;
    }
    // Accumulate all products in T[0] and Q[0]. Warning: contrary to above, here we do not have
    // log2_nb_terms[k-1] = log2_nb_terms[k]+1.
    let mut h = 0u64; // number of accumulated terms in the right part T[k]/Q[k]
    while k > 0 {
        t[k] *= &ptoj[usize::exact_from(log2_nb_terms[k - 1])];
        let mut tk1 = &t[k - 1] * &q[k];
        h += u64::power_of_2(log2_nb_terms[k]);
        tk1 <<= r * h;
        tk1 += &t[k];
        t[k - 1] = tk1;
        let qk = q[k].clone();
        q[k - 1] *= qk;
        k -= 1;
    }
    // implicit multiplier 2^r for Q0
    let mut m = i64::exact_from(r0) + r_i * (i64::exact_from(i) - 1);
    // At this point T[0]/(2^m Q[0]) is an approximation of sin(x) where the first neglected term
    // has contribution < 1/2^prec; since the series has alternating signs, the error is < 1/2^prec.
    //
    // We truncate Q0 to prec bits: the relative error is at most 2^(1-prec), which means that Q0 =
    // Q[0] (1 + theta) with |theta| <= 2^(1-prec), up to a power of two.
    let (q0, l) = reduce(&q[0], prec);
    m += i64::exact_from(l);
    let (t0, l) = reduce(&t[0], prec);
    m -= i64::exact_from(l);
    // multiply by x = p/2^m
    let (s0, l) = reduce(&(t0 * p), prec); // S0 = T[0] (1 + theta)^2 up to a power of two
    m -= i64::exact_from(l);
    // sin(X) ~ S0/Q0 (1 + theta)^3 + err with |theta| <= 2^(1-prec) and |err| <= 2^(-prec), thus
    // since |S0/Q0| <= 1: |sin(X) - S0/Q0| <= 4 |theta S0/Q0| + |err| <= 9 2^(-prec)
    //
    // Compute cos(X) from sin(X): sqrt(1 - (S/Q)^2) = sqrt(Q^2 - S^2)/Q = sqrt(Q0^2 2^(2m) -
    // S0^2)/Q0. Write S/Q = sin(X) + eps with |eps| <= 9 2^(-prec); then sqrt(Q^2 - S^2) = Q cos(X)
    // (1 + eps4) with |eps4| <= 9 2^(-prec), since |Q| >= 2^(prec-1) (see sin_cos.c for the steps).
    // We assume that Q0 2^m >= 2^(prec-1).
    let m = u64::exact_from(m);
    assert!(m + q0.significant_bits() >= prec);
    let c0 = Integer::from(
        (((&q0).square() << (m << 1)) - (&s0).square())
            .unsigned_abs()
            .floor_sqrt(),
    );
    (q0, s0, c0, m)
}

// Returns approximations s and c of sin(x) and cos(x) at precision `prec_s`, and err such that the
// relative error of each is bounded by 2^err ulps. Assumes 0 < x < pi/4 and prec_s >= 10.
//
// This is `sincos_aux` from `sin_cos.c`, MPFR 4.2.2.
fn sincos_aux(x: &Float, prec_s: u64) -> (Float, Float, u64) {
    let mut x2 = x.clone(); // exact
    let mut q_acc = Integer::ONE;
    let mut l = 0i64;
    let mut s_acc = Integer::ZERO; // sin(0) = S/(2^l Q), exact
    let mut c_acc = Integer::ONE; // cos(0) = C/(2^l Q), exact
    // Invariant: x = X + x2/2^(sh-1), where the part X was already treated, S/(2^l Q) ~ sin(X),
    // C/(2^l Q) ~ cos(X), and x2/2^(sh-1) < pi/4. sh-1 is the number of already shifted bits in x2.
    let mut sh = 1u64;
    let mut j = 0u64;
    while x2 != 0u32 && sh <= prec_s {
        let (q2, s2, c2, l2) = if sh > prec_s >> 1 {
            // sin(x) = x + O(x^3), cos(x) = 1 + O(x^2)
            let (s2, e) = get_z_2exp(x2.clone()); // S2/2^l2 = x2
            let mut l2 = -e;
            l2 += i64::exact_from(sh) - 1;
            let q2 = Integer::ONE;
            let c2 = Integer::power_of_2(u64::exact_from(l2));
            x2 = Float::ZERO;
            (q2, s2, c2, l2)
        } else {
            // y <- trunc(x2 * 2^sh) = trunc(x * 2^(2 sh - 1))
            x2 <<= sh; // exact
            // round toward zero: now 0 <= x2 < 2^sh, thus 0 <= x2/2^(sh-1) < 2^(1-sh)
            let y = Integer::rounding_from(&x2, Down).0;
            if y == 0u32 {
                sh <<= 1;
                j += 1;
                continue;
            }
            let x2_prec = x2.get_prec().unwrap();
            // should be exact
            let (d, o) = x2.sub_prec_round(Float::exact_from(&y), x2_prec, Exact);
            assert_eq!(o, Equal);
            x2 = d;
            let (q2, s2, c2, l2) = sin_bs_aux(&y, (sh << 1) - 1, prec_s);
            // we now have |S2/Q2/2^l2 - sin(X)| <= 9 2^(-prec_s) and |C2/Q2/2^l2 - cos(X)| <= 6
            // 2^(-prec_s), with X = y/2^(2 sh - 1)
            (q2, s2, c2, i64::exact_from(l2))
        };
        if sh == 1 {
            // S = 0, C = 1
            l = l2;
            q_acc = q2;
            s_acc = s2;
            c_acc = c2;
        } else {
            // s <- s c2 + c s2, c <- c c2 - s s2, using Karatsuba: a = s + c, b = s2 + c2, t = a b,
            // d = s s2, e = c c2, s <- t - d - e, c <- e - d
            let a = &s_acc + &c_acc;
            let e = c_acc * &c2;
            let b = c2 + &s2;
            let d = s2 * &s_acc;
            let t = a * b;
            s_acc = t - &d - &e;
            c_acc = e - d;
            q_acc *= q2;
            // after j loops, the error is <= (11j - 2) 2^(prec_s)
            l += l2;
            // reduce Q to prec_s bits
            let (qr, lq) = reduce(&q_acc, prec_s);
            q_acc = qr;
            l += i64::exact_from(lq);
            // reduce S, C to prec_s bits, error <= 11 j 2^(prec_s)
            l -= i64::exact_from(reduce2(&mut s_acc, &mut c_acc, prec_s));
        }
        sh <<= 1;
        j += 1;
    }
    let mut j = 11 * j;
    let mut err = 0u64;
    while j > 1 {
        j = j.div_ceil(2);
        err += 1;
    }
    let q_f = Float::exact_from(&q_acc);
    let s = Float::from_integer_prec(s_acc, prec_s)
        .0
        .div_prec_val_ref(&q_f, prec_s)
        .0
        >> l;
    let c = Float::from_integer_prec(c_acc, prec_s)
        .0
        .div_prec(q_f, prec_s)
        .0
        >> l;
    (s, c, err)
}

// Computes sin(x) and/or cos(x) for a finite nonzero `Float` x, rounded to precision `prec` with
// rounding mode `rm`, by binary splitting: the argument is reduced modulo pi/2 and split into
// chunks of doubling bit length, each chunk's sine and cosine are summed by binary splitting of the
// Taylor series in integer arithmetic, and the chunks are combined by the angle-addition formulas.
// Only the selected results are rounded and returned.
//
// This is `mpfr_sincos_fast` from `sin_cos.c`, MPFR 4.2.2.
pub(crate) fn sin_cos_fast(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
    want_sin: bool,
    want_cos: bool,
) -> (Option<(Float, Ordering)>, Option<(Float, Ordering)>) {
    let mut w = prec;
    w += w.ceiling_log_base_2() + 9; // ensures w >= 10 (needed by sincos_aux)
    let mut increment = Limb::WIDTH;
    // 1686629713 / 2^31, just below pi/4
    let pi_over_4 = const { Float::const_from_unsigned(1686629713) } >> 31u32;
    let exp_x = i64::from(x.get_exponent().unwrap());
    loop {
        let (ts, tc, err) = if *x > 0u32 && *x <= pi_over_4 {
            // if 0 < x <= pi/4, we can call sincos_aux directly
            sincos_aux(x, w)
        } else if *x < 0u32 && *x >= -&pi_over_4 {
            // if -pi/4 <= x < 0, use sin(-x) = -sin(x)
            let (ts, tc, err) = sincos_aux(&-x, w);
            (-ts, tc, err)
        } else {
            // argument reduction is needed
            let pi = Float::pi_prec(if exp_x > 0 {
                w + u64::exact_from(exp_x)
            } else {
                w
            })
            .0 >> 1u32; // pi/2
            // x = q (pi/2 + eps1) + x_red + eps2, where |eps1| <= 1/2 ulp(pi/2) =
            // 2^(-w-max(0,EXP(x))) and eps2 <= 1/2 ulp(x_red) <= 1/2 ulp(pi/2) = 2^(-w). Since |q|
            // <= x/(pi/2) <= |x|, we have q |eps1| <= 2^(-w), thus |x - q pi/2 - x_red| <= 2^(1-w).
            let (mut x_red, _, q) = x.ieee_remainder_and_quotient_bits_prec_ref_ref(&pi, w);
            // now -pi/4 <= x_red <= pi/4: if x_red < 0, consider -x_red
            let neg = x_red < 0u32;
            if neg {
                x_red.neg_assign();
            }
            let (mut ts, mut tc, mut err) = sincos_aux(&x_red, w);
            err += 1; // to take into account the argument reduction
            if neg {
                // sin(-x) = -sin(x), cos(-x) = cos(x)
                ts.neg_assign();
            }
            if q & 2 != 0 {
                // sin(x + pi) = -sin(x), cos(x + pi) = -cos(x)
                ts.neg_assign();
                tc.neg_assign();
            }
            if q.odd() {
                // sin(x + pi/2) = cos(x), cos(x + pi/2) = -sin(x)
                ts.neg_assign();
                swap(&mut ts, &mut tc);
            }
            (ts, tc, err)
        };
        // adjust errors with respect to absolute values
        let w_i = i64::exact_from(w);
        let err_i = i64::exact_from(err);
        let can_round = |t: &Float| {
            t.get_exponent().is_some_and(|e| {
                let bits = w_i - (err_i - i64::from(e));
                bits > 0
                    && float_can_round(
                        t.significand_ref().unwrap(),
                        u64::exact_from(bits),
                        prec,
                        rm,
                    )
            })
        };
        if (!want_sin || can_round(&ts)) && (!want_cos || can_round(&tc)) {
            return (
                want_sin.then(|| Float::from_float_prec_round(ts, prec, rm)),
                want_cos.then(|| Float::from_float_prec_round(tc, prec, rm)),
            );
        }
        w += increment;
        increment = w >> 1;
    }
}

// This is mpfr_sin_cos from sin_cos.c, MPFR 4.2.2, including the `mpfr_sincos_fast` tier for
// precisions at or above `SINCOS_THRESHOLD`, with the near-zero paths of `sin` and `cos` added for
// inputs extremely close to a zero of either function. Both results have precision `prec`, where
// MPFR allows two precisions and works at the larger.
fn sin_cos_prec_round_normal_ref(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    assert_ne!(rm, Exact, "Inexact sin_cos");
    let exp_x = i64::from(x.get_exponent().unwrap());
    let mut m = prec + prec.ceiling_log_base_2() + 13;
    // When x is close to 0, say 2^(-k), then there is a cancellation of about 2k bits in
    // 1-cos(x)^2, and both results may round from x and 1 alone: sin(x) = x - x^3/6 + ... has error
    // below 2^(3 EXP(x) - 2), and cos(x) = 1 - x^2/2 + ... has error below 2^(2 EXP(x) - 1). MPFR
    // tries the sine first and then the cosine; here the cosine's bound is the weaker one, and the
    // reference value 1 always rounds, so it decides.
    if exp_x < 0 {
        let neg_two_exp = u64::exact_from(-(exp_x << 1));
        let err_cos = neg_two_exp + 1;
        if err_cos > prec + 1
            && let Some((s, o_s)) =
                float_round_near_x(x, min(neg_two_exp + 2, prec + 2), false, prec, rm)
        {
            let (c, o_c) = near_one(err_cos, false, prec, rm);
            return (s, c, o_s, o_c);
        }
        m += neg_two_exp;
    }
    if prec >= SINCOS_THRESHOLD {
        let (s, c) = sin_cos_fast(x, prec, rm, true, true);
        let (s, o_s) = s.unwrap();
        let (c, o_c) = c.unwrap();
        return (s, c, o_s, o_c);
    }
    sin_cos_basic(x, exp_x, m, prec, rm)
}

// The basic tier of `sin_cos_prec_round_normal_ref`: the Ziv loop of `mpfr_sin_cos` starting at
// working precision `m`, for a finite nonzero x of exponent `exp_x` that the small-input shortcut
// did not settle.
pub(crate) fn sin_cos_basic(
    x: &Float,
    exp_x: i64,
    mut m: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    let reduce = exp_x >= 2;
    let mut increment = Limb::WIDTH;
    loop {
        match sin_cos_ziv_step(x, exp_x, prec, rm, reduce, &mut m) {
            SinCosStep::Done(s, c) => {
                let (s, o_s) = Float::from_float_prec_round(s, prec, rm);
                let (c, o_c) = Float::from_float_prec_round(c, prec, rm);
                return (s, c, o_s, o_c);
            }
            SinCosStep::NearZeroCos {
                cancel,
                sin_negative,
            } => {
                // 1 - |sin(x)| <= cos(x)^2 < 2^-2cancel
                let (c, o_c) = trig_near_zero(x, prec, rm, cancel, true);
                let (s, o_s) = near_one((cancel << 1) + 1, sin_negative, prec, rm);
                return (s, c, o_s, o_c);
            }
            SinCosStep::NearZeroSin {
                cancel,
                cos_negative,
            } => {
                // 1 - |cos(x)| <= sin(x)^2 / 2 < 2^-(2cancel + 1)
                let (s, o_s) = trig_near_zero(x, prec, rm, cancel, false);
                let (c, o_c) = near_one((cancel << 1) + 2, cos_negative, prec, rm);
                return (s, c, o_s, o_c);
            }
            SinCosStep::Retry => {}
        }
        m += increment;
        increment = m >> 1;
    }
}

// One Ziv iteration for the sine and cosine of a fraction of a turn q, given t = 2 pi q (1 +
// theta)^3 with |theta| <= 2^-w, rounded to w bits. Returns both results when they are settled,
// either from the values at the working precision or, for a result tiny enough, from the exact
// near-zero path, with the other then rounded from ±1; returns `None` if the working precision
// must be raised. `q` produces the exact fraction for the near-zero path, and `sin_near_zero` says
// whether q is large enough for a tiny sine to mean cancellation rather than a tiny q.
fn sin_cos_turns_step(
    t: &Float,
    w: u64,
    prec: u64,
    rm: RoundingMode,
    sin_near_zero: bool,
    q: impl Fn() -> Rational,
) -> Option<(Float, Float, Ordering, Ordering)> {
    // A cancellation of this many bits sends a result to the near-zero path, and leaves the other
    // one within 2^-(prec + 2) of ±1, so that it rounds from ±1 alone.
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, (prec >> 1) + 1);
    // since w >= 2, |(1 + theta)^3 - 1| <= 4 theta, so t = 2 pi q + e with |e| <= 2^(EXP(t) + 2 -
    // w), and both sin and cos move by at most |e|
    let w_i = i64::exact_from(w);
    let err_t = i64::from(t.get_exponent().unwrap()) + 2 - w_i;
    // Both rounded away from zero, so that neither is zero (t is not a multiple of pi/2, being a
    // nonzero `Float`) and the computed magnitudes bound the true ones.
    let (s, c, _, _) = t.sin_cos_prec_round_ref(w, Up);
    let exp_s = i64::from(s.get_exponent().unwrap());
    let exp_c = i64::from(c.get_exponent().unwrap());
    // |sin(2 pi q)| <= |s| + |e| < 2^bound_s, and likewise for the cosine
    let bound_s = max(exp_s, err_t) + 1;
    let bound_c = max(exp_c, err_t) + 1;
    // A tiny sine with q not itself tiny means q is close to a multiple of 1/2, and a tiny cosine
    // means it is close to an odd multiple of 1/4. Either is resolved exactly by the near-zero
    // path, where the Ziv loop would need its precision raised by the whole cancellation, and the
    // other function is then within 2^-2cancel of ±1 and rounds from ±1 alone.
    if bound_s < 0 && sin_near_zero {
        let cancel = u64::exact_from(-bound_s);
        if cancel >= near_zero_threshold
            && let Some((s, o_s)) = trig_turns_near_zero(&q(), prec, rm, false)
        {
            // 1 - |cos(2 pi q)| <= sin(2 pi q)^2 / 2 < 2^(2 bound_s - 1)
            let (c, o_c) = near_one((cancel << 1) + 2, c < 0u32, prec, rm);
            return Some((s, c, o_s, o_c));
        }
    }
    if bound_c < 0 {
        let cancel = u64::exact_from(-bound_c);
        if cancel >= near_zero_threshold
            && let Some((c, o_c)) = trig_turns_near_zero(&q(), prec, rm, true)
        {
            // 1 - |sin(2 pi q)| <= cos(2 pi q)^2 < 2^(2 bound_c)
            let (s, o_s) = near_one((cancel << 1) + 1, s < 0u32, prec, rm);
            return Some((s, c, o_s, o_c));
        }
    }
    // The total error on each result is at most |e| + ulp, bounded by 2^(EXP + 1 - w) if err_t <=
    // EXP - w and by 2^(err_t + 1) otherwise; then normalized for can_round. For the sine, |sin(t)|
    // <= |t| gives EXP(s) <= EXP(t) + 1, so its ulp is at most 2^err_t / 2 and the second bound
    // always applies.
    let err_s = exp_s - err_t - 1;
    let err_c = exp_c
        - if err_t <= exp_c - w_i {
            exp_c - w_i + 1
        } else {
            err_t + 1
        };
    if err_s > 0
        && err_c > 0
        && float_can_round(
            s.significand_ref().unwrap(),
            u64::exact_from(err_s),
            prec,
            rm,
        )
        && float_can_round(
            c.significand_ref().unwrap(),
            u64::exact_from(err_c),
            prec,
            rm,
        )
    {
        let (s, o_s) = Float::from_float_prec_round(s, prec, rm);
        let (c, o_c) = Float::from_float_prec_round(c, prec, rm);
        return Some((s, c, o_s, o_c));
    }
    None
}

// Computes sin(2 pi x/u) and cos(2 pi x/u) for a finite nonzero `Float` x and a nonzero u, rounded
// to precision `prec` with rounding mode `rm`. `rm` may be `Exact` only when both results are
// exact, that is, when x/u is a multiple of 1/4.
//
// MPFR has no combined function here. This joins the `mpfr_sinu` and `mpfr_cosu` ports (see
// `sin_with_period_prec_round_normal_ref` and `cos_with_period_prec_round_normal_ref`) around one
// approximation of 2 pi x/u per Ziv iteration, and one `sin_cos` of it, with the near-zero paths of
// both.
fn sin_cos_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    // Range reduction, as in the sine: xr = x mod u, with the sign of x, exactly.
    let xr;
    let xp = if x.lt_abs(&u) {
        x
    } else {
        let p = i64::exact_from(x.get_prec().unwrap()) - i64::from(x.get_exponent().unwrap());
        let (r, o) =
            x.rem_unsigned_prec_round_ref(u, u64::WIDTH + u64::exact_from(max(p, 0)), Exact);
        assert_eq!(o, Equal);
        if r == 0u32 {
            // x is a multiple of u: the sine is zero, with the sign of x, and the cosine is 1
            return (
                if *x < 0u32 {
                    Float::NEGATIVE_ZERO
                } else {
                    Float::ZERO
                },
                Float::one_prec(prec),
                Equal,
                Equal,
            );
        }
        xr = r;
        &xr
    };
    // now |xp/u| < 1
    let exp_x = i64::from(xp.get_exponent().unwrap());
    // For x/u small, the cosine rounds from 1 alone: |cos(2 pi x/u) - 1| < 2^5 (x/u)^2 <= 2^(5 + 2
    // EXP(x) - 2 log2u), with u >= 2^log2u, as in the cosine. The sine has no such shortcut, being
    // close to 2 pi x/u, which must still be computed, so it takes its own path; there is nothing
    // to share.
    let log2u = if u == 1 {
        0
    } else {
        i64::exact_from(u.ceiling_log_base_2()) - 1
    };
    let err = ((log2u - exp_x) << 1) - 5;
    if err > 0 {
        let err = u64::exact_from(err);
        if err > prec + 1 {
            // such a small x/u is never a special case, and its cosine is never exact
            assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
            let (s, o_s) = sin_with_period_prec_round_normal_ref(xp, u, prec, rm);
            let (c, o_c) = near_one(err, false, prec, rm);
            return (s, c, o_s, o_c);
        }
    }
    let u_bits = i64::exact_from(u.significant_bits());
    // The special cases need |x/u| >= 1/20, so the exponent test skips the `Rational` construction
    // for the small x that would make it expensive. Only a fraction of a turn with both closed
    // forms (a multiple of 1/4, or a denominator of 3, 6, 8, or 12) is taken from them; a fifth,
    // tenth, or twentieth of a turn has only one, and goes through the loop like any other input.
    if exp_x >= u_bits - 5 {
        let q = Rational::exact_from(xp) / Rational::from(u);
        if let Some((s, o_s)) = sin_turns_special_case(&q, prec, rm)
            && let Some((c, o_c)) = cos_turns_special_case(&q, prec, rm)
        {
            return (s, c, o_s, o_c);
        }
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
    if exp_x <= SCALED_INPUT_EXPONENT {
        // 2 pi x/u is within a few bits of the bottom of the exponent range, where the sine may
        // underflow while the cosine has not rounded to 1, which needs a precision beyond 2^31
        // bits: the separate functions handle each.
        fail_on_untested_path("sin_cos_with_period_prec_round_normal_ref, tiny x/u");
        let (s, o_s) = sin_with_period_prec_round_normal_ref(xp, u, prec, rm);
        let (c, o_c) = cos_with_period_prec_round_normal_ref(xp, u, prec, rm);
        return (s, c, o_s, o_c);
    }
    // For x large, since argument reduction is expensive, we want to avoid any failure in Ziv's
    // strategy, thus we take into account expx too.
    let mut prec_t =
        prec + u64::exact_from(max(exp_x, i64::exact_from(prec.ceiling_log_base_2()))) + 8;
    let mut increment = Limb::WIDTH;
    let u_float = Float::from(u);
    // A tiny sine with x/u not itself tiny means cancellation; for a tiny x/u the sine is simply
    // close to 2 pi x/u, and its `Rational` form would be expensive.
    let sin_near_zero = exp_x >= u_bits - 2;
    loop {
        // t = 2*pi*x/u * (1 + theta)^3 where |theta| <= 2^-prec_t, from rounding pi, the product,
        // and the quotient
        let mut t = Float::pi_prec(prec_t).0 << 1u32;
        t.mul_prec_assign_ref(xp, prec_t);
        t.div_prec_assign_ref(&u_float, prec_t);
        if let Some(result) = sin_cos_turns_step(&t, prec_t, prec, rm, sin_near_zero, || {
            Rational::exact_from(xp) / Rational::from(u)
        }) {
            return result;
        }
        prec_t += increment;
        increment = prec_t >> 1;
    }
}

impl Float {
    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`
    /// for it.
    ///
    /// The results are the same as those of [`Float::sin_prec_round`] and
    /// [`Float::cos_prec_round`], but the argument reduction and most of the work are shared, so
    /// this is faster than the two calls when both values are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
    /// $$
    /// - If $x$ is not finite, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be
    ///   0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon_s| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$ and $|\varepsilon_c| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// If the outputs have a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm0.0,p,m)=(\pm0.0,1.0)$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$ and $|\cos x|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_prec_round`] or [`Float::cos_prec_round`]
    ///   does: the sine for an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$ or of
    ///   magnitude $2^{-2^{30}}$ rounded toward zero, and the cosine for an input within
    ///   $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes more than $2^{30}$ bits
    ///   of precision. See those functions for the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sin_cos_round`] instead. If both of these things are true, consider using
    /// [`Float::sin_cos`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the sine and cosine at working precision $n$ (for large $n$ by binary
    /// splitting of the Taylor series, otherwise the cosine, from which the sine is derived) cost
    /// the first term, and for $|x| \geq 2$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_cos_prec_round(5, Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_cos_prec_round(5, Ceiling);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_cos_prec_round(20, Nearest);
    /// assert_eq!(s.to_string(), "0.84147072");
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine
    /// and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0;
    /// let (s, c, o_s, o_c) = x.sin_cos_prec_round_ref(5, Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = x.sin_cos_prec_round_ref(20, Nearest);
    /// assert_eq!(s.to_string(), "0.84147072");
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    pub fn sin_cos_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Self::NAN, Equal, Equal),
            // sin(±0) = ±0 and cos(±0) = 1, exactly
            Zero { .. } => (self.clone(), Self::one_prec(prec), Equal, Equal),
            Finite { .. } => sin_cos_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// Two [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sin_cos`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos_prec(5);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos_prec(20);
    /// assert_eq!(s.to_string(), "0.84147072");
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec(self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos_prec_ref(5);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec_ref(&self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the precision of the input and with the specified rounding mode. The [`Float`] is
    /// taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way with `prec` equal to the
    /// precision of the input.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_cos_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sin_cos`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5).0.sin_cos_round(Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5).0.sin_cos_round(Ceiling);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_round(self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_prec_round_ref(prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the precision of the input and with the specified rounding mode. The [`Float`] is
    /// taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine
    /// and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_round`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5)
    ///     .0
    ///     .sin_cos_round_ref(Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_round_ref(&self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(self.significant_bits(), rm)
    }

    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the specified precision and with the specified rounding mode. The previous value of `cos` is
    /// discarded. Two [`Ordering`]s are returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sin_cos_round_assign`] instead. If both of these things are true, consider
    /// using [`Float::sin_cos_assign`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_prec_round_assign(&mut c, 5, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// ```
    #[inline]
    pub fn sin_cos_prec_round_assign(
        &mut self,
        cos: &mut Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, c, o_s, o_c) = self.sin_cos_prec_round_ref(prec, rm);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the nearest value of the specified precision. The previous value of `cos` is discarded. Two
    /// [`Ordering`]s are returned, indicating whether the rounded sine and cosine are less than,
    /// equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_prec_assign(&mut c, 5), (Greater, Less));
    /// assert_eq!(x.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// ```
    #[inline]
    pub fn sin_cos_prec_assign(&mut self, cos: &mut Self, prec: u64) -> (Ordering, Ordering) {
        self.sin_cos_prec_round_assign(cos, prec, Nearest)
    }

    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the precision of the input and with the specified rounding mode. The previous value of `cos`
    /// is discarded. Two [`Ordering`]s are returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_round`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 5).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_round_assign(&mut c, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// ```
    #[inline]
    pub fn sin_cos_round_assign(
        &mut self,
        cos: &mut Self,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_prec_round_assign(cos, prec, rm)
    }
}

impl Float {
    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the specified precision and with the specified rounding mode, and returning
    /// the results as [`Float`]s. The [`Rational`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values.
    ///
    /// The results are the same as those of [`Float::sin_rational_prec_round`] and
    /// [`Float::cos_rational_prec_round`], but the rounding of the input, the argument reduction,
    /// and most of the work are shared, so this is faster than the two calls when both values are
    /// needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p+1}$
    ///   and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon_s| \leq 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and
    ///   $|\varepsilon_c| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// These bounds do not apply when a result underflows.
    ///
    /// The outputs have precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$ and $|\cos x|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_rational_prec_round`] or
    ///   [`Float::cos_rational_prec_round`] does; see those functions for the inputs concerned and
    ///   the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine and cosine taken there together,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the specified precision and with the specified rounding mode, and returning
    /// the results as [`Float`]s. The [`Rational`] is taken by reference. Two [`Ordering`]s are
    /// also returned, indicating whether the rounded sine and cosine are less than, equal to, or
    /// greater than the exact values.
    ///
    /// See [`Float::sin_cos_rational_prec_round`] for the error bounds, the special cases, overflow
    /// and underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(s.to_string(), "0.56464195");
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    pub fn sin_cos_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // sin(0) = 0 and cos(0) = 1, exactly
            return (Self::ZERO, Self::one_prec(prec), Equal, Equal);
        }
        sin_cos_rational_helper(x, prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the nearest value of the specified precision, and returning the results as
    /// [`Float`]s. The [`Rational`] is taken by value. Two [`Ordering`]s are also returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_rational_prec_round`] for the error bounds, the special cases, overflow
    /// and underflow, and the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_rational_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(s.to_string(), "0.56464291");
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_rational_prec(x: Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the nearest value of the specified precision, and returning the results as
    /// [`Float`]s. The [`Rational`] is taken by reference. Two [`Ordering`]s are also returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values.
    ///
    /// See [`Float::sin_cos_rational_prec`] and [`Float::sin_cos_rational_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_rational_prec_round_ref(x, prec, Nearest)
    }
}

// Computes sin(2 pi q) and cos(2 pi q) for a nonzero `Rational` fraction of a turn q in (-1, 1),
// rounded to precision `prec` with rounding mode `rm`. `rm` may be `Exact` only when both results
// are exact, that is, when q is a multiple of 1/4. This is the `Float` algorithm with the fraction
// of a turn taken directly: since q is exact, only pi and the product are rounded.
fn sin_cos_turns_helper(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    let exp_q = q.floor_log_base_2_abs() + 1;
    // for q small, the cosine rounds from 1 alone: |cos(2 pi q) - 1| < 1/2 (2 pi q)^2 < 2^(5 + 2
    // EXP(q)); the sine takes its own path, as in the `Float` version
    let err = -(exp_q << 1) - 5;
    if err > 0 {
        let err = u64::exact_from(err);
        if err > prec + 1 {
            assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
            let (s, o_s) = sin_turns_helper(q, prec, rm);
            let (c, o_c) = near_one(err, false, prec, rm);
            return (s, c, o_s, o_c);
        }
    }
    // The special cases need |q| >= 1/20; only a q with both closed forms is taken from them
    if exp_q >= -4
        && let Some((s, o_s)) = sin_turns_special_case(q, prec, rm)
        && let Some((c, o_c)) = cos_turns_special_case(q, prec, rm)
    {
        return (s, c, o_s, o_c);
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
    if exp_q <= SCALED_INPUT_EXPONENT {
        // as in the `Float` version, only reachable beyond 2^31 bits of precision
        fail_on_untested_path("sin_cos_turns_helper, tiny q");
        let (s, o_s) = sin_turns_helper(q, prec, rm);
        let (c, o_c) = cos_turns_helper(q, prec, rm);
        return (s, c, o_s, o_c);
    }
    let mut w = prec + prec.ceiling_log_base_2() + 8;
    let mut increment = Limb::WIDTH;
    let sin_near_zero = exp_q >= -2;
    loop {
        // t = 2*pi*q * (1 + theta)^3 where |theta| <= 2^-w, from rounding q, pi, and the product
        let t = (Float::pi_prec(w).0 << 1u32)
            .mul_prec(Float::from_rational_prec_ref(q, w).0, w)
            .0;
        if let Some(result) = sin_cos_turns_step(&t, w, prec, rm, sin_near_zero, || q.clone()) {
            return result;
        }
        w += increment;
        increment = w >> 1;
    }
}

impl Float {
    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the specified precision and with the
    /// specified rounding mode. The [`Float`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// The results are the same as those of [`Float::sin_with_period_prec_round`] and
    /// [`Float::cos_with_period_prec_round`], but the argument reduction, the computation of $2\pi
    /// x/u$, and most of the work are shared, so this is faster than the two calls when both values
    /// are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or
    ///   assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon_s| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$ and $|\varepsilon_c| < 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon_s| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$ and $|\varepsilon_c| \leq 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the outputs have a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,u,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm0.0,u,p,m)=(\pm0.0,1.0)$
    /// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign
    ///   of $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$, as for
    ///   [`Float::sin_with_period_prec_round`] and [`Float::cos_with_period_prec_round`].
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 12, one result is exactly $\pm1/2$ or
    /// both are $\pm\sqrt2/2$, and the other is $\pm\sqrt3/2$; these are computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine and cosine, which is far
    /// faster. (A fifth, tenth, or twentieth of a turn has a closed form for only one of the two,
    /// and is computed like any other input.)
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$ and $|\cos(2\pi x/u)|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_with_period_prec_round`] or
    ///   [`Float::cos_with_period_prec_round`] does: the sine for $x/u$ within $2^{-2^{30}}$ of a
    ///   multiple of $1/2$ without being one, or for an $x$ so small that $2\pi x/u$ is below
    ///   $2^{-2^{30}}$, and the cosine for $x/u$ within $2^{-2^{30}}$ of an odd multiple of $1/4$
    ///   without being one, which takes more than $2^{30}$ bits of precision. See those functions
    ///   for the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sin_cos_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n (\log n)^3 \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine and cosine of
    /// $2\pi x/u$ are then taken together at a working precision of about $n + e$ bits, which needs
    /// $\pi$ to that many bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round(7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round(7, 10, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round(7, 10, Nearest);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// // a quarter turn is exact
    /// let (s, c, o_s, o_c) = Float::from(90u32).sin_cos_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(s.to_string(), "1.0000");
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Equal);
    ///
    /// // a twelfth of a turn: 1/2 exactly, and sqrt(3)/2
    /// let (s, c, o_s, o_c) = Float::from(30u32).sin_cos_with_period_prec_round(360, 10, Nearest);
    /// assert_eq!(s.to_string(), "0.50000");
    /// assert_eq!(c.to_string(), "0.86621");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the specified precision and with the
    /// specified rounding mode. The [`Float`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round_ref(7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round_ref(7, 10, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    pub fn sin_cos_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // for u=0, return NaN
            _ if u == 0 => (Self::NAN, Self::NAN, Equal, Equal),
            NaN | Infinity { .. } => (Self::NAN, Self::NAN, Equal, Equal),
            // x is zero: sin(±0) = ±0 and cos(±0) = 1
            Zero { .. } => (self.clone(), Self::one_prec(prec), Equal, Equal),
            Finite { .. } => sin_cos_with_period_prec_round_normal_ref(self, u, prec, rm),
        }
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the nearest value of the specified
    /// precision. The [`Float`] is taken by value. Two [`Ordering`]s are also returned, indicating
    /// whether the rounded sine and cosine are less than, equal to, or greater than the exact
    /// values. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal` for it.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_with_period_prec_round`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::sin_cos_with_period_round`] with
    /// `Nearest` instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec(7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec(360, 53);
    /// assert_eq!(s.to_string(), "0.017452406437283512");
    /// assert_eq!(c.to_string(), "0.99984769515639127");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    ///
    /// // an eighth of a turn: sqrt(2)/2 for both
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec(8, 10);
    /// assert_eq!(s.to_string(), "0.70703");
    /// assert_eq!(c.to_string(), "0.70703");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec(self, u: u64, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the nearest value of the specified
    /// precision. The [`Float`] is taken by reference. Two [`Ordering`]s are also returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_ref(7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_ref(
        &self,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the precision of the input and with
    /// the specified rounding mode. The [`Float`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way with `prec`
    /// equal to the precision of the input.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sin_cos_with_period_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::sin_cos_with_period_prec`] with the precision of the
    /// input instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input (which is the case unless $x/u$ is a multiple of $1/4$, or $x$ is zero or not
    /// finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_cos_with_period_round(7, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_cos_with_period_round(7, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_round(
        self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the precision of the input and with
    /// the specified rounding mode. The [`Float`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_round`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input (which is the case unless $x/u$ is a multiple of $1/4$, or $x$ is zero or not
    /// finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_cos_with_period_round_ref(7, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_round_ref(
        &self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its sine and writes its cosine to
    /// `cos`, rounding both results to the specified precision and with the specified rounding
    /// mode. The previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating
    /// whether the rounded sine and cosine are less than, equal to, or greater than the exact
    /// values. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sin_cos_with_period_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::sin_cos_with_period_round_assign`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 10, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_round_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, c, o_s, o_c) = self.sin_cos_with_period_prec_round_ref(u, prec, rm);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its sine and writes its cosine to
    /// `cos`, rounding both results to the nearest value of the specified precision. The previous
    /// value of `cos` is discarded. Two [`Ordering`]s are returned, indicating whether the rounded
    /// sine and cosine are less than, equal to, or greater than the exact values. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_assign(&mut c, 7, 10),
    ///     (Greater, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        prec: u64,
    ) -> (Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_assign(cos, u, prec, Nearest)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its sine and writes its cosine to
    /// `cos`, rounding both results to the precision of the input and with the specified rounding
    /// mode. The previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating
    /// whether the rounded sine and cosine are less than, equal to, or greater than the exact
    /// values. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_round`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input (which is the case unless $x/u$ is a multiple of $1/4$, or $x$ is zero or not
    /// finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_round_assign(&mut c, 7, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_round_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_with_period_prec_round_assign(cos, u, prec, rm)
    }
}

impl Float {
    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the specified precision and
    /// with the specified rounding mode, and returning the results as [`Float`]s. The [`Rational`]
    /// is taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine
    /// and cosine are less than, equal to, or greater than the exact values. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal` for it.
    ///
    /// The results are the same as those of [`Float::sin_with_period_rational_prec_round`] and
    /// [`Float::cos_with_period_rational_prec_round`], but the reduction of the fraction of a turn,
    /// the computation of $2\pi x/u$, and most of the work are shared, so this is faster than the
    /// two calls when both values are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
    /// $$
    /// - If $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be 0.
    /// - If $u\neq 0$ and $m$ is not `Nearest`, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p+1}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $u\neq 0$ and $m$ is `Nearest`, then $|\varepsilon_s| \leq 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p}$ and $|\varepsilon_c| \leq 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the outputs have a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p,m)=(0,1)$
    /// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign
    ///   of $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$, as for
    ///   [`Float::sin_with_period_rational_prec_round`] and
    ///   [`Float::cos_with_period_rational_prec_round`].
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 12, one result is exactly $\pm1/2$ or
    /// both are $\pm\sqrt2/2$, and the other is $\pm\sqrt3/2$; these are computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine and cosine, which is far
    /// faster. (A fifth, tenth, or twentieth of a turn has a closed form for only one of the two,
    /// and is computed like any other input.)
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$ and $|\cos(2\pi x/u)|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_with_period_rational_prec_round`] or
    ///   [`Float::cos_with_period_rational_prec_round`] does: the sine for $x/u$ within
    ///   $2^{-2^{30}}$ of a multiple of $1/2$ without being one, or for an $x/u$ so small that
    ///   $2\pi x/u$ is below $2^{-2^{30}}$, and the cosine for $x/u$ within $2^{-2^{30}}$ of an odd
    ///   multiple of $1/4$ without being one, which takes a denominator of more than $2^{30}$ bits.
    ///   See those functions for the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sin_cos_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the fraction of a turn is reduced modulo 1 exactly, so only its size
    /// and the precision drive the cost, not the magnitude of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_with_period_rational_prec_round(Rational::ONE, 7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_with_period_rational_prec_round(Rational::ONE, 7, 10, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// // a quarter turn is exact
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 4),
    ///     1,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(s.to_string(), "1.0000");
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Equal);
    ///
    /// // a twelfth of a turn: 1/2 exactly, and sqrt(3)/2
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 12),
    ///     1,
    ///     10,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.50000");
    /// assert_eq!(c.to_string(), "0.86621");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the specified precision and
    /// with the specified rounding mode, and returning the results as [`Float`]s. The [`Rational`]
    /// is taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded
    /// sine and cosine are less than, equal to, or greater than the exact values. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_rational_prec_round`] for the error bounds, the special
    /// cases, overflow and underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// // an eighth of a turn: sqrt(2)/2 for both
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 8),
    ///     1,
    ///     10,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.70703");
    /// assert_eq!(c.to_string(), "0.70703");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    pub fn sin_cos_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        // for u = 0, return NaN
        if u == 0 {
            return (Self::NAN, Self::NAN, Equal, Equal);
        }
        // sin(0) = 0 (a `Rational` zero has no sign) and cos(0) = 1
        if *x == 0u32 {
            return (Self::ZERO, Self::one_prec(prec), Equal, Equal);
        }
        // q = x/u, reduced to (-1, 1) with the sign of x: both functions have period 1 in q, and a
        // multiple of u gives a sine of zero with the sign of x (IEEE 754-2019's sinPi) and a
        // cosine of 1
        let q = x / Rational::from(u);
        let whole = Rational::from(Integer::rounding_from(&q, Down).0);
        let q = q - whole;
        if q == 0u32 {
            return (
                if *x < 0u32 {
                    Self::NEGATIVE_ZERO
                } else {
                    Self::ZERO
                },
                Self::one_prec(prec),
                Equal,
                Equal,
            );
        }
        sin_cos_turns_helper(&q, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the nearest value of the
    /// specified precision, and returning the results as [`Float`]s. The [`Rational`] is taken by
    /// value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`
    /// for it.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_with_period_rational_prec_round`] for the error bounds, the special
    /// cases, overflow and underflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_with_period_rational_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec(Rational::ONE, 7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec(Rational::ONE, 360, 53);
    /// assert_eq!(s.to_string(), "0.017452406437283512");
    /// assert_eq!(c.to_string(), "0.99984769515639127");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_with_period_rational_prec(
        x: Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the nearest value of the
    /// specified precision, and returning the results as [`Float`]s. The [`Rational`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`
    /// for it.
    ///
    /// See [`Float::sin_cos_with_period_rational_prec`] and
    /// [`Float::sin_cos_with_period_rational_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_ref(&Rational::ONE, 7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_rational_prec_ref(
        x: &Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }
}

impl Float {
    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Float`] measured in
    /// half-turns, together, rounding both results to the specified precision and with the
    /// specified rounding mode. The [`Float`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see
    /// [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special and closed-form
    /// cases (multiples of $1/2$ give exact pairs from $\pm0.0$ and $\pm1$, and odd multiples of
    /// $1/6$, $1/4$, and $1/3$ have closed forms for both), overflow and underflow, and the
    /// complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from(0.1f64).sin_cos_pi_prec_round(10, Floor);
    /// assert_eq!(s.to_string(), "0.30859");
    /// assert_eq!(c.to_string(), "0.95020");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from(0.1f64).sin_cos_pi_prec_round(10, Ceiling);
    /// assert_eq!(s.to_string(), "0.30908");
    /// assert_eq!(c.to_string(), "0.95117");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// // a half-turn is exact
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_pi_prec_round(10, Exact);
    /// assert_eq!(s.to_string(), "0.0");
    /// assert_eq!(c.to_string(), "-1.0000");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Equal);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round(2, prec, rm)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Float`] measured in
    /// half-turns, together, rounding both results to the specified precision and with the
    /// specified rounding mode. The [`Float`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see
    /// [`Float::sin_cos_with_period_prec_round_ref`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from(0.1f64).sin_cos_pi_prec_round_ref(10, Floor);
    /// assert_eq!(s.to_string(), "0.30859");
    /// assert_eq!(c.to_string(), "0.95020");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(2, prec, rm)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Float`] measured in
    /// half-turns, together, rounding both results to the nearest value of the specified precision.
    /// The [`Float`] is taken by value. Two [`Ordering`]s are also returned, indicating whether the
    /// rounded sine and cosine are less than, equal to, or greater than the exact values. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see [`Float::sin_cos_with_period_prec`]
    /// for the error bounds, the special and closed-form cases, overflow and underflow, and the
    /// complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from(0.1f64).sin_cos_pi_prec(10);
    /// assert_eq!(s.to_string(), "0.30908");
    /// assert_eq!(c.to_string(), "0.95117");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec(self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec(2, prec)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Float`] measured in
    /// half-turns, together, rounding both results to the nearest value of the specified precision.
    /// The [`Float`] is taken by reference. Two [`Ordering`]s are also returned, indicating whether
    /// the rounded sine and cosine are less than, equal to, or greater than the exact values.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see
    /// [`Float::sin_cos_with_period_prec_ref`] for the error bounds, the special and closed-form
    /// cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from(0.1f64).sin_cos_pi_prec_ref(10);
    /// assert_eq!(s.to_string(), "0.30908");
    /// assert_eq!(c.to_string(), "0.95117");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_ref(&self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_ref(2, prec)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Float`] measured in
    /// half-turns, together, rounding both results to the precision of the input and with the
    /// specified rounding mode. The [`Float`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see [`Float::sin_cos_with_period_round`]
    /// for the error bounds, the special and closed-form cases, overflow and underflow, and the
    /// complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from(0.1f64).sin_cos_pi_round(Floor);
    /// assert_eq!(s.to_string(), "0.30901699437494734");
    /// assert_eq!(c.to_string(), "0.95105651629515342");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_round(self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_round(2, rm)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Float`] measured in
    /// half-turns, together, rounding both results to the precision of the input and with the
    /// specified rounding mode. The [`Float`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see
    /// [`Float::sin_cos_with_period_round_ref`] for the error bounds, the special and closed-form
    /// cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from(0.1f64).sin_cos_pi_round_ref(Floor);
    /// assert_eq!(s.to_string(), "0.30901699437494734");
    /// assert_eq!(c.to_string(), "0.95105651629515342");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_round_ref(&self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_round_ref(2, rm)
    }

    /// Replaces a [`Float`] measured in half-turns with its sine and writes its cosine to `cos`,
    /// rounding both results to the specified precision and with the specified rounding mode. The
    /// previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating whether the
    /// rounded sine and cosine are less than, equal to, or greater than the exact values. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see
    /// [`Float::sin_cos_with_period_prec_round_assign`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sin_cos_pi_prec_round_assign(&mut c, 10, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.30859");
    /// assert_eq!(c.to_string(), "0.95020");
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_round_assign(
        &mut self,
        cos: &mut Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_assign(cos, 2, prec, rm)
    }

    /// Replaces a [`Float`] measured in half-turns with its sine and writes its cosine to `cos`,
    /// rounding both results to the nearest value of the specified precision. The previous value of
    /// `cos` is discarded. Two [`Ordering`]s are returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets a `NaN` it also returns `Equal` for
    /// it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see
    /// [`Float::sin_cos_with_period_prec_assign`] for the error bounds, the special and closed-form
    /// cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_pi_prec_assign(&mut c, 10), (Greater, Greater));
    /// assert_eq!(x.to_string(), "0.30908");
    /// assert_eq!(c.to_string(), "0.95117");
    /// ```
    #[inline]
    pub fn sin_cos_pi_prec_assign(&mut self, cos: &mut Self, prec: u64) -> (Ordering, Ordering) {
        self.sin_cos_with_period_prec_assign(cos, 2, prec)
    }

    /// Replaces a [`Float`] measured in half-turns with its sine and writes its cosine to `cos`,
    /// rounding both results to the precision of the input and with the specified rounding mode.
    /// The previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating whether
    /// the rounded sine and cosine are less than, equal to, or greater than the exact values.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal` for it.
    ///
    /// This is `sin_cos_with_period` with a period of 2: see
    /// [`Float::sin_cos_with_period_round_assign`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_pi_round_assign(&mut c, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "0.30901699437494734");
    /// assert_eq!(c.to_string(), "0.95105651629515342");
    /// ```
    #[inline]
    pub fn sin_cos_pi_round_assign(
        &mut self,
        cos: &mut Self,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        self.sin_cos_with_period_round_assign(cos, 2, rm)
    }
}

impl Float {
    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, together, rounding both results to the specified precision and with the
    /// specified rounding mode, and returning the results as [`Float`]s. The [`Rational`] is taken
    /// by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values.
    ///
    /// This is `sin_cos_with_period_rational` with a period of 2: see
    /// [`Float::sin_cos_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases (multiples of $1/2$ give exact pairs from $\pm0.0$ and $\pm1$, and odd
    /// multiples of $1/6$, $1/4$, and $1/3$ have closed forms for both), overflow and underflow,
    /// and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_pi_rational_prec_round(Rational::from_unsigneds(1u8, 7), 10, Floor);
    /// assert_eq!(s.to_string(), "0.43359");
    /// assert_eq!(c.to_string(), "0.90039");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// // a sixth of a half-turn: 1/2 exactly, and sqrt(3)/2
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_pi_rational_prec_round(Rational::from_unsigneds(1u8, 6), 10, Nearest);
    /// assert_eq!(s.to_string(), "0.50000");
    /// assert_eq!(c.to_string(), "0.86621");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(&x, 2, prec, rm)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, together, rounding both results to the specified precision and with the
    /// specified rounding mode, and returning the results as [`Float`]s. The [`Rational`] is taken
    /// by reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values.
    ///
    /// This is `sin_cos_with_period_rational` with a period of 2: see
    /// [`Float::sin_cos_with_period_rational_prec_round_ref`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_pi_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 7), 10, Floor);
    /// assert_eq!(s.to_string(), "0.43359");
    /// assert_eq!(c.to_string(), "0.90039");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(x, 2, prec, rm)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, together, rounding both results to the nearest value of the specified precision,
    /// and returning the results as [`Float`]s. The [`Rational`] is taken by value. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values.
    ///
    /// This is `sin_cos_with_period_rational` with a period of 2: see
    /// [`Float::sin_cos_with_period_rational_prec`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_pi_rational_prec(Rational::from_unsigneds(1u8, 7), 10);
    /// assert_eq!(s.to_string(), "0.43408");
    /// assert_eq!(c.to_string(), "0.90137");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_pi_rational_prec(x: Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_ref(&x, 2, prec)
    }

    /// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
    /// half-turns, together, rounding both results to the nearest value of the specified precision,
    /// and returning the results as [`Float`]s. The [`Rational`] is taken by reference. Two
    /// [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values.
    ///
    /// This is `sin_cos_with_period_rational` with a period of 2: see
    /// [`Float::sin_cos_with_period_rational_prec_ref`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 7), 10);
    /// assert_eq!(s.to_string(), "0.43408");
    /// assert_eq!(c.to_string(), "0.90137");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_pi_rational_prec_ref(
        x: &Rational,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_ref(x, 2, prec)
    }
}

impl SinCos for Float {
    type Output = Self;

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, taking it by
    /// value.
    ///
    /// If the outputs have a precision, it is the precision of the input. If a result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
    /// $$
    /// - If $x$ is not finite, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be
    ///   0.
    /// - If $x$ is finite, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and
    ///   $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty)=(\text{NaN},\text{NaN})$
    /// - $f(\pm0.0)=(\pm0.0,1.0)$
    ///
    /// See [`Float::sin_cos_prec_round`] for overflow, underflow, and the complexity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_round`] instead. If you want to specify an output precision, consider using
    /// [`Float::sin_cos_prec`] instead. If you want both of these things, consider using
    /// [`Float::sin_cos_prec_round`] instead.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinCos;
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    ///
    /// let (s, c) = Float::NAN.sin_cos();
    /// assert!(s.is_nan());
    /// assert!(c.is_nan());
    ///
    /// let (s, c) = Float::ZERO.sin_cos();
    /// assert_eq!(s.to_string(), "0.0");
    /// assert_eq!(c.to_string(), "1.0");
    ///
    /// let (s, c) = Float::NEGATIVE_ZERO.sin_cos();
    /// assert_eq!(s.to_string(), "-0.0");
    /// assert_eq!(c.to_string(), "1.0");
    ///
    /// let (s, c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos();
    /// assert_eq!(s.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// ```
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        let prec = self.significant_bits();
        let (s, c, _, _) = self.sin_cos_prec_round_ref(prec, Nearest);
        (s, c)
    }
}

impl SinCos for &Float {
    type Output = Float;

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, taking it by
    /// reference.
    ///
    /// See [`Float::sin_cos`]; this function behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinCos;
    /// use malachite_float::Float;
    ///
    /// let (s, c) = (&Float::from_unsigned_prec(1u32, 100).0).sin_cos();
    /// assert_eq!(s.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// ```
    #[inline]
    fn sin_cos(self) -> (Float, Float) {
        let (s, c, _, _) = self.sin_cos_prec_round_ref(self.significant_bits(), Nearest);
        (s, c)
    }
}

impl SinCosAssign for Float {
    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the nearest value of the input's precision. The previous value of `cos` is discarded.
    ///
    /// See [`Float::sin_cos`]; this function behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinCosAssign;
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// x.sin_cos_assign(&mut c);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// ```
    #[inline]
    fn sin_cos_assign(&mut self, cos: &mut Self) {
        let prec = self.significant_bits();
        self.sin_cos_prec_round_assign(cos, prec, Nearest);
    }
}

/// Computes $\sin x$ and $\cos x$, the sine and cosine of a primitive float, together. Using this
/// function is more accurate than using the default `sin_cos` function or the ones provided by
/// `libm`.
///
/// The results are those of
/// [`primitive_float_sin`](crate::float::arithmetic::sin::primitive_float_sin) and
/// [`primitive_float_cos`](crate::float::arithmetic::cos::primitive_float_cos), but the argument
/// reduction and most of the work are shared, so this is faster than the two calls when both values
/// are needed.
///
/// $$
/// f(x) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
/// $$
/// - If $x$ is not finite, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and
///   $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is the precision of the
///   output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=(\text{NaN},\text{NaN})$
/// - $f(\pm\infty)=(\text{NaN},\text{NaN})$
/// - $f(\pm0.0)=(\pm0.0,1.0)$
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine is subnormal only when
/// $x$ is, and then it is $x$ itself; the cosine is never subnormal. See
/// [`primitive_float_sin`](crate::float::arithmetic::sin::primitive_float_sin) and
/// [`primitive_float_cos`](crate::float::arithmetic::cos::primitive_float_cos).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos;
///
/// let (s, c) = primitive_float_sin_cos(f32::NAN);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos(0.0f32);
/// assert_eq!(NiceFloat(s), NiceFloat(0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0));
///
/// let (s, c) = primitive_float_sin_cos(1.0f32);
/// assert_eq!(NiceFloat(s), NiceFloat(0.84147096));
/// assert_eq!(NiceFloat(c), NiceFloat(0.5403023));
///
/// let (s, c) = primitive_float_sin_cos(1.0f64);
/// assert_eq!(NiceFloat(s), NiceFloat(0.8414709848078965));
/// assert_eq!(NiceFloat(c), NiceFloat(0.5403023058681398));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos<T: PrimitiveFloat>(x: T) -> (T, T)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_pair_fn(Float::sin_cos_prec, x)
}

/// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, returning the
/// results as primitive floats.
///
/// The results are those of
/// [`primitive_float_sin_rational`](crate::float::arithmetic::sin::primitive_float_sin_rational)
/// and
/// [`primitive_float_cos_rational`](crate::float::arithmetic::cos::primitive_float_cos_rational),
/// but the rounding of the input, the argument reduction, and most of the work are shared, so this
/// is faster than the two calls when both values are needed.
///
/// $$
/// f(x) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c),
/// $$
/// where $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and $|\varepsilon_c| <
/// 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, and $p$ is the precision of the output (24 if `T` is a
/// [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=(0,1)$
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine underflows, to a
/// subnormal or to zero, when $x$ is tiny, since $\sin x$ is then very close to $x$; the cosine is
/// never subnormal. See
/// [`primitive_float_sin_rational`](crate::float::arithmetic::sin::primitive_float_sin_rational)
/// and
/// [`primitive_float_cos_rational`](crate::float::arithmetic::cos::primitive_float_cos_rational).
///
/// # Worst-case complexity
/// $T(m, e) = O((m+e) (\log (m+e))^2 \log\log (m+e))$
///
/// $M(m, e) = O((m+e) \log (m+e))$
///
/// where $T$ is time, $M$ is additional memory, $m$ is `x.significant_bits()`, and $e$ is
/// `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): for $|x| \geq 2$ the
/// argument is reduced modulo $2\pi$, which needs $\pi$ to about $e$ bits.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_rational;
/// use malachite_q::Rational;
///
/// let (s, c) = primitive_float_sin_cos_rational::<f64>(&Rational::ZERO);
/// assert_eq!(NiceFloat(s), NiceFloat(0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0));
///
/// let (s, c) = primitive_float_sin_cos_rational::<f64>(&Rational::from_unsigneds(1u8, 3));
/// assert_eq!(NiceFloat(s), NiceFloat(0.32719469679615226));
/// assert_eq!(NiceFloat(c), NiceFloat(0.9449569463147377));
///
/// let (s, c) = primitive_float_sin_cos_rational::<f32>(&Rational::from_unsigneds(1u8, 3));
/// assert_eq!(NiceFloat(s), NiceFloat(0.3271947));
/// assert_eq!(NiceFloat(c), NiceFloat(0.94495696));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos_rational<T: PrimitiveFloat>(x: &Rational) -> (T, T)
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_pair_fn(Float::sin_cos_rational_prec_ref, x)
}

/// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a primitive float
/// measured in $u$ths of a turn (so that `u = 360` is degrees), together.
///
/// The results are those of
/// [`primitive_float_sin_with_period`](super::sin::primitive_float_sin_with_period) and
/// [`primitive_float_cos_with_period`](super::cos::primitive_float_cos_with_period), but the
/// argument reduction and most of the work are shared, so this is faster than the two calls when
/// both values are needed.
///
/// $$
/// f(x,u) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
/// $$
/// - If $x$ is not finite or $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed
///   to be 0.
/// - If $x$ is finite and $u\neq 0$, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin(2\pi
///   x/u)|\rfloor-p}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$, where
///   $p$ is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},u)=(\text{NaN},\text{NaN})$
/// - $f(\pm\infty,u)=(\text{NaN},\text{NaN})$
/// - $f(x,0)=(\text{NaN},\text{NaN})$
/// - $f(\pm0.0,u)=(\pm0.0,1.0)$
/// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign of
///   $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$.
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine underflows, to a
/// subnormal or to zero, only when $2\pi x/u$ does, which takes a subnormal $x$ or a large $u$; the
/// cosine is never subnormal. See
/// [`primitive_float_sin_with_period`](super::sin::primitive_float_sin_with_period) and
/// [`primitive_float_cos_with_period`](super::cos::primitive_float_cos_with_period).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_with_period;
///
/// let (s, c) = primitive_float_sin_cos_with_period(f32::NAN, 360);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos_with_period(1.0f32, 0);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos_with_period(90.0f32, 360);
/// assert_eq!(NiceFloat(s), NiceFloat(1.0));
/// assert_eq!(NiceFloat(c), NiceFloat(0.0));
///
/// let (s, c) = primitive_float_sin_cos_with_period(30.0f64, 360);
/// assert_eq!(NiceFloat(s), NiceFloat(0.5));
/// assert_eq!(NiceFloat(c), NiceFloat(0.8660254037844386));
///
/// let (s, c) = primitive_float_sin_cos_with_period(1.0f32, 7);
/// assert_eq!(NiceFloat(s), NiceFloat(0.7818315));
/// assert_eq!(NiceFloat(c), NiceFloat(0.6234898));
///
/// let (s, c) = primitive_float_sin_cos_with_period(1.0f64, 7);
/// assert_eq!(NiceFloat(s), NiceFloat(0.7818314824680298));
/// assert_eq!(NiceFloat(c), NiceFloat(0.6234898018587335));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos_with_period<T: PrimitiveFloat>(x: T, u: u64) -> (T, T)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_pair_fn(|x, prec| Float::sin_cos_with_period_prec(x, u, prec), x)
}

/// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`] measured
/// in $u$ths of a turn (so that `u = 360` is degrees), together, returning the results as primitive
/// floats.
///
/// The results are those of
/// [`primitive_float_sin_with_period_rational`](super::sin::primitive_float_sin_with_period_rational)
/// and
/// [`primitive_float_cos_with_period_rational`](super::cos::primitive_float_cos_with_period_rational),
/// but the reduction of the fraction of a turn and most of the work are shared, so this is faster
/// than the two calls when both values are needed.
///
/// $$
/// f(x,u) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
/// $$
/// - If $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be 0.
/// - If $u\neq 0$, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$ and
///   $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$, where $p$ is the precision of
///   the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,0)=(\text{NaN},\text{NaN})$
/// - $f(0,u)=(0,1)$
/// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign of
///   $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$.
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine underflows, to a
/// subnormal or to zero, only when $2\pi x/u$ does, for a tiny $x/u$; the cosine is never
/// subnormal. See
/// [`primitive_float_sin_with_period_rational`](super::sin::primitive_float_sin_with_period_rational)
/// and
/// [`primitive_float_cos_with_period_rational`](super::cos::primitive_float_cos_with_period_rational).
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`: the fraction of
/// a turn is reduced modulo 1 exactly, so the magnitude of $x$ does not drive the cost.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_with_period_rational;
/// use malachite_q::Rational;
///
/// let (s, c) = primitive_float_sin_cos_with_period_rational::<f64>(&Rational::ZERO, 0);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos_with_period_rational::<f64>(&Rational::ZERO, 360);
/// assert_eq!(NiceFloat(s), NiceFloat(0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0));
///
/// // a twelfth of a turn: exactly 1/2, and sqrt(3)/2
/// let (s, c) =
///     primitive_float_sin_cos_with_period_rational::<f64>(&Rational::from_unsigneds(1u8, 12), 1);
/// assert_eq!(NiceFloat(s), NiceFloat(0.5));
/// assert_eq!(NiceFloat(c), NiceFloat(0.8660254037844386));
///
/// let (s, c) =
///     primitive_float_sin_cos_with_period_rational::<f32>(&Rational::from_unsigneds(1u8, 7), 1);
/// assert_eq!(NiceFloat(s), NiceFloat(0.7818315));
/// assert_eq!(NiceFloat(c), NiceFloat(0.6234898));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub fn primitive_float_sin_cos_with_period_rational<T: PrimitiveFloat>(
    x: &Rational,
    u: u64,
) -> (T, T)
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_pair_fn(
        |x, prec| Float::sin_cos_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}

/// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a primitive float measured in
/// half-turns, together.
///
/// This is `primitive_float_sin_cos_with_period` with a period of 2: see
/// [`primitive_float_sin_cos_with_period`] for the error bounds and the special cases, with $u =
/// 2$. Multiples of $1/2$ give exact pairs from $\pm0.0$ (with the sign of the input for the sine)
/// and $\pm1$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_pi;
///
/// let (s, c) = primitive_float_sin_cos_pi(f32::NAN);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos_pi(0.5f32);
/// assert_eq!(NiceFloat(s), NiceFloat(1.0));
/// assert_eq!(NiceFloat(c), NiceFloat(0.0));
///
/// let (s, c) = primitive_float_sin_cos_pi(1.0f64);
/// assert_eq!(NiceFloat(s), NiceFloat(0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(-1.0));
///
/// let (s, c) = primitive_float_sin_cos_pi(0.1f32);
/// assert_eq!(NiceFloat(s), NiceFloat(0.309017));
/// assert_eq!(NiceFloat(c), NiceFloat(0.95105654));
///
/// let (s, c) = primitive_float_sin_cos_pi(0.1f64);
/// assert_eq!(NiceFloat(s), NiceFloat(0.30901699437494745));
/// assert_eq!(NiceFloat(c), NiceFloat(0.9510565162951535));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos_pi<T: PrimitiveFloat>(x: T) -> (T, T)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_sin_cos_with_period(x, 2)
}

/// Computes $\sin(\pi x)$ and $\cos(\pi x)$, the sine and cosine of a [`Rational`] measured in
/// half-turns, together, returning the results as primitive floats.
///
/// This is `primitive_float_sin_cos_with_period_rational` with a period of 2: see
/// [`primitive_float_sin_cos_with_period_rational`] for the error bounds, the special cases, and
/// the complexity, with $u = 2$.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_pi_rational;
/// use malachite_q::Rational;
///
/// // a sixth of a half-turn: exactly 1/2, and sqrt(3)/2
/// let (s, c) = primitive_float_sin_cos_pi_rational::<f64>(&Rational::from_unsigneds(1u8, 6));
/// assert_eq!(NiceFloat(s), NiceFloat(0.5));
/// assert_eq!(NiceFloat(c), NiceFloat(0.8660254037844386));
///
/// let (s, c) = primitive_float_sin_cos_pi_rational::<f64>(&Rational::from_unsigneds(1u8, 7));
/// assert_eq!(NiceFloat(s), NiceFloat(0.4338837391175581));
/// assert_eq!(NiceFloat(c), NiceFloat(0.9009688679024191));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos_pi_rational<T: PrimitiveFloat>(x: &Rational) -> (T, T)
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_sin_cos_with_period_rational(x, 2)
}
