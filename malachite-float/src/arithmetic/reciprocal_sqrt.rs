// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::emulate_primitive_float_fn;
use crate::{Float, float_either_zero, float_infinity, float_nan, float_zero};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, Parity, PowerOf2, ReciprocalSqrt, ReciprocalSqrtAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::LIMB_HIGH_BIT;
use malachite_nz::natural::arithmetic::float_extras::{
    limbs_float_can_round, limbs_significand_slice_add_limb_in_place,
};
use malachite_nz::natural::arithmetic::float_reciprocal_sqrt::limbs_reciprocal_sqrt;
use malachite_nz::natural::{Natural, bit_to_limb_count_ceiling, limb_to_bit_count};
use malachite_nz::platform::Limb;

impl Float {
    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded reciprocal square root is
    /// less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_sqrt_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_sqrt_round`] instead. If both of these things are true, consider
    /// using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(5, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(5, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.59");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(5, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(20, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(20, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(20, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_round(mut self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let o = self.reciprocal_sqrt_prec_round_assign(prec, rm);
        (self, o)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded reciprocal
    /// square root is less than, equal to, or greater than the exact square root. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_sqrt_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_sqrt_round_ref`] instead. If both of these things are true,
    /// consider using `(&Float).reciprocal_sqrt()`instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(5, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(5, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.59");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(5, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(20, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(20, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(20, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    ///
    /// This is mpfr_mpn_rec_sqrt from rec_sqrt.c, MPFR 4.3.0.
    #[inline]
    pub fn reciprocal_sqrt_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_zero!(), Equal),
            float_either_zero!() => (float_infinity!(), Equal),
            Self(Finite {
                sign,
                exponent: x_exp,
                precision: x_prec,
                significand: x,
                ..
            }) => {
                if !sign {
                    return (float_nan!(), Equal);
                }
                // Let u = U*2^e, where e = EXP(u), and 1/2 <= U < 1. If e is even, we compute an
                // approximation of X of (4U)^{-1/2}, and the result is X*2^(-(e-2)/2) [case s=1].
                // If e is odd, we compute an approximation of X of (2U)^{-1/2}, and the result is
                // X*2^(-(e-1)/2) [case s=0].
                //
                // parity of the exponent of u
                let mut s = i32::from(x_exp.even());
                let in_len = bit_to_limb_count_ceiling(prec);
                // for the first iteration, if rp + 11 fits into rn limbs, we round up up to a full
                // limb to maximize the chance of rounding, while avoiding to allocate extra space
                let mut working_prec = max(prec + 11, limb_to_bit_count(in_len));
                let mut increment = Limb::WIDTH;
                let mut out;
                loop {
                    let working_limbs = bit_to_limb_count_ceiling(working_prec);
                    out = vec![0; working_limbs];
                    limbs_reciprocal_sqrt(
                        &mut out,
                        working_prec,
                        x.as_limbs_asc(),
                        *x_prec,
                        s == 1,
                    );
                    // If the input was not truncated, the error is at most one ulp; if the input
                    // was truncated, the error is at most two ulps (see algorithms.tex).
                    if limbs_float_can_round(
                        &mut out,
                        working_prec - u64::from(working_prec < *x_prec),
                        prec,
                        rm,
                    ) {
                        assert_ne!(rm, Exact, "Inexact float reciprocal square root");
                        break;
                    }
                    // We detect only now the exact case where u = 2 ^ (2e), to avoid slowing down
                    // the average case. This can happen only when the mantissa is exactly 1 / 2 and
                    // the exponent is odd.
                    if s == 0 && x.is_power_of_2() {
                        let pl = limb_to_bit_count(working_limbs) - working_prec;
                        // we should have x=111...111
                        limbs_significand_slice_add_limb_in_place(&mut out, Limb::power_of_2(pl));
                        *out.last_mut().unwrap() = LIMB_HIGH_BIT;
                        s = 2;
                        break;
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
                let reciprocal_sqrt = Self(Finite {
                    sign: true,
                    exponent: (s + 1 - x_exp) >> 1,
                    precision: working_prec,
                    significand: Natural::from_owned_limbs_asc(out),
                });
                Float::from_float_prec_round(reciprocal_sqrt, prec, rm)
            }
        }
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded reciprocal square root is less than, equal
    /// to, or greater than the exact square root. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |1/\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec(5);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec(20);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec(self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_sqrt_prec_round(prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded reciprocal square root is less
    /// than, equal to, or greater than the exact square root. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |1/\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using `(&Float).reciprocal_sqrt()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_ref(5);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_ref(20);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_sqrt_prec_round_ref(prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded reciprocal square root is less than, equal to, or greater
    /// than the exact square root. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_sqrt_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round(Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547756");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round(Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round(Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round(prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded reciprocal square root is less than, equal to, or
    /// greater than the exact square root. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_sqrt_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).reciprocal_sqrt()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round_ref(Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547756");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round_ref(Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round_ref(Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_ref(prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`Float`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded reciprocal square root is less than, equal to, or greater
    /// than the exact square root. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_sqrt_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_sqrt_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_sqrt_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::reciprocal_sqrt_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.56");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.59");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.56");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.564189");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.56419");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.56419");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (reciprocal_sqrt, o) = self.reciprocal_sqrt_prec_round_ref(prec, rm);
        *self = reciprocal_sqrt;
        o
    }

    /// Computes the reciprocal of the square root of a [`Float`] in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded square root is less than, equal to, or greater than the exact square
    /// root. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |1/\sqrt{x}|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_sqrt_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_round_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.56");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.56419");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_assign(&mut self, prec: u64) -> Ordering {
        self.reciprocal_sqrt_prec_round_assign(prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`Float`] in place, rounding the result with
    /// the specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// reciprocal square root is less than, equal to, or greater than the exact square root.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |1/\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::reciprocal_sqrt_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_sqrt_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::reciprocal_sqrt_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.564189583547756");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.564189583547757");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.564189583547757");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_assign(prec, rm)
    }
}

