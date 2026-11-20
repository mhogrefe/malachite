// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::conversion::float_from_big_exponent_float::*;
use crate::big_exponent_float::exponential::exp::power_of_2_integer;
use crate::big_exponent_float::gamma::gamma::{gamma_helper, gamma_rational_helper};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::gamma::traits::{Gamma, GammaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// Gamma(x) is too large to be a `Float` when x >= 2^26, since Gamma(2^26) > 2^(2^30) and the gamma
// function increases from there on. This is the exponent of such an x.
const GAMMA_HUGE_EXPONENT: i32 = 27;

// The result when the value is too large to be a `Float` (if `overflow` is set), or is nonzero and
// so far below the least positive `Float` in absolute value that it rounds as any such value does.
// The value is positive if `sign` is set.
fn far_out_of_range(sign: bool, overflow: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let e = Integer::power_of_2(40);
    let y = power_of_2_integer(if overflow { e } else { -e });
    float_from_rounded_big_exponent_float(
        if sign { y } else { -y },
        match (rm, sign) {
            (Ceiling, _) | (Up, true) | (Down, false) => Greater,
            _ => Less,
        },
        prec,
        rm,
    )
}

fn assert_inexact_allowed(recip: bool, rm: RoundingMode) {
    if recip {
        assert_ne!(rm, Exact, "Inexact reciprocal gamma function");
    } else {
        assert_ne!(rm, Exact, "Inexact gamma function");
    }
}

// Gamma(x), or 1 / Gamma(x) if `recip` is set, rounded to `prec` bits with `rm`.
//
// For a huge |x| the value is out of `Float`'s range, which is decided without computing it: that
// would take about as many bits of working precision as x has before its binary point.
// - x >= 2^26: Gamma(x) > 2^(2^30).
// - x <= -2^26, not an integer: by the reflection formula |Gamma(x)| = pi / (|sin(pi x)| Gamma(1 -
//   x)), where |sin(pi x)| >= 2d for the distance d from x to the nearest integer, and Gamma(1 - x)
//   > 2^(1.6 * 2^30). The lowest bit of x gives d >= 2^(e - p), for the exponent e and the
//   precision p of x, so |Gamma(x)| < 2^(-2^30 - 1) when e - p >= -2^29. The sign of Gamma(x) is
//   negative when the floor of x is odd.
pub(crate) fn gamma_prec_round_helper(
    recip: bool,
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x {
        Float(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
        Float(Infinity { sign: true }) => (
            if recip {
                float_zero!()
            } else {
                float_infinity!()
            },
            Equal,
        ),
        Float(Zero { sign }) => (
            if recip {
                x.clone()
            } else if *sign {
                float_infinity!()
            } else {
                float_negative_infinity!()
            },
            Equal,
        ),
        Float(Finite {
            sign,
            exponent,
            precision,
            ..
        }) => {
            if *exponent >= GAMMA_HUGE_EXPONENT {
                if *sign {
                    assert_inexact_allowed(recip, rm);
                    return far_out_of_range(true, !recip, prec, rm);
                } else if !x.is_integer()
                    && i64::try_from(*precision)
                        .is_ok_and(|p| i64::from(*exponent) - p >= -(1 << 29))
                {
                    assert_inexact_allowed(recip, rm);
                    return far_out_of_range(
                        Integer::rounding_from(x, Floor).0.even(),
                        recip,
                        prec,
                        rm,
                    );
                }
            }
            let (g, o) = gamma_helper(recip, &BigExponentFloat::from(x), prec, rm);
            float_from_rounded_big_exponent_float(g, o, prec, rm)
        }
    }
}

// Gamma(q), or 1 / Gamma(q) if `recip` is set, for a `Rational` q, rounded to `prec` bits with
// `rm`. See `gamma_prec_round_helper`; here the distance from q to the nearest integer is at least
// the reciprocal of the denominator of q.
pub(crate) fn gamma_rational_prec_round_helper(
    recip: bool,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if *q == 0u32 {
        return (
            if recip {
                float_zero!()
            } else {
                float_infinity!()
            },
            Equal,
        );
    }
    if *q >= const { Rational::const_from_unsigned(1 << 26) } {
        assert_inexact_allowed(recip, rm);
        return far_out_of_range(true, !recip, prec, rm);
    }
    if *q <= const { Rational::const_from_signed(-(1 << 26)) }
        && !q.is_integer()
        && q.denominator_ref().significant_bits() <= 1 << 29
    {
        assert_inexact_allowed(recip, rm);
        return far_out_of_range(Integer::rounding_from(q, Floor).0.even(), recip, prec, rm);
    }
    let (g, o) = gamma_rational_helper(recip, q, prec, rm);
    float_from_rounded_big_exponent_float(g, o, prec, rm)
}

impl Float {
    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gamma_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::gamma_round`] instead. If both of these things are true, consider using
    /// [`Float::gamma`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "1.7724533");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.7724552");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "1.7724533");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_prec_round_helper(false, &self, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gamma_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::gamma_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).gamma()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) =
    ///     (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "1.7724533");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.7724552");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_round_ref(20, Nearest);
    /// assert_eq!(g.to_string(), "1.7724533");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gamma_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_prec_round_helper(false, self, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::gamma`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec(5);
    /// assert_eq!(g.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec(20);
    /// assert_eq!(g.to_string(), "1.7724533");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gamma_prec(self, prec: u64) -> (Self, Ordering) {
        self.gamma_prec_round(prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], rounding the result to the nearest
    /// value of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).gamma()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "1.75");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "1.7724533");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gamma_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    /// - $f(x,m)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on overflow and underflow,
    /// and on termination.
    ///
    /// If you want to specify an output precision, consider using [`Float::gamma_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::gamma`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (it can only when it is infinite or `NaN`, or when $x$ is a positive integer and
    /// $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_round(Floor);
    /// assert_eq!(g.to_string(), "1.7724538509055160272981674833397");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_round(Ceiling);
    /// assert_eq!(g.to_string(), "1.7724538509055160272981674833413");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_round(Nearest);
    /// assert_eq!(g.to_string(), "1.7724538509055160272981674833413");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.gamma_prec_round(prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    /// - $f(x,m)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on overflow and underflow,
    /// and on termination.
    ///
    /// If you want to specify an output precision, consider using [`Float::gamma_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).gamma()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (it can only when it is infinite or `NaN`, or when $x$ is a positive integer and
    /// $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_round_ref(Floor);
    /// assert_eq!(g.to_string(), "1.7724538509055160272981674833397");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_round_ref(Ceiling);
    /// assert_eq!(g.to_string(), "1.7724538509055160272981674833413");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = (Float::from_unsigned_prec(1u32, 100).0 >> 1u32).gamma_round_ref(Nearest);
    /// assert_eq!(g.to_string(), "1.7724538509055160272981674833413");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.gamma_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on special cases, overflow
    /// and underflow, and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gamma_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::gamma_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::gamma_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.75");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.81");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.75");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.7724533");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.7724552");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.7724533");
    /// ```
    #[inline]
    pub fn gamma_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).gamma_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`] in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on special cases, overflow
    /// and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::gamma_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.75");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.7724533");
    /// ```
    #[inline]
    pub fn gamma_prec_assign(&mut self, prec: u64) -> Ordering {
        self.gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`] in place, rounding the result with
    /// the specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on special cases, overflow
    /// and underflow, and termination.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::gamma_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::gamma_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (it can only when it is infinite or `NaN`, or when $x$ is a positive integer and
    /// $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.7724538509055160272981674833397");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.7724538509055160272981674833413");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.gamma_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.7724538509055160272981674833413");
    /// ```
    #[inline]
    pub fn gamma_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.gamma_prec_round_assign(prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gamma_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = Float::gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(g.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = Float::gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(g.to_string(), "2.6789360");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(g.to_string(), "2.6789398");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(g.to_string(), "2.6789398");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn gamma_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_rational_prec_round_helper(false, &x, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
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
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gamma_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(g.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(g.to_string(), "2.6789360");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(g.to_string(), "2.6789398");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     Float::gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(g.to_string(), "2.6789398");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gamma_rational_prec_round_helper(false, x, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (g, o) = Float::gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "2.6789398");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::gamma_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Overflow happens for arguments above about $4.5\times10^7$ and for arguments of magnitude at
    /// most about $2^{-2^{30}+1}$, and underflow for arguments below about $-4.5\times10^7$ that
    /// are not integers.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (g, o) = Float::gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = Float::gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "2.6789398");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Gamma for Float {
    type Output = Self;

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], taking it by value.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    /// - $f(x)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on overflow and underflow,
    /// and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::gamma_prec`]. If you want both of these things, consider using
    /// [`Float::gamma_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, Zero};
    /// use malachite_base::num::gamma::traits::Gamma;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.gamma().is_nan());
    /// assert_eq!(Float::INFINITY.gamma(), Float::INFINITY);
    /// assert_eq!(Float::ZERO.gamma(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_ONE.gamma().is_nan());
    /// assert_eq!(
    ///     (Float::from_unsigned_prec(1u32, 100).0 >> 1u32)
    ///         .gamma()
    ///         .to_string(),
    ///     "1.7724538509055160272981674833413"
    /// );
    /// ```
    #[inline]
    fn gamma(self) -> Self {
        let prec = self.significant_bits();
        self.gamma_prec(prec).0
    }
}

impl Gamma for &Float {
    type Output = Float;

    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`], taking it by reference.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    /// - $f(x)=\text{NaN}$ if $x$ is a negative integer
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on overflow and underflow,
    /// and on termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::gamma_prec_ref`]. If you want both of these things, consider using
    /// [`Float::gamma_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, Zero};
    /// use malachite_base::num::gamma::traits::Gamma;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).gamma().is_nan());
    /// assert_eq!((&Float::INFINITY).gamma(), Float::INFINITY);
    /// assert_eq!((&Float::ZERO).gamma(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_ONE).gamma().is_nan());
    /// assert_eq!(
    ///     (&(Float::from_unsigned_prec(1u32, 100).0 >> 1u32))
    ///         .gamma()
    ///         .to_string(),
    ///     "1.7724538509055160272981674833413"
    /// );
    /// ```
    #[inline]
    fn gamma(self) -> Float {
        self.gamma_prec_ref(self.significant_bits()).0
    }
}

impl GammaAssign for Float {
    /// Computes $\Gamma(x)$, the gamma function, of a [`Float`] in place.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::gamma_prec_round`] documentation for information on special cases, overflow
    /// and underflow, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gamma_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::gamma_prec_assign`]. If you want both of these things, consider using
    /// [`Float::gamma_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeOne, Zero};
    /// use malachite_base::num::gamma::traits::GammaAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.gamma_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.gamma_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::ZERO;
    /// x.gamma_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.gamma_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// x.gamma_assign();
    /// assert_eq!(x.to_string(), "1.7724538509055160272981674833413");
    /// ```
    #[inline]
    fn gamma_assign(&mut self) {
        let prec = self.significant_bits();
        self.gamma_prec_assign(prec);
    }
}

/// Computes $\Gamma(x)$, the gamma function, of a primitive float.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
/// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has poles.
/// If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
///
/// $$
/// f(x) = \Gamma(x)+\varepsilon.
/// $$
/// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(0.0)=\infty$
/// - $f(-0.0)=-\infty$
/// - $f(x)=\text{NaN}$ if $x$ is a negative integer
///
/// The result is $\pm\infty$ when $\Gamma(x)$ is too large for `T`: for $x$ above about 35.04
/// (`f32`) or 171.62 (`f64`), and for an $x$ close enough to zero. It is $\pm0.0$ when $\Gamma(x)$
/// is too small, which happens for a negative $x$ of large enough magnitude that is not an integer.
///
/// See the [`Float::gamma_prec_round`] documentation for information on termination.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::gamma::gamma::primitive_float_gamma;
///
/// assert!(primitive_float_gamma(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_gamma(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gamma(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gamma(-0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gamma(0.5f32)),
///     NiceFloat(1.7724539)
/// );
/// assert_eq!(NiceFloat(primitive_float_gamma(5.0f32)), NiceFloat(24.0));
/// assert!(primitive_float_gamma(-1.0f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_gamma(-0.5f32)),
///     NiceFloat(-3.5449078)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_gamma<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::gamma_prec, x)
}

/// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], returning the result as a primitive
/// float.
///
/// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
/// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has poles.
/// If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
///
/// $$
/// f(x) = \Gamma(x)+\varepsilon.
/// $$
/// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\Gamma(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=\infty$
/// - $f(x)=\text{NaN}$ if $x$ is a negative integer
///
/// The result is $\pm\infty$ when $\Gamma(x)$ is too large for `T`: for $x$ above about 35.04
/// (`f32`) or 171.62 (`f64`), and for an $x$ close enough to zero. It is $\pm0.0$ when $\Gamma(x)$
/// is too small, which happens for a negative $x$ of large enough magnitude that is not an integer.
///
/// See the [`Float::gamma_rational_prec_round`] documentation for information on termination.
///
/// # Worst-case complexity
/// $T(n) = O(n^3 \log n)$
///
/// $M(n) = O(n^2 \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::gamma::gamma::primitive_float_gamma_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_gamma_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gamma_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(2.6789385347077475)
/// );
/// assert!(primitive_float_gamma_rational::<f64>(&Rational::NEGATIVE_ONE).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_gamma_rational::<f64>(&Rational::from(5))),
///     NiceFloat(24.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_gamma_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::gamma_rational_prec_ref, x)
}
