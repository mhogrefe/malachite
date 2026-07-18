// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's exponential. `mpfr_exp` (`exp.c`) is a dispatcher; the medium-precision workhorse
// `mpfr_exp_2` (`exp_2.c`) uses Brent's method -- reduce x = n*log(2) + 2^K*r, sum the Taylor
// series for the small r, raise to the 2^K power by K squarings, then scale by 2^n -- with the
// series summed in fixed point. That fixed point is represented here as a malachite `Integer`
// mantissa paired with an `i64` 2-exponent (MPFR's `mpz_t` + `mpfr_exp_t`).
//
// The Paterson-Stockmeyer series (exp2_aux2) and the high-precision exp_3 are not yet ported.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, floor_and_ceiling};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, Exp, ExpAssign, FloorRoot, FloorSqrt, IsPowerOf2, NegAssign, Parity, PowerOf2,
    ShrRoundAssign, Sign, Square, SquareAssign, WrappingAddAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_q::Rational;

// If the number of bits `k` of `z` exceeds `q`, divides `z` by `2 ^ (k - q)` (flooring) and returns
// `k - q`; otherwise leaves `z` unchanged and returns 0.
//
// This is `mpz_normalize` from `exp_2.c`, MPFR 4.2.2.
fn mpz_normalize(z: Integer, q: i64) -> (Integer, i64) {
    let k = z.significant_bits();
    if q < 0 || k > u64::exact_from(q) {
        let shift = i64::exact_from(k) - q;
        (z >> shift, shift)
    } else {
        // Currently unreachable from the naive series (`exp2_aux` always grows `t`/`rr` past `q`
        // bits before truncating, and the squaring loop doubles past `q`); exercised once the
        // Paterson-Stockmeyer path (`exp2_aux2`) is ported.
        (z, 0)
    }
}

// Shifts `z` so that its 2-exponent becomes `target`: right (flooring) by `target - expz` if
// `target > expz`, otherwise left by `expz - target`. Returns `target`.
//
// This is `mpz_normalize2` from `exp_2.c`, MPFR 4.2.2. (A negative shift count reverses direction,
// so the single `>>` covers both of MPFR's branches.)
fn mpz_normalize2(z: Integer, expz: i64, target: i64) -> (Integer, i64) {
    (z >> (target - expz), target)
}

// Returns the integer mantissa `m` and 2-exponent `e` of a finite nonzero `x`, so that `x = m *
// 2^e` (the sign is carried by `m`). For a Malachite `Float`, `m` is the significand as a signed
// integer and `e = exponent - significand_bits` (verified against 1.0: significand 2^63, exponent
// 1, giving 2^63 * 2^(1-64) = 1).
//
// This is equivalent to `mpfr_get_z_2exp` from MPFR 4.2.2.
fn get_z_2exp(x: Float) -> (Integer, i64) {
    if let Finite {
        sign,
        exponent,
        significand,
        ..
    } = x.0
    {
        let bits = significand.significant_bits();
        let m = Integer::from_sign_and_abs(sign, significand);
        (m, i64::from(exponent) - i64::exact_from(bits))
    } else {
        unreachable!()
    }
}

// Computes `s = 1 + r/1! + r^2/2! + ... + r^l/l!` (continuing while the term is still significant
// at precision `q`) in fixed point, where the returned `Integer` `s` and 2-exponent `exps` satisfy
// (sum) = s * 2^exps. `r` must be pure FP (here it is positive and tiny). The naive method, O(l)
// multiplications; the absolute error on the sum is less than `3*l*(l+1)*2^(-q)`, and that
// `3*l*(l+1)` bound is the returned value. (`l` stays small for the precisions `exp_2` handles, so
// the bound fits in a `u64`.)
//
// This is `mpfr_exp2_aux` from `exp_2.c`, MPFR 4.2.2.
fn exp2_aux(r: Float, q: u64) -> (Integer, i64, u64) {
    let qi = i64::exact_from(q);
    let mut expt: i64 = 0;
    let exps: i64 = 1 - qi; // s = 2^(q-1), i.e. the value 1
    let mut t = Integer::ONE;
    let mut s = Integer::power_of_2(q - 1);
    let (mut rr, mut expr) = get_z_2exp(r); // rr * 2^expr = r, no error
    let mut l: u64 = 0;
    loop {
        l += 1;
        t *= &rr;
        expt += expr;
        let sbit = i64::exact_from(s.significant_bits());
        let tbit = i64::exact_from(t.significant_bits());
        let dif = exps + sbit - expt - tbit;
        // truncate the bits of t that are below ulp(s) = 2^(1-q); error at most 2^(1-q)
        let (t2, sh) = mpz_normalize(t, qi - dif);
        t = t2;
        expt += sh;
        if l > 1 {
            // divide by l to build r^l/l! (t >= 0, so truncation equals MPFR's floored division)
            if l.is_power_of_2() {
                // GMP doesn't optimize the power-of-2 case
                t >>= l.ceiling_log_base_2();
            } else {
                t /= Integer::from(l);
            }
            debug_assert_eq!(expt, exps);
        }
        if t == 0 {
            break;
        }
        s += &t; // exact
        // keep rr the same size as t: the error on rr stays at most ulp(t) = ulp(s)
        let tbit = i64::exact_from(t.significant_bits());
        let (rr2, sh) = mpz_normalize(rr, tbit);
        rr = rr2;
        expr += sh;
    }
    (s, exps, 3 * l * (l + 1))
}

// Precision (in bits) at which `exp_2` switches from the naive `exp2_aux` (square-root `K`) to the
// Paterson-Stockmeyer `exp2_aux2` (cube-root `K`). MPFR tunes `MPFR_EXP_2_THRESHOLD` per platform;
// this is the generic default (`generic/mparam.h`), pending Malachite tuning.
const EXP_2_THRESHOLD: u64 = 100;

