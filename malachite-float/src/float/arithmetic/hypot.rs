// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::WIDTH_MINUS_1;
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::{
    Float, emulate_float_float_to_float_fn, float_either_infinity, float_either_zero,
    float_infinity, float_nan, significand_bits,
};
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{Abs, CeilingLogBase2, Hypot, HypotAssign, Square};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::natural::arithmetic::float::sqrt::sqrt_float_significand_ref;
use malachite_nz::platform::Limb;

const MAX_EXPONENT_I64: i64 = Float::MAX_EXPONENT as i64;

// Exact integer-level path. Both inputs are finite and nonzero. The exact sum of squares is formed
// as x^2 + y^2 = s * 2^(2k) with s a `Natural`, and its square root is taken by the raw
// significand-level kernel behind `Float::sqrt_prec_round`, which consumes the whole of s (its
// sticky accounting covers every dropped bit) but only produces `prec` bits. Since the arithmetic
// is on integers and the kernel is indifferent to where the binade actually lies -- only the
// exponent's parity matters -- no exponent-range trouble is possible until the result is assembled,
// where a too-large exponent saturates just as an overflowing `shl` would. This path has no
// analogue in the C code; it replaces MPFR's FIXME concerning the underflow of the scaled y, and
// also decides `Exact` directly: the kernel's ternary value is `Equal` if and only if the square
// root is exactly representable at `prec` bits.
fn hypot_exact_helper(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (
        Float(Finite {
            exponent: x_exponent,
            significand: x_significand,
            ..
        }),
        Float(Finite {
            exponent: y_exponent,
            significand: y_significand,
            ..
        }),
    ) = (x, y)
    else {
        unreachable!()
    };
    // lsb-anchored decompositions |x| = mx * 2^lx and |y| = my * 2^ly, with trailing zeros stripped
    // to keep the squares small
    let decompose = |significand: &Natural, exponent: i32| {
        let tz = significand.trailing_zeros().unwrap();
        (
            significand >> tz,
            i64::from(exponent) - i64::exact_from(significand_bits(significand))
                + i64::exact_from(tz),
        )
    };
    let (mx, lx) = decompose(x_significand, *x_exponent);
    let (my, ly) = decompose(y_significand, *y_exponent);
    let a = min(lx, ly);
    let s = (mx.square() << (u64::exact_from(lx - a) << 1))
        + (my.square() << (u64::exact_from(ly - a) << 1));
    // Normalize s into significand form (top bit at a limb boundary; the low padding bits are
    // zero), representing the value 0.s * 2^e with e = bits(s) + 2k, where k = a. The kernel only
    // cares about e's parity, so it is passed as just the parity bit, and the discarded even part
    // is restored afterwards.
    let s_bits = s.significant_bits();
    let sn = s << (s_bits.wrapping_neg() & WIDTH_MINUS_1);
    let e = i64::exact_from(s_bits) + (a << 1);
    let e_syn = i32::exact_from(e.rem_euclid(2));
    let delta = (e - i64::from(e_syn)) >> 1;
    let (root, out_exp, o) = sqrt_float_significand_ref(
        &sn,
        e_syn,
        s_bits,
        prec,
        if rm == Exact { Floor } else { rm },
    );
    if rm == Exact {
        assert_eq!(o, Equal, "Inexact Float hypot");
    }
    let exp = i64::from(out_exp) + delta;
    // The true result is in [|x|, sqrt(2) * (|x| + ulp)), so its exponent is E_x or E_x + 1, and
    // underflow is impossible; overflow is possible only when E_x is at the very top of the range.
    if exp > MAX_EXPONENT_I64 {
        assert_ne!(rm, Exact, "Inexact Float hypot");
        return match rm {
            Floor | Down => {
                // Rounding toward zero cannot leave the exponent range: the exact path is only
                // reached with an exponent gap too large for the second operand to affect the first
                // operand's binade, so the true result is strictly below 2^Emax whenever its floor
                // is representable at all. The arm is only defensive.
                fail_on_untested_path("hypot_exact_helper, overflow with Floor or Down");
                (Float::max_finite_value_with_prec(prec), Less)
            }
            _ => (float_infinity!(), Greater),
        };
    }
    (
        Float(Finite {
            sign: true,
            exponent: i32::exact_from(exp),
            precision: prec,
            significand: root,
        }),
        o,
    )
}

