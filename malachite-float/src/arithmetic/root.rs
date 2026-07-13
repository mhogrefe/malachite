// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2005-2025 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_nan};
use core::cmp::Ordering::{self, *};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, CheckedRoot, IsPowerOf2, Parity, Reciprocal, Root, RootAssign, RootRem,
    Sign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// For roots of order above this threshold, x^(1/k) is computed as exp(ln|x|/k) rather than by an
// integer root of the scaled significand. This is the k > 100 threshold from root.c, MPFR 4.3.0.
const HIGH_K_THRESHOLD: u64 = 100;

// Decides an `Exact`-rounding-mode root exactly: x^(1/k) is exactly representable iff, writing |x|
// = m * 2^e with m odd, e is divisible by k and m is a perfect kth power. Panics if the root is
// inexact; otherwise returns it, rounded (exactly) to `prec` bits.
fn root_u_exact(x: &Float, k: u64, prec: u64) -> (Float, Ordering) {
    let m = x.significand_ref().unwrap();
    let nu = m.trailing_zeros().unwrap();
    let m_odd = m >> nu;
    let e =
        i128::from(x.get_exponent().unwrap()) - i128::from(m.significant_bits()) + i128::from(nu);
    let ki = i128::from(k);
    assert_eq!(e.rem_euclid(ki), 0, "Inexact root");
    let root = (&m_odd).checked_root(k).expect("Inexact root");
    let (root, o) = Float::from_natural_prec_round(root, prec, Exact);
    debug_assert_eq!(o, Equal);
    let root = root << i64::exact_from(e / ki);
    if x.is_sign_negative() {
        (-root, Equal)
    } else {
        (root, Equal)
    }
}

// The integer-root path for 2 <= k <= 100: scale the significand m so that its integer kth root has
// exactly n = prec (+ 1 for `Nearest`) bits, take the root, and round.
//
// This is mpfr_rootn_ui from root.c, MPFR 4.3.0, where the input is finite, nonzero, not a singular
// value, |x| != 1, x is negative only for odd k, and 2 <= k <= 100 -- with one improvement adopted
// from mpfr_cbrt (cbrt.c, MPFR 4.3.0) and generalized from k = 3 to any k: when m has more than k *
// n bits, it is truncated rather than used in full. Any rounding breakpoint at precision n
// (including the `Nearest` midpoints, since n already includes the round bit) has n significant
// bits, so its kth power has at most k * n bits; bits of m below that can never move the root
// across a breakpoint, and only need to be folded into the inexact flag. This makes the root's cost
// independent of the input precision. (mpfr_rootn_ui itself keeps all of m.)
fn root_u_integer(x: &Float, k: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let negative = x.is_sign_negative();
    // x = m * 2^e
    let mut m = x.significand_ref().unwrap().clone();
    let mut e = i128::from(x.get_exponent().unwrap()) - i128::from(m.significant_bits());
    let ki = i128::from(k);
    let r = e.rem_euclid(ki); // r = e mod k with 0 <= r < k
    // For rounding to nearest, we want the round bit to be in the root.
    let n = i128::from(prec) + i128::from(rm == Nearest);
    let size_m = i128::from(m.significant_bits());
    // Shift m by t = k * f + r bits (leftwards for positive t, with truncation for negative t) so
    // that the root of m has exactly n bits: we want k * (n - 1) + 1 <= size_m + t <= k * n, so f =
    // floor((k * n - size_m - r) / k).
    let t = (ki * n - size_m - r).div_euclid(ki) * ki + r;
    let mut inexact = false;
    if t >= 0 {
        m <<= u64::exact_from(t);
    } else {
        // Truncate, folding the dropped bits into the inexact flag. If any dropped bit is nonzero,
        // the true root cannot be exactly representable at n bits either: an exact root s with at
        // most n bits would make x = s^k * 2^(k * e') need at most k * n significant bits.
        let cut = u64::exact_from(-t);
        inexact = m.trailing_zeros().unwrap() < cut;
        m >>= cut;
    }
    e -= t;
    // Invariant: x = m * 2^e (up to truncated bits), with e divisible by k.
    let (mut root, rem) = m.root_rem(k);
    inexact = inexact || rem != 0u32;
    // If the root has more than n bits, flush the low sh2 bits into the inexact flag. (The size
    // invariant above makes this unreachable: m has between k * (n - 1) + 1 and k * n bits, so its
    // kth root has exactly n bits. It is kept as a safeguard, mirroring mpfr_rootn_ui.)
    let size_root = root.significant_bits();
    let n = u64::exact_from(n);
    if size_root > n {
        fail_on_untested_path(
            "root_u_integer, root wider than n: the truncation above sizes m so that its root has \
             exactly n bits",
        );
        let sh2 = size_root - n;
        inexact = inexact || root.trailing_zeros().unwrap() < sh2;
        root >>= sh2;
        e += i128::from(sh2) * ki;
    }
    let mut o = Equal;
    if inexact {
        assert_ne!(rm, Exact, "Inexact root");
        // Rounding modes are inverted for negative x, since the rounding decision below is made on
        // the magnitude.
        let rm_abs = if negative { -rm } else { rm };
        if rm_abs == Ceiling || rm_abs == Up || (rm_abs == Nearest && root.odd()) {
            root += Natural::ONE;
            o = Greater;
        } else {
            o = Less;
        }
    }
    // Either o is not `Equal` and the conversion is exact, or o is `Equal` and the conversion
    // rounds only when rm is `Nearest` and the exact root has n = prec + 1 bits (a midpoint).
    let (y, o2) = Float::from_natural_prec(root, prec);
    debug_assert!(o == Equal || o2 == Equal);
    if o == Equal {
        o = o2;
    }
    debug_assert_eq!(e.rem_euclid(ki), 0);
    // The result's exponent is about EXP(x) / k, always within the exponent range for k >= 2, so
    // the shift is exact and cannot overflow or underflow.
    let y = y << i64::exact_from(e / ki);
    if negative { (-y, o.reverse()) } else { (y, o) }
}