// Computes `s = 1 + r/1! + r^2/2! + ... + r^l/l!` (continuing while r^l/l! is still significant at
// precision `q`) in fixed point, where the returned `Integer` `s` and 2-exponent `exps` satisfy
// (sum) = s * 2^exps. `r` must be pure FP with exponent < 0 (here it is positive and tiny). Uses
// the Paterson-Stockmeyer scheme: about `m + l/m` full multiplications (`2*sqrt(l)` for `m =
// sqrt(l)`), versus `exp2_aux`'s O(l). The error is bounded by `l^2 + 4*l` ulps, and that `l*(l+4)`
// bound is the returned value.
//
// This is `mpfr_exp2_aux2` from `exp_2.c`, MPFR 4.2.2.
fn exp2_aux2(r: Float, q: u64) -> (Integer, i64, u64) {
    let qi = i64::exact_from(q);
    let one_minus_q = 1 - qi;
    // estimate the value of l, then m ~ sqrt(l); we access R[2], so we need m >= 2
    let expr0 = i64::from(r.get_exponent().unwrap());
    debug_assert!(expr0 < 0);
    let l_est = q / u64::exact_from(-expr0);
    let m = max(2, usize::exact_from(l_est.floor_sqrt()));
    // r_pows[i] = r^i (integer mantissa), exp_r_pows[i] its 2-exponent
    let mut r_pows = vec![Integer::ZERO; m + 1];
    let mut exp_r_pows = vec![0i64; m + 1];
    let exps = one_minus_q; // 1 ulp = 2^(1-q)
    let mut s = Integer::ZERO;
    let (r1, e1) = get_z_2exp(r); // exact: no error
    // normalize R[1] to exponent 1 - q (error <= 1 ulp)
    let r1 = mpz_normalize2(r1, e1, one_minus_q).0;
    r_pows[1] = r1;
    exp_r_pows[1] = one_minus_q;
    // R[2] = R[1]^2 >> (q - 1) (err <= 3 ulps)
    let qm1 = q - 1;
    r_pows[2] = (&r_pows[1]).square() >> qm1;
    exp_r_pows[2] = one_minus_q;
    for i in 3..=m {
        // err(R[i]) <= 2*i-1 ulps
        let t = if i.odd() {
            &r_pows[i - 1] * &r_pows[1]
        } else {
            (&r_pows[i >> 1]).square()
        };
        r_pows[i] = t >> qm1;
        exp_r_pows[i] = one_minus_q;
    }
    r_pows[0] = Integer::power_of_2(q - 1); // R[0] = 1
    exp_r_pows[0] = one_minus_q;
    let mut rr = Integer::ONE;
    let mut expr: i64 = 0; // rr contains r^l/l!; by induction err(rr) <= 2*l ulps
    let mut l: u64 = 0;
    let mut ql = q; // precision used for the current giant step
    loop {
        let one_minus_ql = 1 - i64::exact_from(ql);
        // all R[i] (i < m) must have exponent 1 - ql
        if l != 0 {
            for (r_pow, exp_r_pow) in r_pows[..m].iter_mut().zip(exp_r_pows[..m].iter_mut()) {
                let z = core::mem::replace(r_pow, Integer::ZERO);
                (*r_pow, *exp_r_pow) = mpz_normalize2(z, *exp_r_pow, one_minus_ql);
            }
        }
        // t = R[m-1] normalized to exponent 1 - ql (err(t) <= 2*m-1 ulps)
        let (mut t, mut expt) =
            mpz_normalize2(r_pows[m - 1].clone(), exp_r_pows[m - 1], one_minus_ql);
        // t = 1 + r/(l+1) + ... + r^(m-1)*l!/(l+m-1)! via Horner's scheme
        for i in (0..m - 1).rev() {
            t /= Integer::from(l + i as u64 + 1); // err(t) += 1 ulp
            t += &r_pows[i];
        }
        // multiply t by r^l/l! and add to s
        t *= &rr;
        expt += expr;
        let (t, et) = mpz_normalize2(t, expt, exps);
        debug_assert_eq!(et, exps);
        s += &t; // no error here
        // update rr to r^(l+m)/(l+m)!
        let mut t = &rr * &r_pows[m]; // err(t) <= err(rr) + 2m-1
        expr += exp_r_pows[m];
        let mut tmp = Integer::ONE;
        for i in 1..=m {
            tmp *= Integer::from(l + i as u64);
        }
        t /= tmp; // err(t) <= err(rr) + 2m
        l += m as u64;
        if t == 0 {
            break;
        }
        let (rr2, sh) = mpz_normalize(t, i64::exact_from(ql));
        rr = rr2;
        expr += sh;
        // in late giant steps `ql` can go <= 0 (s has grown past the working precision), so
        // normalizing t to ql bits can shift it away entirely; rr is then 0.
        let rrbit = if rr == 0 {
            1
        } else {
            i64::exact_from(rr.significant_bits())
        };
        let sbit = i64::exact_from(s.significant_bits());
        ql = (qi - exps - sbit + expr + rrbit) as u64;
        // MPFR's own `(size_t)` cast here is admittedly dubious (see its TODO), but the operands
        // cluster near -q, far from the wrap, so the unsigned and signed comparisons agree.
        if (expr as u64).wrapping_add(rrbit as u64) <= q.wrapping_neg() {
            break;
        }
    }
    (s, exps, l * (l + 4))
}

// Precision (in bits) at or above which `exp` uses the binary-splitting `exp_3` (O(M(n) log(n)^2))
// instead of `exp_2`. MPFR's generic `MPFR_EXP_THRESHOLD` default (`generic/mparam.h`), untuned.
const EXP_THRESHOLD: u64 = 25000;

// Extracts the `i`-th binary-splitting chunk of the mantissa of `p`, where `0 <= |p| < 1`, carrying
// `p`'s sign. With `B = 2 ^ Limb::WIDTH`: chunk 0 is `floor(|p| * B)` (the top limb), and for `i >
// 0`, chunk `i` is `(|p| * B^(2^i)) mod B^(2^(i-1))` -- the window of `2^(i-1)` limbs ending
// `2^(i-1)` limbs below where chunk `i - 1` ends.
//
// This is `mpfr_extract` from `extract.c`, MPFR 4.2.2.
fn extract(p: &Float, i: u64) -> Integer {
    if let Finite {
        sign, significand, ..
    } = &p.0
    {
        let limbs = significand.as_limbs_asc();
        let size_p = limbs.len();
        let two_i = usize::power_of_2(i);
        let two_i_2 = if i == 0 { 1 } else { two_i >> 1 };
        let mut y = vec![0 as Limb; two_i_2];
        if size_p < two_i {
            // The window extends past the bottom of the mantissa: zero-fill and copy what's there.
            if size_p >= two_i_2 {
                let count = size_p - two_i_2;
                y[two_i - size_p..][..count].copy_from_slice(&limbs[..count]);
            } else {
                // The whole window is below the mantissa (chunk all zero). Unreachable from
                // `exp_3`: it only extracts chunks `i <= prec_x`, and `size_p > 2^(prec_x - 1) >=
                // two_i_2`.
                fail_on_untested_path("extract, window entirely below the mantissa");
            }
        } else {
            y.copy_from_slice(&limbs[size_p - two_i..][..two_i_2]);
        }
        Integer::from_sign_and_abs(*sign, Natural::from_owned_limbs_asc(y))
    } else {
        unreachable!()
    }
}

