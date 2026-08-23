// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2014-2025 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::emulate_float_slice_to_float_fn;
use crate::float::{MAX_EXPONENT_I64, MIN_EXPONENT_I64};
use crate::{
    Float, float_infinity, float_nan, float_negative_infinity, float_negative_zero, float_zero,
};
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::iter::Sum;
use malachite_base::num::arithmetic::traits::ShlRoundAssign;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float::sum::{
    FloatSumInput, FloatSumResult, sum_float_significands,
};

// Update the sticky sign of an all-zero result with a new zero term of sign `s` (1 or -1): if
// all the zeros seen so far have the same sign, the result keeps that sign; otherwise the sign
// of the zero result depends only on the rounding mode.
pub(crate) fn update_zero_sign(sign_zero: &mut i8, s: i8, rm: RoundingMode) {
    if *sign_zero == 0 {
        *sign_zero = s;
    } else if *sign_zero != s {
        *sign_zero = if rm == Floor { -1 } else { 1 };
    }
}

// This is mpfr_sum from sum.c, MPFR 4.2.2, split into the singular scan below and the
// significand-level sum_aux in malachite-nz. The `Exact` rounding mode is handled by computing with
// `Nearest` and panicking if the result is inexact.
fn sum_prec_round_helper(xs: &[&Float], prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let n = xs.len();
    if n == 0 {
        return (float_zero!(), Equal);
    } else if n == 1 {
        return Float::from_float_prec_round_ref(xs[0], prec, rm);
    } else if n == 2 {
        return xs[0].add_prec_round_ref_ref(xs[1], prec, rm);
    }
    // Check for special inputs, and determine the sign of an infinite result, the sign of an
    // all-zero result, and the regular inputs.
    let mut sign_inf = 0i8;
    let mut sign_zero = 0i8;
    let mut regulars: Vec<&Float> = Vec::new();
    for x in xs {
        match x {
            float_nan!() => return (float_nan!(), Equal),
            float_infinity!() => {
                if sign_inf == 0 {
                    sign_inf = 1;
                } else if sign_inf < 0 {
                    return (float_nan!(), Equal);
                }
            }
            float_negative_infinity!() => {
                if sign_inf == 0 {
                    sign_inf = -1;
                } else if sign_inf > 0 {
                    return (float_nan!(), Equal);
                }
            }
            float_zero!() => {
                if regulars.is_empty() {
                    // This choice is sticky when new zeros are considered.
                    update_zero_sign(&mut sign_zero, 1, rm);
                }
            }
            float_negative_zero!() => {
                if regulars.is_empty() {
                    update_zero_sign(&mut sign_zero, -1, rm);
                }
            }
            _ => regulars.push(x),
        }
    }
    // At this point the result cannot be NaN.
    if sign_inf != 0 {
        // At least one infinity, and all of them have the same sign. The sum is the infinity of
        // this sign.
        return if sign_inf > 0 {
            (float_infinity!(), Equal)
        } else {
            (float_negative_infinity!(), Equal)
        };
    }
    // At this point, all the inputs are finite numbers.
    if regulars.is_empty() {
        // All the numbers were zeros (and there is at least one). The sum is zero with sign
        // sign_zero.
        assert_ne!(sign_zero, 0);
        return if sign_zero > 0 {
            (float_zero!(), Equal)
        } else {
            (float_negative_zero!(), Equal)
        };
    }
    // Optimize the case where there are only one or two regular numbers.
    if regulars.len() == 1 {
        return Float::from_float_prec_round_ref(regulars[0], prec, rm);
    } else if regulars.len() == 2 {
        return regulars[0].add_prec_round_ref_ref(regulars[1], prec, rm);
    }
    let (kernel_rm, exact) = if rm == Exact {
        (Nearest, true)
    } else {
        (rm, false)
    };
    let inputs: Vec<FloatSumInput> = regulars
        .iter()
        .map(|x| {
            let Float(Finite {
                sign,
                exponent,
                precision,
                significand,
            }) = x
            else {
                unreachable!()
            };
            FloatSumInput {
                sign: *sign,
                exp: i64::from(*exponent),
                prec: *precision,
                significand,
            }
        })
        .collect();
    complete_sum_result(
        sum_float_significands(&inputs, prec, kernel_rm),
        prec,
        rm,
        exact,
        "Inexact Float sum",
    )
}