// This is mpfr_hypot from hypot.c, MPFR 4.2.2, with two house deviations, described below. Both
// inputs are finite and nonzero; the singular cases are handled by the callers.
fn hypot_prec_round_helper(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // Ensure |x| >= |y|.
    let (x, y) = if x.lt_abs(y) { (y, x) } else { (x, y) };
    let ex = i64::from(x.get_exponent().unwrap());
    let ey = i64::from(y.get_exponent().unwrap());
    let diff_exp = u64::exact_from(ex - ey);
    let px = x.significant_bits();
    let py = y.significant_bits();
    // Is |x| a suitable approximation to the precision `prec`? When the exponent gap is above this
    // threshold, hypot(x, y) = |x| + g with 0 < g < 2^(E_x - 2 * diff_exp), and the result is never
    // exactly representable (see algorithms.tex), so the rounding can be determined from |x| alone,
    // and `Exact` always panics. The C code hand-rolls the rounding (including the round-up-on-tie
    // behavior under Nearest and the add-one-ulp adjustments); float_round_near_x is the same
    // computation.
    let threshold = (max(px, prec) + u64::from(rm == Nearest)) << 1;
    if diff_exp > threshold {
        assert_ne!(rm, Exact, "Inexact Float hypot");
        // Only take the absolute value if it actually changes anything: `Abs` on a reference copies
        // the whole significand, which can be huge in exactly this regime.
        let abs_x_owned;
        let abs_x = if *x > 0u32 {
            x
        } else {
            abs_x_owned = x.abs();
            &abs_x_owned
        };
        if let Some(r) = float_round_near_x(abs_x, diff_exp << 1, true, prec, rm) {
            return r;
        }
        // Since diff_exp > threshold, the error exponent exceeds both precision bounds that
        // float_round_near_x checks, so it never declines; the fallthrough is only defensive.
        fail_on_untested_path("hypot_prec_round_helper, float_round_near_x declined");
        // Since diff_exp > threshold, the error exponent 2 * diff_exp exceeds both the working
        // precision bounds float_round_near_x checks, so it always succeeds; but fall through to
        // the general path defensively.
    }
    // General case. The Ziv loop below scales x and y to avoid any overflow and underflow in x^2
    // (as |x| >= |y|): x = Mx * 2^Ex with 1/2 <= |Mx| < 1, and sh = floor((Emax - 1) / 2) - Ex, so
    // that (x * 2^sh)^2 = Mx^2 * 2^(2 * floor((Emax - 1) / 2)) has an exponent of at most Emax - 1,
    // and (x * 2^sh)^2 + (y * 2^sh)^2 one of at most Emax, even after rounding, as the intermediate
    // operations round toward zero.
    //
    // First house deviation: the C code has a FIXME admitting that the scaled y can underflow (the
    // shortcut above bounds diff_exp by about 2 * max(px, prec), which for huge precisions exceeds
    // the exponent range). Instead of inheriting that wrong-result corner, such cases go through
    // the exact integer-level path, which is immune to the exponent range. Second house deviation:
    // `Exact` also goes through the exact path, which decides exactness directly instead of
    // looping; the C code does not support an `Exact` mode at all.
    let sh = const { (MAX_EXPONENT_I64 - 1) >> 1 } - ex;
    if rm == Exact || ey + sh < const { Float::MIN_EXPONENT as i64 } {
        return hypot_exact_helper(x, y, prec, rm);
    }
    let n = max(px, py);
    let mut working_prec = prec + prec.ceiling_log_base_2() + 4;
    let mut increment = Limb::WIDTH;
    loop {
        // All intermediate operations round toward zero.
        let (mut te, o1) = x.shl_prec_round_ref(sh, working_prec, Down);
        let (ti, o2) = y.shl_prec_round_ref(sh, working_prec, Down);
        let o3 = te.square_round_assign(Down);
        // Use fma in order to avoid underflow of ti * ti.
        let (mut t, o4) = te.add_mul_round_val_ref_ref(&ti, &ti, Down);
        let o5 = t.sqrt_round_assign(Down);
        let exact = o1 == Equal && o2 == Equal && o3 == Equal && o4 == Equal && o5 == Equal;
        if exact {
            // t is exactly the scaled hypotenuse; the final rounding determines everything.
            return t.shr_prec_round(sh, prec, rm);
        }
        let err = if working_prec < n { 4 } else { 2 };
        if float_can_round(t.significand_ref().unwrap(), working_prec - err, prec, rm) {
            let (z, o) = t.shr_prec_round(sh, prec, rm);
            // mirrors MPFR_ASSERTD (exact == 0 || inexact != 0), which is also debug-only
            debug_assert_ne!(o, Equal);
            return (z, o);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded hypotenuse is less
    /// than, equal to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\pm\infty,x,p,m)=f(x,\pm\infty,p,m)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ if $x$ is not infinite
    /// - $f(\pm0.0,\pm0.0,p,m)=0.0$
    ///
    /// The result is never negative, and a zero result is always positive.
    ///
    /// Overflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    ///
    /// Underflow is not possible, since the hypotenuse is at least as large as the absolute value
    /// of each argument.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::hypot_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::hypot_round`] instead. If both of these things are true, consider using
    /// [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the hypotenuse is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round(Float::TWO, 5, Floor);
    /// assert_eq!(hypot.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round(Float::TWO, 5, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round(Float::TWO, 5, Nearest);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round(Float::TWO, 20, Floor);
    /// assert_eq!(hypot.to_string(), "2.2360649");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round(Float::TWO, 20, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round(Float::TWO, 20, Nearest);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn hypot_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (&self, &other) {
            // Return +Infinity, even when the other number is NaN.
            (float_either_infinity!(), _) | (_, float_either_infinity!()) => {
                (float_infinity!(), Equal)
            }
            (float_nan!(), _) | (_, float_nan!()) => (float_nan!(), Equal),
            (float_either_zero!(), _) => Self::from_float_prec_round(other.abs(), prec, rm),
            (_, float_either_zero!()) => Self::from_float_prec_round(self.abs(), prec, rm),
            _ => hypot_prec_round_helper(&self, &other, prec, rm),
        }
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::hypot_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::hypot_round_val_ref`] instead. If both of these things are true,
    /// consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the hypotenuse is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_val_ref(&Float::TWO, 5, Floor);
    /// assert_eq!(hypot.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_val_ref(&Float::TWO, 5, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_val_ref(&Float::TWO, 5, Nearest);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_val_ref(&Float::TWO, 20, Floor);
    /// assert_eq!(hypot.to_string(), "2.2360649");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_val_ref(&Float::TWO, 20, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_val_ref(&Float::TWO, 20, Nearest);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn hypot_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (&self, other) {
            // Return +Infinity, even when the other number is NaN.
            (float_either_infinity!(), _) | (_, float_either_infinity!()) => {
                (float_infinity!(), Equal)
            }
            (float_nan!(), _) | (_, float_nan!()) => (float_nan!(), Equal),
            (float_either_zero!(), _) => Self::from_float_prec_round(other.abs(), prec, rm),
            (_, float_either_zero!()) => Self::from_float_prec_round(self.abs(), prec, rm),
            _ => hypot_prec_round_helper(&self, other, prec, rm),
        }
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::hypot_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::hypot_round_ref_val`] instead. If both of these things are true,
    /// consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the hypotenuse is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_val(Float::TWO, 5, Floor);
    /// assert_eq!(hypot.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_val(Float::TWO, 5, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_val(Float::TWO, 5, Nearest);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_val(Float::TWO, 20, Floor);
    /// assert_eq!(hypot.to_string(), "2.2360649");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_val(Float::TWO, 20, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_val(Float::TWO, 20, Nearest);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn hypot_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (self, &other) {
            // Return +Infinity, even when the other number is NaN.
            (float_either_infinity!(), _) | (_, float_either_infinity!()) => {
                (float_infinity!(), Equal)
            }
            (float_nan!(), _) | (_, float_nan!()) => (float_nan!(), Equal),
            (float_either_zero!(), _) => Self::from_float_prec_round(other.abs(), prec, rm),
            (_, float_either_zero!()) => Self::from_float_prec_round(self.abs(), prec, rm),
            _ => hypot_prec_round_helper(self, &other, prec, rm),
        }
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded hypotenuse is
    /// less than, equal to, or greater than the exact hypotenuse. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::hypot_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::hypot_round_ref_ref`] instead. If both of these things are true,
    /// consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the hypotenuse is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_ref(&Float::TWO, 5, Floor);
    /// assert_eq!(hypot.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_ref(&Float::TWO, 5, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_ref(&Float::TWO, 5, Nearest);
    /// assert_eq!(hypot.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_ref(&Float::TWO, 20, Floor);
    /// assert_eq!(hypot.to_string(), "2.2360649");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_ref(&Float::TWO, 20, Ceiling);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::ONE.hypot_prec_round_ref_ref(&Float::TWO, 20, Nearest);
    /// assert_eq!(hypot.to_string(), "2.2360687");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn hypot_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            // Return +Infinity, even when the other number is NaN.
            (float_either_infinity!(), _) | (_, float_either_infinity!()) => {
                (float_infinity!(), Equal)
            }
            (float_nan!(), _) | (_, float_nan!()) => (float_nan!(), Equal),
            (float_either_zero!(), _) => Self::from_float_prec_round(other.abs(), prec, rm),
            (_, float_either_zero!()) => Self::from_float_prec_round(self.abs(), prec, rm),
            _ => hypot_prec_round_helper(self, other, prec, rm),
        }
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// nearest value of the specified precision. Both [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded hypotenuse is less than, equal
    /// to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hypotenuse is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::hypot_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the two inputs, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec(Float::from(E), 5);
    /// assert_eq!(hypot.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec(Float::from(E), 20);
    /// assert_eq!(hypot.to_string(), "4.1543579");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.hypot_prec_round(other, prec, Nearest)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] is taken by value and the
    /// second by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the hypotenuse is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::hypot_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(hypot.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(hypot.to_string(), "4.1543579");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.hypot_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] is taken by reference and the
    /// second by value. An [`Ordering`] is also returned, indicating whether the rounded hypotenuse
    /// is less than, equal to, or greater than the exact hypotenuse. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hypotenuse is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::hypot_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(hypot.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(hypot.to_string(), "4.1543579");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.hypot_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result to the
    /// nearest value of the specified precision. Both [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded hypotenuse is less than, equal
    /// to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hypotenuse is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::hypot_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(hypot.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(hypot.to_string(), "4.1543579");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn hypot_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.hypot_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result with the
    /// specified rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded hypotenuse is less than, equal to, or greater than
    /// the exact hypotenuse. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::hypot_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the hypotenuse is not exactly representable with the maximum
    /// of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round(Float::from(E), Floor);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round(Float::from(E), Ceiling);
    /// assert_eq!(hypot.to_string(), "4.1543544023133139");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round(Float::from(E), Nearest);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn hypot_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round(other, prec, rm)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by value and the second by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded hypotenuse is less than,
    /// equal to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::hypot_prec_round_val_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the hypotenuse is not exactly representable with the maximum
    /// of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_val_ref(&Float::from(E), Floor);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_val_ref(&Float::from(E), Ceiling);
    /// assert_eq!(hypot.to_string(), "4.1543544023133139");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_val_ref(&Float::from(E), Nearest);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn hypot_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by reference and the second by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded hypotenuse is less than,
    /// equal to, or greater than the exact hypotenuse. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::hypot_prec_round_ref_val`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the hypotenuse is not exactly representable with the maximum
    /// of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_ref_val(Float::from(E), Floor);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_ref_val(Float::from(E), Ceiling);
    /// assert_eq!(hypot.to_string(), "4.1543544023133139");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_ref_val(Float::from(E), Nearest);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn hypot_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, rounding the result with the
    /// specified rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded hypotenuse is less than, equal to, or greater than
    /// the exact hypotenuse. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::hypot_prec_round_ref_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the hypotenuse is not exactly representable with the maximum
    /// of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_ref_ref(&Float::from(E), Floor);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_ref_ref(&Float::from(E), Ceiling);
    /// assert_eq!(hypot.to_string(), "4.1543544023133139");
    /// assert_eq!(o, Greater);
    ///
    /// let (hypot, o) = Float::from(PI).hypot_round_ref_ref(&Float::from(E), Nearest);
    /// assert_eq!(hypot.to_string(), "4.1543544023133130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn hypot_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, mutating the first one in
    /// place, and rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded hypotenuse is less than, equal to, or greater than the exact
    /// hypotenuse. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::hypot_prec_assign`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::hypot_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the hypotenuse is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign(Float::TWO, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign(Float::TWO, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.25");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign(Float::TWO, 5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.25");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign(Float::TWO, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.2360649");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign(Float::TWO, 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.2360687");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign(Float::TWO, 20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.2360687");
    /// ```
    pub fn hypot_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let o;
        let mut x = Self::ZERO;
        swap(&mut x, self);
        (*self, o) = x.hypot_prec_round(other, prec, rm);
        o
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, mutating the first one in
    /// place, and rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded hypotenuse is less than, equal to, or greater than
    /// the exact hypotenuse. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::hypot_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::hypot_round_assign_ref`] instead. If both of these things
    /// are true, consider using [`Float::hypot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the hypotenuse is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign_ref(&Float::TWO, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&Float::TWO, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.25");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&Float::TWO, 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.25");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(x.hypot_prec_round_assign_ref(&Float::TWO, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.2360649");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&Float::TWO, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.2360687");
    ///
    /// let mut x = Float::ONE;
    /// assert_eq!(
    ///     x.hypot_prec_round_assign_ref(&Float::TWO, 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.2360687");
    /// ```
    pub fn hypot_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let o;
        let mut x = Self::ZERO;
        swap(&mut x, self);
        (*self, o) = x.hypot_prec_round_val_ref(other, prec, rm);
        o
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, mutating the first one in
    /// place, and rounding the result to the nearest value of the specified precision. The
    /// [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned, indicating
    /// whether the rounded hypotenuse is less than, equal to, or greater than the exact hypotenuse.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hypotenuse is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::hypot_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::hypot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_prec_assign(Float::from(E), 5), Greater);
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_prec_assign(Float::from(E), 20), Greater);
    /// assert_eq!(x.to_string(), "4.1543579");
    /// ```
    #[inline]
    pub fn hypot_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.hypot_prec_round_assign(other, prec, Nearest)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, mutating the first one in
    /// place, and rounding the result to the nearest value of the specified precision. The
    /// [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded hypotenuse is less than, equal to, or greater than the exact
    /// hypotenuse. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hypotenuse is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::hypot_prec_round_assign_ref`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using [`Float::hypot_assign`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_prec_assign_ref(&Float::from(E), 5), Greater);
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_prec_assign_ref(&Float::from(E), 20), Greater);
    /// assert_eq!(x.to_string(), "4.1543579");
    /// ```
    #[inline]
    pub fn hypot_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.hypot_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, mutating the first one in
    /// place, and rounding the result with the specified rounding mode. The [`Float`] on the
    /// right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::hypot_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::hypot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the hypotenuse is not exactly representable with the maximum
    /// of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_round_assign(Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "4.1543544023133130");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_round_assign(Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "4.1543544023133139");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_round_assign(Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "4.1543544023133130");
    /// ```
    #[inline]
    pub fn hypot_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_assign(other, prec, rm)
    }

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$, mutating the first one in
    /// place, and rounding the result with the specified rounding mode. The [`Float`] on the
    /// right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded hypotenuse is less than, equal to, or greater than the exact hypotenuse. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::hypot_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::hypot_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the hypotenuse is not exactly representable with the maximum
    /// of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_round_assign_ref(&Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "4.1543544023133130");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_round_assign_ref(&Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "4.1543544023133139");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.hypot_round_assign_ref(&Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "4.1543544023133130");
    /// ```
    #[inline]
    pub fn hypot_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_assign_ref(other, prec, rm)
    }
}