// Computes `y ~ exp(p / 2^r)` to precision `prec`, within 1 ulp, for `|p / 2^r| < 1`, using up to
// `2^m` terms of the Taylor series summed by binary splitting. With `P(a,b) = p` if `a+1=b` else
// `P(a,c)*P(c,b)`, `Q(a,b) = a*2^r` if `a+1=b` (except `Q(0,1)=1`) else `Q(a,c)*Q(c,b)`, and
// `T(a,b) = P(a,b)` if `a+1=b` else `Q(c,b)*T(a,c) + P(a,c)*T(c,b)`, one has `exp(p/2^r) ~
// T(0,i)/Q(0,i)`. Since `P(a,b) = p^(b-a)` and only `b-a = 2^j` occur, only the powers `p^(2^j)`
// (the `ptoj` array) are precomputed; and since `Q(a,b)` is divisible by `2^(r*(b-a-1))`, that
// power of two is tracked separately rather than stored.
//
// This is `mpfr_exp_rational` from `exp3.c`, MPFR 4.2.2.
fn exp_rational(p: Integer, mut r: i64, m: usize, prec: u64) -> Float {
    // Normalize p (strip trailing zeros); since |p/2^r| < 1 and p != 0, r stays >= 1.
    let nz = p.trailing_zeros().unwrap();
    let p = p >> nz;
    r -= i64::exact_from(nz);
    let scratch_len = m + 1;
    let mut scratch = vec![Integer::ZERO; 3 * scratch_len];
    split_into_chunks_mut!(scratch, scratch_len, [q, s], ptoj); // ptoj[k] = p^(2^k)
    let mut scratch = vec![0u64; scratch_len << 1];
    // P[k]/Q[k] for the remaining terms is <= 2^(-mult[k])
    let (mult, log2_nb_terms) = scratch.split_at_mut(scratch_len);
    ptoj[0] = p;
    for k in 1..m {
        ptoj[k] = (&ptoj[k - 1]).square();
    }
    q[0] = Integer::ONE;
    s[0] = Integer::ONE;
    let mut k = 0usize;
    let mut prec_i_have: u64 = 0;
    // Main loop: Q[0]*Q[1]*...*Q[k] equals i! as an invariant.
    let n_terms = u64::power_of_2(u64::exact_from(m));
    let mut i = 1u64;
    while prec_i_have < prec && i < n_terms {
        k += 1;
        log2_nb_terms[k] = 0; // 1 term
        q[k] = Integer::from(i + 1);
        s[k] = Integer::from(i + 1);
        let mut j = i + 1; // terms computed so far
        let mut l = 0u32;
        while j.even() {
            // Combine and reduce: S[k] covers 2^l consecutive terms.
            s[k] *= &ptoj[l as usize];
            let mut t = &s[k - 1] * &q[k];
            // Q[k] lacks the 2^(r*2^l) factor, so multiply it in when merging.
            t <<= r << l;
            t += &s[k];
            s[k - 1] = t;
            let (q_lo, q_hi) = q.split_at_mut(k);
            *q_lo.last_mut().unwrap() *= &q_hi[0];
            log2_nb_terms[k - 1] += 1;
            prec_i_have = q[k].significant_bits();
            let prec_ptoj = ptoj[l as usize].significant_bits();
            mult[k - 1].wrapping_add_assign(
                prec_i_have
                    .wrapping_add(u64::wrapping_from(r << l))
                    .wrapping_sub(prec_ptoj)
                    .wrapping_sub(1),
            );
            prec_i_have = mult[k - 1];
            mult[k] = mult[k - 1];
            l += 1;
            j >>= 1;
            k -= 1;
        }
        i += 1;
    }
    // Accumulate all products into S[0] and Q[0].
    let mut h = 0u64; // accumulated terms in the right part S[k]/Q[k]
    while k > 0 {
        let jj = log2_nb_terms[k - 1] as usize;
        s[k] *= &ptoj[jj];
        let mut t = &s[k - 1] * &q[k];
        h += u64::power_of_2(log2_nb_terms[k]);
        t <<= r * i64::exact_from(h);
        t += &s[k];
        s[k - 1] = t;
        let (q_lo, q_hi) = q.split_at_mut(k);
        *q_lo.last_mut().unwrap() *= &q_hi[0];
        k -= 1;
    }
    // Q[0] now equals i!. Scale S[0] to ~2*prec bits and Q[0] to ~prec bits, then divide.
    let mut s0 = core::mem::replace(&mut s[0], Integer::ZERO);
    let mut q0 = core::mem::replace(&mut q[0], Integer::ZERO);
    let mut diff = i64::exact_from(s0.significant_bits()) - (i64::exact_from(prec) << 1);
    let mut expo = diff;
    s0 >>= diff; // negative shift is a left shift, covering MPFR's mul_2exp branch
    diff = i64::exact_from(q0.significant_bits()) - i64::exact_from(prec);
    expo -= diff;
    q0 >>= diff;
    s0 /= q0; // truncating division (both positive)
    // y = (S[0] rounded to prec) * 2^(expo - r*(i-1)); MPFR sets the mantissa via set_z then
    // overrides the exponent, which is exactly this scaling. A direct `from_integer_prec_round(s0,
    // ..)` would pass through the intermediate exponent sb(s0) ~ 2 * prec, which exceeds
    // MAX_EXPONENT once prec approaches it (malachite's precision range exceeds its exponent range,
    // unlike MPFR's, whose emax dwarfs any practical precision) and would silently saturate at the
    // largest finite value. Attaching the scaling before the conversion keeps the exponent in
    // range: the scaled value is a factor of exp(chunk), of ordinary size.
    Float::from_rational_prec_round(
        Rational::from(s0) << (expo - r * (i64::exact_from(i) - 1)),
        prec,
        Floor,
    )
    .0
}