// Convert a summation-kernel result into a `Float`, applying the cancellation-zero sign rule and
// the exponent range check. `exact_message` is the panic message demanded when the caller's
// rounding mode was `Exact` (indicated by `exact`) but the result is inexact.
pub(crate) fn complete_sum_result(
    result: FloatSumResult,
    prec: u64,
    rm: RoundingMode,
    exact: bool,
    exact_message: &str,
) -> (Float, Ordering) {
    match result {
        FloatSumResult::Zero => {
            // The exact sum of nonzero values is zero, which is +0 except in the Floor rounding
            // mode, as specified according to the IEEE 754 rules for the addition of two numbers.
            if rm == Floor {
                (float_negative_zero!(), Equal)
            } else {
                (float_zero!(), Equal)
            }
        }
        FloatSumResult::Regular {
            sign,
            exp,
            significand,
            o,
        } => {
            if exact {
                assert_eq!(o, Equal, "{exact_message}");
            }
            if (MIN_EXPONENT_I64..=MAX_EXPONENT_I64).contains(&exp) {
                (
                    Float(Finite {
                        sign,
                        exponent: i32::exact_from(exp),
                        precision: prec,
                        significand,
                    }),
                    o,
                )
            } else {
                // The exponent is out of range; construct the value at a safe exponent and use a
                // saturating shift to apply the overflow or underflow rules, in the same
                // round-then-check-range order as MPFR.
                assert!(!exact, "{exact_message}");
                let mut f = Float(Finite {
                    sign,
                    exponent: 1,
                    precision: prec,
                    significand,
                });
                let o_shift = f.shl_round_assign(exp - 1, rm);
                (f, if o_shift == Equal { o } else { o_shift })
            }
        }
    }
}

// The precision used by `sum_round` and the `Sum` implementations: the maximum precision of the
// inputs, or 1 if there are none.
pub(crate) fn max_prec<'a, I: Iterator<Item = &'a Float>>(xs: I) -> u64 {
    xs.map(SignificantBits::significant_bits).max().unwrap_or(1)
}

