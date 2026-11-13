// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, from_float_result,
};
use crate::big_exponent_float::arithmetic::reciprocal::reciprocal_helper;
use crate::big_exponent_float::arithmetic::square::square_helper;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{AddMul, CeilingLogBase2, Parity, Pow, PowAssign};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;

// The largest |n| for which `pow_prec_round_float_kernels` uses `Float`'s kernels. With x scaled to
// have exponent 1, x^n has an exponent between n and n + 1 for positive n, and between n + 1 and 1
// for negative n, so it is within `Float`'s exponent range.
const MAX_KERNEL_POW: u64 = (crate::Float::MAX_EXPONENT as u64) - 2;

// x^n, rounded to `prec` bits with `rm`.
pub(crate) fn pow_helper(
    x: Cow<'_, BigExponentFloat>,
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    pow_helper_with(x, n, prec, rm, false)
}

crate_test_fn! {
    // x^n rounded to `prec` bits with `rm`, with `Float`'s kernels: x is scaled to have exponent 1,
    // raised to the nth power with `Float`'s `pow_u_prec_round` or `pow_s_prec_round`, and the
    // scaling is undone; binary exponentiation is used only when |n| is too large for `Float`'s
    // exponent range. This was the main algorithm until benchmarks showed binary exponentiation to
    // be as fast for positive n and several times faster for negative n; it is kept to check and
    // benchmark that algorithm.
    pow_prec_round_float_kernels(
        x: &BigExponentFloat,
        n: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        pow_helper_with(Cow::Borrowed(x), n, prec, rm, true)
    }
}