// Computes `exp(x)` rounded to precision `precy` with rounding mode `rm`. Decomposes `x` into
// limb-window chunks (`extract`), exponentiates each chunk's contribution with binary splitting
// (`exp_rational`), and multiplies them, using O(M(n) log(n)^2) for high precision.
//
// This is `mpfr_exp_3` from `exp3.c`, MPFR 4.2.2.
pub(crate) fn exp_3(x: &Float, precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    const SHIFT: u64 = Limb::WIDTH >> 1;
    // prec_x: number of chunk levels, ~log2 of x's limb count.
    let prec_x = x
        .get_prec()
        .unwrap()
        .ceiling_log_base_2()
        .saturating_sub(Limb::LOG_WIDTH);
    let mut ttt = i64::from(x.get_exponent().unwrap());
    let mut x_copy = x.clone();
    let shift_x = if ttt > 0 {
        // Shift x down to magnitude < 1.
        let s = u64::exact_from(ttt);
        x_copy = x >> s;
        ttt = i64::from(x_copy.get_exponent().unwrap());
        s
    } else {
        0
    };
    debug_assert!(ttt <= 0);
    let mut realprec = precy + (prec_x + precy).ceiling_log_base_2();
    let mut prec = realprec + SHIFT + 2 + shift_x;
    let mut increment = Limb::WIDTH;
    loop {
        let k = prec.ceiling_log_base_2().saturating_sub(Limb::LOG_WIDTH);
        let mut twopoweri = Limb::WIDTH;
        // Particular case i = 0.
        let uk = extract(&x_copy, 0);
        debug_assert_ne!(uk, 0);
        let mut tmp = exp_rational(
            uk,
            i64::exact_from(SHIFT + twopoweri) - ttt,
            usize::exact_from(k + 1),
            prec,
        );
        for _ in 0..SHIFT {
            tmp.square_prec_round_assign(prec, Floor);
        }
        twopoweri *= 2;
        // General case.
        let iter = k.min(prec_x);
        for i in 1..=iter {
            let uk = extract(&x_copy, i);
            if uk != 0 {
                let t = exp_rational(
                    uk,
                    i64::exact_from(twopoweri) - ttt,
                    usize::exact_from(k - i + 1),
                    prec,
                );
                tmp.mul_prec_round_assign(t, prec, Floor);
            }
            twopoweri <<= 1;
        }
        // Raise tmp to 2^shift_x to undo the initial down-shift of x; detect over/underflow.
        let (val, scaled) = if shift_x > 0 {
            for _ in 0..shift_x - 1 {
                tmp.square_prec_round_assign(prec, Floor);
            }
            let mut t = tmp.square_prec_round_ref(prec, Floor).0;
            if t.is_infinite() {
                // Unreachable: `normal_ref` decides the overflow boundary exactly, so here exp(x) <
                // 2^emax, every Floor-rounded intermediate lies below its true value, and no
                // squaring can overflow. (Even if one somehow did, Floor rounding saturates at the
                // largest finite value rather than reaching infinity.)
                fail_on_untested_path("exp_3, overflow above normal_ref's bound_emax");
                return exp_overflow(precy, rm);
            }
            let mut scaled = false;
            if matches!(t.0, Zero { .. }) {
                // Possibly spurious underflow: rescale by 2 and retry. Reachable only for x in the
                // narrow band just above `normal_ref`'s `bound_emin`; exp's own test inputs never
                // land there, but `Float::pow`'s do (its Ziv loop feeds y * ln|x| here at boundary
                // magnitudes), and pow's property tests validate this path against MPFR.
                tmp <<= 1;
                t = tmp.square_prec_round_ref(prec, Floor).0;
                if matches!(t.0, Zero { .. }) {
                    // exact result < 2^(emin - 2): genuine underflow.
                    return exp_underflow(precy, if rm == Nearest { Down } else { rm });
                }
                scaled = true;
            }
            (t, scaled)
        } else {
            (tmp, false)
        };
        if float_can_round(val.significand_ref().unwrap(), realprec, precy, rm) {
            let mut y = val;
            let mut inexact = y.set_prec_round(precy, rm);
            if scaled && y.is_normal() {
                // Undo the *2 scaling: y /= 4.
                let ey = i64::from(y.get_exponent().unwrap());
                let inex2 = y.shr_round_assign(2, rm);
                if inex2 != Equal {
                    // Underflow while unscaling.
                    if rm == Nearest
                        && inexact == Less
                        && matches!(y.0, Zero { .. })
                        && ey == i64::from(Float::MIN_EXPONENT) + 1
                    {
                        // Double rounding: RNDN rounded the scaled result down to 2^emin, but the
                        // exact result is > 2^(emin - 2), so round up instead.
                        (y, inexact) = (Float::min_positive_value_prec(precy), Greater);
                    } else {
                        inexact = inex2;
                    }
                }
            }
            return (y, inexact);
        }
        realprec += increment;
        increment = realprec >> 1;
        prec = realprec + SHIFT + 2 + shift_x;
    }
}

