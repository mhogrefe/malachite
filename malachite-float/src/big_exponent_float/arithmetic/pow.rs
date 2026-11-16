// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, from_float_result,
};
use crate::big_exponent_float::arithmetic::reciprocal::reciprocal_helper;
use crate::big_exponent_float::arithmetic::round_to_integer::finite_is_integer;
use crate::big_exponent_float::arithmetic::square::square_helper;
use crate::big_exponent_float::exponential::exp::ziv_loop;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    Abs, AddMul, CeilingLogBase2, CheckedRoot, CheckedSqrt, DivRem, Parity, Pow, PowAssign,
    Reciprocal, UnsignedAbs,
};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeOne, One,
    Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_q::Rational;

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
    /// assert_eq!(pow.to_string(), "31.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 5, Ceiling);
    /// assert_eq!(pow.to_string(), "32.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 5, Nearest);
    /// assert_eq!(pow.to_string(), "31.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 20, Floor);
    /// assert_eq!(pow.to_string(), "31.006256");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 20, Ceiling);
    /// assert_eq!(pow.to_string(), "31.00629");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec_round(3, 20, Nearest);
    /// assert_eq!(pow.to_string(), "31.00629");
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
    /// assert_eq!(pow.to_string(), "31.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 5, Ceiling);
    /// assert_eq!(pow.to_string(), "32.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 5, Nearest);
    /// assert_eq!(pow.to_string(), "31.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 20, Floor);
    /// assert_eq!(pow.to_string(), "31.006256");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 20, Ceiling);
    /// assert_eq!(pow.to_string(), "31.00629");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_round_ref(3, 20, Nearest);
    /// assert_eq!(pow.to_string(), "31.00629");
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
    /// assert_eq!(x.to_string(), "31.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "32.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "31.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "31.006256");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "31.00629");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_round_assign(3, 20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "31.00629");
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
    /// assert_eq!(pow.to_string(), "31.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_u_prec(3, 20);
    /// assert_eq!(pow.to_string(), "31.00629");
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
    /// assert_eq!(pow.to_string(), "31.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_u_prec_ref(3, 20);
    /// assert_eq!(pow.to_string(), "31.00629");
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
    /// assert_eq!(x.to_string(), "31.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_u_prec_assign(3, 20), Greater);
    /// assert_eq!(x.to_string(), "31.00629");
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
    /// assert_eq!(pow.to_string(), "0.031");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 5, Ceiling);
    /// assert_eq!(pow.to_string(), "0.0332");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 5, Nearest);
    /// assert_eq!(pow.to_string(), "0.0332");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 20, Floor);
    /// assert_eq!(pow.to_string(), "0.03225148");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 20, Ceiling);
    /// assert_eq!(pow.to_string(), "0.032251537");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec_round(-3, 20, Nearest);
    /// assert_eq!(pow.to_string(), "0.032251537");
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
    /// assert_eq!(pow.to_string(), "0.031");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 5, Ceiling);
    /// assert_eq!(pow.to_string(), "0.0332");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 5, Nearest);
    /// assert_eq!(pow.to_string(), "0.0332");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 20, Floor);
    /// assert_eq!(pow.to_string(), "0.03225148");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 20, Ceiling);
    /// assert_eq!(pow.to_string(), "0.032251537");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_round_ref(-3, 20, Nearest);
    /// assert_eq!(pow.to_string(), "0.032251537");
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
    /// assert_eq!(x.to_string(), "0.031");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.0332");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.0332");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.03225148");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.032251537");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_round_assign(-3, 20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.032251537");
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
    /// assert_eq!(pow.to_string(), "0.0332");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_s_prec(-3, 20);
    /// assert_eq!(pow.to_string(), "0.032251537");
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
    /// assert_eq!(pow.to_string(), "0.0332");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_s_prec_ref(-3, 20);
    /// assert_eq!(pow.to_string(), "0.032251537");
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
    /// assert_eq!(x.to_string(), "0.0332");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_s_prec_assign(-3, 20), Greater);
    /// assert_eq!(x.to_string(), "0.032251537");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 5, Ceiling);
    /// assert_eq!(pow.to_string(), "0.0000110");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 5, Nearest);
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 20, Floor);
    /// assert_eq!(pow.to_string(), "0.000010678268");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 20, Ceiling);
    /// assert_eq!(pow.to_string(), "0.00001067828");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_integer_prec_round(Integer::from(-10), 20, Nearest);
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.0000110");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.000010678268");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.00001067828");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_round_val_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.0000110");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.000010678268");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.00001067828");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_val(
    ///     Integer::from(-10),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.0000110");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.000010678268");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.00001067828");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_integer_prec_round_ref_ref(
    ///     &Integer::from(-10),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(x.to_string(), "0.0000105");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.0000110");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.0000105");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.000010678268");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.00001067828");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign(Integer::from(-10), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.00001067828");
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
    /// assert_eq!(x.to_string(), "0.0000105");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.0000110");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.0000105");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.000010678268");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.00001067828");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_round_assign_ref(&Integer::from(-10), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.00001067828");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec(Integer::from(-10), 20);
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_integer_prec_val_ref(&Integer::from(-10), 20);
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_integer_prec_ref_val(Integer::from(-10), 20);
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(pow.to_string(), "0.0000105");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_integer_prec_ref_ref(&Integer::from(-10), 20);
    /// assert_eq!(pow.to_string(), "0.00001067828");
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
    /// assert_eq!(x.to_string(), "0.0000105");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_integer_prec_assign(Integer::from(-10), 20), Greater);
    /// assert_eq!(x.to_string(), "0.00001067828");
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
    /// assert_eq!(x.to_string(), "0.0000105");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_integer_prec_assign_ref(&Integer::from(-10), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.00001067828");
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

// -- General powers --

// The base of a general power: a `BigExponentFloat`, or a `Rational` that is not a dyadic rational
// (a dyadic `Rational` is converted to a `BigExponentFloat` first, and so is a `u64`).
pub(crate) enum PowBase<'a> {
    BigExponentFloat(Cow<'a, BigExponentFloat>),
    Rational(&'a Rational),
}

// The exponent of a general power: a `BigExponentFloat`, or a `Rational` that is not a dyadic
// rational (and therefore not an integer).
pub(crate) enum PowExponent<'a> {
    BigExponentFloat(Cow<'a, BigExponentFloat>),
    Rational(&'a Rational),
}

impl<'a> PowBase<'a> {
    pub(crate) fn of_rational(x: &'a Rational) -> Self {
        BigExponentFloat::try_from(x).map_or(PowBase::Rational(x), |b| {
            PowBase::BigExponentFloat(Cow::Owned(b))
        })
    }
}

impl<'a> PowExponent<'a> {
    pub(crate) fn of_rational(y: &'a Rational) -> Self {
        BigExponentFloat::try_from(y).map_or(PowExponent::Rational(y), |b| {
            PowExponent::BigExponentFloat(Cow::Owned(b))
        })
    }
}

// What the special values of a power depend on, for the base.
#[derive(Clone, Copy)]
enum BaseClass {
    NaN,
    Infinity,
    NegativeInfinity,
    Zero,
    One,
    NegativeOne,
    Finite {
        negative: bool,
        abs_greater_than_one: bool,
    },
}

// What the special values of a power depend on, for the exponent.
#[derive(Clone, Copy)]
enum ExponentClass {
    NaN,
    Infinity,
    NegativeInfinity,
    Zero,
    Finite {
        positive: bool,
        integer: bool,
        odd: bool,
    },
}

// Whether the finite integer x is odd.
fn finite_integer_is_odd(x: &BigExponentFloat) -> bool {
    let BigExponentFloat(Finite {
        exponent,
        significand,
        ..
    }) = x
    else {
        unreachable!()
    };
    // the lowest set bit of the significand has value 2^(exponent - bits + trailing zeros)
    let low = exponent - Integer::from(significand.significant_bits())
        + Integer::from(significand.trailing_zeros().unwrap());
    low == 0u32
}

fn base_class(x: &PowBase<'_>) -> BaseClass {
    match x {
        PowBase::BigExponentFloat(x) => match &**x {
            BigExponentFloat(NaN) => BaseClass::NaN,
            BigExponentFloat(Infinity { sign: true }) => BaseClass::Infinity,
            BigExponentFloat(Infinity { sign: false }) => BaseClass::NegativeInfinity,
            BigExponentFloat(Zero) => BaseClass::Zero,
            BigExponentFloat(Finite { sign, .. }) => {
                match x.partial_cmp_abs(&BigExponentFloat::ONE) {
                    Some(Equal) if *sign => BaseClass::One,
                    Some(Equal) => BaseClass::NegativeOne,
                    o => BaseClass::Finite {
                        negative: !sign,
                        abs_greater_than_one: o == Some(Greater),
                    },
                }
            }
        },
        // a non-dyadic `Rational` is neither 0 nor ±1
        PowBase::Rational(q) => BaseClass::Finite {
            negative: **q < 0u32,
            abs_greater_than_one: q.partial_cmp_abs(&Rational::ONE) == Some(Greater),
        },
    }
}

fn exponent_class(y: &PowExponent<'_>) -> ExponentClass {
    match y {
        PowExponent::BigExponentFloat(y) => match &**y {
            BigExponentFloat(NaN) => ExponentClass::NaN,
            BigExponentFloat(Infinity { sign: true }) => ExponentClass::Infinity,
            BigExponentFloat(Infinity { sign: false }) => ExponentClass::NegativeInfinity,
            BigExponentFloat(Zero) => ExponentClass::Zero,
            BigExponentFloat(Finite { sign, .. }) => {
                let integer = finite_is_integer(y);
                ExponentClass::Finite {
                    positive: *sign,
                    integer,
                    odd: integer && finite_integer_is_odd(y),
                }
            }
        },
        PowExponent::Rational(q) => ExponentClass::Finite {
            positive: **q > 0u32,
            integer: false,
            odd: false,
        },
    }
}

// The value of x^y (or, if `powr`, of e^(y log x)) when it is special: those of `Float`'s `pow` and
// `powr`, except that with no negative zero, 0^y for a negative odd integer y could be either
// infinity, and is NaN, like 1/0 (as for `pow_integer`), and results of `Float`'s -0.0 are 0.
const fn pow_special(powr: bool, x: BaseClass, y: ExponentClass) -> Option<BigExponentFloat> {
    use BaseClass as B;
    use ExponentClass as E;
    Some(if powr {
        match (x, y) {
            (B::One, E::NaN) => BigExponentFloat::ONE,
            (
                B::NaN | B::NegativeInfinity | B::NegativeOne | B::Finite { negative: true, .. },
                _,
            )
            | (_, E::NaN)
            | (B::Zero | B::Infinity, E::Zero)
            | (B::One, E::Infinity | E::NegativeInfinity) => BigExponentFloat::NAN,
            (_, E::Zero) | (B::One, _) => BigExponentFloat::ONE,
            (B::Infinity, E::Finite { positive: true, .. } | E::Infinity)
            | (
                B::Zero,
                E::Finite {
                    positive: false, ..
                }
                | E::NegativeInfinity,
            ) => BigExponentFloat::INFINITY,
            (B::Infinity | B::Zero, _) => BigExponentFloat::ZERO,
            (
                B::Finite {
                    abs_greater_than_one,
                    ..
                },
                E::Infinity,
            ) => {
                if abs_greater_than_one {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::ZERO
                }
            }
            (
                B::Finite {
                    abs_greater_than_one,
                    ..
                },
                E::NegativeInfinity,
            ) => {
                if abs_greater_than_one {
                    BigExponentFloat::ZERO
                } else {
                    BigExponentFloat::INFINITY
                }
            }
            _ => return None,
        }
    } else {
        match (x, y) {
            (_, E::Zero) | (B::One, _) | (B::NegativeOne, E::Infinity | E::NegativeInfinity) => {
                BigExponentFloat::ONE
            }
            (B::NaN, _) | (_, E::NaN) => BigExponentFloat::NAN,
            (B::Infinity | B::NegativeInfinity, E::Infinity) | (B::Zero, E::NegativeInfinity) => {
                BigExponentFloat::INFINITY
            }
            (B::Infinity | B::NegativeInfinity, E::NegativeInfinity) | (B::Zero, E::Infinity) => {
                BigExponentFloat::ZERO
            }
            (
                B::Finite {
                    abs_greater_than_one,
                    ..
                },
                E::Infinity,
            ) => {
                if abs_greater_than_one {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::ZERO
                }
            }
            (
                B::Finite {
                    abs_greater_than_one,
                    ..
                },
                E::NegativeInfinity,
            ) => {
                if abs_greater_than_one {
                    BigExponentFloat::ZERO
                } else {
                    BigExponentFloat::INFINITY
                }
            }
            (B::Infinity, E::Finite { positive, .. }) => {
                if positive {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::ZERO
                }
            }
            (B::NegativeInfinity, E::Finite { positive, odd, .. }) => match (positive, odd) {
                (true, true) => BigExponentFloat::NEGATIVE_INFINITY,
                (true, false) => BigExponentFloat::INFINITY,
                _ => BigExponentFloat::ZERO,
            },
            (B::Zero, E::Finite { positive, odd, .. }) => match (positive, odd) {
                (true, _) => BigExponentFloat::ZERO,
                (false, true) => BigExponentFloat::NAN,
                (false, false) => BigExponentFloat::INFINITY,
            },
            (B::NegativeOne, E::Finite { integer, odd, .. }) => match (integer, odd) {
                (true, true) => BigExponentFloat::NEGATIVE_ONE,
                (true, false) => BigExponentFloat::ONE,
                (false, _) => BigExponentFloat::NAN,
            },
            (B::Finite { negative: true, .. }, E::Finite { integer: false, .. }) => {
                BigExponentFloat::NAN
            }
            _ => return None,
        }
    })
}

// The finite nonzero x as (a, b) with x = ±a * 2^b and a odd.
fn odd_mantissa_and_exponent(x: &BigExponentFloat) -> (Natural, Integer) {
    let BigExponentFloat(Finite {
        exponent,
        significand,
        ..
    }) = x
    else {
        unreachable!()
    };
    let zeros = significand.trailing_zeros().unwrap();
    (
        significand >> zeros,
        exponent - Integer::from(significand.significant_bits()) + Integer::from(zeros),
    )
}

// The positive `Rational` x as (a, b, e) with x = (a / b) * 2^e, a and b odd and coprime.
fn rational_odd_parts(x: &Rational) -> (Natural, Natural, Integer) {
    let (n, d) = x.to_numerator_and_denominator();
    let n_zeros = n.trailing_zeros().unwrap();
    let d_zeros = d.trailing_zeros().unwrap();
    (
        n >> n_zeros,
        d >> d_zeros,
        Integer::from(n_zeros) - Integer::from(d_zeros),
    )
}

// For x = a * 2^b > 0 (a odd, x not 1) and a finite non-integer y = c * 2^d (c odd, d < 0): if x^y
// is a dyadic rational, a dyadic r and an integer n with x^y = r^n. This is `mpfr_pow_is_exact`
// from `pow.c`, MPFR 4.3.0, extended to powers of 2: x = 2^b gives x^y = (2^(b / 2^-d))^c, a dyadic
// rational exactly when 2^-d divides b. Otherwise x^y is (x^(2^d))^c, and x^(2^d) is found by -d
// exact square roots, all of which must succeed; each halves the length of a, which can never reach
// 1, so at most about log_2 of that length of them are tried. A negative c then gives a power of
// the non-dyadic reciprocal of an odd number greater than 1.
fn pow_is_exact(x: &BigExponentFloat, y: &BigExponentFloat) -> Option<(BigExponentFloat, Integer)> {
    let (c_abs, d) = odd_mantissa_and_exponent(y);
    let c = Integer::from_sign_and_abs(*y > 0u32, c_abs);
    let (mut a, mut b) = odd_mantissa_and_exponent(x);
    let k = u64::exact_from(&-d);
    if a == 1u32 {
        // b != 0, since x != 1
        if b.trailing_zeros().unwrap() < k {
            return None;
        }
        return Some((power_of_2_integer(&(b >> k)), c));
    }
    if c < 0u32 {
        return None;
    }
    for _ in 0..k {
        if b.odd() {
            a <<= 1u32;
            b -= Integer::ONE;
        }
        a = a.checked_sqrt()?;
        b >>= 1u32;
    }
    Some((
        BigExponentFloat::from_parts(true, b + Integer::from(a.significant_bits()), a),
        c,
    ))
}

// 2^n for an integer n.
fn power_of_2_integer(n: &Integer) -> BigExponentFloat {
    BigExponentFloat::from_parts(true, n + Integer::ONE, Natural::ONE)
}

// For x = a * 2^b > 0 (a odd, x not 1) and a non-dyadic `Rational` y = r / s in lowest terms: if
// x^y is a dyadic rational, a dyadic t and an integer n with x^y = t^n. Since r and s are coprime,
// x^y is a dyadic rational exactly when x^(1/s) is (if x^y = D, then x^(1/s) = D^u x^v with u r + v
// s = 1), that is, when a is a perfect s-th power and s divides b; and then, unless x is a power of
// 2, r must be positive, as for `pow_is_exact`. An odd a > 1 that is an s-th power has more than s
// bits.
fn pow_rational_is_exact(
    x: &BigExponentFloat,
    y: &Rational,
) -> Option<(BigExponentFloat, Integer)> {
    let (r, s) = y.to_numerator_and_denominator();
    let r = Integer::from_sign_and_abs(*y > 0u32, r);
    let (a, b) = odd_mantissa_and_exponent(x);
    let (q, rem) = (&b).div_rem(Integer::from(&s));
    if rem != 0u32 {
        return None;
    }
    if a == 1u32 {
        return Some((power_of_2_integer(&q), r));
    }
    if r < 0u32 || s > a.significant_bits() {
        return None;
    }
    let root = a.checked_root(u64::exact_from(&s))?;
    Some((
        BigExponentFloat::from_parts(true, q + Integer::from(root.significant_bits()), root),
        r,
    ))
}

// For a positive non-dyadic `Rational` x = (a / b) * 2^e (a and b odd and coprime, b > 1) and a
// finite non-integer y = c * 2^d (c odd, d < 0): if x^y is a dyadic rational, a dyadic t and an
// integer n with x^y = t^n. As in `pow_is_exact`, x^(2^d) must be found by exact square roots of a,
// b, and 2^e; then, since b > 1 stays greater than 1, the power is a dyadic rational only if c < 0
// and a has become 1, and then x^y = (b * 2^(-e))^(-c). This follows
// `rational_pow_exact_decomposition` of `Float`'s `rational_pow`.
fn rational_pow_is_exact(
    x: &Rational,
    y: &BigExponentFloat,
) -> Option<(BigExponentFloat, Integer)> {
    let (c_abs, d) = odd_mantissa_and_exponent(y);
    if *y > 0u32 {
        return None;
    }
    let (mut a, mut b, mut e) = rational_odd_parts(x);
    for _ in 0..u64::exact_from(&-d) {
        if a != 1u32 {
            a = a.checked_sqrt()?;
        }
        b = b.checked_sqrt()?;
        if e.odd() {
            return None;
        }
        e >>= 1u32;
    }
    if a != 1u32 {
        return None;
    }
    let bits = Integer::from(b.significant_bits());
    Some((
        BigExponentFloat::from_parts(true, bits - e, b),
        Integer::from(c_abs),
    ))
}

// For a positive non-dyadic `Rational` x = (a / b) * 2^e (a and b odd and coprime, b > 1) and a
// non-dyadic `Rational` y = r / s in lowest terms: if x^y is a dyadic rational, a dyadic t and an
// integer n with x^y = t^n. As in `pow_rational_is_exact`, x^(1/s) must be a rational, so a and b
// must be perfect s-th powers and s must divide e; then, since b > 1, the power is a dyadic
// rational only if r < 0 and a = 1, and then x^y = (b^(1/s) * 2^(-e / s))^(-r).
fn rational_pow_rational_is_exact(
    x: &Rational,
    y: &Rational,
) -> Option<(BigExponentFloat, Integer)> {
    if *y > 0u32 {
        return None;
    }
    let (r, s) = y.to_numerator_and_denominator();
    let (a, b, e) = rational_odd_parts(x);
    if a != 1u32 || s > b.significant_bits() {
        return None;
    }
    let (q, rem) = e.div_rem(Integer::from(&s));
    if rem != 0u32 {
        return None;
    }
    let root = b.checked_root(u64::exact_from(&s))?;
    let bits = Integer::from(root.significant_bits());
    Some((
        BigExponentFloat::from_parts(true, bits - q, root),
        Integer::from(r),
    ))
}

// The exponent of a finite nonzero base, as for a `BigExponentFloat`: |x| is in [2^(e - 1), 2^e).
fn base_exponent(x: &PowBase<'_>) -> Integer {
    match x {
        PowBase::BigExponentFloat(x) => x.get_exponent().unwrap().clone(),
        PowBase::Rational(q) => Integer::from(q.floor_log_base_2_abs()) + Integer::ONE,
    }
}

fn exponent_exponent(y: &PowExponent<'_>) -> Integer {
    match y {
        PowExponent::BigExponentFloat(y) => y.get_exponent().unwrap().clone(),
        PowExponent::Rational(q) => Integer::from(q.floor_log_base_2_abs()) + Integer::ONE,
    }
}

// |x|^y for a finite base x with |x| not 1 and a finite nonzero exponent y, rounded to `prec` bits
// with `rm`, as 2^(y log_2 |x|), by a Ziv loop. The power must not be a dyadic rational (or it
// could be a rounding breakpoint, and the loop might not end): callers rule those out.
//
// With e the exponent of x, |log_2 |x|| < |e| + 1 < 2^l, where l is the length of |e| + 1, and with
// f the exponent of y, |y log_2 |x|| < 2^(f + l). Let w = wp + max(f + l, 0) + 8. Then L = log_2
// |x| and (for a `Rational` y) y are rounded to w bits, and so is their product t, with a total
// relative error below 3.01 * 2^(-w); so the absolute error of t is below 3.01 * 2^(f + l - w) <=
// 3.01 * 2^(-wp - 8), which changes 2^t by a relative amount below 2^(-wp - 6). Rounding 2^t to wp
// + 2 bits adds at most 2^(-wp - 2), so the result's relative error is below 2^(-wp - 1), and its
// error below 2^(EXP - wp).
fn pow_ziv(
    x: &PowBase<'_>,
    y: &PowExponent<'_>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let e = base_exponent(x);
    let l = (e.unsigned_abs() + Natural::ONE).significant_bits();
    let extra = u64::exact_from(&max(exponent_exponent(y) + Integer::from(l), Integer::ZERO));
    ziv_loop(prec, rm, |wp| {
        let w = wp + extra + 8;
        let log = match x {
            PowBase::BigExponentFloat(x) => (&**x).abs().log_base_2_prec(w).0,
            PowBase::Rational(q) => BigExponentFloat::log_base_2_rational_prec((*q).abs(), w).0,
        };
        let t = match y {
            PowExponent::BigExponentFloat(y) => log.mul_prec_val_ref(y, w).0,
            PowExponent::Rational(q) => {
                log.mul_prec(BigExponentFloat::from_rational_prec_ref(q, w).0, w)
                    .0
            }
        };
        (
            BigExponentFloat::power_of_2_of_big_exponent_float_prec(t, wp + 2).0,
            wp,
        )
    })
}

// x^y for a finite nonzero x and a finite nonzero y that are not special (see `pow_special`): x is
// negative only if y is an integer. Integer powers with |y| < 2^64 are `pow_integer`'s; a power of
// a power of 2 is exact; a dyadic result with a non-integer y is found by the `*_is_exact`
// functions, and computed as an integer power; and any other result, which is not a dyadic
// rational, is computed by `pow_ziv`, with the sign of an odd power of a negative x restored.
fn pow_finite(
    x: &PowBase<'_>,
    y: &PowExponent<'_>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let negative = match x {
        PowBase::BigExponentFloat(x) => **x < 0u32,
        PowBase::Rational(q) => **q < 0u32,
    };
    if let PowExponent::BigExponentFloat(y) = y
        && finite_is_integer(y)
    {
        let odd = finite_integer_is_odd(y);
        let small = *y.get_exponent().unwrap() <= 64u32;
        // A dyadic base, or the dyadic reciprocal of a Rational base, raised to an integer power.
        let dyadic = match x {
            PowBase::BigExponentFloat(x) => Some(((**x).clone(), Cow::Borrowed(&**y))),
            PowBase::Rational(q) => {
                let r = BigExponentFloat::try_from(&(*q).reciprocal()).ok();
                r.filter(|_| **y < 0u32).map(|r| (r, Cow::Owned(-&**y)))
            }
        };
        if let Some((base, n)) = dyadic {
            if small {
                return pow_helper(Cow::Owned(base), &Integer::exact_from(&*n), prec, rm);
            }
            let (a, b) = odd_mantissa_and_exponent(&base);
            if a == 1u32 {
                // ±2^b raised to n is ±2^(bn), exactly
                let p = power_of_2_integer(&(b * Integer::exact_from(&*n)));
                return (if base < 0u32 && odd { -p } else { p }, Equal);
            }
        }
    } else if !negative {
        let exact = match (x, y) {
            (PowBase::BigExponentFloat(x), PowExponent::BigExponentFloat(y)) => pow_is_exact(x, y),
            (PowBase::BigExponentFloat(x), PowExponent::Rational(y)) => pow_rational_is_exact(x, y),
            (PowBase::Rational(x), PowExponent::BigExponentFloat(y)) => rational_pow_is_exact(x, y),
            (PowBase::Rational(x), PowExponent::Rational(y)) => {
                rational_pow_rational_is_exact(x, y)
            }
        };
        if let Some((t, n)) = exact {
            return pow_helper(Cow::Owned(t), &n, prec, rm);
        }
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat power");
    let odd = matches!(y, PowExponent::BigExponentFloat(y) if finite_integer_is_odd(y));
    if negative && odd {
        let (p, o) = pow_ziv(x, y, prec, -rm);
        (-p, o.reverse())
    } else {
        pow_ziv(x, y, prec, rm)
    }
}

// Whether the finite nonzero x has an exponent within `Float`'s range, and if so, x as a `Float`.
fn to_float_in_range(x: &BigExponentFloat) -> Option<Float> {
    let e = i32::try_from(x.get_exponent()?).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then(|| finite_to_float(Cow::Borrowed(x), e))
}

// x^y, or e^(y log x) if `powr`, rounded to `prec` bits with `rm`. The special values are those of
// `pow_special`. If `use_float` and every `BigExponentFloat` argument has an exponent within
// `Float`'s range, `Float`'s function is used unless its result overflows or underflows; otherwise
// `pow_finite` (`powr` and `pow` agree on the positive bases that remain).
pub(crate) fn pow_general_helper(
    x: &PowBase<'_>,
    y: &PowExponent<'_>,
    powr: bool,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(z) = pow_special(powr, base_class(x), exponent_class(y)) {
        return (z, Equal);
    }
    if use_float {
        let frm = float_rm(rm);
        let result = match (x, y) {
            (PowBase::BigExponentFloat(xb), PowExponent::BigExponentFloat(yb)) => {
                match (to_float_in_range(xb), to_float_in_range(yb)) {
                    (Some(xf), Some(yf)) if powr => {
                        Some(xf.powr_prec_round_ref_ref(&yf, prec, frm))
                    }
                    (Some(xf), Some(yf)) => Some(xf.pow_prec_round_ref_ref(&yf, prec, frm)),
                    _ => None,
                }
            }
            (PowBase::BigExponentFloat(xb), PowExponent::Rational(q)) => {
                to_float_in_range(xb).map(|xf| xf.pow_rational_prec_round_ref_ref(q, prec, frm))
            }
            (PowBase::Rational(p), PowExponent::BigExponentFloat(yb)) => to_float_in_range(yb)
                .map(|yf| Float::rational_pow_prec_round_ref_ref(p, &yf, prec, frm)),
            (PowBase::Rational(p), PowExponent::Rational(q)) => {
                Some(Float::rational_pow_rational_prec_round_ref(p, q, prec, frm))
            }
        };
        if let Some((z, o)) = result.and_then(|r| from_float_result(r, None)) {
            assert!(rm != Exact || o == Equal, "Inexact BigExponentFloat power");
            return (z, o);
        }
    }
    pow_finite(x, y, prec, rm)
}

crate_test_fn! {
    // x^y, or e^(y log x) if `powr`, rounded to `prec` bits with `rm`, without `Float`'s power
    // functions: see `pow_finite`. Used to check `pow_general_helper`.
    pow_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        powr: bool,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(x)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            powr,
            prec,
            rm,
            false,
        )
    }
}

crate_test_fn! {
    // x^y for a `Rational` y, rounded to `prec` bits with `rm`, as in `pow_prec_round_generic`.
    pow_rational_prec_round_generic(
        x: &BigExponentFloat,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(x)),
            &PowExponent::of_rational(y),
            false,
            prec,
            rm,
            false,
        )
    }
}

