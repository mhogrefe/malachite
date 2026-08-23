// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::emulate_float_slice_to_float_fn;
use crate::float::arithmetic::sum::max_prec;
use crate::{
    Float, float_infinity, float_nan, float_negative_infinity, float_negative_zero, float_zero,
};
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::iter::Product;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, NegAssign, PowerOf2, ShlRoundAssign, ShrRound,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// A shift so far out of range that `shl_round` saturates for any starting exponent, but which is
// comfortably within `i64`.
const SATURATING_SHIFT: i128 = 1 << 40;

// Apply the accumulated exponent offset to a rounded Float, saturating on overflow or underflow in
// the same round-then-check-range order as the rest of the library.
fn apply_shift(f: &mut Float, shift: i128, rm: RoundingMode) -> Ordering {
    let clamped = shift.clamp(const { -SATURATING_SHIFT }, SATURATING_SHIFT);
    f.shl_round_assign(i64::exact_from(clamped), rm)
}

// Force the sign positive and the exponent to 1, absorbing the true exponent into `drift`.
fn normalize(t: &mut Float, drift: &mut i128) {
    let Float(Finite { sign, exponent, .. }) = t else {
        unreachable!()
    };
    *sign = true;
    *drift += i128::from(*exponent) - 1;
    *exponent = 1;
}

// Truncate toward zero (which is sign-independent), then normalize.
fn truncate(x: &Float, working_prec: u64, exact_all: &mut bool, drift: &mut i128) -> Float {
    let (mut t, o) = Float::from_float_prec_round_ref(x, working_prec, Down);
    if o != Equal {
        *exact_all = false;
    }
    normalize(&mut t, drift);
    t
}