// Computes `exp(x)` rounded to precision `precy` with rounding mode `rm`, returning the rounded
// value and an [`Ordering`] comparing it to the exact result. `x` must be finite and nonzero and
// `exp(x)` must be in range; the dispatcher (`exp`) guarantees both. Uses Brent's method: `exp(x) =
// (1 + r + r^2/2! + ...)^(2^K) * 2^n` with `x = n*log(2) + 2^K*r`.
//
// Below `EXP_2_THRESHOLD` the naive series (`exp2_aux`) is used with the square-root `K`; at or
// above it the Paterson-Stockmeyer series (`exp2_aux2`) is used with the cube-root `K`.
//
// This is `mpfr_exp_2` from `exp_2.c`, MPFR 4.2.2.
pub(crate) fn exp_2(x: &Float, precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    let expx = i64::from(x.get_exponent().unwrap());
    // Argument reduction: n ~ round(x / log(2)) (need not be exact).
    let mut n: i64 = if expx <= -2 {
        // |x| <= 0.25, so n = 0
        0
    } else {
        let log2_est = Float::ln_2_prec_round(Limb::WIDTH - 1, Down).0;
        let r_est = x.div_prec_ref_val(log2_est, Limb::WIDTH - 1).0;
        i64::rounding_from(r_est, Nearest).0
    };
    // error_r bounds the bits cancelled in x - n*log(2)
    let error_r: u64 = if n == 0 {
        0
    } else {
        (n.unsigned_abs() + 1).significant_bits()
    };
    // Working-precision setup. Square-root K for the naive series, cube-root K for
    // Paterson-Stockmeyer.
    let k_param = if precy < EXP_2_THRESHOLD {
        precy.div_ceil(2).floor_sqrt() + 3
    } else {
        (4 * precy).floor_root(3)
    };
    let l = (precy - 1) / k_param + 1;
    let mut err = k_param + ((l << 1) + 18).ceiling_log_base_2();
    let mut q = precy + err + k_param + 10;
    // if |x| >> 1, account for the cancelled bits
    if expx > 0 {
        q += u64::exact_from(expx);
    }
    let mut increment = Limb::WIDTH;
    loop {
        let working = q + error_r;
        // s is within 1 ulp of log(2), rounded so that r = x - n*log(2) is bounded above.
        let s = Float::ln_2_prec_round(working, if n >= 0 { Down } else { Up }).0;
        // r = |n| * log(2) (directed); negate when n < 0, so r <= n*log(2) within 3 ulps.
        let mut r = s
            .mul_prec_round_ref_val(
                Float::from(n.unsigned_abs()),
                working,
                if n >= 0 { Down } else { Up },
            )
            .0;
        if n < 0 {
            r.neg_assign();
        }
        r = x.sub_prec_round_ref_val(r, working, Up).0;
        // if the initial n was too large, r came out negative: reduce n
        while r.is_normal() && r.is_sign_negative() {
            n -= 1;
            r.add_prec_round_assign_ref(&s, working, Up);
        }
        // if r is 0 we cannot round correctly; otherwise sum the series
        if r.is_normal() {
            // the cancelled low error_r bits of r are non-significant, so drop them
            if error_r > 0 {
                r.set_prec_round(q, Up);
            }
            // r = (x - n*log(2)) / 2^K, exact
            r >>= k_param;
            // ss <- 1 + r + r^2/2! + ... (naive method below the threshold, Paterson-Stockmeyer at
            // or above it)
            let (mut ss, mut exps, l_err) = if precy < EXP_2_THRESHOLD {
                exp2_aux(r, q)
            } else {
                exp2_aux2(r, q)
            };
            // raise to the 2^K power by K squarings
            for _ in 0..k_param {
                ss.square_assign();
                exps <<= 1;
                let (ss2, sh) = mpz_normalize(ss, i64::exact_from(q));
                ss = ss2;
                exps += sh;
            }
            // s = ss * 2^exps (exact: ss has at most q bits and working >= q)
            let s = Float::from_integer_prec(ss, working).0 << exps;
            // error is at most 2^K * l_err, plus 2 for the 3-ulp error on r
            err = k_param + l_err.ceiling_log_base_2() + 2;
            if float_can_round(s.significand_ref().unwrap(), q - err, precy, rm) {
                // y = s * 2^n, rounded to precy. `float_can_round` only returns true when s's
                // trusted bits below precy are not all equal, i.e. s is not exactly representable
                // at precy; since `shl_prec_round` rounds those same bits, it cannot come out Equal
                // here. (This matches MPFR, which rounds and breaks with no special case -- exp of
                // a finite nonzero value is irrational, never exactly representable.)
                return s.shl_prec_round(n, precy, rm);
            }
        }
        // If `r` is not normal it is 0: the rounded x - n*log(2) cancelled exactly, which happens
        // iff x equals the working-precision rounding of n*log(2). The series can't be summed (it
        // needs `r != 0`), so fall through to raise `q`; the higher-precision log(2) no longer
        // rounds to x, so `r != 0` next time. This is MPFR's `MPFR_IS_ZERO(r)` case.
        q += increment;
        increment = q >> 1;
    }
}

// The overflow result of exp (the value, which is positive, exceeds the maximum finite Float).
//
// This is `mpfr_overflow` (with positive sign) as used by `mpfr_exp`, MPFR 4.2.2.
pub(crate) fn exp_overflow(precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    match rm {
        Nearest | Up | Ceiling => (Float::INFINITY, Greater),
        Down | Floor => (Float::max_finite_value_with_prec(precy), Less),
        Exact => panic!("exp: Exact rounding was requested, but the result overflows"),
    }
}

// The underflow result of exp (the value, which is positive, is below the minimum positive Float).
// MPFR maps Nearest to toward-zero here, so Nearest joins Down/Floor.
//
// This is `mpfr_underflow` (with positive sign) as used by `mpfr_exp`, MPFR 4.2.2.
pub(crate) fn exp_underflow(precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    match rm {
        Nearest | Down | Floor => (Float::ZERO, Less),
        Up | Ceiling => (Float::min_positive_value_prec(precy), Greater),
        Exact => panic!("exp: Exact rounding was requested, but the result underflows"),
    }
}