// The special cases follow `Float`'s powers, except at zero: with no negative zero, 0^n for
// negative odd n could be either infinity, and is NaN, like 1/0. For even negative n it is
// infinity.
//
// A power of a power of 2 is computed exactly, and powers 1, 2, and -1 are rounding, squaring, and
// reciprocation. Other powers are computed by binary exponentiation (`pow_finite_generic`), or,
// with `use_float_kernels`, as described for `pow_prec_round_float_kernels`.
fn pow_helper_with(
    x: Cow<'_, BigExponentFloat>,
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
    use_float_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *n == 0u32 {
        return (BigExponentFloat::ONE, Equal);
    }
    let n_positive = *n > 0u32;
    let n_odd = n.odd();
    let (sign, exponent, significand) = match &*x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign }) => {
            return (
                if !n_positive {
                    BigExponentFloat::ZERO
                } else if *sign || !n_odd {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        BigExponentFloat(Zero) => {
            return (
                if n_positive {
                    BigExponentFloat::ZERO
                } else if n_odd {
                    BigExponentFloat::NAN
                } else {
                    BigExponentFloat::INFINITY
                },
                Equal,
            );
        }
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => (*sign, exponent, significand),
    };
    if significand_prec(significand) == 1 {
        // |x| = 2^(e - 1) for x's exponent e, so |x^n| = 2^((e - 1) * n), whose exponent is (e - 1)
        // * n + 1.
        let exponent = Integer::ONE.add_mul(n, exponent - Integer::ONE);
        return (
            BigExponentFloat(Finite {
                sign: sign || !n_odd,
                exponent,
                significand: significand.clone(),
            }),
            Equal,
        );
    }
    if *n == 1u32 {
        return x.into_owned().round_to_prec_round(prec, rm);
    } else if *n == 2u32 {
        return square_helper(x, Some((prec, rm)));
    } else if *n == -1i32 {
        return reciprocal_helper(x, prec, rm);
    }
    if !use_float_kernels || *n.unsigned_abs_ref() > MAX_KERNEL_POW {
        return pow_finite_generic(&x, n, prec, rm);
    }
    let shift = (exponent - Integer::ONE) * n;
    let x = finite_to_float(x, 1);
    let frm = float_rm(rm);
    let result = if n_positive {
        x.pow_u_prec_round(u64::exact_from(n), prec, frm)
    } else {
        x.pow_s_prec_round(i64::exact_from(n), prec, frm)
    };
    let (y, o) = from_float_result(result, Some(&shift)).unwrap();
    assert!(rm != Exact || o == Equal, "Inexact BigExponentFloat power");
    (y, o)
}

// An approximation of x^n for n >= 2, by binary exponentiation with every product rounded to the
// nearest value with `prec` bits; and whether every product was exact. The relative error is at
// most (1 + 2^-prec)^(2 * bits(n)) - 1.
fn pow_positive_approx(x: &BigExponentFloat, n: &Natural, prec: u64) -> (BigExponentFloat, bool) {
    let bits = n.significant_bits();
    let (mut y, o) = x.square_prec_ref(prec);
    let mut inexact = o != Equal;
    if n.get_bit(bits - 2) {
        inexact |= y.mul_prec_assign_ref(x, prec) != Equal;
    }
    for i in (0..bits - 2).rev() {
        inexact |= y.square_prec_assign(prec) != Equal;
        if n.get_bit(i) {
            inexact |= y.mul_prec_assign_ref(x, prec) != Equal;
        }
    }
    (y, inexact)
}

// x^n for finite x that is not a power of 2 and for |n| >= 2, by the generic algorithm. This is
// `mpfr_pow_pos_z` and the negative-exponent path of `mpfr_pow_z` from `pow_z.c`, MPFR 4.3.0,
// without their overflow and underflow handling. The rounding test is MPFR's `MPFR_CAN_ROUND`,
// which also makes sure that rounding the approximation gives the right `Ordering`.
//
// For positive n, if the exact power has at most `working_prec` bits, every product is exact, which
// is detected. Otherwise the power, a dyadic rational with more than `prec + 1` bits, is not a
// rounding breakpoint, and the Ziv loop terminates. For negative n, the power of the non-dyadic 1/x
// is not a dyadic rational, so it is never a rounding breakpoint either.
fn pow_finite_generic(
    x: &BigExponentFloat,
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let frm = float_rm(rm);
    let n_abs = n.unsigned_abs_ref();
    let n_bits = n_abs.significant_bits();
    let mut working_prec = prec + 3 + n_bits + prec.ceiling_log_base_2();
    let (y, o) = loop {
        if *n > 0u32 {
            let (y, inexact) = pow_positive_approx(x, n_abs, working_prec);
            if !inexact
                || float_can_round(
                    y.significand_ref().unwrap(),
                    working_prec - 1 - n_bits,
                    prec,
                    frm,
                )
            {
                break y.round_to_prec_round(prec, frm);
            }
        } else {
            // 1/x, then its |n|th power: 2 * bits(|n|) + 1 roundings in all.
            let reciprocal = reciprocal_helper(Cow::Borrowed(x), working_prec, Nearest).0;
            let y = pow_positive_approx(&reciprocal, n_abs, working_prec).0;
            if float_can_round(
                y.significand_ref().unwrap(),
                working_prec - 2 - n_bits,
                prec,
                frm,
            ) {
                break y.round_to_prec_round(prec, frm);
            }
        }
        working_prec += working_prec >> 1;
    };
    assert!(rm != Exact || o == Equal, "Inexact BigExponentFloat power");
    (y, o)
}

// x^n exactly, for the exact `Pow` implementations.
fn pow_exact(x: &BigExponentFloat, n: u64) -> BigExponentFloat {
    if n == 0 {
        return BigExponentFloat::ONE;
    }
    match x {
        BigExponentFloat(NaN) => BigExponentFloat::NAN,
        BigExponentFloat(Infinity { sign }) => {
            if *sign || n.even() {
                BigExponentFloat::INFINITY
            } else {
                BigExponentFloat::NEGATIVE_INFINITY
            }
        }
        BigExponentFloat(Zero) => BigExponentFloat::ZERO,
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => {
            let zeros = significand.trailing_zeros().unwrap();
            let mut m = significand >> zeros;
            // |x| = m * 2^t, so |x^n| = m^n * 2^(t * n).
            let t = exponent - Integer::from(m.significant_bits());
            m.pow_assign(n);
            let exponent = Integer::from(m.significant_bits()).add_mul(t, Integer::from(n));
            BigExponentFloat::from_parts(*sign || n.even(), exponent, m)
        }
    }
}

impl BigExponentFloat {
    /// Raises a [`BigExponentFloat`] to the power of a [`u64`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n>0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is odd
    /// - $f(0,n,p,m)=0$ if $n>0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::pow_u_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 5, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x1f.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 5, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x2.0E+1");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 5, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x1f.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 20, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019a");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 20, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019c");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 20, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec_round(self, n: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), &Integer::from(n), prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`u64`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n>0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is odd
    /// - $f(0,n,p,m)=0$ if $n>0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 5, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x1f.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 5, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x2.0E+1");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 5, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x1f.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 20, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019a");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 20, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019c");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 20, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec_round_ref(&self, n: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), &Integer::from(n), prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`u64`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x1f.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x2.0E+1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 5, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x1f.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x1f.019a");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1f.019c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 20, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1f.019c");
    /// ```
    #[inline]
    pub fn pow_u_prec_round_assign(&mut self, n: u64, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), &Integer::from(n), prec, rm);
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`u64`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n>0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is odd
    /// - $f(0,n,p)=0$ if $n>0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::pow_u_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec(3, 5);
    /// assert_eq!(format!("{pow:?}"), "0x1f.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec(3, 20);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec(self, n: u64, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), &Integer::from(n), prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`u64`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n>0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is odd
    /// - $f(0,n,p)=0$ if $n>0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_ref(3, 5);
    /// assert_eq!(format!("{pow:?}"), "0x1f.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_ref(3, 20);
    /// assert_eq!(format!("{pow:?}"), "0x1f.019c");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec_ref(&self, n: u64, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), &Integer::from(n), prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`u64`] in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_assign(3, 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x1f.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_assign(3, 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1f.019c");
    /// ```
    #[inline]
    pub fn pow_u_prec_assign(&mut self, n: u64, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), &Integer::from(n), prec, Nearest);
        *self = result;
        o
    }
}