// The product of at least 3 finite nonzero `Float`s, whose sign is `sign`, rounded to `prec` bits
// with rounding mode `rm` (which must not be `Exact`; the caller handles that mode). Since a
// product cannot cancel, the result is computed by a truncated Ziv iteration: multiply the
// normalized significands at a working precision, rounding toward zero, and accept as soon as the
// one-sided error interval is known not to straddle a rounding boundary. The exponents are
// accumulated separately in an `i128` (the sum of up to `usize::MAX` exponents of absolute value at
// most $2^{30}$ fits comfortably), so intermediate overflow and underflow are impossible; the final
// exponent is applied with a single saturating shift.
fn product_of_regulars(
    xs: &[&Float],
    sign: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let n = xs.len();
    // Decompose each input as odd-significand times a power of 2, accumulating the powers of 2. The
    // bit length of the product of the odd parts is at least b_min (an odd number times an odd
    // number of bit lengths a and b has bit length at least a + b - 1).
    let mut exp_offset = 0i128;
    let mut b_min = 1u64;
    let mut odd_bits = Vec::with_capacity(n);
    for x in xs {
        let Float(Finite {
            exponent,
            significand,
            ..
        }) = x
        else {
            unreachable!()
        };
        let tz = significand.trailing_zeros().unwrap();
        let sig_len = significand.significant_bits();
        exp_offset += i128::from(*exponent) - i128::from(sig_len) + i128::from(tz);
        odd_bits.push((sig_len - tz, tz));
        b_min += sig_len - tz - 1;
    }
    // Rounding the magnitude with the negated mode agrees with rounding the negated value with the
    // original mode.
    let rm_mag = if sign { rm } else { -rm };
    if b_min <= prec + 1 {
        // The exact product of the odd parts has at most b_min + n' bits, where n' counts the
        // inputs with a nontrivial odd part, and n' <= b_min - 1; so the exact product is small and
        // can be computed directly. This path covers every input set whose product could be exactly
        // representable or exactly halfway between representable values.
        let g = Natural::product(xs.iter().zip(odd_bits.iter()).filter_map(|(x, &(b, tz))| {
            if b == 1 {
                None
            } else {
                let Float(Finite { significand, .. }) = x else {
                    unreachable!()
                };
                Some(significand >> tz)
            }
        }));
        let g_len = g.significant_bits();
        let (h, o_mag, h_shift) = if g_len > prec {
            let (h, o) = g.shr_round(g_len - prec, rm_mag);
            (h, o, i128::from(g_len - prec))
        } else {
            (g, Equal, 0)
        };
        // h has at most prec + 1 significant bits (prec plus a possible rounding carry), so this
        // conversion is exact and its exponent is small.
        let mut f = Float::from_natural_prec_round(h, prec, Exact).0;
        let mut o = if sign {
            o_mag
        } else {
            f.neg_assign();
            o_mag.reverse()
        };
        let o_shift = apply_shift(&mut f, exp_offset + h_shift, rm);
        if o_shift != Equal {
            o = o_shift;
        }
        return (f, o);
    }
    // The product of the odd parts is an odd number with more than prec + 1 bits, so it cannot be
    // exactly representable with prec bits, nor exactly halfway between two representable values. A
    // truncated Ziv iteration therefore terminates.
    let logn = u64::exact_from(n).ceiling_log_base_2();
    let mut working_prec = prec + logn + 5;
    let mut increment = Limb::WIDTH;
    loop {
        let mut drift = 0i128;
        let mut exact_all = true;
        let (first, rest) = xs.split_first().unwrap();
        let mut acc = truncate(first, working_prec, &mut exact_all, &mut drift);
        for x in rest {
            let t = truncate(x, working_prec, &mut exact_all, &mut drift);
            if acc.mul_prec_round_assign(t, working_prec, Down) != Equal {
                exact_all = false;
            }
            normalize(&mut acc, &mut drift);
        }
        let finish = |mut acc: Float, inexact_guaranteed: bool| {
            if !sign {
                acc.neg_assign();
            }
            let (mut f, mut o) = Float::from_float_prec_round(acc, prec, rm);
            if inexact_guaranteed {
                assert_ne!(o, Equal);
            }
            let o_shift = apply_shift(&mut f, drift, rm);
            if o_shift != Equal {
                o = o_shift;
            }
            (f, o)
        };
        if exact_all
            || float_can_round(
                acc.significand_ref().unwrap(),
                working_prec - (logn + 3),
                prec,
                rm_mag,
            )
        {
            // float_can_round is conservative: it refuses any approximation that is exactly
            // representable at the target precision (the ternary would be undecidable), so a
            // successful can_round implies the final rounding is inexact.
            return finish(acc, !exact_all);
        }
        if Float::from_float_prec_round_ref(&acc, prec, Floor).1 == Equal {
            // The truncated accumulator is exactly representable at the target precision, so
            // can_round can never succeed, no matter how large the working precision grows. But the
            // error is one-sided — the true magnitude strictly exceeds the accumulator — and
            // smaller than half an ulp of the target precision, so nudging the accumulator up by
            // less than an ulp of the working precision and rounding that yields the correctly
            // rounded value and ternary under every rounding mode.
            let bump = Float::power_of_2(-i64::exact_from(working_prec) - 1);
            acc.add_prec_round_assign(bump, working_prec + 2, Exact);
            return finish(acc, true);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// The product of a slice of `Float`s, with correct rounding: only a single rounding is performed.
// The `Exact` rounding mode is handled by computing with `Nearest` and panicking if the result is
// inexact.
fn product_prec_round_helper(xs: &[&Float], prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let n = xs.len();
    if n == 0 {
        return (Float::one_prec(prec), Equal);
    } else if n == 1 {
        return Float::from_float_prec_round_ref(xs[0], prec, rm);
    } else if n == 2 {
        return xs[0].mul_prec_round_ref_ref(xs[1], prec, rm);
    }
    // Check for special inputs. The sign of any zero or infinite result, like the sign of a regular
    // result, is the XOR of the signs of all the inputs.
    let mut sign = true;
    let mut any_zero = false;
    let mut any_inf = false;
    for x in xs {
        match x {
            float_nan!() => {
                return (float_nan!(), Equal);
            }
            float_infinity!() => {
                any_inf = true;
            }
            float_negative_infinity!() => {
                any_inf = true;
                sign.not_assign();
            }
            float_zero!() => {
                any_zero = true;
            }
            float_negative_zero!() => {
                any_zero = true;
                sign.not_assign();
            }
            Float(Finite { sign: s, .. }) => {
                if !s {
                    sign.not_assign();
                }
            }
        }
    }
    if any_inf {
        // Any zero times any infinity is NaN.
        return if any_zero {
            (float_nan!(), Equal)
        } else if sign {
            (float_infinity!(), Equal)
        } else {
            (float_negative_infinity!(), Equal)
        };
    }
    if any_zero {
        return if sign {
            (float_zero!(), Equal)
        } else {
            (float_negative_zero!(), Equal)
        };
    }
    // At this point every input is finite and nonzero.
    let (kernel_rm, exact) = if rm == Exact {
        (Nearest, true)
    } else {
        (rm, false)
    };
    let (f, o) = product_of_regulars(xs, sign, prec, kernel_rm);
    if exact {
        assert_eq!(o, Equal, "Inexact Float product");
    }
    (f, o)
}

impl Float {
    /// Computes the product of a slice of [`Float`]s, rounding the result to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded product is less than, equal to, or greater than the exact product.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// Only a single rounding is performed, no matter how many inputs there are: the result is the
    /// correctly-rounded exact product, with no intermediate rounding, overflow, or underflow. MPFR
    /// has no equivalent of this function.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, p, m) = \prod_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\prod_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\prod_ {i=0}^{n-1} x_i|\rfloor-p+1}$.
    /// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\prod_ {i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - The product of no [`Float`]s is 1.
    /// - If any input is `NaN`, or if the inputs include both a zero and an infinity, the product
    ///   is `NaN`.
    /// - Otherwise, if any input is infinite, the product is infinite; and if any input is a zero,
    ///   the product is a zero. In both cases, as for a regular product, the sign is negative if
    ///   and only if an odd number of the inputs are negative, negative zeros and negative
    ///   infinities included.
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
    /// If you know you'll be using `Nearest`, consider using [`Float::product_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::product_round`] instead. If both of these things are true, consider taking
    /// the product of an iterator with [`Product`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, p) = O(n (m + p) \log (m + p) \log\log (m + p) + p (\log p)^2 \log\log p)$
    ///
    /// $M(n, m, p) = O(n + (m + p) \log (m + p))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`, and $p$ is `prec`: the working precision starts
    /// at $p + \log n$ and, for adversarially boundary-hugging products, grows geometrically until
    /// the computation becomes exact at the total input size, with each round multiplying $n$
    /// truncated factors at the working precision; products whose odd parts are small enough to be
    /// exactly representable are instead computed exactly with a product tree.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact product is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [Float::from(3), Float::from(5), Float::from(7)];
    ///
    /// let (product, o) = Float::product_prec_round(&xs, 10, Floor);
    /// assert_eq!(product.to_string(), "105.00");
    /// assert_eq!(o, Equal);
    ///
    /// let (product, o) = Float::product_prec_round(&xs, 3, Floor);
    /// assert_eq!(product.to_string(), "96.0");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::product_prec_round(&xs, 3, Ceiling);
    /// assert_eq!(product.to_string(), "1.1e2");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::product_prec_round(&xs, 3, Nearest);
    /// assert_eq!(product.to_string(), "1.1e2");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn product_prec_round(xs: &[Self], prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let refs: Vec<&Self> = xs.iter().collect();
        product_prec_round_helper(&refs, prec, rm)
    }

    /// Computes the product of a slice of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// product is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// Only a single rounding is performed, no matter how many inputs there are: the result is the
    /// correctly-rounded exact product, with no intermediate rounding, overflow, or underflow. MPFR
    /// has no equivalent of this function.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, p) = \prod_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\prod_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\prod_ {i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// See [`Float::product_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider taking the product of an iterator with [`Product`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, p) = O(n (m + p) \log (m + p) \log\log (m + p) + p (\log p)^2 \log\log p)$
    ///
    /// $M(n, m, p) = O(n + (m + p) \log (m + p))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`, and $p$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [Float::from(3), Float::from(5), Float::from(7)];
    ///
    /// let (product, o) = Float::product_prec(&xs, 10);
    /// assert_eq!(product.to_string(), "105.00");
    /// assert_eq!(o, Equal);
    ///
    /// let (product, o) = Float::product_prec(&xs, 3);
    /// assert_eq!(product.to_string(), "1.1e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn product_prec(xs: &[Self], prec: u64) -> (Self, Ordering) {
        Self::product_prec_round(xs, prec, Nearest)
    }

    /// Computes the product of a slice of [`Float`]s, rounding the result with the specified
    /// rounding mode. The precision of the result is the maximum of the precisions of the inputs
    /// (or 1 if there are no inputs). An [`Ordering`] is also returned, indicating whether the
    /// rounded product is less than, equal to, or greater than the exact product. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// Only a single rounding is performed, no matter how many inputs there are: the result is the
    /// correctly-rounded exact product, with no intermediate rounding, overflow, or underflow. MPFR
    /// has no equivalent of this function.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}, m) = \prod_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\prod_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\prod_ {i=0}^{n-1} x_i|\rfloor-p+1}$, where $p$ is the
    ///   maximum precision of the inputs.
    /// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\prod_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the
    ///   maximum precision of the inputs.
    ///
    /// See [`Float::product_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// If you know you'll be using `Nearest`, consider taking the product of an iterator with
    /// [`Product`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact product is not exactly representable with the
    /// maximum of the precisions of the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [Float::from(3), Float::from(5), Float::from(7)];
    ///
    /// let (product, o) = Float::product_round(&xs, Floor);
    /// assert_eq!(product.to_string(), "96.0");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::product_round(&xs, Ceiling);
    /// assert_eq!(product.to_string(), "1.1e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn product_round(xs: &[Self], rm: RoundingMode) -> (Self, Ordering) {
        Self::product_prec_round(xs, max_prec(xs.iter()), rm)
    }
}