// Computes `exp(x)` for finite nonzero `x`, rounded to precision `precy` with rounding mode `rm`.
// Detects overflow/underflow against `log(2)`-scaled exponent bounds, takes a fast path for tiny
// `x` (where `exp(x) = 1 +/- ulp(1)`), and otherwise dispatches to `exp_2` (below `EXP_THRESHOLD`)
// or the binary-splitting `exp_3` (at or above it).
//
// This is the finite-nonzero branch of `mpfr_exp` from `exp.c`, MPFR 4.2.2.
fn exp_prec_round_normal_ref(x: &Float, precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    // exp of a finite nonzero value is transcendental, hence never exactly representable.
    assert_ne!(rm, Exact, "Inexact exp");
    // Overflow/underflow bounds, as ~64-bit Floats. Directed rounding makes `bound_emax` an upper
    // bound on emax*log(2) and `bound_emin` a lower bound on (emin - 2)*log(2), so the comparisons
    // below are sound one-sided tests.
    const BP: u64 = 64;
    let log2_up = Float::ln_2_prec_round(BP, Up).0;
    let bound_emax = log2_up
        .mul_prec_round_ref_val(
            const { Float::const_from_signed(Float::MAX_EXPONENT as SignedLimb) },
            BP,
            Up,
        )
        .0;
    if *x >= bound_emax {
        // x > log(2^emax), so exp(x) > 2^emax
        return exp_overflow(precy, rm);
    }
    // `bound_emax` is an upper bound with ~2^-33 of slack, so an x just below it may still
    // overflow. That sliver must be decided here: below the threshold, every intermediate in
    // `exp_2` and `exp_3` stays under 2^emax, but a true overflow inside `exp_3` would saturate its
    // Floor-rounded final squarings at the largest finite value instead of reaching infinity, and
    // the saturated all-ones significand is one that `float_can_round` never certifies -- the Ziv
    // loop would grow forever. Decide the sliver exactly, by comparing x with brackets of emax *
    // log(2) as exact Rationals at widening precision; x is dyadic and the threshold is irrational,
    // so the comparison always resolves. This mirrors the role of MPFR's overflow flag, which lets
    // mpfr_exp detect the overflow after the fact.
    let bound_emax_lo = Float::ln_2_prec_round(BP, Floor)
        .0
        .mul_prec_round_ref_val(
            const { Float::const_from_signed(Float::MAX_EXPONENT as SignedLimb) },
            BP,
            Floor,
        )
        .0;
    if *x >= bound_emax_lo {
        let xr = Rational::exact_from(x);
        let emax_r = Rational::from(Float::MAX_EXPONENT);
        let mut p = 128;
        loop {
            let lo = Rational::exact_from(Float::ln_2_prec_round(p, Floor).0) * &emax_r;
            if xr < lo {
                break;
            }
            let hi = Rational::exact_from(Float::ln_2_prec_round(p, Ceiling).0) * &emax_r;
            if xr >= hi {
                // x > emax * log(2), so exp(x) > 2^emax
                return exp_overflow(precy, rm);
            }
            p <<= 1;
        }
    }
    let bound_emin = log2_up
        .mul_prec_round(
            const { Float::const_from_signed((Float::MIN_EXPONENT as SignedLimb) - 2) },
            BP,
            Floor,
        )
        .0;
    if *x <= bound_emin {
        // x < log(2^(emin - 2)), so exp(x) < 2^(emin - 2)
        return exp_underflow(precy, rm);
    }
    let expx = i64::from(x.get_exponent().unwrap());
    // tiny x: if x < 2^(-precy), then exp(x) = 1 +/- ulp(1)
    if expx < 0 && u64::exact_from(-expx) > precy {
        return if x.is_sign_negative() && (rm == Down || rm == Floor) {
            (one_neighbor(precy, false), Less) // 1 - ulp
        } else if x.is_sign_positive() && (rm == Up || rm == Ceiling) {
            (one_neighbor(precy, true), Greater) // 1 + ulp
        } else {
            (
                Float::one_prec(precy),
                if x.is_sign_positive() { Less } else { Greater },
            )
        };
    }
    if precy >= EXP_THRESHOLD {
        exp_3(x, precy, rm)
    } else {
        exp_2(x, precy, rm)
    }
}

// The neighbor of 1 at precision `prec`: the successor `1 + 2 ^ (1 - prec)` if `above`, otherwise
// the predecessor `1 - 2 ^ (-prec)`. Both are exactly representable at precision `prec`. (Note that
// `Float::increment`/`decrement` cannot be used here: they keep the ulp of the current binade, so
// they bump the precision when crossing into the next binade and overshoot the true predecessor.
// Also note that the significand cannot be built as a `Natural` and shifted into place: the
// unshifted intermediate has exponent `prec`, which overflows to infinity when `prec` exceeds
// `MAX_EXPONENT`, even though the final value's exponent is 0 or 1. Going through a `Rational`
// keeps every intermediate exponent small. The `i64` conversion fails only for `prec >= 2^63`,
// where a `Float` of that precision could not be materialized at all.)
pub(crate) fn one_neighbor(prec: u64, above: bool) -> Float {
    let p = i64::exact_from(prec);
    Float::from_rational_prec_round(
        if above {
            Rational::ONE + Rational::power_of_2(1 - p)
        } else {
            Rational::ONE - Rational::power_of_2(-p)
        },
        prec,
        Exact,
    )
    .0
}

// Computes `exp(x)` for a nonzero `Rational` `x` with `|x| < 1`, by summing its Taylor series
// `exp(x) = sum x^k / k!`. Used when `x` is too small to be represented as a normal `Float` (so the
// squeeze in `exp_rational_helper` cannot bracket it), in which case `exp(x)` is very close to 1
// but may still be more than one ulp away from 1 when `prec` is enormous. The series is summed term
// by term, bracketing the exact value between two rationals (consecutive partial sums for `x < 0`,
// a partial sum and a remainder bound for `x > 0`) until both ends round to the same `Float`.
// Working entirely with values near 1, this avoids ever representing `x` itself as a `Float`.
pub(crate) fn exp_rational_near_one(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let negative = x.sign() == Less;
    let mut s = Rational::ONE; // partial sum S_{k-1}
    let mut term = Rational::ONE; // x^(k-1) / (k-1)!
    let mut k = 1u64;
    loop {
        term *= x;
        term /= Rational::from(k); // term = x^k / k!
        let s_next = &s + &term; // S_k
        let (lo, hi) = if negative {
            // The terms alternate in sign with strictly decreasing magnitude (|x| / (k + 1) < 1),
            // so exp(x) lies between consecutive partial sums.
            if s < s_next {
                (s.clone(), s_next.clone())
            } else {
                (s_next.clone(), s.clone())
            }
        } else {
            // Every term is positive, so S_k < exp(x), and the remainder is bounded by t_{k+1} / (1
            // - x).
            let next = (&term * x) / Rational::from(k + 1); // t_{k+1}
            (s_next.clone(), &s_next + next / (Rational::ONE - x))
        };
        s = s_next;
        k += 1;
        let (f_lo, mut o_lo) = Float::from_rational_prec_round_ref(&lo, prec, rm);
        let (f_hi, mut o_hi) = Float::from_rational_prec_round_ref(&hi, prec, rm);
        // A bound that is exactly representable at `prec` rounds with `Equal`; treat it as agreeing
        // with the other bound. (`hi == 1` triggers this for small negative x, since 1 is exact;
        // the `lo` case only arises when a partial sum lands exactly on a `prec`-bit Float, which
        // needs an enormous `prec`.)
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && f_lo == f_hi {
            return (f_lo, o_lo);
        }
    }
}

