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
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, from_float_result,
};
use crate::big_exponent_float::arithmetic::round_to_integer::finite_is_integer;
use crate::big_exponent_float::exponential::exp::{
    MIN_SAFE_MINUS_1_EXPONENT, SAFE_EXPONENT, SAFE_EXPONENT_I64, exp_x_minus_1_series,
    far_below_minus_one, round_nudged, ziv_loop,
};
use crate::big_exponent_float::exponential::power_of_10::{
    power_of_10_approx, power_of_10_fraction_approx,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeOne, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// An approximation of 10^x - 1 = e^t - 1, with t = x log 10, for a tiny nonzero x, and an `err` as
// in `ziv_loop`. t is computed with a relative error of less than 2^(-wp - 3), and since t is tiny,
// e^t - 1 has about the same relative error: less than half an ulp.
fn power_of_10_x_minus_1_tiny_approx(x: &BigExponentFloat, wp: u64) -> (BigExponentFloat, u64) {
    let ln_10 = BigExponentFloat::from(Float::ln_10_prec(wp + 4).0);
    let t = x.mul_prec_ref_val(ln_10, wp + 4).0;
    let (s, loss) = exp_x_minus_1_series(&t, wp);
    (s, wp - loss - 1)
}

// 10^n - 1 for an integer n with |n| >= 2^28, rounded to `prec` bits with `rm`, if it is exact or a
// rounding breakpoint, which can only happen if n > 0 and 10^n - 1, an odd number with more than 3n
// bits, has at most prec + 1 bits; in that case it is computed exactly. Otherwise `None`.
fn power_of_10_x_minus_1_integer_exact(
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let n = u64::try_from(n).ok()?;
    if n.checked_mul(3)? > prec + 1 {
        return None;
    }
    let x = const { Natural::const_from(10) }.pow(n) - Natural::ONE;
    Some(BigExponentFloat::from(x).round_to_prec_round(prec, rm))
}

// 10^x - 1 for an x with 2^28 <= |x|, where x = n + f with an integer n and 0 <= f < 1, given n and
// an approximation of 10^f; rounded to `prec` bits with `rm`, which is not `Exact`. Subtracting 1
// from the approximation of 10^x adds half an ulp, without magnifying relative errors, since the
// result is about 10^x if x >= 2^28 and about -1 if x <= -2^28.
fn power_of_10_x_minus_1_huge<F: FnMut(u64) -> (BigExponentFloat, u64)>(
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
    mut fraction_approx: F,
) -> (BigExponentFloat, Ordering) {
    ziv_loop(prec, rm, |wp| {
        let (y, err) = power_of_10_approx(n, fraction_approx(wp), wp);
        (y.sub_prec(BigExponentFloat::ONE, wp).0, err - 2)
    })
}

// 10^x - 1, rounded to `prec` bits with `rm`.
pub(crate) fn power_of_10_x_minus_1_helper(
    x: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let (sign, exponent) = match &*x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Infinity { sign: false }) => {
            return (BigExponentFloat::NEGATIVE_ONE, Equal);
        }
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite { sign, exponent, .. }) => (*sign, exponent),
    };
    if let Ok(e) = i32::try_from(exponent)
        && (MIN_SAFE_MINUS_1_EXPONENT..SAFE_EXPONENT).contains(&e)
    {
        // The result is at least half of |x| in absolute value, so its exponent is at least e - 1,
        // above `Float`'s minimum. `Float`'s function also handles integer x.
        let (y, o) = from_float_result(
            finite_to_float(x, e).power_of_10_x_minus_1_prec_round(prec, float_rm(rm)),
            None,
        )
        .unwrap();
        assert!(
            rm != Exact || o == Equal,
            "Inexact BigExponentFloat power_of_10_x_minus_1"
        );
        return (y, o);
    }
    let integer = finite_is_integer(&x);
    if integer
        && let Some(result) =
            power_of_10_x_minus_1_integer_exact(&Integer::try_from(&*x).unwrap(), prec, rm)
    {
        return result;
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat power_of_10_x_minus_1");
    if *exponent < 0u32 {
        // x is near or below the bottom of `Float`'s exponent range.
        return ziv_loop(prec, rm, |wp| power_of_10_x_minus_1_tiny_approx(&x, wp));
    }
    if !sign && far_below_minus_one(exponent, prec) {
        return round_nudged(&BigExponentFloat::NEGATIVE_ONE, true, prec, rm);
    }
    let n = Integer::rounding_from(&*x, Floor).0;
    if integer {
        return power_of_10_x_minus_1_huge(&n, prec, rm, |wp| (BigExponentFloat::ONE, wp));
    }
    let f = x.into_owned() - BigExponentFloat::from(n.clone());
    power_of_10_x_minus_1_huge(&n, prec, rm, |wp| power_of_10_fraction_approx(&f, wp))
}