impl Hypot<Self> for Float {
    type Output = Self;

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$. Both [`Float`]s are taken by
    /// value.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. If the hypotenuse
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\pm\infty,x)=f(x,\pm\infty)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=\text{NaN}$ if $x$ is not infinite
    /// - $f(\pm0.0,\pm0.0)=0.0$
    ///
    /// The result is never negative, and a zero result is always positive.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on overflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::Hypot;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(PI).hypot(Float::from(E)).to_string(),
    ///     "4.1543544023133130"
    /// );
    /// ```
    #[inline]
    fn hypot(self, other: Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round(other, prec, Nearest).0
    }
}

impl Hypot<&Self> for Float {
    type Output = Self;

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$. The first [`Float`] is taken by
    /// value and the second by reference.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. If the hypotenuse
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\pm\infty,x)=f(x,\pm\infty)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=\text{NaN}$ if $x$ is not infinite
    /// - $f(\pm0.0,\pm0.0)=0.0$
    ///
    /// The result is never negative, and a zero result is always positive.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on overflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::Hypot;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(PI).hypot(&Float::from(E)).to_string(),
    ///     "4.1543544023133130"
    /// );
    /// ```
    #[inline]
    fn hypot(self, other: &Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Hypot<Float> for &Float {
    type Output = Float;

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$. The first [`Float`] is taken by
    /// reference and the second by value.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. If the hypotenuse
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\pm\infty,x)=f(x,\pm\infty)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=\text{NaN}$ if $x$ is not infinite
    /// - $f(\pm0.0,\pm0.0)=0.0$
    ///
    /// The result is never negative, and a zero result is always positive.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on overflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::Hypot;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(PI)).hypot(Float::from(E)).to_string(),
    ///     "4.1543544023133130"
    /// );
    /// ```
    #[inline]
    fn hypot(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Hypot<&Float> for &Float {
    type Output = Float;

    /// Computes the hypotenuse of two [`Float`]s, $\sqrt{x^2+y^2}$. Both [`Float`]s are taken by
    /// reference.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. If the hypotenuse
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\pm\infty,x)=f(x,\pm\infty)=\infty$, even when the other argument is `NaN`
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=\text{NaN}$ if $x$ is not infinite
    /// - $f(\pm0.0,\pm0.0)=0.0$
    ///
    /// The result is never negative, and a zero result is always positive.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on overflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::Hypot;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(PI)).hypot(&Float::from(E)).to_string(),
    ///     "4.1543544023133130"
    /// );
    /// ```
    #[inline]
    fn hypot(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl HypotAssign<Self> for Float {
    /// Replaces a [`Float`] with the hypotenuse of it and another [`Float`], $\sqrt{x^2+y^2}$. The
    /// [`Float`] on the right-hand side is taken by value.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. If the hypotenuse
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::HypotAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// x.hypot_assign(Float::from(E));
    /// assert_eq!(x.to_string(), "4.1543544023133130");
    /// ```
    #[inline]
    fn hypot_assign(&mut self, other: Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_assign(other, prec, Nearest);
    }
}

impl HypotAssign<&Self> for Float {
    /// Replaces a [`Float`] with the hypotenuse of it and another [`Float`], $\sqrt{x^2+y^2}$. The
    /// [`Float`] on the right-hand side is taken by reference.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. If the hypotenuse
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \sqrt{x^2+y^2}+\varepsilon.
    /// $$
    /// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::hypot_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::HypotAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// x.hypot_assign(&Float::from(E));
    /// assert_eq!(x.to_string(), "4.1543544023133130");
    /// ```
    #[inline]
    fn hypot_assign(&mut self, other: &Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.hypot_prec_round_assign_ref(other, prec, Nearest);
    }
}

/// Computes the hypotenuse of two primitive floats, $\sqrt{x^2+y^2}$, with a single rounding.
///
/// $$
/// f(x,y) = \sqrt{x^2+y^2}+\varepsilon.
/// $$
/// - If $\sqrt{x^2+y^2}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\sqrt{x^2+y^2}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   \sqrt{x^2+y^2}\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\pm\infty,x)=f(x,\pm\infty)=\infty$, even when the other argument is `NaN`
/// - $f(\text{NaN},x)=f(x,\text{NaN})=\text{NaN}$ if $x$ is not infinite
/// - $f(\pm0.0,\pm0.0)=0.0$
///
/// The result is never negative, and a zero result is always positive.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, PI};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::hypot::primitive_float_hypot;
///
/// assert_eq!(
///     NiceFloat(primitive_float_hypot(PI, E)),
///     NiceFloat(4.154354402313313)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_hypot<T: PrimitiveFloat>(x: T, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(Float::hypot_prec, x, y)
}