impl Float {
    /// Computes the sum of a slice of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode. An [`Ordering`] is also returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// Only a single rounding is performed, no matter how many inputs there are: the result is the
    /// correctly-rounded exact sum, with no intermediate rounding, overflow, or underflow.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, p, m) = \sum_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p+1}$.
    /// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - The sum of no [`Float`]s is $0.0$.
    /// - If any input is `NaN`, or if the inputs include both $\infty$ and $-\infty$, the sum is
    ///   `NaN`.
    /// - Otherwise, if any input is $\infty$, the sum is $\infty$, and if any input is $-\infty$,
    ///   the sum is $-\infty$.
    /// - If every input is a zero and all of them have the same sign, the sum is a zero of that
    ///   sign.
    /// - If every input is a zero and they do not all have the same sign, the sum is $0.0$, unless
    ///   $m$ is `Floor`, in which case it is $-0.0$.
    /// - If the inputs include a nonzero value but sum to zero exactly, the sum is $0.0$, unless
    ///   $m$ is `Floor`, in which case it is $-0.0$.
    ///
    /// Overflow and underflow:
    /// - If $f((x_i)_ {i=0}^{n-1},p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`,
    ///   $\infty$ is returned instead.
    /// - If $f((x_i)_ {i=0}^{n-1},p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead.
    /// - If $f((x_i)_ {i=0}^{n-1},p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`,
    ///   $-\infty$ is returned instead.
    /// - If $f((x_i)_ {i=0}^{n-1},p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead.
    /// - If $0<f((x_i)_ {i=0}^{n-1},p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is
    ///   returned instead.
    /// - If $0<f((x_i)_ {i=0}^{n-1},p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$
    ///   is returned instead.
    /// - If $0<f((x_i)_ {i=0}^{n-1},p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned
    ///   instead.
    /// - If $2^{-2^{30}-1}<f((x_i)_ {i=0}^{n-1},p,m)<2^{-2^{30}}$, and $m$ is `Nearest`,
    ///   $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<f((x_i)_ {i=0}^{n-1},p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f((x_i)_ {i=0}^{n-1},p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$
    ///   is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f((x_i)_ {i=0}^{n-1},p,m)<0$, and $m$ is `Nearest`, $-0.0$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f((x_i)_ {i=0}^{n-1},p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`,
    ///   $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sum_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sum_round`] instead. If both of these things are true, consider summing an
    /// iterator with [`Sum`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, p) = O(n + m (n + p))$
    ///
    /// $M(n, p) = O(n + p)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`, and $p$ is `prec`: each significand bit enters
    /// the accumulator at most once, but adversarially-placed cancelling clusters can force a pass
    /// over all $n$ inputs and the $O(p + \log n)$-bit accumulator for every such bit.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact sum is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [Float::ONE, Float::power_of_2(-20i64), Float::power_of_2(-40i64)];
    ///
    /// let (sum, o) = Float::sum_prec_round(&xs, 10, Floor);
    /// assert_eq!(sum.to_string(), "1.0000");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::sum_prec_round(&xs, 10, Ceiling);
    /// assert_eq!(sum.to_string(), "1.0020");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::sum_prec_round(&xs, 10, Nearest);
    /// assert_eq!(sum.to_string(), "1.0000");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::sum_prec_round(&xs, 30, Floor);
    /// assert_eq!(sum.to_string(), "1.0000009537");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::sum_prec_round(&xs, 30, Ceiling);
    /// assert_eq!(sum.to_string(), "1.0000009555");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::sum_prec_round(&xs, 30, Nearest);
    /// assert_eq!(sum.to_string(), "1.0000009537");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sum_prec_round(xs: &[Self], prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let refs: Vec<&Self> = xs.iter().collect();
        sum_prec_round_helper(&refs, prec, rm)
    }

    /// Computes the sum of a slice of [`Float`]s, rounding the result to the nearest value of the
    /// specified precision. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// Only a single rounding is performed, no matter how many inputs there are: the result is the
    /// correctly-rounded exact sum, with no intermediate rounding, overflow, or underflow.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, p) = \sum_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See [`Float::sum_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider summing an iterator with [`Sum`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, p) = O(n + m (n + p))$
    ///
    /// $M(n, p) = O(n + p)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`, and $p$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::sum_prec(&[Float::ONE, Float::TWO, Float::from(3)], 10);
    /// assert_eq!(sum.to_string(), "6.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (sum, o) = Float::sum_prec(
    ///     &[Float::ONE, Float::power_of_2(-20i64), Float::power_of_2(-40i64)],
    ///     30,
    /// );
    /// assert_eq!(sum.to_string(), "1.0000009537");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sum_prec(xs: &[Self], prec: u64) -> (Self, Ordering) {
        Self::sum_prec_round(xs, prec, Nearest)
    }

    /// Computes the sum of a slice of [`Float`]s, rounding the result with the specified rounding
    /// mode. The precision of the result is the maximum of the precisions of the inputs (or 1 if
    /// there are no inputs). An [`Ordering`] is also returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// Only a single rounding is performed, no matter how many inputs there are: the result is the
    /// correctly-rounded exact sum, with no intermediate rounding, overflow, or underflow.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, m) = \sum_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p+1}$, where $p$ is the
    ///   maximum precision of the inputs.
    /// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the
    ///   maximum precision of the inputs.
    ///
    /// See [`Float::sum_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// If you know you'll be using `Nearest`, consider summing an iterator with [`Sum`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact sum is not exactly representable with the maximum of
    /// the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [Float::one_prec(10), Float::power_of_2(-20i64), Float::power_of_2(-40i64)];
    ///
    /// let (sum, o) = Float::sum_round(&xs, Floor);
    /// assert_eq!(sum.to_string(), "1.0000");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::sum_round(&xs, Ceiling);
    /// assert_eq!(sum.to_string(), "1.0020");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sum_round(xs: &[Self], rm: RoundingMode) -> (Self, Ordering) {
        Self::sum_prec_round(xs, max_prec(xs.iter()), rm)
    }
}