/// Computes the product of a slice of primitive floats, with a single rounding.
///
/// The result is correctly rounded to the nearest value: the product is computed as if in infinite
/// precision and rounded only once, at the end, no matter how many inputs there are. This includes
/// gradual underflow: results in the subnormal range are correctly rounded to their reduced
/// precisions. Intermediate overflow and underflow cannot occur.
///
/// $$
/// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i + \varepsilon.
/// $$
/// - If $\prod_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
///   assumed to be 0.
/// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\prod_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - The product of no floats is $1.0$.
/// - If any input is `NaN`, or if the inputs include both a zero and an infinity, the product is
///   `NaN`.
/// - Otherwise, if any input is infinite, the product is infinite; and if any input is a zero, the
///   product is a zero. In both cases, as for a regular product, the sign is negative if and only
///   if an odd number of the inputs are negative, negative zeros and negative infinities included.
///
/// If the result overflows, $\pm\infty$ is returned, and if it underflows, $\pm0.0$ is returned.
///
/// # Worst-case complexity
/// $T(n) = O(n^2 \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`: for adversarially
/// boundary-hugging products the working precision grows to the total input size, though typical
/// inputs are handled in linear time.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::product::primitive_float_product;
///
/// // A naive fold underflows to zero and stays there; the correctly-rounded product does not.
/// let xs = [1.0e-200f64, 1.0e-200, 1.0e300, 1.0e300];
/// assert_eq!(
///     NiceFloat(primitive_float_product(&xs)),
///     NiceFloat(1.0000000000000001e200)
/// );
/// assert_eq!(NiceFloat(xs.iter().product::<f64>()), NiceFloat(0.0));
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_product<T: PrimitiveFloat>(xs: &[T]) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_slice_to_float_fn(Float::product_prec, xs)
}

