// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::log_base_2::extended_log_base_2_of_rational;
use crate::basic::extended::ExtendedFloat;
use crate::{Float, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase, Sign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN, NegativeInfinity, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// Returns `Some(log_base(x))` when it is rational, and `None` when it is irrational (or when the
// inputs are too large for the check to be worthwhile). `x` must be positive and not equal to 1,
// and `base` must be greater than 1.
//
// `log_base(x)` is rational exactly when `x` and `base` are both powers of a common rational `g`,
// say `x = g^a` and `base = g^e_base`; then `log_base(x) = a / e_base`. Taking `g` to be the
// primitive root of `base` (`base.express_as_power()`), this holds iff `x` is an integer power of
// `g`, found by `Rational::checked_log_base` (which also covers `x < 1`, giving a negative `a`).
//
// Detecting these rational results up front is essential, not just an optimization: when the result
// is exactly representable (for example `log_9(3) = 1/2`), the Ziv loop in
// `log_base_rational_rational_base_prec_round_normal` would never terminate, because the rounding
// test can never certify a value sitting exactly on a representable point or tie.
//
// `express_as_power` perfect-power-tests `base`'s numerator and denominator, which is infeasible
// for an astronomically large (for example near-1) base, so the whole check is skipped when either
// input exceeds `64 * prec` bits. Such an input cannot be a power of `g` with a representable
// exponent at this precision (the common cases like `log_4(8)` involve tiny operands), so it is
// left to the Ziv loop. The one residual gap is a representable *fractional* result with a
// perfect-power base larger than the bound -- which would require multi-hundred-megabyte
// commensurable `x` and `base` -- where the loop would not terminate; such inputs are far beyond
// any realistic or testable range.
pub(crate) fn rational_log_base_rational_rational_base(
    x: &Rational,
    base: &Rational,
) -> Option<Rational> {
    // No size cutoff: skipping the check when the result is exactly representable would leave the
    // Ziv loop unable to terminate, and representable results exist at any input size
    // (`log_{3^k}(3) = 1/k` whenever `k` is a power of 2). Both inputs are already materialized
    // `Rational`s, so `express_as_power` and `checked_log_base` cost polynomial in the inputs.
    // `express_as_power` returns `None` when `base` is not a perfect power, in which case `base`
    // itself is `g` (with exponent 1).
    let (root, e_base) = base.express_as_power().unwrap_or_else(|| (base.clone(), 1));
    let a = x.checked_log_base(&root)?;
    Some(Rational::from_signeds(a, i64::exact_from(e_base)))
}

// Computes log_base(x) for `Rational` `x` and `base`, by log_base(x) = log_2(x) / log_2(base). The
// input `x` is positive, and `base` is greater than 1.
//
// Both logs are computed in the extended exponent range (see `extended_log_base_2_of_rational`) so
// that neither operand underflows when `x` or `base` is near 1, and so that their quotient may
// temporarily leave the representable range. The single conversion back to a `Float`, via
// `ExtendedFloat::into_float_helper`, performs the one correctly-rounded clamp to an infinity,
// maximum, zero, or minimum as dictated by the rounding mode. This handles both overflow (a base
// near 1, so `log_2(base)` is tiny and the quotient is huge) and underflow (an `x` near 1, so
// `log_2(x)` is tiny).
fn log_base_rational_rational_base_prec_round_normal(
    x: &Rational,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // If x is 1, the result is 0.
    if *x == 1u32 {
        return (Float::ZERO, Equal);
    }
    // If log_base(x) is rational -- x and base are both powers of a common rational -- compute it
    // directly. This includes exactly-representable results (which the Ziv loop could never
    // certify) as well as non-representable rationals (cheaper and exact this way).
    if let Some(q) = rational_log_base_rational_rational_base(x, base) {
        return Float::from_rational_prec_round(q, prec, rm);
    }
    // The result is irrational, so it is never exactly representable.
    assert_ne!(rm, Exact, "Inexact log_base_rational_rational_base");
    // The initial slack keeps working_prec at least 7, so the working_prec - 6 below stays
    // positive.
    let mut working_prec = prec + 6 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // log_2(x), extended (finite and nonzero, since x is positive and not 1).
        let num = extended_log_base_2_of_rational(x, working_prec);
        // log_2(base) > 0, extended.
        let den = extended_log_base_2_of_rational(base, working_prec);
        // log_2(x) / log_2(base) in the extended range; cannot overflow or underflow here.
        let quotient = num.div_prec_val_ref(&den, working_prec).0;
        // Each log is accurate to within 2 ulps and the division adds at most 1 more, for at most 5
        // ulps total; working_prec - 6 correct bits comfortably suffice for the rounding test.
        if float_can_round(
            quotient.x.significand_ref().unwrap(),
            working_prec - 6,
            prec,
            rm,
        ) {
            // Round the mantissa to prec, then place the extended exponent, clamping once to the
            // Float range as the rounding mode dictates.
            let (rounded, o) = Float::from_float_prec_round(quotient.x, prec, rm);
            let mut result = ExtendedFloat::from(rounded);
            result.exp = result.exp.checked_add(quotient.exp).unwrap();
            return result.into_float_helper(prec, rm, o);
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Rational`]s with $b>1$, returning
    /// a [`Float`] rounded to the specified precision and with the specified rounding mode. Both
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This computes $\log_2 x / \log_2 b$. Both logarithms are evaluated in an extended exponent
    /// range, so that an $x$ or $b$ extremely close to 1 (where the logarithm is tiny) does not
    /// lose accuracy, and the single conversion of the quotient back to a [`Float`] performs the
    /// one correctly-rounded clamp.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,b,p,m)=-\infty$
    /// - $f(x,b,p,m)=\text{NaN}$ for $x<0$
    /// - $f(1,b,p,m)=0$
    /// - $f(x,b,p,m)=a/e$ when $x=g^a$, where $g$ is the primitive root of $b$ and $b=g^e$, rounded
    ///   to precision $p$; the result is exact if and only if $a/e$ is representable with precision
    ///   $p$ (for example $\log_4 8=3/2$ is exact)
    ///
    /// Like a logarithm of a [`Float`] with a [`Rational`] base, this can both overflow (for a base
    /// near 1) and underflow (for an $x$ near 1).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_rational_base_prec_round(
    ///     Rational::from(8),
    ///     Rational::from(4),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::log_base_rational_rational_base_prec_round(
    ///     Rational::from(2),
    ///     Rational::from(3),
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(log.to_string(), "0.631"); // log_3(2) = 0.6309...
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_rational_rational_base_prec_round(
        x: Rational,
        base: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::log_base_rational_rational_base_prec_round_ref(&x, &base, prec, rm)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Rational`]s with $b>1$, returning
    /// a [`Float`] rounded to the specified precision and with the specified rounding mode. Both
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See [`Float::log_base_rational_rational_base_prec_round`] for details, special cases, and a
    /// description of the rounding behavior.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than or equal to 1, or if `rm` is `Exact` but
    /// the result cannot be represented exactly with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_rational_base_prec_round_ref(
    ///     &Rational::from(9),
    ///     &Rational::from(3),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    ///
    /// let (log, o) = Float::log_base_rational_rational_base_prec_round_ref(
    ///     &Rational::from_signeds(1, 8),
    ///     &Rational::from(2),
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(log.to_string(), "-3.0"); // log_2(1/8) = -3
    /// assert_eq!(o, Equal);
    /// ```
    pub fn log_base_rational_rational_base_prec_round_ref(
        x: &Rational,
        base: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert!(*base > 1u32, "Logarithm base must be greater than 1");
        match x.sign() {
            Less => (Self::NAN, Equal),
            Equal => (Self::NEGATIVE_INFINITY, Equal),
            Greater => log_base_rational_rational_base_prec_round_normal(x, base, prec, rm),
        }
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Rational`]s with $b>1$, returning
    /// a [`Float`] rounded to the nearest value of the specified precision. Both are taken by
    /// value. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_rational_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) =
    ///     Float::log_base_rational_rational_base_prec(Rational::from(8), Rational::from(4), 10);
    /// assert_eq!(log.to_string(), "1.5"); // log_4(8) = 3/2
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn log_base_rational_rational_base_prec(
        x: Rational,
        base: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::log_base_rational_rational_base_prec_round_ref(&x, &base, prec, Nearest)
    }

    /// Computes $\log_b x$, where $x$ and the base $b$ are both [`Rational`]s with $b>1$, returning
    /// a [`Float`] rounded to the nearest value of the specified precision. Both are taken by
    /// reference. An [`Ordering`] is also returned.
    ///
    /// See [`Float::log_base_rational_rational_base_prec_round`] for details and special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than or equal to 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (log, o) = Float::log_base_rational_rational_base_prec_ref(
    ///     &Rational::from(9),
    ///     &Rational::from(3),
    ///     10,
    /// );
    /// assert_eq!(log.to_string(), "2.0"); // log_3(9) = 2
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn log_base_rational_rational_base_prec_ref(
        x: &Rational,
        base: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::log_base_rational_rational_base_prec_round_ref(x, base, prec, Nearest)
    }
}

/// Computes $\log_b x$, the base-$b$ logarithm of a [`Rational`], where the base $b$ is also a
/// [`Rational`] greater than 1, returning a primitive float result. Using this function is more
/// accurate than computing the logarithm using the standard library, whose logarithm functions are
/// not always correctly rounded.
///
/// If the logarithm is equidistant from two primitive floats, the primitive float with fewer 1s in
/// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
/// mode.
///
/// The base-$b$ logarithm of any negative number is `NaN`.
///
/// $$
/// f(x,b) = \log_b x+\varepsilon.
/// $$
/// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |\log_b
///   x|\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`] and 53
///   if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0,b)=-\infty$
/// - $f(x,b)=\text{NaN}$ for $x<0$
/// - $f(1,b)=0.0$
///
/// Unlike a logarithm with an integer base, this function can both overflow (for a base near 1) and
/// underflow (for an $x$ near 1).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `base` is less than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::log_base_rational_rational_base::*;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_rational_base::<f32>(
///         &Rational::ZERO,
///         &Rational::from(10)
///     )),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// // log_4(8) = 3/2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_rational_base::<f32>(
///         &Rational::from(8),
///         &Rational::from(4)
///     )),
///     NiceFloat(1.5)
/// );
/// // log_(3/2)(9/4) = 2
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_rational_base::<f32>(
///         &Rational::from_unsigneds(9u8, 4),
///         &Rational::from_unsigneds(3u8, 2)
///     )),
///     NiceFloat(2.0)
/// );
/// // log_10(50)
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_rational_base::<f32>(
///         &Rational::from(50),
///         &Rational::from(10)
///     )),
///     NiceFloat(1.69897)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_log_base_rational_rational_base::<f32>(
///         &Rational::from(-1000),
///         &Rational::from(10)
///     )),
///     NiceFloat(f32::NAN)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_log_base_rational_rational_base<T: PrimitiveFloat>(
    x: &Rational,
    base: &Rational,
) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::log_base_rational_rational_base_prec_ref(x, base, prec),
        x,
    )
}
