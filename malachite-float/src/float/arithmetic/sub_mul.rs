// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float::arithmetic::add_mul::{
    add_mul_helper, add_mul_rational_helper, add_mul_val_helper,
};
use crate::{Float, emulate_float_float_float_to_float_fn, emulate_float_float_to_float_fn};
use core::cmp::{Ordering, max};
use malachite_base::max;
use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};
use malachite_q::Rational;

// This is mpfr_fms from fms.c, MPFR 4.2.2, up to a sign convention: mpfr_fms computes x * y - z by
// negating its addend, while Malachite's sub_mul computes self - y * z by negating the product, so
// mpfr_fms(x, y, z) = -sub_mul(z, x, y) with the rounding mode negated (an exact identity, since
// negation is exact).
impl Float {
    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. All three [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded diff is less than,
    /// equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_round(y.clone(), z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round(y.clone(), z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round(y.clone(), z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round(y.clone(), z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round(y.clone(), z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round(y.clone(), z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round(
        self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, &y, &z, true, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first two [`Float`]s are taken
    /// by value and the third by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_val_ref(y.clone(), &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_val_ref(y.clone(), &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_val_ref(y.clone(), &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_val_ref(y.clone(), &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_val_ref(y.clone(), &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_val_ref(y.clone(), &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_val_val_ref(
        self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, &y, z, true, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first and third [`Float`]s are
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_val(&y, z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_val(&y, z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_val(&y, z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_val(&y, z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_val(&y, z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_val(&y, z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_val_ref_val(
        self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, y, &z, true, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// value and the second and third by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_round_val_ref_ref(&y, &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_round_val_ref_ref(&y, &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_round_val_ref_ref(&y, &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_round_val_ref_ref(&y, &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_ref(&y, &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_prec_round_val_ref_ref(&y, &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_mul_prec_round_val_ref_ref(
        self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, y, z, true, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// reference and the second and third by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_val(y.clone(), z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_val(y.clone(), z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_val(y.clone(), z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_val(y.clone(), z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_val(y.clone(), z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_val(y.clone(), z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_ref_val_val(
        &self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, &y, &z, true, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first and third [`Float`]s are
    /// taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_ref(y.clone(), &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_ref(y.clone(), &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_ref(y.clone(), &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_ref(y.clone(), &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_ref(y.clone(), &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_val_ref(y.clone(), &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_ref_val_ref(
        &self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, &y, z, true, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The first two [`Float`]s are taken
    /// by reference and the third by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_val(&y, z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_val(&y, z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_val(&y, z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_val(&y, z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_val(&y, z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_val(&y, z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_ref_ref_val(
        &self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, y, &z, true, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. All three [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::sub_mul_round`] instead. If both of these things are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-0.719");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263767");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_mul_prec_round_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, y, z, true, prec, rm)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. Both [`Float`]s on the
    /// right-hand side are taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign(y.clone(), z.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-0.719");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign(y.clone(), z.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign(y.clone(), z.clone(), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, &y, &z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The first [`Float`] on the
    /// right-hand side is taken by value and the second by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_val_ref(y.clone(), &z, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-0.719");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_val_ref(y.clone(), &z, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_val_ref(y.clone(), &z, 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_assign_val_ref(
        &mut self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, &y, z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. The first [`Float`] on the
    /// right-hand side is taken by reference and the second by value. An [`Ordering`] is returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_ref_val(&y, z.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-0.719");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_ref_val(&y, z.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_ref_val(&y, z.clone(), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_round_assign_ref_val(
        &mut self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, y, &z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. Both [`Float`]s on the
    /// right-hand side are taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_round_assign_ref_ref(&y, &z, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "-0.719");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_ref_ref(&y, &z, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_prec_round_assign_ref_ref(&y, &z, 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.688");
    /// ```
    #[inline]
    pub fn sub_mul_prec_round_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, y, z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. All three [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec(y.clone(), z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec(y.clone(), z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec(self, y: Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The first two [`Float`]s are taken by value and
    /// the third by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_val_val_ref(y.clone(), &z, 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_val_val_ref(y.clone(), &z, 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_val_val_ref(self, y: Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round_val_val_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The first and third [`Float`]s are taken by value
    /// and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_val_ref_val(&y, z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_val_ref_val(&y, z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_val_ref_val(self, y: &Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round_val_ref_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] is taken by value and the
    /// second and third by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_val_ref_ref(&y, &z, 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_prec_val_ref_ref(&y, &z, 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_mul_prec_val_ref_ref(self, y: &Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round_val_ref_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] is taken by reference and the
    /// second and third by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_val_val(y.clone(), z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_val_val(y.clone(), z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_ref_val_val(&self, y: Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round_ref_val_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The first and third [`Float`]s are taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_val_ref(y.clone(), &z, 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_val_ref(y.clone(), &z, 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_ref_val_ref(&self, y: Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round_ref_val_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The first two [`Float`]s are taken by reference
    /// and the third by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_ref_val(&y, z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_ref_val(&y, z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_ref_ref_val(&self, y: &Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round_ref_ref_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result to the
    /// nearest value of the specified precision. All three [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z,p)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_ref_ref(&y, &z, 5);
    /// assert_eq!(diff.to_string(), "-0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_prec_ref_ref_ref(&y, &z, 20);
    /// assert_eq!(diff.to_string(), "-0.70263863");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_mul_prec_ref_ref_ref(&self, y: &Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.sub_mul_prec_round_ref_ref_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the nearest value of the specified precision. Both [`Float`]s on the right-hand side are
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign(y.clone(), z.clone(), 5), Greater);
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign(y.clone(), z.clone(), 20), Less);
    /// assert_eq!(x.to_string(), "-0.70263863");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_assign(&mut self, y: Self, z: Self, prec: u64) -> Ordering {
        self.sub_mul_prec_round_assign(y, z, prec, Nearest)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the nearest value of the specified precision. The first [`Float`] on the right-hand side is
    /// taken by value and the second by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign_val_ref(y.clone(), &z, 5), Greater);
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign_val_ref(y.clone(), &z, 20), Less);
    /// assert_eq!(x.to_string(), "-0.70263863");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_assign_val_ref(&mut self, y: Self, z: &Self, prec: u64) -> Ordering {
        self.sub_mul_prec_round_assign_val_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the nearest value of the specified precision. The first [`Float`] on the right-hand side is
    /// taken by reference and the second by value. An [`Ordering`] is returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign_ref_val(&y, z.clone(), 5), Greater);
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign_ref_val(&y, z.clone(), 20), Less);
    /// assert_eq!(x.to_string(), "-0.70263863");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_prec_assign_ref_val(&mut self, y: &Self, z: Self, prec: u64) -> Ordering {
        self.sub_mul_prec_round_assign_ref_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result to
    /// the nearest value of the specified precision. Both [`Float`]s on the right-hand side are
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign_ref_ref(&y, &z, 5), Greater);
    /// assert_eq!(x.to_string(), "-0.688");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_prec_assign_ref_ref(&y, &z, 20), Less);
    /// assert_eq!(x.to_string(), "-0.70263863");
    /// ```
    #[inline]
    pub fn sub_mul_prec_assign_ref_ref(&mut self, y: &Self, z: &Self, prec: u64) -> Ordering {
        self.sub_mul_prec_round_assign_ref_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. All three [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_round(y.clone(), z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().sub_mul_round(y.clone(), z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_round(y.clone(), z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round(self, y: Self, z: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round(y, z, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. The first two [`Float`]s are taken by value and the third by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_val_ref(y.clone(), &z, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_val_ref(y.clone(), &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_val_ref(y.clone(), &z, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_val_val_ref(
        self,
        y: Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_val_val_ref(y, z, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. The first and third [`Float`]s are taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_ref_val(&y, z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_ref_val(&y, z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_ref_val(&y, z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_val_ref_val(
        self,
        y: &Self,
        z: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_val_ref_val(y, z, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by value and the second and third by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_ref_ref(&y, &z, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_ref_ref(&y, &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().sub_mul_round_val_ref_ref(&y, &z, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sub_mul_round_val_ref_ref(
        self,
        y: &Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_val_ref_ref(y, z, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by reference and the second and third
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_val_val(y.clone(), z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_val_val(y.clone(), z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_val_val(y.clone(), z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_ref_val_val(
        &self,
        y: Self,
        z: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_ref_val_val(y, z, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. The first and third [`Float`]s are taken by reference and the
    /// second by value. An [`Ordering`] is also returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_val_ref(y.clone(), &z, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_val_ref(y.clone(), &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_val_ref(y.clone(), &z, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_ref_val_ref(
        &self,
        y: Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_ref_val_ref(y, z, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. The first two [`Float`]s are taken by reference and the third by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded diff is less than,
    /// equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_ref_val(&y, z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_ref_val(&y, z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_ref_val(&y, z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_ref_ref_val(
        &self,
        y: &Self,
        z: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_ref_ref_val(y, z, prec, rm)
    }

    /// Subtracts the product of two other [`Float`]s from a [`Float`], rounding the result with the
    /// specified rounding mode. All three [`Float`]s are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded diff is less than, equal to, or greater than
    /// the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is not
    ///   `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of the same sign and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_ref_ref(&y, &z, Floor);
    /// assert_eq!(diff.to_string(), "-0.70263837456932388");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_ref_ref(&y, &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_round_ref_ref_ref(&y, &z, Nearest);
    /// assert_eq!(diff.to_string(), "-0.70263837456932376");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sub_mul_round_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_ref_ref_ref(y, z, prec, rm)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result with
    /// the specified rounding mode. Both [`Float`]s on the right-hand side are taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_round_assign(y.clone(), z.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "-0.70263837456932388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_round_assign(y.clone(), z.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_round_assign(y.clone(), z.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_assign(&mut self, y: Self, z: Self, rm: RoundingMode) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_assign(y, z, prec, rm)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by value
    /// and the second by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_round_assign_val_ref(y.clone(), &z, Floor), Less);
    /// assert_eq!(x.to_string(), "-0.70263837456932388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_round_assign_val_ref(y.clone(), &z, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_round_assign_val_ref(y.clone(), &z, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_assign_val_ref(
        &mut self,
        y: Self,
        z: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_assign_val_ref(y, z, prec, rm)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by
    /// reference and the second by value. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_round_assign_ref_val(&y, z.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "-0.70263837456932388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_round_assign_ref_val(&y, z.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_round_assign_ref_val(&y, z.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_round_assign_ref_val(
        &mut self,
        y: &Self,
        z: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_assign_ref_val(y, z, prec, rm)
    }

    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, rounding the result with
    /// the specified rounding mode. Both [`Float`]s on the right-hand side are taken by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_round_assign_ref_ref(&y, &z, Floor), Less);
    /// assert_eq!(x.to_string(), "-0.70263837456932388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_round_assign_ref_ref(&y, &z, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_round_assign_ref_ref(&y, &z, Nearest), Greater);
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    pub fn sub_mul_round_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_round_assign_ref_ref(y, z, prec, rm)
    }
}

impl SubMul<Self, Self> for Float {
    type Output = Self;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking all three by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.sub_mul(y, z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: Self, z: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec(y, z, prec).0
    }
}

impl SubMul<Self, &Self> for Float {
    type Output = Self;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking the first two by
    /// value and the third by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.sub_mul(y, &z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: Self, z: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_val_val_ref(y, z, prec).0
    }
}

impl SubMul<&Self, Self> for Float {
    type Output = Self;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking the first and third
    /// by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.sub_mul(&y, z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Self, z: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_val_ref_val(y, z, prec).0
    }
}

impl SubMul<&Self, &Self> for Float {
    type Output = Self;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking the first by value
    /// and the second and third by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.sub_mul(&y, &z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Self, z: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_val_ref_ref(y, z, prec).0
    }
}

impl SubMul<Float, Float> for &Float {
    type Output = Float;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking the first by
    /// reference and the second and third by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.sub_mul(y, z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: Float, z: Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_ref_val_val(y, z, prec).0
    }
}

impl SubMul<Float, &Float> for &Float {
    type Output = Float;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking the first and third
    /// by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.sub_mul(y, &z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: Float, z: &Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_ref_val_ref(y, z, prec).0
    }
}

impl SubMul<&Float, Float> for &Float {
    type Output = Float;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking the first two by
    /// reference and the third by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.sub_mul(&y, z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Float, z: Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_ref_ref_val(y, z, prec).0
    }
}

impl SubMul<&Float, &Float> for &Float {
    type Output = Float;
    /// Subtracts the product of two other [`Float`]s from a [`Float`], taking all three by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=-0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of the same sign
    /// - $f(x,y,z)=0.0$ if $x=yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.sub_mul(&y, &z).to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Float, z: &Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_ref_ref_ref(y, z, prec).0
    }
}

impl SubMulAssign<Self, Self> for Float {
    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, both [`Float`]s on the
    /// right-hand side being taken by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.sub_mul_assign(y, z);
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: Self, z: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_assign(y, z, prec);
    }
}

impl SubMulAssign<Self, &Self> for Float {
    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, the first [`Float`] on
    /// the right-hand side being taken by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.sub_mul_assign(y, &z);
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: Self, z: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_assign_val_ref(y, z, prec);
    }
}

impl SubMulAssign<&Self, Self> for Float {
    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, the first [`Float`] on
    /// the right-hand side being taken by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.sub_mul_assign(&y, z);
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: &Self, z: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_assign_ref_val(y, z, prec);
    }
}

impl SubMulAssign<&Self, &Self> for Float {
    /// Subtracts the product of two [`Float`]s from a [`Float`] in place, both [`Float`]s on the
    /// right-hand side being taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.sub_mul_assign(&y, &z);
    /// assert_eq!(x.to_string(), "-0.70263837456932376");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: &Self, z: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.sub_mul_prec_assign_ref_ref(y, z, prec);
    }
}

impl Float {
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`]s and
    /// the [`Rational`] are all taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round(y.clone(), z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round(y.clone(), z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round(y.clone(), z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round(y.clone(), z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round(y.clone(), z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round(y.clone(), z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round(
        self,
        y: Self,
        z: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(&self, &y, &z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`]s are
    /// taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_val_val_ref(
        self,
        y: Self,
        z: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(&self, &y, z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The first [`Float`]
    /// and the [`Rational`] are taken by value and the second [`Float`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_val_ref_val(
        self,
        y: &Self,
        z: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(&self, y, &z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The first [`Float`]
    /// is taken by value and the second [`Float`] and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_ref(&y, &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_ref(&y, &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_ref(&y, &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_ref(&y, &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_ref(&y, &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_round_val_ref_ref(&y, &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_val_ref_ref(
        self,
        y: &Self,
        z: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(&self, y, z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The first [`Float`]
    /// is taken by reference and the second [`Float`] and the [`Rational`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_ref_val_val(
        &self,
        y: Self,
        z: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(self, &y, &z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The second [`Float`]
    /// is taken by value and the first [`Float`] and the [`Rational`] by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded diff is less than, equal to, or greater
    /// than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_ref_val_ref(
        &self,
        y: Self,
        z: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(self, &y, z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`]s are
    /// taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_ref_ref_val(
        &self,
        y: &Self,
        z: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(self, y, &z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`]s and
    /// the [`Rational`] are all taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::sub_mul_rational_round`] instead. If both of these things
    /// are true, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, 5, Floor);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, 5, Nearest);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, 20, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015732");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, 20, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_mul_rational_prec_round_ref_ref_ref(
        &self,
        y: &Self,
        z: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_rational_helper(self, y, z, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`]
    /// and the [`Rational`] on the right-hand side are both taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`Float::sub_mul_rational_round_assign`] instead. If both of these things are true, consider
    /// using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign(y.clone(), z.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign(y.clone(), z.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign(y.clone(), z.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_assign(
        &mut self,
        y: Self,
        z: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_rational_helper(self, &y, &z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] on
    /// the right-hand side is taken by value and the [`Rational`] by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`Float::sub_mul_rational_round_assign`] instead. If both of these things are true, consider
    /// using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_val_ref(y.clone(), &z, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_val_ref(y.clone(), &z, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_val_ref(y.clone(), &z, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_assign_val_ref(
        &mut self,
        y: Self,
        z: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_rational_helper(self, &y, z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] on
    /// the right-hand side is taken by reference and the [`Rational`] by value. An [`Ordering`] is
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`Float::sub_mul_rational_round_assign`] instead. If both of these things are true, consider
    /// using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_ref_val(&y, z.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_ref_val(&y, z.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_ref_val(&y, z.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_round_assign_ref_val(
        &mut self,
        y: &Self,
        z: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_rational_helper(self, y, &z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`]
    /// and the [`Rational`] on the right-hand side are both taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`Float::sub_mul_rational_round_assign`] instead. If both of these things are true, consider
    /// using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-subtract is not
    /// exactly representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_ref_ref(&y, &z, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_ref_ref(&y, &z, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_round_assign_ref_ref(&y, &z, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    /// ```
    #[inline]
    pub fn sub_mul_rational_prec_round_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_rational_helper(self, y, z, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`]s and the [`Rational`]
    /// are all taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.clone().sub_mul_rational_prec(y.clone(), z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().sub_mul_rational_prec(y.clone(), z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec(self, y: Self, z: Rational, prec: u64) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`]s are taken by value
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_val_val_ref(y.clone(), &z, 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_val_val_ref(y.clone(), &z, 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_val_val_ref(
        self,
        y: Self,
        z: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round_val_val_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The first [`Float`] and the
    /// [`Rational`] are taken by value and the second [`Float`] by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded diff is less than, equal to, or greater than
    /// the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_val_ref_val(&y, z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_prec_val_ref_val(&y, z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_val_ref_val(
        self,
        y: &Self,
        z: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round_val_ref_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The first [`Float`] is taken by
    /// value and the second [`Float`] and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.clone().sub_mul_rational_prec_val_ref_ref(&y, &z, 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().sub_mul_rational_prec_val_ref_ref(&y, &z, 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_val_ref_ref(
        self,
        y: &Self,
        z: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round_val_ref_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The first [`Float`] is taken by
    /// reference and the second [`Float`] and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_val_val(y.clone(), z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_val_val(y.clone(), z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_ref_val_val(
        &self,
        y: Self,
        z: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round_ref_val_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The second [`Float`] is taken by
    /// value and the first [`Float`] and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_val_ref(y.clone(), &z, 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_val_ref(y.clone(), &z, 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_ref_val_ref(
        &self,
        y: Self,
        z: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round_ref_val_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`]s are taken by
    /// reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_ref_val(&y, z.clone(), 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_ref_val(&y, z.clone(), 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_ref_ref_val(
        &self,
        y: &Self,
        z: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round_ref_ref_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`]s and the [`Rational`]
    /// are all taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=\text{NaN}$
    /// - $f(x,\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,p)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,p)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_ref_ref(&y, &z, 5);
    /// assert_eq!(diff.to_string(), "-5.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_prec_ref_ref_ref(&y, &z, 20);
    /// assert_eq!(diff.to_string(), "-5.4015808");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sub_mul_rational_prec_ref_ref_ref(
        &self,
        y: &Self,
        z: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.sub_mul_rational_prec_round_ref_ref_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] and the
    /// [`Rational`] on the right-hand side are both taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_assign(y.clone(), z.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_assign(y.clone(), z.clone(), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.4015808");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_assign(&mut self, y: Self, z: Rational, prec: u64) -> Ordering {
        self.sub_mul_rational_prec_round_assign(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] on the right-hand
    /// side is taken by value and the [`Rational`] by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_assign_val_ref(y.clone(), &z, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_assign_val_ref(y.clone(), &z, 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.4015808");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_assign_val_ref(
        &mut self,
        y: Self,
        z: &Rational,
        prec: u64,
    ) -> Ordering {
        self.sub_mul_rational_prec_round_assign_val_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] on the right-hand
    /// side is taken by reference and the [`Rational`] by value. An [`Ordering`] is returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_assign_ref_val(&y, z.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_prec_assign_ref_val(&y, z.clone(), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.4015808");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_prec_assign_ref_val(
        &mut self,
        y: &Self,
        z: Rational,
        prec: u64,
    ) -> Ordering {
        self.sub_mul_rational_prec_round_assign_ref_val(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] and the
    /// [`Rational`] on the right-hand side are both taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `max(self.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_rational_prec_assign_ref_ref(&y, &z, 5), Less);
    /// assert_eq!(x.to_string(), "-5.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_rational_prec_assign_ref_ref(&y, &z, 20), Less);
    /// assert_eq!(x.to_string(), "-5.4015808");
    /// ```
    #[inline]
    pub fn sub_mul_rational_prec_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Rational,
        prec: u64,
    ) -> Ordering {
        self.sub_mul_rational_prec_round_assign_ref_ref(y, z, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`]s and the [`Rational`] are all taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round(y.clone(), z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round(y.clone(), z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round(y.clone(), z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round(
        self,
        y: Self,
        z: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`]s are taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_val_ref(y.clone(), &z, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_val_ref(y.clone(), &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_val_ref(y.clone(), &z, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_val_val_ref(
        self,
        y: Self,
        z: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_val_val_ref(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The first [`Float`] and the [`Rational`] are taken
    /// by value and the second [`Float`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_ref_val(&y, z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_ref_val(&y, z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_ref_val(&y, z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_val_ref_val(
        self,
        y: &Self,
        z: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_val_ref_val(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The first [`Float`] is taken by value and the
    /// second [`Float`] and the [`Rational`] by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.clone().sub_mul_rational_round_val_ref_ref(&y, &z, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_ref_ref(&y, &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .sub_mul_rational_round_val_ref_ref(&y, &z, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_val_ref_ref(
        self,
        y: &Self,
        z: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_val_ref_ref(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The first [`Float`] is taken by reference and the
    /// second [`Float`] and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_val_val(y.clone(), z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_val_val(y.clone(), z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_val_val(y.clone(), z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_ref_val_val(
        &self,
        y: Self,
        z: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_ref_val_val(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The second [`Float`] is taken by value and the
    /// first [`Float`] and the [`Rational`] by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_val_ref(y.clone(), &z, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_val_ref(y.clone(), &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_val_ref(y.clone(), &z, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_ref_val_ref(
        &self,
        y: Self,
        z: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_ref_val_ref(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`]s are taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded diff
    /// is less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_ref_val(&y, z.clone(), Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_ref_val(&y, z.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_ref_val(&y, z.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_ref_ref_val(
        &self,
        y: &Self,
        z: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_ref_ref_val(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`]s and the [`Rational`] are all taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=\text{NaN}$
    /// - $f(x,\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z,m)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z,m)=0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`sub_mul`](malachite_base::num::arithmetic::traits::SubMul::sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_ref_ref(&y, &z, Floor);
    /// assert_eq!(diff.to_string(), "-5.4015788072814921");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_ref_ref(&y, &z, Ceiling);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.sub_mul_rational_round_ref_ref_ref(&y, &z, Nearest);
    /// assert_eq!(diff.to_string(), "-5.4015788072814912");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sub_mul_rational_round_ref_ref_ref(
        &self,
        y: &Self,
        z: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_ref_ref_ref(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result with the specified rounding mode. The [`Float`] and the [`Rational`] on the
    /// right-hand side are both taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign(y.clone(), z.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814921");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign(y.clone(), z.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign(y.clone(), z.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_assign(
        &mut self,
        y: Self,
        z: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_assign(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result with the specified rounding mode. The [`Float`] on the right-hand side is taken
    /// by value and the [`Rational`] by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_val_ref(y.clone(), &z, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814921");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_val_ref(y.clone(), &z, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_val_ref(y.clone(), &z, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_assign_val_ref(
        &mut self,
        y: Self,
        z: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_assign_val_ref(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result with the specified rounding mode. The [`Float`] on the right-hand side is taken
    /// by reference and the [`Rational`] by value. An [`Ordering`] is returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_ref_val(&y, z.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814921");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_ref_val(&y, z.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_ref_val(&y, z.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sub_mul_rational_round_assign_ref_val(
        &mut self,
        y: &Self,
        z: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_assign_ref_val(y, z, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place, rounding
    /// the result with the specified rounding mode. The [`Float`] and the [`Rational`] on the
    /// right-hand side are both taken by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// The precision of the output is the maximum of the precisions of the input [`Float`]s. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p+1}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    /// - If $x-yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input
    ///   [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`sub_mul_assign`](malachite_base::num::arithmetic::traits::SubMulAssign::sub_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the input [`Float`]s is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_mul_rational_round_assign_ref_ref(&y, &z, Floor), Less);
    /// assert_eq!(x.to_string(), "-5.4015788072814921");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_ref_ref(&y, &z, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_mul_rational_round_assign_ref_ref(&y, &z, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    pub fn sub_mul_rational_round_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_round_assign_ref_ref(y, z, prec, rm)
    }
}

impl SubMul<Self, Rational> for Float {
    type Output = Self;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking all
    /// three by value.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(x.sub_mul(y, z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: Self, z: Rational) -> Self {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec(y, z, prec).0
    }
}

impl SubMul<Self, &Rational> for Float {
    type Output = Self;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking the
    /// [`Float`]s by value and the [`Rational`] by reference.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(x.sub_mul(y, &z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: Self, z: &Rational) -> Self {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_val_val_ref(y, z, prec).0
    }
}

impl SubMul<&Self, Rational> for Float {
    type Output = Self;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking the
    /// first [`Float`] and the [`Rational`] by value and the second [`Float`] by reference.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(x.sub_mul(&y, z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Self, z: Rational) -> Self {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_val_ref_val(y, z, prec).0
    }
}

impl SubMul<&Self, &Rational> for Float {
    type Output = Self;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking the
    /// first [`Float`] by value and the second [`Float`] and the [`Rational`] by reference.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(x.sub_mul(&y, &z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Self, z: &Rational) -> Self {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_val_ref_ref(y, z, prec).0
    }
}

impl SubMul<Float, Rational> for &Float {
    type Output = Float;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking the
    /// first [`Float`] by reference and the second [`Float`] and the [`Rational`] by value.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(&x.sub_mul(y, z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: Float, z: Rational) -> Float {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_ref_val_val(y, z, prec).0
    }
}

impl SubMul<Float, &Rational> for &Float {
    type Output = Float;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking the
    /// second [`Float`] by value and the first [`Float`] and the [`Rational`] by reference.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(&x.sub_mul(y, &z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: Float, z: &Rational) -> Float {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_ref_val_ref(y, z, prec).0
    }
}

impl SubMul<&Float, Rational> for &Float {
    type Output = Float;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking the
    /// [`Float`]s by reference and the [`Rational`] by value.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(&x.sub_mul(&y, z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Float, z: Rational) -> Float {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_ref_ref_val(y, z, prec).0
    }
}

impl SubMul<&Float, &Rational> for &Float {
    type Output = Float;
    /// Subtracts the product of a [`Float`] and a [`Rational`] from another [`Float`], taking all
    /// three by reference.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=\text{NaN}$
    /// - $f(x,\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(\infty,y,z)=\infty$ if $y$ is not `NaN` and $yz\neq\infty$
    /// - $f(-\infty,y,z)=-\infty$ if $y$ is not `NaN` and $yz\neq-\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=-\infty$
    /// - If $x$ and the product $yz$ are both zeros, the sign rules of [`Float`] addition apply to
    ///   $x$ and $-yz$; the product is a zero whose sign is the XOR of the signs of $y$ and $z$, a
    ///   zero [`Rational`] counting as positive.
    /// - $f(x,y,z)=0.0$ if $x=yz$ and $x$ is finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::sub_mul_rational_prec`]. If you want both of these things, consider using
    /// [`Float::sub_mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// assert_eq!(&x.sub_mul(&y, &z).to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul(self, y: &Float, z: &Rational) -> Float {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_ref_ref_ref(y, z, prec).0
    }
}

impl SubMulAssign<Self, Rational> for Float {
    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place. The
    /// [`Float`] and the [`Rational`] on the right-hand side are both taken by value.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::sub_mul_rational_prec_assign`]. If you want both of these things,
    /// consider using [`Float::sub_mul_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// x.sub_mul_assign(y, z);
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: Self, z: Rational) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_assign(y, z, prec);
    }
}

impl SubMulAssign<Self, &Rational> for Float {
    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place. The
    /// [`Float`] on the right-hand side is taken by value and the [`Rational`] by reference.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::sub_mul_rational_prec_assign`]. If you want both of these things,
    /// consider using [`Float::sub_mul_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// x.sub_mul_assign(y, &z);
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: Self, z: &Rational) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_assign_val_ref(y, z, prec);
    }
}

impl SubMulAssign<&Self, Rational> for Float {
    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place. The
    /// [`Float`] on the right-hand side is taken by reference and the [`Rational`] by value.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::sub_mul_rational_prec_assign`]. If you want both of these things,
    /// consider using [`Float::sub_mul_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// x.sub_mul_assign(&y, z);
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: &Self, z: Rational) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_assign_ref_val(y, z, prec);
    }
}

impl SubMulAssign<&Self, &Rational> for Float {
    /// Subtracts the product of a [`Float`] and a [`Rational`] from a [`Float`] in place. The
    /// [`Float`] and the [`Rational`] on the right-hand side are both taken by reference.
    ///
    /// The [`Rational`] multiplicand enters the product exactly: it is never rounded to a [`Float`]
    /// first, so the result is the true value of $x-yz$ with a single rounding at the end. Rounding
    /// the [`Rational`] first would perturb the result by $y$ times the conversion error.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-yz+\varepsilon.
    /// $$
    /// - If $x-yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x-yz|\rfloor-p}$, where $p$ is the maximum precision of the input [`Float`]s.
    ///
    /// See the [`Float::sub_mul_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::sub_mul_rational_prec_assign`]. If you want both of these things,
    /// consider using [`Float::sub_mul_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Rational::from_signeds(22, 7);
    /// x.sub_mul_assign(&y, &z);
    /// assert_eq!(x.to_string(), "-5.4015788072814912");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: &Self, z: &Rational) {
        let prec = max(self.significant_bits(), y.significant_bits());
        self.sub_mul_rational_prec_assign_ref_ref(y, z, prec);
    }
}

/// Subtracts the product of two primitive floats from another primitive float with a single
/// rounding, using emulated [`Float`] arithmetic.
///
/// This is a correctly-rounded fused multiply-subtract: the product is not rounded before the
/// subtraction, so the result is the true value of $x-yz$ rounded once to the nearest representable
/// value. It agrees with the standard library's hardware-backed `mul_add` with the multiplicand
/// negated, up to argument order.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, PI, SQRT_2};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sub_mul::*;
///
/// assert_eq!(
///     NiceFloat(primitive_float_sub_mul(PI, E, SQRT_2)),
///     NiceFloat(-0.7026383745693238)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_sub_mul<T: PrimitiveFloat>(x: T, y: T, z: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_float_to_float_fn(Float::sub_mul_prec, x, y, z)
}

/// Subtracts the product of a primitive float and a [`Rational`] from another primitive float, with
/// a single rounding, using emulated [`Float`] arithmetic.
///
/// The [`Rational`] multiplicand enters the product exactly, and the result is the true value of
/// $x-yz$ rounded once to the nearest representable value.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `z.significant_bits()`.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, PI};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sub_mul::*;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_sub_mul_rational(
///         PI,
///         E,
///         &Rational::from_signeds(22, 7)
///     )),
///     NiceFloat(-5.401578807281491)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_sub_mul_rational<T: PrimitiveFloat>(x: T, y: T, z: &Rational) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(
        |x, y, prec| x.sub_mul_rational_prec_val_val_ref(y, z, prec),
        x,
        y,
    )
}