// Computes `exp(x)` for a nonzero `Rational` `x`, rounded to precision `prec` with rounding mode
// `rm`. (`exp(0) = 1` is handled by the caller.) Because the exponential of a nonzero rational is
// transcendental, the result is never exactly representable, so `rm` must not be `Exact`.
fn exp_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact exp");
    let positive = x.sign() == Greater;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // x is too small to be represented as a normal Float (|x| < 2^MIN_EXPONENT). The squeeze below
    // cannot bracket it (its Float bounds would be 0 or out of range), so sum the Taylor series
    // instead. exp(x) is near 1 but, for an enormous `prec`, possibly more than one ulp away.
    if exp_x <= const { Float::MIN_EXPONENT as i64 } {
        return exp_rational_near_one(x, prec, rm);
    }
    // Tiny x: if |x| < 2^(-prec-1) then exp(x) is within half an ulp of 1, so it rounds to 1 (or,
    // for directed rounding away from 1, to the neighbor of 1). This mirrors exp's tiny-x fast
    // path.
    if -exp_x > i64::exact_from(prec) {
        return match (positive, rm) {
            (false, Down | Floor) => (one_neighbor(prec, false), Less), // 1 - ulp
            (true, Up | Ceiling) => (one_neighbor(prec, true), Greater), // 1 + ulp
            (true, _) => (Float::one_prec(prec), Less),
            (false, _) => (Float::one_prec(prec), Greater),
        };
    }
    // |x| is too large to be a finite Float, so exp(x) overflows (x > 0) or underflows (x < 0).
    // Smaller x that still overflow/underflow exp are caught by `exp_prec_round_normal_ref` in the
    // loop below.
    if exp_x >= const { Float::MAX_EXPONENT as i64 } {
        return if positive {
            exp_overflow(prec, rm)
        } else {
            exp_underflow(prec, rm)
        };
    }
    // General case: bracket x between the Floats x_lo <= x <= x_hi, exponentiate both, and increase
    // the working precision until the two bounds round to the same result. exp is monotonic, so
    // once the bounds agree the exact exp(x) (which lies between them) rounds the same way.
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            // x is exactly representable at `working_prec`, so exp(x) is simply exp(x_lo).
            return exp_prec_round_normal_ref(&x_lo, prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        // exp of a finite nonzero Float is transcendental, so `exp_prec_round_normal_ref` is never
        // exact: both orderings are `Less` or `Greater`, never `Equal`.
        let (e_lo, o_lo) = exp_prec_round_normal_ref(&x_lo, prec, rm);
        let (e_hi, o_hi) = exp_prec_round_normal_ref(&x_hi, prec, rm);
        if o_lo == o_hi && e_lo == e_hi {
            return (e_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $e^x$, the exponential of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded exponential is less than,
    /// equal to, or greater than the exact exponential. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::exp_round`] instead. If both of these things are true, consider using
    /// [`Float::exp`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round(5, Floor);
    /// assert_eq!(e.to_string(), "2.6");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round(5, Ceiling);
    /// assert_eq!(e.to_string(), "2.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round(5, Nearest);
    /// assert_eq!(e.to_string(), "2.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round(20, Floor);
    /// assert_eq!(e.to_string(), "2.718281");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round(20, Ceiling);
    /// assert_eq!(e.to_string(), "2.718285");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round(20, Nearest);
    /// assert_eq!(e.to_string(), "2.718281");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.exp_prec_round_ref(prec, rm)
    }

    /// Computes $e^x$, the exponential of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded exponential is less than,
    /// equal to, or greater than the exact exponential. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::exp_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).exp()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round_ref(5, Floor);
    /// assert_eq!(e.to_string(), "2.6");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round_ref(5, Ceiling);
    /// assert_eq!(e.to_string(), "2.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round_ref(5, Nearest);
    /// assert_eq!(e.to_string(), "2.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round_ref(20, Floor);
    /// assert_eq!(e.to_string(), "2.718281");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round_ref(20, Ceiling);
    /// assert_eq!(e.to_string(), "2.718285");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_prec_round_ref(20, Nearest);
    /// assert_eq!(e.to_string(), "2.718281");
    /// assert_eq!(o, Less);
    /// ```
    pub fn exp_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // exp(+inf) = +inf; exp(-inf) = +0
            Infinity { sign } => {
                if *sign {
                    (Self::INFINITY, Equal)
                } else {
                    (Self::ZERO, Equal)
                }
            }
            // exp(+0) = exp(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => exp_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $e^x$, the exponential of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded exponential is less than, equal to, or greater than the exact
    /// exponential. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the exponential is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::exp`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_prec(5);
    /// assert_eq!(e.to_string(), "2.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_prec(20);
    /// assert_eq!(e.to_string(), "2.718281");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_prec(self, prec: u64) -> (Self, Ordering) {
        self.exp_prec_round(prec, Nearest)
    }

    /// Computes $e^x$, the exponential of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded exponential is less than, equal to, or greater than
    /// the exact exponential. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the exponential is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).exp()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_prec_ref(5);
    /// assert_eq!(e.to_string(), "2.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_prec_ref(20);
    /// assert_eq!(e.to_string(), "2.718281");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.exp_prec_round_ref(prec, Nearest)
    }

    /// Computes $e^x$, the exponential of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded exponential is less than, equal to, or greater than the exact
    /// exponential. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::exp_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::exp_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::exp`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_round(Floor);
    /// assert_eq!(e.to_string(), "2.718281828459045235360287471351");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_round(Ceiling);
    /// assert_eq!(e.to_string(), "2.718281828459045235360287471354");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_round(Nearest);
    /// assert_eq!(e.to_string(), "2.718281828459045235360287471351");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.exp_prec_round(prec, rm)
    }

    /// Computes $e^x$, the exponential of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded exponential is less than, equal to, or greater than the exact
    /// exponential. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// See the [`Float::exp_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::exp_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).exp()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100).0.exp_round_ref(Floor);
    /// assert_eq!(e.to_string(), "2.718281828459045235360287471351");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_round_ref(Ceiling);
    /// assert_eq!(e.to_string(), "2.718281828459045235360287471354");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .exp_round_ref(Nearest);
    /// assert_eq!(e.to_string(), "2.718281828459045235360287471351");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn exp_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.exp_prec_round_ref(prec, rm)
    }

    /// Computes $e^x$, the exponential of a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded exponential is less than, equal to, or greater than the exact
    /// exponential. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::exp_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::exp_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::exp_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.6");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.718281");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.718285");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.718281");
    /// ```
    #[inline]
    pub fn exp_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        let o;
        (*self, o) = x.exp_prec_round(prec, rm);
        o
    }

    /// Computes $e^x$, the exponential of a [`Float`], in place, rounding the result to the nearest
    /// value of the specified precision. An [`Ordering`] is returned, indicating whether the
    /// rounded exponential is less than, equal to, or greater than the exact exponential. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// If the exponential is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::exp_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::exp_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "2.718281");
    /// ```
    #[inline]
    pub fn exp_prec_assign(&mut self, prec: u64) -> Ordering {
        self.exp_prec_round_assign(prec, Nearest)
    }

    /// Computes $e^x$, the exponential of a [`Float`], in place, rounding the result with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// exponential is less than, equal to, or greater than the exact exponential. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 e^x\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $e^x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 e^x\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::exp_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::exp_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::exp_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "2.718281828459045235360287471351");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.718281828459045235360287471354");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.exp_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "2.718281828459045235360287471351");
    /// ```
    #[inline]
    pub fn exp_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.exp_prec_round_assign(prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded exponential is less than, equal to, or greater than the exact exponential.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) = Float::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(e.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(e.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) = Float::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(e.to_string(), "1.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(e.to_string(), "1.822121");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn exp_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::exp_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded exponential is less than, equal to, or greater than the exact exponential.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = e^x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$.
    ///
    /// These bounds do not apply when the result overflows or underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::exp_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (e, o) =
    ///     Float::exp_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(e.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) =
    ///     Float::exp_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(e.to_string(), "1.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (e, o) =
    ///     Float::exp_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(e.to_string(), "1.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) =
    ///     Float::exp_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(e.to_string(), "1.822121");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn exp_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // exp(0) = 1, exactly.
            return (Self::one_prec(prec), Equal);
        }
        exp_rational_helper(x, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded exponential
    /// is less than, equal to, or greater than the exact exponential.
    ///
    /// If the exponential is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$ (unless the result overflows or
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// let (e, o) = Float::exp_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(e.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(e.to_string(), "1.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_rational_prec(Rational::from(0), 10);
    /// assert_eq!(e.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn exp_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::exp_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $e^x$, the exponential of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`Float`]. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// exponential is less than, equal to, or greater than the exact exponential.
    ///
    /// If the exponential is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = e^x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 e^x\rfloor-p}$ (unless the result overflows or
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
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
    /// let (e, o) = Float::exp_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(e.to_string(), "1.81");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(e.to_string(), "1.822119");
    /// assert_eq!(o, Less);
    ///
    /// let (e, o) = Float::exp_rational_prec_ref(&Rational::from(0), 10);
    /// assert_eq!(e.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn exp_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::exp_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Exp for Float {
    type Output = Self;

    /// Computes $e^x$, the exponential of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the exponential is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::exp_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::exp_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::exp_prec`]. If
    /// you want both of these things, consider using [`Float::exp_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Exp;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.exp().is_nan());
    /// assert_eq!(Float::INFINITY.exp(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.exp(), Float::ZERO);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.exp().to_string(),
    ///     "2.718281828459045235360287471351"
    /// );
    /// ```
    #[inline]
    fn exp(self) -> Self {
        let prec = self.significant_bits();
        self.exp_prec_round(prec, Nearest).0
    }
}

impl Exp for &Float {
    type Output = Float;

    /// Computes $e^x$, the exponential of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the exponential is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::exp_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::exp_prec_ref`]. If you want both of these things, consider using
    /// [`Float::exp_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Exp;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).exp().is_nan());
    /// assert_eq!((&Float::INFINITY).exp(), Float::INFINITY);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).exp(), Float::ZERO);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).exp().to_string(),
    ///     "2.718281828459045235360287471351"
    /// );
    /// ```
    #[inline]
    fn exp(self) -> Float {
        let prec = self.significant_bits();
        self.exp_prec_round_ref(prec, Nearest).0
    }
}

impl ExpAssign for Float {
    /// Computes $e^x$, the exponential of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the exponential is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets e^x+\varepsilon.
    /// $$
    /// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$,
    ///   where $p$ is the precision of the input.
    ///
    /// See the [`Float::exp`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::exp_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::exp_prec_assign`]. If you want both of these things, consider using
    /// [`Float::exp_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExpAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.exp_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.exp_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.exp_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.exp_assign();
    /// assert_eq!(x.to_string(), "2.718281828459045235360287471351");
    /// ```
    #[inline]
    fn exp_assign(&mut self) {
        let prec = self.significant_bits();
        self.exp_prec_round_assign(prec, Nearest);
    }
}

/// Computes $e^x$, the exponential of a primitive float. Using this function is more accurate than
/// using the default `exp` function or the one provided by `libm`.
///
/// $$
/// f(x) = e^x+\varepsilon.
/// $$
/// - If $e^x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=0.0$
/// - $f(\pm0.0)=1.0$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a large negative
/// `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::exp::primitive_float_exp;
///
/// assert!(primitive_float_exp(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_exp(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp(f32::NEGATIVE_INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(NiceFloat(primitive_float_exp(0.0f32)), NiceFloat(1.0));
/// assert_eq!(NiceFloat(primitive_float_exp(1.0f32)), NiceFloat(2.7182817));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_exp<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::exp_prec, x)
}

/// Computes $e^x$, the exponential of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = e^x+\varepsilon.
/// $$
/// - If $e^x$ is infinite or zero, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $e^x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 e^x\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=1$
///
/// Overflow and underflow are possible: a large positive `x` gives $\infty$, and a large negative
/// `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::exp::primitive_float_exp_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_exp_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(1.3956124250860895)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_exp_rational::<f64>(&Rational::from(-10000))),
///     NiceFloat(0.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_exp_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::exp_rational_prec_ref, x)
}