impl BigExponentFloat {
    /// Raises a [`BigExponentFloat`] to the power of an [`i64`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p,m)=0$ if $n<0$
    /// - $f(0,n,p,m)=0$ if $n>0$
    /// - $f(0,n,p,m)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p,m)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::pow_s_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 5, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x0.08");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 5, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x0.088");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 5, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x0.088");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 20, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a2");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 20, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a3");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 20, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a3");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_prec_round(self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), &Integer::from(n), prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`i64`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p,m)=0$ if $n<0$
    /// - $f(0,n,p,m)=0$ if $n>0$
    /// - $f(0,n,p,m)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p,m)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 5, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x0.08");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 5, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x0.088");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 5, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x0.088");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 20, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a2");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 20, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a3");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 20, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a3");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_prec_round_ref(&self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), &Integer::from(n), prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`i64`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.08");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.088");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 5, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.088");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.0841a2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.0841a3");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 20, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.0841a3");
    /// ```
    #[inline]
    pub fn pow_s_prec_round_assign(&mut self, n: i64, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), &Integer::from(n), prec, rm);
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`i64`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p)=0$ if $n<0$
    /// - $f(0,n,p)=0$ if $n>0$
    /// - $f(0,n,p)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::pow_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec(-3, 5);
    /// assert_eq!(format!("{pow:?}"), "0x0.088");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec(-3, 20);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a3");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_prec(self, n: i64, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), &Integer::from(n), prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`i64`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p)=0$ if $n<0$
    /// - $f(0,n,p)=0$ if $n>0$
    /// - $f(0,n,p)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_ref(-3, 5);
    /// assert_eq!(format!("{pow:?}"), "0x0.088");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_ref(-3, 20);
    /// assert_eq!(format!("{pow:?}"), "0x0.0841a3");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_prec_ref(&self, n: i64, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), &Integer::from(n), prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`i64`] in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_assign(-3, 5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.088");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_assign(-3, 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.0841a3");
    /// ```
    #[inline]
    pub fn pow_s_prec_assign(&mut self, n: i64, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), &Integer::from(n), prec, Nearest);
        *self = result;
        o
    }
}

impl BigExponentFloat {
    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Integer`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p,m)=0$ if $n<0$
    /// - $f(0,n,p,m)=0$ if $n>0$
    /// - $f(0,n,p,m)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p,m)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::pow_integer_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 5, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 5, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 5, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 20, Floor);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326d");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 20, Ceiling);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 20, Nearest);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn pow_integer_prec_round(
        self,
        other: Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), &other, prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value and the [`Integer`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p,m)=0$ if $n<0$
    /// - $f(0,n,p,m)=0$ if $n>0$
    /// - $f(0,n,p,m)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p,m)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326d");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_val_ref(
        self,
        other: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), other, prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference and the [`Integer`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p,m)=0$ if $n<0$
    /// - $f(0,n,p,m)=0$ if $n>0$
    /// - $f(0,n,p,m)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p,m)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326d");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn pow_integer_prec_round_ref_val(
        &self,
        other: Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), &other, prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Integer`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$
    /// - $f(-\infty,n,p,m)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p,m)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p,m)=0$ if $n<0$
    /// - $f(0,n,p,m)=0$ if $n>0$
    /// - $f(0,n,p,m)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p,m)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326d");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_ref_ref(
        &self,
        other: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), other, prec, rm)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`] in place, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Integer`] is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded power is less than, equal
    /// to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326d");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326e");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn pow_integer_prec_round_assign(
        &mut self,
        other: Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), &other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`] in place, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Integer`] is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded power is less than,
    /// equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the power cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326d");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326e");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326e");
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_assign_ref(
        &mut self,
        other: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] and the [`Integer`] are
    /// both taken by value. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p)=0$ if $n<0$
    /// - $f(0,n,p)=0$ if $n>0$
    /// - $f(0,n,p)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::pow_integer_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec(Integer::from(-10), 5);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec(Integer::from(-10), 20);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn pow_integer_prec(self, other: Integer, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), &other, prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by value and the
    /// [`Integer`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p)=0$ if $n<0$
    /// - $f(0,n,p)=0$ if $n>0$
    /// - $f(0,n,p)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_val_ref(&Integer::from(-10), 5);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_val_ref(&Integer::from(-10), 20);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_val_ref(self, other: &Integer, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Owned(self), other, prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference and
    /// the [`Integer`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p)=0$ if $n<0$
    /// - $f(0,n,p)=0$ if $n>0$
    /// - $f(0,n,p)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_integer_prec_ref_val(Integer::from(-10), 5);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_integer_prec_ref_val(Integer::from(-10), 20);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn pow_integer_prec_ref_val(&self, other: Integer, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), &other, prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] and the [`Integer`] are
    /// both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$
    /// - $f(\infty,n,p)=\infty$ if $n>0$
    /// - $f(-\infty,n,p)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n,p)=-\infty$ if $n$ is positive and odd
    /// - $f(\pm\infty,n,p)=0$ if $n<0$
    /// - $f(0,n,p)=0$ if $n>0$
    /// - $f(0,n,p)=\infty$ if $n<0$ and $n$ is even
    /// - $f(0,n,p)=\text{NaN}$ if $n<0$ and $n$ is odd
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_integer_prec_ref_ref(&Integer::from(-10), 5);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_integer_prec_ref_ref(&Integer::from(-10), 20);
    /// assert_eq!(format!("{pow:?}"), "0x0.0000b326e");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_ref_ref(&self, other: &Integer, prec: u64) -> (Self, Ordering) {
        pow_helper(Cow::Borrowed(self), other, prec, Nearest)
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`] in place, rounding the result
    /// to the nearest value of the specified precision. The [`Integer`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded power is less than, equal to, or
    /// greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_integer_prec_assign(Integer::from(-10), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.0000b");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_integer_prec_assign(Integer::from(-10), 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326e");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn pow_integer_prec_assign(&mut self, other: Integer, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), &other, prec, Nearest);
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of an [`Integer`] in place, rounding the result
    /// to the nearest value of the specified precision. The [`Integer`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded power is less than, equal to, or
    /// greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m(n + m) \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_integer_prec_assign_ref(&Integer::from(-10), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.0000b");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_assign_ref(&Integer::from(-10), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x0.0000b326e");
    /// ```
    #[inline]
    pub fn pow_integer_prec_assign_ref(&mut self, other: &Integer, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_helper(Cow::Owned(x), other, prec, Nearest);
        *self = result;
        o
    }
}