/// Computes the sum of a slice of primitive floats, with a single rounding.
///
/// The result is correctly rounded to the nearest value: the sum is computed as if in infinite
/// precision and rounded only once, at the end, no matter how many inputs there are. This includes
/// gradual underflow: results in the subnormal range are correctly rounded to their reduced
/// precisions.
///
/// $$
/// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i + \varepsilon.
/// $$
/// - If $\sum_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
///   assumed to be 0.
/// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, then
///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the
///   precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a [`f64`], but
///   less if the output is subnormal).
///
/// Special cases:
/// - The sum of no floats is $0.0$.
/// - If any input is `NaN`, or if the inputs include both $\infty$ and $-\infty$, the sum is
///   `NaN`.
/// - Otherwise, if any input is $\infty$, the sum is $\infty$, and if any input is $-\infty$,
///   the sum is $-\infty$.
/// - If every input is a zero and all of them have the same sign, the sum is a zero of that sign.
///   If they do not all have the same sign, or if the inputs include a nonzero value but sum to
///   zero exactly, the sum is $0.0$.
///
/// If the result overflows, $\pm\infty$ is returned, and if it underflows, $\pm0.0$ is
/// returned.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`: a primitive float's
/// exponent range is bounded, so the summation window is repositioned only a constant number of
/// times.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sum::primitive_float_sum;
///
/// // Each addition of 0.1 in a naive fold rounds, but here only one rounding is performed.
/// assert_eq!(
///     NiceFloat(primitive_float_sum(&[0.1f64; 10])),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat([0.1f64; 10].iter().sum::<f64>()),
///     NiceFloat(0.9999999999999999)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_sum<T: PrimitiveFloat>(xs: &[T]) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_slice_to_float_fn(Float::sum_prec, xs)
}

impl Sum<Self> for Float {
    /// Adds up all the [`Float`]s in an iterator.
    ///
    /// The result has the maximum of the precisions of the inputs (or 1 if there are no inputs),
    /// and the sum is rounded to nearest. Only a single rounding is performed, no matter how many
    /// inputs there are: the result is the correctly-rounded exact sum, with no intermediate
    /// rounding, overflow, or underflow.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the maximum precision of
    ///   the inputs.
    ///
    /// See [`Float::sum_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    ///
    /// let sum = Float::sum([Float::ONE, Float::TWO, Float::from(3)].into_iter());
    /// assert_eq!(sum.to_string(), "6.0");
    ///
    /// // All twenty inputs have precision 1, so the result has precision 1, but the sum is still
    /// // exact: it is computed as if in infinite precision and rounded only once.
    /// let sum = Float::sum(vec![Float::ONE; 20].into_iter());
    /// assert_eq!(sum.to_string(), "16.0");
    /// ```
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let xs: Vec<Self> = xs.collect();
        let refs: Vec<&Self> = xs.iter().collect();
        sum_prec_round_helper(&refs, max_prec(xs.iter()), Nearest).0
    }
}

impl<'a> Sum<&'a Self> for Float {
    /// Adds up all the [`Float`]s in an iterator of [`Float`] references.
    ///
    /// The result has the maximum of the precisions of the inputs (or 1 if there are no inputs),
    /// and the sum is rounded to nearest. Only a single rounding is performed, no matter how many
    /// inputs there are: the result is the correctly-rounded exact sum, with no intermediate
    /// rounding, overflow, or underflow.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the maximum precision of
    ///   the inputs.
    ///
    /// See [`Float::sum_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m (n + m))$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    ///
    /// let xs = vec![Float::ONE, Float::TWO, Float::from(3)];
    /// assert_eq!(Float::sum(xs.iter()).to_string(), "6.0");
    /// ```
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let xs: Vec<&Self> = xs.collect();
        sum_prec_round_helper(&xs, max_prec(xs.iter().copied()), Nearest).0
    }
}