// The large-k path: compute x^(1/k) as exp(ln|x|/k), with a Ziv loop and explicit detection of
// exact and midpoint results (which the Ziv loop could never certify).
//
// This is mpfr_root_aux from root.c, MPFR 4.3.0, where the input is finite, nonzero, not a singular
// value, |x| != 1, x is negative only for odd k, and rm is not `Exact`.
fn root_u_aux(x: &Float, k: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let negative = x.is_sign_negative();
    // Rounding modes are inverted for negative x, since t below approximates the magnitude of the
    // root.
    let rm_abs = if negative { -rm } else { rm };
    let abs_x = x.abs();
    let ex = i64::from(x.get_exponent().unwrap());
    // Take some guard bits to prepare for the `exp_t` lost bits below. If 2^-k < |x| < 2^k, then
    // |ln(x)| < k ln(2), thus taking log2(k) bits would be fine; in general |ln(x)| < |EXP(x)|
    // ln(2), so take log2(|EXP(x)|) bits. (MPFR only guards for positive exponents; guarding for
    // negative ones as well keeps the error bound below the working precision for tiny x at low
    // target precisions.)
    let mut working_prec = prec
        + 10
        + if ex == 0 {
            0
        } else {
            ex.unsigned_abs().ceiling_log_base_2()
        };
    let kf = Float::exact_from(k);
    let mut increment = Limb::WIDTH;
    loop {
        // t = ln|x| * (1 + theta) with |theta| <= 2^-working_prec
        let mut t = abs_x.ln_prec_ref(working_prec).0;
        // t = ln|x|/k * (1 + theta)^2; the total error is bounded by 1.5 * 2^(EXP(t) -
        // working_prec)
        t.div_prec_assign_ref(&kf, working_prec);
        let exp_t = i64::from(t.get_exponent().unwrap());
        // t = |x|^(1/k) * (1 + 2^(err - working_prec)); see root.c for the error analysis
        t.exp_prec_assign(working_prec);
        let err = match (exp_t + 2).sign() {
            Greater => u64::exact_from(exp_t + 3),
            Equal => 2,
            Less => 1,
        };
        if working_prec > err
            && float_can_round(
                t.significand_ref().unwrap(),
                working_prec - err,
                prec,
                rm_abs,
            )
        {
            let (root, o) = Float::from_float_prec_round(t, prec, rm_abs);
            return if negative {
                (-root, o.reverse())
            } else {
                (root, o)
            };
        }
        // If we fail to round correctly, check for an exact result or a midpoint result with
        // `Nearest` (regarded as hard-to-round in all precisions in order to determine the ternary
        // value).
        let z = Float::from_float_prec_ref(&t, prec + u64::from(rm == Nearest)).0;
        let (zk, o_pow) = z.pow_u_prec_ref(k, x.significant_bits());
        if o_pow == Equal && zk == abs_x {
            // z is the exact root, so round z directly.
            let (root, o) = Float::from_float_prec_round(z, prec, rm_abs);
            return if negative {
                (-root, o.reverse())
            } else {
                (root, o)
            };
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes the kth root of a nonzero `Rational` whose root is irrational, for k >= 2. The exponent
// of x may be far outside the `Float` exponent range, so it is reduced first.
fn root_u_rational_generic(x: &Rational, k: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    let e = x.floor_log_base_2_abs();
    if e.gt_abs(&0x3fff_0000) && k > 0x3fff_0000 {
        // Both the exponent of x and k are huge. The exponent cannot be reduced to the
        // representable range by shifting x by a multiple of k, so compute the root as root(x /
        // 2^e) * 2^(e/k), where the second factor is 2 raised to a rational power. No overflow or
        // underflow is possible: |e| is bounded by the bit size of x's numerator and denominator,
        // so |e / k| < 2^30 - 1 by an enormous margin.
        let xm = x >> e; // |xm| in [1, 2)
        let p2_exp = Rational::from_signeds(e, i64::exact_from(k));
        loop {
            let mut root = Float::from_rational_prec_round_ref(&xm, working_prec, Floor)
                .0
                .root_u_prec(k, working_prec)
                .0;
            let p2 = Float::power_of_2_rational_prec_ref(&p2_exp, working_prec).0;
            root.mul_prec_assign(p2, working_prec);
            // Three roundings at working_prec plus the initial conversion error 2^(1 - wp) / k: the
            // total relative error is below 2^(2 - wp), or 4 ulps.
            if float_can_round(root.significand_ref().unwrap(), working_prec - 3, prec, rm) {
                return Float::from_float_prec_round(root, prec, rm);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }
    let ki = i64::exact_from(k);
    let mut end_shift = e;
    let x2;
    let reduced_x: &Rational;
    if end_shift.gt_abs(&0x3fff_0000) {
        // Reduce the exponent of x to the representable range by shifting by a multiple of k, so
        // that the root of the reduced value can be shifted back exactly (up to overflow or
        // underflow, which the final `shl_prec_round_assign_helper` handles).
        end_shift -= end_shift.rem_euclid(ki);
        x2 = x >> end_shift;
        reduced_x = &x2;
    } else {
        end_shift = 0;
        reduced_x = x;
    }
    loop {
        let root = Float::from_rational_prec_round_ref(reduced_x, working_prec, Floor)
            .0
            .root_u_prec(k, working_prec)
            .0;
        // The conversion error 2^(1 - wp) is contracted by the root to 2^(1 - wp) / k, and the root
        // itself contributes at most 1/2 ulp, so the total relative error is below 2^(1 - wp), or 2
        // ulps.
        if float_can_round(root.significand_ref().unwrap(), working_prec - 2, prec, rm) {
            let (mut root, mut o) = Float::from_float_prec_round(root, prec, rm);
            if end_shift != 0 {
                o = root.shl_prec_round_assign_helper(i128::from(end_shift / ki), prec, rm, o);
            }
            return (root, o);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Takes the $k$th root of a [`Float`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded root is less than, equal to, or greater than the
    /// exact root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm0.0$ if $k$ is odd and $0.0$ if $k$ is even
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// The result never overflows or underflows: its exponent is close to the exponent of $x$
    /// divided by $k$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_u_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::root_u_round`] instead. If both of these things are true, consider using the
    /// [`Root`](malachite_base::num::arithmetic::traits::Root) implementation instead.
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec_round(3, 20, Floor);
    /// assert_eq!(root.to_string(), "1.25992");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec_round(3, 20, Ceiling);
    /// assert_eq!(root.to_string(), "1.259922");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_u_prec_round(self, k: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.root_u_prec_round_ref(k, prec, rm)
    }

    /// Takes the $k$th root of a [`Float`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded root is less than, equal to, or greater than the
    /// exact root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm0.0$ if $k$ is odd and $0.0$ if $k$ is even
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// The result never overflows or underflows: its exponent is close to the exponent of $x$
    /// divided by $k$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_u_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::root_u_round`] instead. If both of these things are true, consider using the
    /// [`Root`](malachite_base::num::arithmetic::traits::Root) implementation instead.
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec_round_ref(3, 20, Floor);
    /// assert_eq!(root.to_string(), "1.25992");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec_round_ref(3, 20, Ceiling);
    /// assert_eq!(root.to_string(), "1.259922");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn root_u_prec_round_ref(&self, k: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if k == 0 {
            // The 0th root of anything is NaN (IEEE 754-2008).
            return (float_nan!(), Equal);
        } else if k == 1 {
            // x^(1/1) = x
            return Self::from_float_prec_round_ref(self, prec, rm);
        }
        match self {
            Self(NaN) => (float_nan!(), Equal),
            Self(Infinity { sign }) => {
                if *sign {
                    // (+Infinity)^(1/k) = +Infinity
                    (Self(Infinity { sign: true }), Equal)
                } else if k.odd() {
                    // (-Infinity)^(1/k) = -Infinity for odd k
                    (Self(Infinity { sign: false }), Equal)
                } else {
                    // (-Infinity)^(1/k) = NaN for even k
                    (float_nan!(), Equal)
                }
            }
            // (+0.0)^(1/k) = +0.0; (-0.0)^(1/k) = +0.0 for even k and -0.0 for odd k
            Self(Zero { sign }) => (
                Self(Zero {
                    sign: *sign || k.even(),
                }),
                Equal,
            ),
            _ => {
                if self.is_sign_negative() && k.even() {
                    // Negative x has no real kth root for even k.
                    return (float_nan!(), Equal);
                }
                // |x| = 1: the root is x itself (if x is -1, then k is odd).
                if *self == 1u32 || *self == -1i32 {
                    return Self::from_float_prec_round_ref(self, prec, rm);
                }
                if k > HIGH_K_THRESHOLD {
                    if rm == Exact {
                        root_u_exact(self, k, prec)
                    } else {
                        root_u_aux(self, k, prec, rm)
                    }
                } else {
                    root_u_integer(self, k, prec, rm)
                }
            }
        }
    }

    /// Takes the $k$th root of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded root is less than, equal to, or greater than the exact root.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$
    /// - $f(-\infty,k,p)=-\infty$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p)=\pm0.0$ if $k$ is odd and $0.0$ if $k$ is even
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// The result never overflows or underflows: its exponent is close to the exponent of $x$
    /// divided by $k$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_u_prec_round`] instead.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec(3, 20);
    /// assert_eq!(root.to_string(), "1.25992");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec(3, 53);
    /// assert_eq!(root.to_string(), "1.2599210498948732");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_u_prec(self, k: u64, prec: u64) -> (Self, Ordering) {
        self.root_u_prec_round(k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded root is less than, equal to, or greater than the exact root.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$
    /// - $f(-\infty,k,p)=-\infty$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p)=\pm0.0$ if $k$ is odd and $0.0$ if $k$ is even
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// The result never overflows or underflows: its exponent is close to the exponent of $x$
    /// divided by $k$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_u_prec_round`] instead.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec_ref(3, 20);
    /// assert_eq!(root.to_string(), "1.25992");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::from(2.0).root_u_prec_ref(3, 53);
    /// assert_eq!(root.to_string(), "1.2599210498948732");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_u_prec_ref(&self, k: u64, prec: u64) -> (Self, Ordering) {
        self.root_u_prec_round_ref(k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Float`], rounding the result with the specified rounding mode.
    /// The precision of the output is the precision of the input. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded root is less than, equal
    /// to, or greater than the exact root. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm0.0$ if $k$ is odd and $0.0$ if $k$ is even
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// The result never overflows or underflows: its exponent is close to the exponent of $x$
    /// divided by $k$.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_rational_prec(Rational::from(2u32), 20).0;
    /// let (root, o) = x.root_u_round(3, Floor);
    /// assert_eq!(root.to_string(), "1.25992");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_u_round(self, k: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.root_u_prec_round(k, prec, rm)
    }

    /// Takes the $k$th root of a [`Float`], rounding the result with the specified rounding mode.
    /// The precision of the output is the precision of the input. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded root is less
    /// than, equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm0.0$ if $k$ is odd and $0.0$ if $k$ is even
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// The result never overflows or underflows: its exponent is close to the exponent of $x$
    /// divided by $k$.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_rational_prec(Rational::from(2u32), 20).0;
    /// let (root, o) = x.root_u_round_ref(3, Floor);
    /// assert_eq!(root.to_string(), "1.25992");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_u_round_ref(&self, k: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.root_u_prec_round_ref(k, self.significant_bits(), rm)
    }

    /// Takes the $k$th root of a [`Float`] in place, rounding the result to the specified precision
    /// and with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded root is less than, equal to, or greater than the exact root.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// Special cases: see [`Float::root_u_prec_round`].
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(2.0);
    /// assert_eq!(x.root_u_prec_round_assign(3, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.25992");
    /// ```
    #[inline]
    pub fn root_u_prec_round_assign(&mut self, k: u64, prec: u64, rm: RoundingMode) -> Ordering {
        let (root, o) = core::mem::take(self).root_u_prec_round(k, prec, rm);
        *self = root;
        o
    }

    /// Takes the $k$th root of a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. An [`Ordering`] is returned, indicating whether the rounded root is
    /// less than, equal to, or greater than the exact root.
    ///
    /// Special cases: see [`Float::root_u_prec`].
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
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(2.0);
    /// assert_eq!(x.root_u_prec_assign(3, 20), Less);
    /// assert_eq!(x.to_string(), "1.25992");
    /// ```
    #[inline]
    pub fn root_u_prec_assign(&mut self, k: u64, prec: u64) -> Ordering {
        self.root_u_prec_round_assign(k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The precision is retained. An [`Ordering`] is returned, indicating whether
    /// the rounded root is less than, equal to, or greater than the exact root.
    ///
    /// Special cases: see [`Float::root_u_round`].
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_rational_prec(Rational::from(2u32), 20).0;
    /// assert_eq!(x.root_u_round_assign(3, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.25992");
    /// ```
    #[inline]
    pub fn root_u_round_assign(&mut self, k: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.root_u_prec_round_assign(k, prec, rm)
    }

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded root is less than, equal
    /// to, or greater than the exact root. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=0.0$
    /// - $f(-\infty,k,p,m)=-0.0$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm\infty$ if $k$ is odd and $\infty$ if $k$ is even
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible only for $k=-1$ (the reciprocal); for other $k$ the
    /// result's exponent is close to the exponent of $x$ divided by $k$, always within range.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_s_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::root_s_round`] instead. If both of these things are true, consider using the
    /// [`Root`](malachite_base::num::arithmetic::traits::Root) implementation instead.
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_s_prec_round(-3, 20, Floor);
    /// assert_eq!(root.to_string(), "0.7937");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::from(2.0).root_s_prec_round(-3, 20, Ceiling);
    /// assert_eq!(root.to_string(), "0.793701");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_s_prec_round(self, k: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.root_s_prec_round_ref(k, prec, rm)
    }

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded root is less
    /// than, equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=0.0$
    /// - $f(-\infty,k,p,m)=-0.0$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm\infty$ if $k$ is odd and $\infty$ if $k$ is even
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible only for $k=-1$ (the reciprocal); for other $k$ the
    /// result's exponent is close to the exponent of $x$ divided by $k$, always within range.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_s_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::root_s_round`] instead. If both of these things are true, consider using the
    /// [`Root`](malachite_base::num::arithmetic::traits::Root) implementation instead.
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_s_prec_round_ref(-3, 20, Floor);
    /// assert_eq!(root.to_string(), "0.7937");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::from(2.0).root_s_prec_round_ref(-3, 20, Ceiling);
    /// assert_eq!(root.to_string(), "0.793701");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn root_s_prec_round_ref(&self, k: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        if k >= 0 {
            return self.root_u_prec_round_ref(k.unsigned_abs(), prec, rm);
        }
        assert_ne!(prec, 0);
        // Singular values for k < 0
        match self {
            Self(NaN) => (float_nan!(), Equal),
            Self(Infinity { sign }) => {
                if *sign || k.odd() {
                    // (+Infinity)^(1/k) = +0.0; (-Infinity)^(1/k) = -0.0 for odd k
                    (Self(Zero { sign: *sign }), Equal)
                } else {
                    // (-Infinity)^(1/k) = NaN for even k
                    (float_nan!(), Equal)
                }
            }
            Self(Zero { sign }) => {
                // (+0.0)^(1/k) = +Infinity; (-0.0)^(1/k) = +Infinity for even k and -Infinity for
                // odd k
                (
                    Self(Infinity {
                        sign: *sign || k.even(),
                    }),
                    Equal,
                )
            }
            _ => {
                if self.is_sign_negative() && k.even() {
                    // Negative x has no real kth root for even k.
                    return (float_nan!(), Equal);
                }
                // |x| = 1: the root is x itself (if x is -1, then k is odd).
                if *self == 1u32 || *self == -1i32 {
                    return Self::from_float_prec_round_ref(self, prec, rm);
                }
                // k = -1 is x^-1 and k = -2 is 1/sqrt(x); both have specialized implementations
                // that also handle the overflow and underflow that are possible when k = -1.
                if k == -1 {
                    return self.reciprocal_prec_round_ref(prec, rm);
                } else if k == -2 {
                    return self.reciprocal_sqrt_prec_round_ref(prec, rm);
                }
                let ku = k.unsigned_abs();
                if rm == Exact {
                    // The root is exactly representable iff |x| is 2 raised to a multiple of k.
                    let e = i64::from(self.get_exponent().unwrap()) - 1;
                    if self.significand_ref().unwrap().is_power_of_2() && e % k == 0 {
                        let (root, o) =
                            Self::from_float_prec_round_ref(&(Self::ONE << (e / k)), prec, Exact);
                        debug_assert_eq!(o, Equal);
                        return if self.is_sign_negative() {
                            (-root, Equal)
                        } else {
                            (root, Equal)
                        };
                    }
                    panic!("Inexact root");
                }
                // General case: compute the |k|th root, then invert. The root is computed before
                // the division so that overflow and underflow are impossible, and midpoints are
                // impossible as well. An exact case implies that |x| is a power of 2; it is
                // detected after `float_can_round`. The root is exactly representable iff |x| is 2
                // raised to a multiple of k.
                let exact_representable = self.significand_ref().unwrap().is_power_of_2()
                    && (i64::from(self.get_exponent().unwrap()) - 1) % k == 0;
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                loop {
                    // (MPFR uses faithful rounding here to avoid the potentially costly detection
                    // of exact cases in the underlying root; `Nearest` is used instead, which only
                    // improves the error bound.)
                    let mut t = self.root_u_prec_ref(ku, working_prec).0;
                    let o = t.reciprocal_prec_round_assign(working_prec, rm);
                    // The final error is bounded by 5 ulps (see algorithms.tex, "Generic error of
                    // inverse"), which is <= 2^3 ulps.
                    //
                    // The exact-case escape must verify divisibility of the exponent, not just that
                    // |x| is a power of 2 (MPFR's condition): otherwise a root that merely rounds
                    // to a power of 2 at the working precision (for example 0.5^(1/50000) ~ 1 at
                    // low precisions), followed by an exact reciprocal, is mistaken for an exact
                    // result.
                    if float_can_round(t.significand_ref().unwrap(), working_prec - 3, prec, rm)
                        || (o == Equal && exact_representable)
                    {
                        return Self::from_float_prec_round(t, prec, rm);
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded root is less than, equal to, or greater
    /// than the exact root. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=0.0$
    /// - $f(-\infty,k,p)=-0.0$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p)=\pm\infty$ if $k$ is odd and $\infty$ if $k$ is even
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible only for $k=-1$ (the reciprocal); for other $k$ the
    /// result's exponent is close to the exponent of $x$ divided by $k$, always within range.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_s_prec_round`] instead.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_s_prec(-3, 53);
    /// assert_eq!(root.to_string(), "0.7937005259840998");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_s_prec(self, k: i64, prec: u64) -> (Self, Ordering) {
        self.root_s_prec_round(k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded root is less than, equal to,
    /// or greater than the exact root. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=0.0$
    /// - $f(-\infty,k,p)=-0.0$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p)=\pm\infty$ if $k$ is odd and $\infty$ if $k$ is even
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible only for $k=-1$ (the reciprocal); for other $k$ the
    /// result's exponent is close to the exponent of $x$ divided by $k$, always within range.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_s_prec_round`] instead.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = Float::from(2.0).root_s_prec_ref(-3, 53);
    /// assert_eq!(root.to_string(), "0.7937005259840998");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_s_prec_ref(&self, k: i64, prec: u64) -> (Self, Ordering) {
        self.root_s_prec_round_ref(k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded root is less than, equal to, or greater than the exact root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=0.0$
    /// - $f(-\infty,k,p,m)=-0.0$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm\infty$ if $k$ is odd and $\infty$ if $k$ is even
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible only for $k=-1$ (the reciprocal); for other $k$ the
    /// result's exponent is close to the exponent of $x$ divided by $k$, always within range.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_rational_prec(Rational::from(2u32), 20).0;
    /// let (root, o) = x.root_s_round(-3, Ceiling);
    /// assert_eq!(root.to_string(), "0.793701");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_s_round(self, k: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.root_s_prec_round(k, prec, rm)
    }

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded root is less than, equal to, or greater than the exact root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=0.0$
    /// - $f(-\infty,k,p,m)=-0.0$ if $k$ is odd and `NaN` if $k$ is even
    /// - $f(\pm0.0,k,p,m)=\pm\infty$ if $k$ is odd and $\infty$ if $k$ is even
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible only for $k=-1$ (the reciprocal); for other $k$ the
    /// result's exponent is close to the exponent of $x$ divided by $k$, always within range.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_rational_prec(Rational::from(2u32), 20).0;
    /// let (root, o) = x.root_s_round_ref(-3, Ceiling);
    /// assert_eq!(root.to_string(), "0.793701");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_s_round_ref(&self, k: i64, rm: RoundingMode) -> (Self, Ordering) {
        self.root_s_prec_round_ref(k, self.significant_bits(), rm)
    }

    /// Takes the $k$th root of a [`Float`] in place, where $k$ may be negative, rounding the result
    /// to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded root is less than, equal to, or greater than the
    /// exact root.
    ///
    /// Special cases: see [`Float::root_s_prec_round`].
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(2.0);
    /// assert_eq!(x.root_s_prec_round_assign(-3, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.7937");
    /// ```
    #[inline]
    pub fn root_s_prec_round_assign(&mut self, k: i64, prec: u64, rm: RoundingMode) -> Ordering {
        let (root, o) = core::mem::take(self).root_s_prec_round(k, prec, rm);
        *self = root;
        o
    }

    /// Takes the $k$th root of a [`Float`] in place, where $k$ may be negative, rounding the result
    /// to the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded root is less than, equal to, or greater than the exact root.
    ///
    /// Special cases: see [`Float::root_s_prec`].
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
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(2.0);
    /// assert_eq!(x.root_s_prec_assign(-3, 20), Less);
    /// assert_eq!(x.to_string(), "0.7937");
    /// ```
    #[inline]
    pub fn root_s_prec_assign(&mut self, k: i64, prec: u64) -> Ordering {
        self.root_s_prec_round_assign(k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Float`] in place, where $k$ may be negative, rounding the result
    /// with the specified rounding mode. The precision is retained. An [`Ordering`] is returned,
    /// indicating whether the rounded root is less than, equal to, or greater than the exact root.
    ///
    /// Special cases: see [`Float::root_s_round`].
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_rational_prec(Rational::from(2u32), 20).0;
    /// assert_eq!(x.root_s_round_assign(-3, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.7937");
    /// ```
    #[inline]
    pub fn root_s_round_assign(&mut self, k: i64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.root_s_prec_round_assign(k, prec, rm)
    }

    /// Takes the $k$th root of a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded root is
    /// less than, equal to, or greater than the exact root. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p,m)=0.0$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $k$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_u_rational_prec`]
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) =
    ///     Float::root_u_rational_prec_round(Rational::from_signeds(3, 5), 3, 20, Floor);
    /// assert_eq!(root.to_string(), "0.843432");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) =
    ///     Float::root_u_rational_prec_round(Rational::from_signeds(3, 5), 3, 20, Ceiling);
    /// assert_eq!(root.to_string(), "0.843433");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn root_u_rational_prec_round(
        x: Rational,
        k: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::root_u_rational_prec_round_ref(&x, k, prec, rm)
    }

    /// Takes the $k$th root of a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode and returning the result as a [`Float`]. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded root
    /// is less than, equal to, or greater than the exact root. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p,m)=0.0$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $k$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_u_rational_prec`]
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) =
    ///     Float::root_u_rational_prec_round_ref(&Rational::from_signeds(3, 5), 3, 20, Floor);
    /// assert_eq!(root.to_string(), "0.843432");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) =
    ///     Float::root_u_rational_prec_round_ref(&Rational::from_signeds(3, 5), 3, 20, Ceiling);
    /// assert_eq!(root.to_string(), "0.843433");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn root_u_rational_prec_round_ref(
        x: &Rational,
        k: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if k == 0 {
            // The 0th root of anything is NaN (IEEE 754-2008).
            return (float_nan!(), Equal);
        } else if k == 1 {
            // x^(1/1) = x
            return Self::from_rational_prec_round_ref(x, prec, rm);
        }
        if *x == 0u32 {
            // 0^(1/k) = 0
            return (Self(Zero { sign: true }), Equal);
        }
        if *x < 0u32 && k.even() {
            // Negative x has no real kth root for even k.
            return (float_nan!(), Equal);
        }
        // A rational kth root is exact iff the numerator and denominator are both perfect kth
        // powers.
        if let Some(root) = x.checked_root(k) {
            return Self::from_rational_prec_round(root, prec, rm);
        }
        // The root of any other rational is irrational.
        assert_ne!(rm, Exact, "Inexact root");
        root_u_rational_generic(x, k, prec, rm)
    }

    /// Takes the $k$th root of a [`Rational`], rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded root is less than,
    /// equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p)=0.0$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $k$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_u_rational_prec_round`] instead.
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
    /// let (root, o) = Float::root_u_rational_prec(Rational::from_signeds(3, 5), 3, 20);
    /// assert_eq!(root.to_string(), "0.843432");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::root_u_rational_prec(Rational::from_signeds(8, 27), 3, 10);
    /// assert_eq!(root.to_string(), "0.667");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_u_rational_prec(x: Rational, k: u64, prec: u64) -> (Self, Ordering) {
        Self::root_u_rational_prec_round(x, k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Rational`], rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded root is less
    /// than, equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,k,p)=0.0$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(x,1,p,m)=x$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $k$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_u_rational_prec_round`] instead.
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
    /// let (root, o) = Float::root_u_rational_prec_ref(&Rational::from_signeds(3, 5), 3, 20);
    /// assert_eq!(root.to_string(), "0.843432");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = Float::root_u_rational_prec_ref(&Rational::from_signeds(8, 27), 3, 10);
    /// assert_eq!(root.to_string(), "0.667");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_u_rational_prec_ref(x: &Rational, k: u64, prec: u64) -> (Self, Ordering) {
        Self::root_u_rational_prec_round_ref(x, k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Rational`], where $k$ may be negative, rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded root is less than, equal to, or greater than the exact root. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(0,k,p,m)=\infty$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $|k|$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_s_rational_prec`]
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) =
    ///     Float::root_s_rational_prec_round(Rational::from_signeds(3, 5), -3, 20, Nearest);
    /// assert_eq!(root.to_string(), "1.185631");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn root_s_rational_prec_round(
        x: Rational,
        k: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::root_s_rational_prec_round_ref(&x, k, prec, rm)
    }

    /// Takes the $k$th root of a [`Rational`], where $k$ may be negative, rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded root is less than, equal to, or greater than the exact root.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p+1}$.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(0,k,p,m)=\infty$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $|k|$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::root_s_rational_prec`]
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) =
    ///     Float::root_s_rational_prec_round_ref(&Rational::from_signeds(3, 5), -3, 20, Nearest);
    /// assert_eq!(root.to_string(), "1.185631");
    /// assert_eq!(o, Less);
    /// ```
    pub fn root_s_rational_prec_round_ref(
        x: &Rational,
        k: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if k >= 0 {
            return Self::root_u_rational_prec_round_ref(x, k.unsigned_abs(), prec, rm);
        }
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // 0^(1/k) = +Infinity for k < 0
            return (Self(Infinity { sign: true }), Equal);
        }
        if *x < 0u32 && k.even() {
            // Negative x has no real kth root for even k.
            return (float_nan!(), Equal);
        }
        // x^(1/k) = (1/x)^(1/-k), and the reciprocal of a rational is exact.
        Self::root_u_rational_prec_round(x.reciprocal(), k.unsigned_abs(), prec, rm)
    }

    /// Takes the $k$th root of a [`Rational`], where $k$ may be negative, rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded root is less than, equal to, or greater than the exact root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(0,k,p)=\infty$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $|k|$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_s_rational_prec_round`] instead.
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
    /// let (root, o) = Float::root_s_rational_prec(Rational::from_signeds(27, 8), -3, 10);
    /// assert_eq!(root.to_string(), "0.667");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_s_rational_prec(x: Rational, k: i64, prec: u64) -> (Self, Ordering) {
        Self::root_s_rational_prec_round(x, k, prec, Nearest)
    }

    /// Takes the $k$th root of a [`Rational`], where $k$ may be negative, rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded root is less than, equal to, or greater than the exact root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the root is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,k,p) = \sqrt\[k\]{x}+\varepsilon.
    /// $$
    /// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\sqrt\[k\]{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases for $k<0$ (for $k\geq 0$ the cases are those of the unsigned version):
    /// - $f(0,k,p)=\infty$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// Overflow and underflow are possible when the exponent of $x$ exceeds $|k|$ times the maximum
    /// [`Float`] exponent in absolute value.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::root_s_rational_prec_round`] instead.
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
    /// let (root, o) = Float::root_s_rational_prec_ref(&Rational::from_signeds(27, 8), -3, 10);
    /// assert_eq!(root.to_string(), "0.667");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn root_s_rational_prec_ref(x: &Rational, k: i64, prec: u64) -> (Self, Ordering) {
        Self::root_s_rational_prec_round_ref(x, k, prec, Nearest)
    }
}

impl Root<u64> for Float {
    type Output = Self;

    /// Takes the $k$th root of a [`Float`], rounding the result to the nearest value at the
    /// precision of the input. The [`Float`] is taken by value.
    ///
    /// If the root is equidistant from two [`Float`]s with that precision, the [`Float`] with fewer
    /// 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// See the [`Float::root_u_prec_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::root_u_prec`] instead.
    /// If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::root_u_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Root;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(27.0).root(3u64).to_string(), "3.0");
    /// ```
    #[inline]
    fn root(self, pow: u64) -> Self {
        let prec = self.significant_bits();
        self.root_u_prec(pow, prec).0
    }
}

impl Root<u64> for &Float {
    type Output = Float;

    /// Takes the $k$th root of a [`Float`], rounding the result to the nearest value at the
    /// precision of the input. The [`Float`] is taken by reference.
    ///
    /// If the root is equidistant from two [`Float`]s with that precision, the [`Float`] with fewer
    /// 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// See the [`Float::root_u_prec_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::root_u_prec`] instead.
    /// If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::root_u_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Root;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(27.0)).root(3u64).to_string(), "3.0");
    /// ```
    #[inline]
    fn root(self, pow: u64) -> Float {
        self.root_u_prec_ref(pow, self.significant_bits()).0
    }
}

impl RootAssign<u64> for Float {
    /// Takes the $k$th root of a [`Float`] in place, rounding the result to the nearest value at
    /// the precision of the input.
    ///
    /// If the root is equidistant from two [`Float`]s with that precision, the [`Float`] with fewer
    /// 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// See the [`Float::root_u_prec_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::root_u_prec`] instead.
    /// If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::root_u_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(32.0);
    /// x.root_assign(5u64);
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    #[inline]
    fn root_assign(&mut self, pow: u64) {
        let prec = self.significant_bits();
        self.root_u_prec_round_assign(pow, prec, Nearest);
    }
}

impl Root<i64> for Float {
    type Output = Self;

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result to the
    /// nearest value at the precision of the input. The [`Float`] is taken by value.
    ///
    /// If the root is equidistant from two [`Float`]s with that precision, the [`Float`] with fewer
    /// 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// See the [`Float::root_s_prec_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::root_s_prec`] instead.
    /// If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::root_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Root;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(8.0).root(-3i64).to_string(), "0.5");
    /// ```
    #[inline]
    fn root(self, pow: i64) -> Self {
        let prec = self.significant_bits();
        self.root_s_prec(pow, prec).0
    }
}

impl Root<i64> for &Float {
    type Output = Float;

    /// Takes the $k$th root of a [`Float`], where $k$ may be negative, rounding the result to the
    /// nearest value at the precision of the input. The [`Float`] is taken by reference.
    ///
    /// If the root is equidistant from two [`Float`]s with that precision, the [`Float`] with fewer
    /// 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// See the [`Float::root_s_prec_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::root_s_prec`] instead.
    /// If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::root_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Root;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(8.0)).root(-3i64).to_string(), "0.5");
    /// ```
    #[inline]
    fn root(self, pow: i64) -> Float {
        self.root_s_prec_ref(pow, self.significant_bits()).0
    }
}

impl RootAssign<i64> for Float {
    /// Takes the $k$th root of a [`Float`] in place, where $k$ may be negative, rounding the result
    /// to the nearest value at the precision of the input.
    ///
    /// If the root is equidistant from two [`Float`]s with that precision, the [`Float`] with fewer
    /// 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// See the [`Float::root_s_prec_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::root_s_prec`] instead.
    /// If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::root_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(4.0);
    /// x.root_assign(-2i64);
    /// assert_eq!(x.to_string(), "0.5");
    /// ```
    #[inline]
    fn root_assign(&mut self, pow: i64) {
        let prec = self.significant_bits();
        self.root_s_prec_round_assign(pow, prec, Nearest);
    }
}

/// Takes the $k$th root of a primitive float. The result is correctly rounded. In particular,
/// `primitive_float_root_u(x, 3)` is a correctly-rounded cube root, unlike the standard library's
/// `cbrt`.
///
/// $$
/// f(x,k) = \sqrt\[k\]{x}+\varepsilon.
/// $$
/// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\sqrt\[k\]{x}|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases: see [`Float::root_u_prec_round`].
///
/// # Worst-case complexity
/// $T(n) = O(n^{3/2} \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the output.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::root::primitive_float_root_u;
///
/// assert_eq!(
///     NiceFloat(primitive_float_root_u(27.0f64, 3)),
///     NiceFloat(3.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_root_u(2.0f64, 3)),
///     NiceFloat(1.2599210498948732)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_root_u<T: PrimitiveFloat>(x: T, k: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|f, prec| Float::root_u_prec(f, k, prec), x)
}

/// Takes the $k$th root of a primitive float, where $k$ may be negative. The result is correctly
/// rounded.
///
/// $$
/// f(x,k) = \sqrt\[k\]{x}+\varepsilon.
/// $$
/// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\sqrt\[k\]{x}|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases: see [`Float::root_s_prec_round`].
///
/// # Worst-case complexity
/// $T(n) = O(n^{3/2} \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the output.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::root::primitive_float_root_s;
///
/// assert_eq!(
///     NiceFloat(primitive_float_root_s(8.0f64, -3)),
///     NiceFloat(0.5)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_root_s(2.0f64, -3)),
///     NiceFloat(0.7937005259840998)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_root_s<T: PrimitiveFloat>(x: T, k: i64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|f, prec| Float::root_s_prec(f, k, prec), x)
}

/// Takes the $k$th root of a [`Rational`], returning the result as a primitive float. The result is
/// correctly rounded.
///
/// $$
/// f(x,k) = \sqrt\[k\]{x}+\varepsilon.
/// $$
/// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\sqrt\[k\]{x}|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases: see [`Float::root_u_rational_prec_round`].
///
/// # Worst-case complexity
/// $T(n) = O(n^{3/2} \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the output.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::root::primitive_float_root_u_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_root_u_rational::<f32>(
///         &Rational::from_signeds(8, 27),
///         3
///     )),
///     NiceFloat(0.6666667)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_root_u_rational<T: PrimitiveFloat>(x: &Rational, k: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(|q, prec| Float::root_u_rational_prec_ref(q, k, prec), x)
}

/// Takes the $k$th root of a [`Rational`], where $k$ may be negative, returning the result as a
/// primitive float. The result is correctly rounded.
///
/// $$
/// f(x,k) = \sqrt\[k\]{x}+\varepsilon.
/// $$
/// - If $\sqrt\[k\]{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\sqrt\[k\]{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\sqrt\[k\]{x}|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases: see [`Float::root_s_rational_prec_round`].
///
/// # Worst-case complexity
/// $T(n) = O(n^{3/2} \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the output.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::root::primitive_float_root_s_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_root_s_rational::<f32>(
///         &Rational::from(8),
///         -3
///     )),
///     NiceFloat(0.5)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_root_s_rational<T: PrimitiveFloat>(x: &Rational, k: i64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(|q, prec| Float::root_s_rational_prec_ref(q, k, prec), x)
}
