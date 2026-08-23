// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::sum::{complete_sum_result, max_prec, update_zero_sign};
use crate::{
    Float, float_infinity, float_nan, float_negative_infinity, float_negative_zero, float_zero,
};
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::sum::{FloatSumInput, sum_float_significands};

// A Float's finite fields: the sign, the exponent, the precision, and a reference to the
// significand.
fn parts(f: &Float) -> (bool, i32, u64, &Natural) {
    let Float(Finite {
        sign,
        exponent,
        precision,
        significand,
    }) = f
    else {
        unreachable!()
    };
    (*sign, *exponent, *precision, significand)
}

impl Float {
    /// Computes the dot product of two equal-length slices of [`Float`]s, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is also
    /// returned, indicating whether the rounded dot product is less than, equal to, or greater
    /// than the exact dot product. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are never rounded, and only a single rounding is performed, at the end: the
    /// result is the correctly-rounded exact dot product. Intermediate overflow and underflow
    /// cannot occur: each product is computed exactly at the significand level, with its exponent
    /// tracked over a range twice as wide as a [`Float`]'s, and only the final sum is subject to
    /// the exponent range check. (MPFR's `mpfr_dot`, which is documented as experimental,
    /// computes each product at full precision and requires those multiplications to be exact, so
    /// it does not handle inputs whose products leave the exponent range.)
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, (y_i)_ {i=0}^{n-1}, p, m) = \sum_ {i=0}^{n-1} x_i y_i +
    /// \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i y_i|\rfloor-p+1}$.
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i y_i|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Each term $x_iy_i$ follows the rules of [`Float`] multiplication, and the terms are then
    /// combined following the rules of [`Float`] addition:
    /// - The dot product of empty slices is $0.0$.
    /// - If any term is a `NaN` — because an input is `NaN`, or because a zero is paired with an
    ///   infinity — the dot product is `NaN`.
    /// - If two infinite terms have different signs, the dot product is `NaN`. Otherwise, if any
    ///   term is infinite, the dot product is an infinity of that sign.
    /// - If every term is a zero and all the terms have the same sign, the dot product is a zero
    ///   of that sign. If they do not all have the same sign, the dot product is $0.0$, unless
    ///   $m$ is `Floor`, in which case it is $-0.0$.
    /// - If some terms are nonzero but the exact dot product is zero, the dot product is $0.0$,
    ///   unless $m$ is `Floor`, in which case it is $-0.0$.
    ///
    /// Overflow and underflow:
    /// - If $f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)\geq 2^{2^{30}-1}$ and $m$ is
    ///   `Ceiling`, `Up`, or `Nearest`, $\infty$ is returned instead.
    /// - If $f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`
    ///   or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead.
    /// - If $f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`,
    ///   `Up`, or `Nearest`, $-\infty$ is returned instead.
    /// - If $f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)\leq -2^{2^{30}-1}$ and $m$ is
    ///   `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead.
    /// - If $0<f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)<2^{-2^{30}}$, and $m$ is `Floor` or
    ///   `Down`, $0.0$ is returned instead.
    /// - If $0<f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)<2^{-2^{30}}$, and $m$ is `Ceiling`
    ///   or `Up`, $2^{-2^{30}}$ is returned instead.
    /// - If $0<f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)\leq2^{-2^{30}-1}$, and $m$ is
    ///   `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)<2^{-2^{30}}$, and $m$ is
    ///   `Nearest`, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)<0$, and $m$ is `Ceiling`
    ///   or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)<0$, and $m$ is `Floor` or
    ///   `Up`, $-2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)<0$, and $m$ is
    ///   `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f((x_i)_ {i=0}^{n-1},(y_i)_ {i=0}^{n-1},p,m)<-2^{-2^{30}-1}$, and $m$
    ///   is `Nearest`, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::dot_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::dot_round`] instead. If both of these things are true, consider using
    /// [`Float::dot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, p) = O(n + m (n + p) + m \log m \log\log m)$
    ///
    /// $M(n, m, p) = O(n + p + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, $m$ is the sum of the
    /// significant bits of the elements of `xs` and `ys`, and $p$ is `prec`: each term is an
    /// exact significand product (mul-class in the pair's bits), and the terms then feed the
    /// summation kernel, which inherits the summation bound.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `xs` and `ys` have different lengths, or if `rm` is `Exact`
    /// and the exact dot product is not exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [Float::ONE, Float::TWO, Float::from(3)];
    /// let ys = [Float::from(4), Float::from(5), Float::from(6)];
    /// let (dot, o) = Float::dot_prec_round(&xs, &ys, 10, Floor);
    /// assert_eq!(dot.to_string(), "32.000");
    /// assert_eq!(o, Equal);
    ///
    /// // 0.25 * 0.25 + 2 * 5 = 10.0625
    /// let xs = [Float::power_of_2(-2i64), Float::TWO];
    /// let ys = [Float::power_of_2(-2i64), Float::from(5)];
    /// let (dot, o) = Float::dot_prec_round(&xs, &ys, 3, Floor);
    /// assert_eq!(dot.to_string(), "10.0");
    /// assert_eq!(o, Less);
    ///
    /// let (dot, o) = Float::dot_prec_round(&xs, &ys, 3, Ceiling);
    /// assert_eq!(dot.to_string(), "12.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn dot_prec_round(
        xs: &[Self],
        ys: &[Self],
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        // The dot product is correctly rounded: the products are never rounded, and only a
        // single rounding is performed, at the end. Unlike MPFR's experimental `mpfr_dot`,
        // intermediate overflow and underflow cannot occur: each product is computed exactly at
        // the significand level, with its exponent tracked in an `i64` (twice the `Float`
        // exponent range fits comfortably), and only the final sum is subject to the exponent
        // range check.
        //
        // The `Exact` rounding mode is handled by computing with `Nearest` and panicking if the
        // result is inexact.
        assert_ne!(prec, 0);
        assert_eq!(
            xs.len(),
            ys.len(),
            "dot product requires slices of equal length"
        );
        let n = xs.len();
        if n == 0 {
            return (float_zero!(), Equal);
        } else if n == 1 {
            return xs[0].mul_prec_round_ref_ref(&ys[0], prec, rm);
        }
        // Classify each term x * y according to the multiplication rules, then combine the terms
        // according to the addition rules, determining the sign of an infinite result, the sign
        // of an all-zero result, and the regular terms.
        let mut sign_inf = 0i8;
        let mut sign_zero = 0i8;
        let mut regulars: Vec<(&Self, &Self)> = Vec::new();
        for (x, y) in xs.iter().zip(ys.iter()) {
            if x.is_nan() || y.is_nan() {
                return (float_nan!(), Equal);
            }
            let term_sign = if x.is_sign_negative() == y.is_sign_negative() {
                1
            } else {
                -1
            };
            if x.is_infinite() || y.is_infinite() {
                if *x == 0u32 || *y == 0u32 {
                    // A zero times an infinity is NaN.
                    return (float_nan!(), Equal);
                }
                if sign_inf == 0 {
                    sign_inf = term_sign;
                } else if sign_inf != term_sign {
                    // Infinite terms of opposite signs add to NaN.
                    return (float_nan!(), Equal);
                }
            } else if *x == 0u32 || *y == 0u32 {
                if regulars.is_empty() {
                    // This choice is sticky when new zeros are considered.
                    update_zero_sign(&mut sign_zero, term_sign, rm);
                }
            } else {
                regulars.push((x, y));
            }
        }
        // At this point the result cannot be NaN.
        if sign_inf != 0 {
            return if sign_inf > 0 {
                (float_infinity!(), Equal)
            } else {
                (float_negative_infinity!(), Equal)
            };
        }
        // At this point every term is finite.
        if regulars.is_empty() {
            // All the terms were zeros (and there is at least one). The dot product is zero with
            // sign sign_zero.
            assert_ne!(sign_zero, 0);
            return if sign_zero > 0 {
                (float_zero!(), Equal)
            } else {
                (float_negative_zero!(), Equal)
            };
        }
        // Optimize the case where there are only one or two regular terms, delegating to the
        // correctly-rounded multiplication and fused multiply-add-multiply.
        if regulars.len() == 1 {
            return regulars[0]
                .0
                .mul_prec_round_ref_ref(regulars[0].1, prec, rm);
        } else if regulars.len() == 2 {
            return regulars[0].0.mul_add_mul_prec_round_ref_ref_ref_ref(
                regulars[0].1,
                regulars[1].0,
                regulars[1].1,
                prec,
                rm,
            );
        }
        let (kernel_rm, exact) = if rm == Exact {
            (Nearest, true)
        } else {
            (rm, false)
        };
        // Compute each product exactly at the significand level. A Float's significand is stored
        // limb-aligned with its top bit set, and its value is significand * 2^(exponent - 64 *
        // len); the product of two such significands has its top bit either exactly at the
        // combined width (in which case the product is already aligned) or one position below it
        // (in which case a shift by 1 restores the alignment and the exponent decreases by 1).
        let terms: Vec<(bool, i64, u64, Natural)> = regulars
            .iter()
            .map(|&(x, y)| {
                let (sx, ex, px, sig_x) = parts(x);
                let (sy, ey, py, sig_y) = parts(y);
                let mut s = sig_x * sig_y;
                let mut exp = i64::from(ex) + i64::from(ey);
                let full = sig_x.significant_bits() + sig_y.significant_bits();
                let mut term_prec = px + py;
                if s.significant_bits() < full {
                    s <<= 1;
                    exp -= 1;
                    term_prec -= 1;
                }
                (sx == sy, exp, term_prec, s)
            })
            .collect();
        let inputs: Vec<FloatSumInput> = terms
            .iter()
            .map(|(sign, exp, term_prec, s)| FloatSumInput {
                sign: *sign,
                exp: *exp,
                prec: *term_prec,
                significand: s,
            })
            .collect();
        complete_sum_result(
            sum_float_significands(&inputs, prec, kernel_rm),
            prec,
            rm,
            exact,
            "Inexact Float dot product",
        )
    }

    /// Computes the dot product of two equal-length slices of [`Float`]s, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded dot product is less than, equal to, or greater than the exact dot
    /// product. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The products are never rounded, and only a single rounding is performed, at the end: the
    /// result is the correctly-rounded exact dot product, and intermediate overflow and underflow
    /// cannot occur.
    ///
    /// If the dot product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, (y_i)_ {i=0}^{n-1}, p) = \sum_ {i=0}^{n-1} x_i y_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is finite and nonzero, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i y_i|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See [`Float::dot_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::dot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, p) = O(n + m (n + p) + m \log m \log\log m)$
    ///
    /// $M(n, m, p) = O(n + p + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, $m$ is the sum of the
    /// significant bits of the elements of `xs` and `ys`, and $p$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `xs` and `ys` have different lengths.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [Float::ONE, Float::TWO, Float::from(3)];
    /// let ys = [Float::from(4), Float::from(5), Float::from(6)];
    /// let (dot, o) = Float::dot_prec(&xs, &ys, 10);
    /// assert_eq!(dot.to_string(), "32.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (dot, o) = Float::dot_prec(&xs, &ys, 3);
    /// assert_eq!(dot.to_string(), "32.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn dot_prec(xs: &[Self], ys: &[Self], prec: u64) -> (Self, Ordering) {
        Self::dot_prec_round(xs, ys, prec, Nearest)
    }

    /// Computes the dot product of two equal-length slices of [`Float`]s, rounding the result
    /// with the specified rounding mode. The precision of the result is the maximum of the
    /// precisions of the inputs (or 1 if there are no inputs). An [`Ordering`] is also returned,
    /// indicating whether the rounded dot product is less than, equal to, or greater than the
    /// exact dot product. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are never rounded, and only a single rounding is performed, at the end: the
    /// result is the correctly-rounded exact dot product, and intermediate overflow and underflow
    /// cannot occur.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, (y_i)_ {i=0}^{n-1}, m) = \sum_ {i=0}^{n-1} x_i y_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i y_i|\rfloor-p+1}$, where $p$
    ///   is the maximum precision of the inputs.
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i y_i|\rfloor-p}$, where
    ///   $p$ is the maximum precision of the inputs.
    ///
    /// See [`Float::dot_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::dot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m (n + m))$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs` and `ys`.
    ///
    /// # Panics
    /// Panics if `xs` and `ys` have different lengths, or if `rm` is `Exact` and the exact dot
    /// product is not exactly representable with the maximum of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // 0.25 * 0.25 + 2 * 5 = 10.0625, whose inputs have maximum precision 3
    /// let xs = [Float::power_of_2(-2i64), Float::TWO];
    /// let ys = [Float::power_of_2(-2i64), Float::from(5)];
    /// let (dot, o) = Float::dot_round(&xs, &ys, Floor);
    /// assert_eq!(dot.to_string(), "10.0");
    /// assert_eq!(o, Less);
    ///
    /// let (dot, o) = Float::dot_round(&xs, &ys, Ceiling);
    /// assert_eq!(dot.to_string(), "12.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn dot_round(xs: &[Self], ys: &[Self], rm: RoundingMode) -> (Self, Ordering) {
        Self::dot_prec_round(xs, ys, max_prec(xs.iter().chain(ys.iter())), rm)
    }

    /// Computes the dot product of two equal-length slices of [`Float`]s. The precision of the
    /// result is the maximum of the precisions of the inputs (or 1 if there are no inputs), and
    /// the dot product is rounded to nearest.
    ///
    /// The products are never rounded, and only a single rounding is performed, at the end: the
    /// result is the correctly-rounded exact dot product, and intermediate overflow and underflow
    /// cannot occur.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, (y_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i y_i + \varepsilon.
    /// $$
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\sum_ {i=0}^{n-1} x_i y_i$ is finite and nonzero, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_ {i=0}^{n-1} x_i y_i|\rfloor-p}$, where $p$
    ///   is the maximum precision of the inputs.
    ///
    /// See [`Float::dot_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m (n + m))$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs` and `ys`.
    ///
    /// # Panics
    /// Panics if `xs` and `ys` have different lengths.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    ///
    /// let xs = [Float::ONE, Float::TWO, Float::from(3)];
    /// let ys = [Float::from(4), Float::from(5), Float::from(6)];
    /// assert_eq!(Float::dot(&xs, &ys).to_string(), "32.0");
    /// ```
    #[inline]
    pub fn dot(xs: &[Self], ys: &[Self]) -> Self {
        Self::dot_round(xs, ys, Nearest).0
    }
}