impl Product<Self> for Float {
    /// Multiplies together all the [`Float`]s in an iterator.
    ///
    /// The result has the maximum of the precisions of the inputs (or 1 if there are no inputs),
    /// and the product is rounded to nearest. Only a single rounding is performed, no matter how
    /// many inputs there are: the result is the correctly-rounded exact product, with no
    /// intermediate rounding, overflow, or underflow. MPFR has no equivalent of this function.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\prod_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\prod_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the maximum precision
    ///   of the inputs.
    ///
    /// See [`Float::product_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    ///
    /// let product = Float::product([Float::ONE, Float::TWO, Float::from(3)].into_iter());
    /// assert_eq!(product.to_string(), "6.0");
    ///
    /// // All twenty inputs have precision 2, so the result has precision 2, but only a single
    /// // rounding is performed at the end: the result is the correctly-rounded value of 3^20.
    /// let product = Float::product(vec![Float::from(3); 20].into_iter());
    /// assert_eq!(product.to_string(), "3.2e9");
    /// ```
    fn product<I>(xs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let xs: Vec<Self> = xs.collect();
        let refs: Vec<&Self> = xs.iter().collect();
        product_prec_round_helper(&refs, max_prec(xs.iter()), Nearest).0
    }
}

impl<'a> Product<&'a Self> for Float {
    /// Multiplies together all the [`Float`]s in an iterator of [`Float`] references.
    ///
    /// The result has the maximum of the precisions of the inputs (or 1 if there are no inputs),
    /// and the product is rounded to nearest. Only a single rounding is performed, no matter how
    /// many inputs there are: the result is the correctly-rounded exact product, with no
    /// intermediate rounding, overflow, or underflow. MPFR has no equivalent of this function.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i + \varepsilon.
    /// $$
    /// - If $\prod_ {i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\prod_ {i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\prod_ {i=0}^{n-1} x_i|\rfloor-p}$, where $p$ is the maximum precision
    ///   of the inputs.
    ///
    /// See [`Float::product_prec_round`] for a description of the special cases and of overflow and
    /// underflow behavior.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n m \log m \log\log m + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `u64::sum(xs.map(Float::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    ///
    /// let xs = vec![Float::ONE, Float::TWO, Float::from(3)];
    /// assert_eq!(Float::product(xs.iter()).to_string(), "6.0");
    /// ```
    fn product<I>(xs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let xs: Vec<&Self> = xs.collect();
        product_prec_round_helper(&xs, max_prec(xs.iter().copied()), Nearest).0
    }
}