impl ReciprocalSqrt for Float {
    type Output = Self;

    /// Computes the reciprocal of the square root of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal square
    /// root is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |1/\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_sqrt_round`]. If you want both of these things, consider
    /// using [`Float::reciprocal_sqrt_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalSqrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.reciprocal_sqrt().is_nan());
    /// assert_eq!(Float::INFINITY.reciprocal_sqrt(), Float::ZERO);
    /// assert!(Float::NEGATIVE_INFINITY.reciprocal_sqrt().is_nan());
    /// assert_eq!(Float::from(1.5).reciprocal_sqrt().to_string(), "0.8");
    /// assert!(Float::from(-1.5).reciprocal_sqrt().is_nan());
    /// ```
    #[inline]
    fn reciprocal_sqrt(self) -> Self {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round(prec, Nearest).0
    }
}

impl ReciprocalSqrt for &Float {
    type Output = Float;

    /// Computes the reciprocal of the square root of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal square
    /// root is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// f(x) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |1/\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_sqrt_round_ref`]. If you want both of these things,
    /// consider using [`Float::reciprocal_sqrt_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalSqrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).reciprocal_sqrt().is_nan());
    /// assert_eq!((&Float::INFINITY).reciprocal_sqrt(), Float::ZERO);
    /// assert!((&Float::NEGATIVE_INFINITY).reciprocal_sqrt().is_nan());
    /// assert_eq!((&Float::from(1.5)).reciprocal_sqrt().to_string(), "0.8");
    /// assert!((&Float::from(-1.5)).reciprocal_sqrt().is_nan());
    /// ```
    #[inline]
    fn reciprocal_sqrt(self) -> Float {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_ref(prec, Nearest).0
    }
}

impl ReciprocalSqrtAssign for Float {
    /// Computes the reciprocal of the square root of a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal square
    /// root is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// $$
    /// x\gets = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |1/\sqrt{x}|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::reciprocal_sqrt`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_sqrt_round_assign`]. If you want both of these things,
    /// consider using [`Float::reciprocal_sqrt_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalSqrtAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.reciprocal_sqrt_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.reciprocal_sqrt_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.reciprocal_sqrt_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x.reciprocal_sqrt_assign();
    /// assert_eq!(x.to_string(), "0.8");
    ///
    /// let mut x = Float::from(-1.5);
    /// x.reciprocal_sqrt_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn reciprocal_sqrt_assign(&mut self) {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_assign(prec, Nearest);
    }
}

/// Computes the reciprocal of the square root of a [`f32`]. Using this function is more accurate
/// than using `powf(0.5)` or taking the square root and then the reciprocal, or vice versa.
///
/// If the reciprocal square root is equidistant from two [`f32`]s, the [`f32`] with fewer 1s in its
/// binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
/// mode.
///
/// The reciprocal square root of any nonzero negative number is `NaN`.
///
/// $$
/// f(x) = 1/\sqrt{x}+\varepsilon.
/// $$
/// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |1/\sqrt{x}|\rfloor-p}$, where $p$ is precision of the output (typically 24, but less if the
///   output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$
/// - $f(-\infty)=\text{NaN}$
/// - $f(0.0)=\infty$
/// - $f(-0.0)=\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::reciprocal_sqrt::f32_reciprocal_sqrt;
///
/// assert!(f32_reciprocal_sqrt(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(f32_reciprocal_sqrt(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert!(f32_reciprocal_sqrt(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(f32_reciprocal_sqrt(3.0f32)),
///     NiceFloat(0.57735026)
/// );
/// assert!(f32_reciprocal_sqrt(-3.0f32).is_nan());
/// ```
pub fn f32_reciprocal_sqrt(x: f32) -> f32 {
    emulate_primitive_float_fn(|x, prec| x.reciprocal_sqrt_prec(prec).0, x)
}

/// Computes the reciprocal of the square root of a [`f64`]. Using this function is more accurate
/// than using `powf(0.5)` or taking the square root and then the reciprocal, or vice versa.
///
/// If the reciprocal square root is equidistant from two [`f64`]s, the [`f64`] with fewer 1s in its
/// binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding
/// mode.
///
/// The reciprocal square root of any nonzero negative number is `NaN`.
///
/// $$
/// f(x) = 1/\sqrt{x}+\varepsilon.
/// $$
/// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |1/\sqrt{x}|\rfloor-p}$, where $p$ is precision of the output (typically 53, but less if the
///   output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$
/// - $f(-\infty)=\text{NaN}$
/// - $f(0.0)=\infty$
/// - $f(-0.0)=\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::reciprocal_sqrt::f64_reciprocal_sqrt;
///
/// assert!(f64_reciprocal_sqrt(f64::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(f64_reciprocal_sqrt(f64::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert!(f64_reciprocal_sqrt(f64::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(f64_reciprocal_sqrt(3.0f64)),
///     NiceFloat(0.5773502691896257)
/// );
/// assert!(f64_reciprocal_sqrt(-3.0f64).is_nan());
/// ```
pub fn f64_reciprocal_sqrt(x: f64) -> f64 {
    emulate_primitive_float_fn(|x, prec| x.reciprocal_sqrt_prec(prec).0, x)
}