impl Pow<u64> for BigExponentFloat {
    type Output = Self;

    /// Raises a [`BigExponentFloat`] to the power of a [`u64`], taking it by value. The power is
    /// exact.
    ///
    /// $$
    /// f(x,n) = x^n.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,0)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n)=\text{NaN}$ if $n>0$
    /// - $f(\infty,n)=\infty$ if $n>0$
    /// - $f(-\infty,n)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n)=-\infty$ if $n$ is odd
    /// - $f(0,n)=0$ if $n>0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(1.5).pow(3),
    ///     BigExponentFloat::from(3.375)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from(3).pow(100),
    ///     BigExponentFloat::from(Natural::from(3u32).pow(100))
    /// );
    /// assert_eq!(BigExponentFloat::NAN.pow(0), BigExponentFloat::ONE);
    /// ```
    #[inline]
    fn pow(self, pow: u64) -> Self {
        pow_exact(&self, pow)
    }
}

impl Pow<u64> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Raises a [`BigExponentFloat`] to the power of a [`u64`], taking it by reference. The power
    /// is exact.
    ///
    /// $$
    /// f(x,n) = x^n.
    /// $$
    ///
    /// Special cases:
    /// - $f(x,0)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},n)=\text{NaN}$ if $n>0$
    /// - $f(\infty,n)=\infty$ if $n>0$
    /// - $f(-\infty,n)=\infty$ if $n$ is positive and even
    /// - $f(-\infty,n)=-\infty$ if $n$ is odd
    /// - $f(0,n)=0$ if $n>0$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&BigExponentFloat::from(1.5)).pow(3),
    ///     BigExponentFloat::from(3.375)
    /// );
    /// assert_eq!(
    ///     (&BigExponentFloat::from(3)).pow(100),
    ///     BigExponentFloat::from(Natural::from(3u32).pow(100))
    /// );
    /// assert_eq!((&BigExponentFloat::NAN).pow(0), BigExponentFloat::ONE);
    /// ```
    #[inline]
    fn pow(self, pow: u64) -> BigExponentFloat {
        pow_exact(self, pow)
    }
}

impl PowAssign<u64> for BigExponentFloat {
    /// Raises a [`BigExponentFloat`] to the power of a [`u64`] in place. The power is exact.
    ///
    /// $$
    /// x \gets x^n.
    /// $$
    ///
    /// The special cases are the same as those of [`Pow::pow`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x.pow_assign(3);
    /// assert_eq!(x, BigExponentFloat::from(3.375));
    /// ```
    #[inline]
    fn pow_assign(&mut self, pow: u64) {
        *self = pow_exact(self, pow);
    }
}