crate_test_fn! {
    // x^y for a `Rational` x, rounded to `prec` bits with `rm`, as in `pow_prec_round_generic`.
    rational_pow_prec_round_generic(
        x: &Rational,
        y: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            false,
            prec,
            rm,
            false,
        )
    }
}

crate_test_fn! {
    // x^y for `Rational`s x and y, rounded to `prec` bits with `rm`, as in
    // `pow_prec_round_generic`.
    rational_pow_rational_prec_round_generic(
        x: &Rational,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::of_rational(y),
            false,
            prec,
            rm,
            false,
        )
    }
}

impl BigExponentFloat {
    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. Both
    /// [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::pow_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. Both
    /// [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] in place,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "23.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.459137");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn pow_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            false,
            prec,
            rm,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] in place,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "23.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.459137");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn pow_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            false,
            prec,
            rm,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The first [`BigExponentFloat`] is
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The first [`BigExponentFloat`] is
    /// taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).pow_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] in place,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_prec_assign(BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_prec_assign(BigExponentFloat::from(E), 20), Greater);
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn pow_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            false,
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] in place,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.pow_prec_assign_ref(&BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn pow_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            false,
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p,m)=1$ if $x$ is finite and positive
    /// - $f(1,y,p,m)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::powr_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).powr_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).powr_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).powr_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).powr_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).powr_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).powr_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            true,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`BigExponentFloat`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p,m)=1$ if $x$ is finite and positive
    /// - $f(1,y,p,m)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            true,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`BigExponentFloat`] is taken by reference and the second
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p,m)=1$ if $x$ is finite and positive
    /// - $f(1,y,p,m)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            true,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p,m)=1$ if $x$ is finite and positive
    /// - $f(1,y,p,m)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "23.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "22.459137");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).powr_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            true,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function) in place, rounding the result to the specified precision and
    /// with the specified rounding mode. The [`BigExponentFloat`] on the right-hand side is taken
    /// by value. An [`Ordering`] is returned, indicating whether the rounded power is less than,
    /// equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p,m)=1$ if $x$ is finite and positive
    /// - $f(1,y,p,m)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "23.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.459137");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn powr_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            true,
            prec,
            rm,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function) in place, rounding the result to the specified precision and
    /// with the specified rounding mode. The [`BigExponentFloat`] on the right-hand side is taken
    /// by reference. An [`Ordering`] is returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p,m)=1$ if $x$ is finite and positive
    /// - $f(1,y,p,m)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "23.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "22.459137");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn powr_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            true,
            prec,
            rm,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the nearest value of the specified
    /// precision. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p)=1$ if $x$ is finite and positive
    /// - $f(1,y,p)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::powr_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            true,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the nearest value of the specified
    /// precision. The first [`BigExponentFloat`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p)=1$ if $x$ is finite and positive
    /// - $f(1,y,p)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).powr_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            true,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the nearest value of the specified
    /// precision. The first [`BigExponentFloat`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p)=1$ if $x$ is finite and positive
    /// - $f(1,y,p)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).powr_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).powr_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            true,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function), rounding the result to the nearest value of the specified
    /// precision. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p)=1$ if $x$ is finite and positive
    /// - $f(1,y,p)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).powr_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "22.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     (&BigExponentFloat::from(PI)).powr_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "22.45917");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn powr_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            true,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function) in place, rounding the result to the nearest value of the
    /// specified precision. The [`BigExponentFloat`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded power is less than, equal to, or
    /// greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p)=1$ if $x$ is finite and positive
    /// - $f(1,y,p)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.powr_prec_assign(BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.powr_prec_assign(BigExponentFloat::from(E), 20), Greater);
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn powr_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Owned(other)),
            true,
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of another [`BigExponentFloat`] as $e^{y \ln x}$
    /// (the IEEE 754 `powr` function) in place, rounding the result to the nearest value of the
    /// specified precision. The [`BigExponentFloat`] on the right-hand side is taken by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded power is less than, equal to, or
    /// greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $0$ or $\infty$ and $y=0$, if
    ///   $x=1$ and $y$ is infinite, or if $y$ is `NaN` and $x \neq 1$
    /// - $f(x,0,p)=1$ if $x$ is finite and positive
    /// - $f(1,y,p)=1$ if $y$ is finite or `NaN`
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $0<x<1$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $0<x<1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::powr_prec), this function accepts arguments
    /// with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`, plus the exponent of `other` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.powr_prec_assign_ref(&BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "22.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.powr_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "22.45917");
    /// ```
    #[inline]
    pub fn powr_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::BigExponentFloat(Cow::Borrowed(other)),
            true,
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::pow_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.4645901");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::of_rational(&other),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.4645901");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI).pow_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::of_rational(other),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.4645901");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::of_rational(&other),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.4645901");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI)).pow_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::of_rational(other),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`] in place, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Rational`] on the
    /// right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.4645901");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.464592");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.464592");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::of_rational(&other),
            false,
            prec,
            rm,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`] in place, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Rational`] on the
    /// right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.4645901");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.464592");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.464592");
    /// ```
    #[inline]
    pub fn pow_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::of_rational(other),
            false,
            prec,
            rm,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are
    /// both taken by value. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::pow_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::from(PI).pow_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::of_rational(&other),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::from(PI)
    ///     .pow_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::from(PI)
    ///     .pow_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(self)),
            &PowExponent::of_rational(other),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference and
    /// the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI))
    ///     .pow_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI))
    ///     .pow_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::of_rational(&other),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are
    /// both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI))
    ///     .pow_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(pow.to_string(), "1.44");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = (&BigExponentFloat::from(PI))
    ///     .pow_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(pow.to_string(), "1.464592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Borrowed(self)),
            &PowExponent::of_rational(other),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`] in place, rounding the result
    /// to the nearest value of the specified precision. The [`Rational`] on the right-hand side is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.464592");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::of_rational(&other),
            false,
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`BigExponentFloat`] to the power of a [`Rational`] in place, rounding the result
    /// to the nearest value of the specified precision. The [`Rational`] on the right-hand side is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// x \gets x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, and $0$ if $y<0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::pow_rational_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.44");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.pow_rational_prec_assign_ref(&Rational::from_unsigneds(1u8, 3), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.464592");
    /// ```
    #[inline]
    pub fn pow_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(x)),
            &PowExponent::of_rational(other),
            false,
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::rational_pow_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.0488");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.05047190");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_pow_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(&x),
            &PowExponent::BigExponentFloat(Cow::Owned(y)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded power is less than, equal
    /// to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.0488");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.05047190");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_pow_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(&x),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded power is less than, equal
    /// to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.0488");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.05047190");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::BigExponentFloat(Cow::Owned(y)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.0488");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.05047190");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even $0$
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::rational_pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_pow_prec(x: Rational, y: Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(&x),
            &PowExponent::BigExponentFloat(Cow::Owned(y)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even $0$
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_pow_prec_val_ref(x: Rational, y: &Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(&x),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even $0$
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_ref_val(x: &Rational, y: Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::BigExponentFloat(Cow::Owned(y)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of a [`BigExponentFloat`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even $0$
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1,\pm\infty,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`, plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(pow.to_string(), "0.0508");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(pow.to_string(), "0.050471961");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_ref_ref(x: &Rational, y: &Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of another [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. Both [`Rational`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even $0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::rational_pow_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.57734966");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.5773506");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.5773506");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_pow_rational_prec_round(
        x: Rational,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(&x),
            &PowExponent::of_rational(&y),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of another [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. Both [`Rational`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$, even $0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(1,y,p,m)=1$
    /// - $f(-1,y,p,m)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "0.57734966");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "0.5773506");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "0.5773506");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_rational_prec_round_ref(
        x: &Rational,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::of_rational(y),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of another [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// Both [`Rational`]s are taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even $0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_rational_prec), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::rational_pow_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     5,
    /// );
    /// assert_eq!(pow.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::ONE_HALF,
    ///     20,
    /// );
    /// assert_eq!(pow.to_string(), "0.5773506");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_pow_rational_prec(x: Rational, y: Rational, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(&x),
            &PowExponent::of_rational(&y),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`Rational`] to the power of another [`Rational`], returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// Both [`Rational`]s are taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$, even $0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    /// - $f(1,y,p)=1$
    /// - $f(-1,y,p)=1$ if $y$ is an even integer, and $-1$ if $y$ is an odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::rational_pow_rational_prec), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     5,
    /// );
    /// assert_eq!(pow.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::rational_pow_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::ONE_HALF,
    ///     20,
    /// );
    /// assert_eq!(pow.to_string(), "0.5773506");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_rational_prec_ref(
        x: &Rational,
        y: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::of_rational(x),
            &PowExponent::of_rational(y),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`BigExponentFloat`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ if $x \neq 1$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $x=0$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $x=0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::unsigned_pow_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`,
    /// plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(pow.to_string(), "19.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(pow.to_string(), "20.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(pow.to_string(), "20.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(pow.to_string(), "19.8130");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(pow.to_string(), "19.813019");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(pow.to_string(), "19.8130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_prec_round(
        x: u64,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::BigExponentFloat(Cow::Owned(y)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`BigExponentFloat`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$
    /// - $f(1,y,p,m)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ if $x \neq 1$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0$ if $x=0$
    /// - $f(x,-\infty,p,m)=0$ if $x>1$, and $\infty$ if $x=0$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_prec_round), this function
    /// accepts arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`,
    /// plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round_ref(3, &BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(pow.to_string(), "19.0");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec_round_ref(
    ///     3,
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "20.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec_round_ref(
    ///     3,
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "20.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_prec_round_ref(3, &BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(pow.to_string(), "19.8130");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec_round_ref(
    ///     3,
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "19.813019");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec_round_ref(
    ///     3,
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "19.8130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_prec_round_ref(
        x: u64,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`BigExponentFloat`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ if $x \neq 1$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $x=0$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $x=0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::unsigned_pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`,
    /// plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec(3, BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "20.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec(3, BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "19.8130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_prec(x: u64, y: Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::BigExponentFloat(Cow::Owned(y)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`BigExponentFloat`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$
    /// - $f(1,y,p)=1$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ if $x \neq 1$
    /// - $f(x,\infty,p)=\infty$ if $x>1$, and $0$ if $x=0$
    /// - $f(x,-\infty,p)=0$ if $x>1$, and $\infty$ if $x=0$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_prec), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// Warning: unless $|x|$ is 1 or a power of 2, an exponent $y$ with exponent $e > 0$ makes
    /// $x^y$ be computed from $\log_2 |x|$ with about $e$ more bits of precision. The time and
    /// memory needed therefore grow with the exponent, without any limit: an exponent near
    /// $2^{2^{40}}$ needs about $2^{40}$ bits, or 128 GiB, of $\log_2 |x|$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`,
    /// plus the exponent of `y` if it is positive.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec_ref(3, &BigExponentFloat::from(E), 5);
    /// assert_eq!(pow.to_string(), "20.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_prec_ref(3, &BigExponentFloat::from(E), 20);
    /// assert_eq!(pow.to_string(), "19.8130");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_prec_ref(x: u64, y: &Self, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::BigExponentFloat(Cow::Borrowed(y)),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$
    /// - $f(1,y,p,m)=1$ for any $y$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::unsigned_pow_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_rational_prec_round(3, Rational::ONE_HALF, 5, Floor);
    /// assert_eq!(pow.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_rational_prec_round(3, Rational::ONE_HALF, 5, Ceiling);
    /// assert_eq!(pow.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_rational_prec_round(3, Rational::ONE_HALF, 5, Nearest);
    /// assert_eq!(pow.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_rational_prec_round(3, Rational::ONE_HALF, 20, Floor);
    /// assert_eq!(pow.to_string(), "1.7320499");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_rational_prec_round(3, Rational::ONE_HALF, 20, Ceiling);
    /// assert_eq!(pow.to_string(), "1.732052");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) =
    ///     BigExponentFloat::unsigned_pow_rational_prec_round(3, Rational::ONE_HALF, 20, Nearest);
    /// assert_eq!(pow.to_string(), "1.7320499");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn unsigned_pow_rational_prec_round(
        x: u64,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::of_rational(&y),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$
    /// - $f(1,y,p,m)=1$ for any $y$
    /// - $f(0,y,p,m)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_rational_prec_round), this
    /// function never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_round_ref(
    ///     3,
    ///     &Rational::ONE_HALF,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.69");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_round_ref(
    ///     3,
    ///     &Rational::ONE_HALF,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_round_ref(
    ///     3,
    ///     &Rational::ONE_HALF,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_round_ref(
    ///     3,
    ///     &Rational::ONE_HALF,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(pow.to_string(), "1.7320499");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_round_ref(
    ///     3,
    ///     &Rational::ONE_HALF,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(pow.to_string(), "1.732052");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_round_ref(
    ///     3,
    ///     &Rational::ONE_HALF,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(pow.to_string(), "1.7320499");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_rational_prec_round_ref(
        x: u64,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::of_rational(y),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. The [`Rational`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded power is less than,
    /// equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$
    /// - $f(1,y,p)=1$ for any $y$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_rational_prec), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::unsigned_pow_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec(3, Rational::ONE_HALF, 5);
    /// assert_eq!(pow.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec(3, Rational::ONE_HALF, 20);
    /// assert_eq!(pow.to_string(), "1.7320499");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn unsigned_pow_rational_prec(x: u64, y: Rational, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::of_rational(&y),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$
    /// - $f(1,y,p)=1$ for any $y$
    /// - $f(0,y,p)=0$ if $y>0$, $\infty$ if $y<0$ and $y$ is not an odd integer, and $\text{NaN}$
    ///   if $y$ is a negative odd integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_rational_prec), this function
    /// never overflows or underflows.
    ///
    /// A [`BigExponentFloat`] has no negative zero, so for a negative odd integer $y$, $0^y$, whose
    /// one-sided limits are $\infty$ and $-\infty$, is `NaN`, as $1/0$ is, where the `Float`
    /// function gives an infinity.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_ref(3, &Rational::ONE_HALF, 5);
    /// assert_eq!(pow.to_string(), "1.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_rational_prec_ref(3, &Rational::ONE_HALF, 20);
    /// assert_eq!(pow.to_string(), "1.7320499");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_rational_prec_ref(x: u64, y: &Rational, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::of_rational(y),
            false,
            prec,
            Nearest,
            true,
        )
    }

    /// Raises a [`u64`] to the power of another [`u64`], returning a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1$ for any $x$
    /// - $f(0,y,p,m)=0$ if $y>0$
    /// - $f(1,y,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_unsigned_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::unsigned_pow_unsigned_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the number of
    /// significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 5, Floor);
    /// assert_eq!(pow.to_string(), "5.02e47");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 5, Ceiling);
    /// assert_eq!(pow.to_string(), "5.25e47");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 5, Nearest);
    /// assert_eq!(pow.to_string(), "5.25e47");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 20, Floor);
    /// assert_eq!(pow.to_string(), "5.153772e47");
    /// assert_eq!(o, Less);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 20, Ceiling);
    /// assert_eq!(pow.to_string(), "5.1537789e47");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 20, Nearest);
    /// assert_eq!(pow.to_string(), "5.153772e47");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_unsigned_prec_round(
        x: u64,
        y: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::BigExponentFloat(Cow::Owned(Self::from(y))),
            false,
            prec,
            rm,
            true,
        )
    }

    /// Raises a [`u64`] to the power of another [`u64`], returning a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=1$ for any $x$
    /// - $f(0,y,p)=0$ if $y>0$
    /// - $f(1,y,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::unsigned_pow_unsigned_prec), this function
    /// never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::unsigned_pow_unsigned_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the number of
    /// significant bits of the exponent.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec(3, 100, 5);
    /// assert_eq!(pow.to_string(), "5.25e47");
    /// assert_eq!(o, Greater);
    ///
    /// let (pow, o) = BigExponentFloat::unsigned_pow_unsigned_prec(3, 100, 20);
    /// assert_eq!(pow.to_string(), "5.153772e47");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_unsigned_prec(x: u64, y: u64, prec: u64) -> (Self, Ordering) {
        pow_general_helper(
            &PowBase::BigExponentFloat(Cow::Owned(Self::from(x))),
            &PowExponent::BigExponentFloat(Cow::Owned(Self::from(y))),
            false,
            prec,
            Nearest,
            true,
        )
    }
}