// 10^x - 1 for a `Rational` x, rounded to `prec` bits with `rm`.
pub(crate) fn power_of_10_x_minus_1_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *x == 0u32 {
        return (BigExponentFloat::ZERO, Equal);
    }
    let e = x.floor_log_base_2_abs() + 1;
    if e < SAFE_EXPONENT_I64 {
        if let Some((y, o)) = from_float_result(
            Float::power_of_10_x_minus_1_rational_prec_round_ref(x, prec, float_rm(rm)),
            None,
        ) {
            assert!(
                rm != Exact || o == Equal,
                "Inexact BigExponentFloat power_of_10_x_minus_1"
            );
            return (y, o);
        }
        // The result underflowed `Float`'s exponent range: x is tiny, and not an integer. Rounding
        // it to wp + 4 bits adds a relative error of at most 2^(-wp - 4).
        assert_ne!(rm, Exact, "Inexact BigExponentFloat power_of_10_x_minus_1");
        return ziv_loop(prec, rm, |wp| {
            let x = BigExponentFloat::from_rational_prec_ref(x, wp + 4).0;
            let (y, err) = power_of_10_x_minus_1_tiny_approx(&x, wp);
            (y, err - 1)
        });
    }
    let integer = Integer::try_from(x).ok();
    if let Some(n) = &integer
        && let Some(result) = power_of_10_x_minus_1_integer_exact(n, prec, rm)
    {
        return result;
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat power_of_10_x_minus_1");
    if *x < 0u32 && far_below_minus_one(&Integer::from(e), prec) {
        return round_nudged(&BigExponentFloat::NEGATIVE_ONE, true, prec, rm);
    }
    if let Some(n) = integer {
        return power_of_10_x_minus_1_huge(&n, prec, rm, |wp| (BigExponentFloat::ONE, wp));
    }
    let n = Integer::rounding_from(x, Floor).0;
    let f = x - Rational::from(&n);
    power_of_10_x_minus_1_huge(&n, prec, rm, |wp| {
        (
            BigExponentFloat::from(Float::power_of_10_rational_prec_ref(&f, wp).0),
            wp - 1,
        )
    })
}

impl BigExponentFloat {
    /// Computes $10^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |10^x-1|\rfloor-p+1}$.
    /// - If $10^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-1$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_x_minus_1_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec_round(5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x5.4E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec_round(5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec_round(5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec_round(20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x568.748");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec_round(20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x568.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec_round(20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x568.748");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        power_of_10_x_minus_1_helper(Cow::Owned(self), prec, rm)
    }

    /// Computes $10^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |10^x-1|\rfloor-p+1}$.
    /// - If $10^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=-1$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_round_ref(5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x5.4E+2");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_round_ref(5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_round_ref(5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_round_ref(20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x568.748");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_round_ref(20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x568.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_round_ref(20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x568.748");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        power_of_10_x_minus_1_helper(Cow::Borrowed(self), prec, rm)
    }

    /// Computes $10^x-1$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |10^x-1|\rfloor-p+1}$.
    /// - If $10^x-1$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |10^x-1|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.power_of_10_x_minus_1_prec_round_assign(5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x5.4E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_x_minus_1_prec_round_assign(5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.8E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_x_minus_1_prec_round_assign(5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.8E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.power_of_10_x_minus_1_prec_round_assign(20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x568.748");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.power_of_10_x_minus_1_prec_round_assign(20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x568.75");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.power_of_10_x_minus_1_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x568.748");
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = power_of_10_x_minus_1_helper(Cow::Owned(x), prec, rm);
        *self = result;
        o
    }

    /// Computes $10^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-1$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_prec_round), this
    /// function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_x_minus_1_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec(5);
    /// assert_eq!(format!("{y:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).power_of_10_x_minus_1_prec(20);
    /// assert_eq!(format!("{y:?}"), "0x568.748");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(1.0e10).power_of_10_x_minus_1_prec(10);
    /// assert_eq!(format!("{y:?}"), "0x1.d5E+8304820237");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        power_of_10_x_minus_1_helper(Cow::Owned(self), prec, Nearest)
    }

    /// Computes $10^x-1$, where $x$ is a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=-1$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (y, o) = (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_ref(5);
    /// assert_eq!(format!("{y:?}"), "0x5.8E+2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).power_of_10_x_minus_1_prec_ref(20);
    /// assert_eq!(format!("{y:?}"), "0x568.748");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        power_of_10_x_minus_1_helper(Cow::Borrowed(self), prec, Nearest)
    }

    /// Computes $10^x-1$, where $x$ is a [`BigExponentFloat`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |10^x-1|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_prec_round), this
    /// function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
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
    /// assert_eq!(x.power_of_10_x_minus_1_prec_assign(5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x5.8E+2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.power_of_10_x_minus_1_prec_assign(20), Less);
    /// assert_eq!(format!("{x:?}"), "0x568.748");
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = power_of_10_x_minus_1_helper(Cow::Owned(x), prec, Nearest);
        *self = result;
        o
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |10^x-1|\rfloor-p+1}$.
    /// - If $10^x-1$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_x_minus_1_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb278");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb27c");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb278");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        power_of_10_x_minus_1_rational_helper(&x, prec, rm)
    }

    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |10^x-1|\rfloor-p+1}$.
    /// - If $10^x-1$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.e");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb278");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb27c");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb278");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        power_of_10_x_minus_1_rational_helper(x, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`BigExponentFloat`]. The [`Rational`]
    /// is taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value.
    ///
    /// $$
    /// f(x,p) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::power_of_10_x_minus_1_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb278");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec(
    ///     Rational::from_unsigneds(30000000001u64, 3),
    ///     10,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.f3E+8304820237");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        power_of_10_x_minus_1_rational_helper(&x, prec, Nearest)
    }

    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`BigExponentFloat`]. The [`Rational`]
    /// is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// $$
    /// f(x,p) = 10^x-1+\varepsilon.
    /// $$
    /// - If $10^x-1$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $10^x-1$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |10^x-1|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::power_of_10_x_minus_1_rational_prec_round),
    /// this function never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + mn \log n \log\log n)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x3.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_10_x_minus_1_rational_prec_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    /// );
    /// assert_eq!(format!("{y:?}"), "0x2.fb278");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        power_of_10_x_minus_1_rational_helper(x, prec, Nearest)
    }
}
