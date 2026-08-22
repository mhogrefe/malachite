// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2016-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float::arithmetic::mul_add_mul::{mul_add_mul_helper, mul_add_mul_rational_helper};
use crate::{
    Float, emulate_float_float_float_float_to_float_fn, emulate_float_float_float_to_float_fn,
};
use core::cmp::Ordering;
use malachite_base::max;
use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};
use malachite_q::Rational;

// This is mpfr_fms from fmma.c, MPFR 4.2.2: mul_sub_mul computes a * b - c * d, which is mpfr_fmms
// exactly -- unlike sub_mul, no sign convention differs between the two libraries.
impl Float {
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. All four [`Float`]s
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_round(y.clone(), z.clone(), w.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round(y.clone(), z.clone(), w.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round(y.clone(), z.clone(), w.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round(y.clone(), z.clone(), w.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round(y.clone(), z.clone(), w.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round(y.clone(), z.clone(), w.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round(
        self,
        y: Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, &z, &w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. The first three
    /// [`Float`]s are taken by value and the fourth by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, &z, w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. The third [`Float`]
    /// is taken by reference and the others by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, z, &w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. The first two
    /// [`Float`]s are taken by value and the last two by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, z, w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. The second
    /// [`Float`] is taken by reference and the others by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, &z, &w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. The second and
    /// fourth [`Float`]s are taken by reference and the others by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, &z, w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. The second and
    /// third [`Float`]s are taken by reference and the others by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, z, &w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. The first [`Float`]
    /// is taken by value and the others by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, z, w, true, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the specified precision and with the specified rounding mode; the products are
    /// not rounded before the final subtraction, so there is a single rounding. All four [`Float`]s
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_sub_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "7.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "7.5594711");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(self, y, z, w, true, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, &z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by reference
    /// and the others by value. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(y.clone(), z.clone(), &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(y.clone(), z.clone(), &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_val_ref(y.clone(), z.clone(), &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, &z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(y.clone(), &z, w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(y.clone(), &z, w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_val(y.clone(), &z, w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(&y, z.clone(), w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(&y, z.clone(), w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_val(&y, z.clone(), w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, &z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, &z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by value and
    /// the others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded diff is less than,
    /// equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. All four [`Float`]s are taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec(y.clone(), z.clone(), w.clone(), 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec(y.clone(), z.clone(), w.clone(), 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec(self, y: Self, z: Self, w: Self, prec: u64) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. The first three [`Float`]s are
    /// taken by value and the fourth by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_val_val_ref(y.clone(), z.clone(), &w, 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_val_val_ref(y.clone(), z.clone(), &w, 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_val_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. The third [`Float`] is taken by
    /// reference and the others by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_val_ref_val(y.clone(), &z, w.clone(), 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_val_ref_val(y.clone(), &z, w.clone(), 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_val_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. The first two [`Float`]s are
    /// taken by value and the last two by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_val_ref_ref(y.clone(), &z, &w, 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_val_ref_ref(y.clone(), &z, &w, 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_val_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. The second [`Float`] is taken
    /// by reference and the others by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_ref_val_val(&y, z.clone(), w.clone(), 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_ref_val_val(&y, z.clone(), w.clone(), 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_val_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. The second and fourth
    /// [`Float`]s are taken by reference and the others by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_ref_val_ref(&y, z.clone(), &w, 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_ref_val_ref(&y, z.clone(), &w, 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_val_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. The second and third [`Float`]s
    /// are taken by reference and the others by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_ref_ref_val(&y, &z, w.clone(), 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_prec_val_ref_ref_val(&y, &z, w.clone(), 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_val_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. The first [`Float`] is taken by
    /// value and the others by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_prec_val_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_prec_val_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_val_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result to the nearest value of the specified precision; the products are not rounded
    /// before the final subtraction, so there is a single rounding. All four [`Float`]s are taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_ref_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(diff.to_string(), "7.50");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_prec_ref_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(diff.to_string(), "7.5594788");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_prec_round_ref_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign(y.clone(), z.clone(), w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign(y.clone(), z.clone(), w.clone(), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign(&mut self, y: Self, z: Self, w: Self, prec: u64) -> Ordering {
        self.mul_sub_mul_prec_round_assign(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by reference and the
    /// others by value. An [`Ordering`] is returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_prec_round_assign_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_prec_round_assign_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_ref(y.clone(), &z, &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_val_ref_ref(y.clone(), &z, &w, 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_prec_round_assign_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_prec_round_assign_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_ref(&y, z.clone(), &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_val_ref(&y, z.clone(), &w, 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_prec_round_assign_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_ref_val(&y, &z, w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_ref_val(&y, &z, w.clone(), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_prec_round_assign_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_sub_mul_prec_assign_ref_ref_ref(&y, &z, &w, 5), Less);
    /// assert_eq!(x.to_string(), "7.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_prec_assign_ref_ref_ref(&y, &z, &w, 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594788");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_prec_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_prec_round_assign_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. All four [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round(y.clone(), z.clone(), w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round(y.clone(), z.clone(), w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round(y.clone(), z.clone(), w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round(
        self,
        y: Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. The first three [`Float`]s are taken by value
    /// and the fourth by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_val_val_ref(y.clone(), z.clone(), &w, Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_val_val_ref(y.clone(), z.clone(), &w, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_val_val_ref(y.clone(), z.clone(), &w, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_val_val_val_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. The third [`Float`] is taken by reference and
    /// the others by value. An [`Ordering`] is also returned, indicating whether the rounded diff
    /// is less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_val_ref_val(y.clone(), &z, w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_val_ref_val(y.clone(), &z, w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_val_ref_val(y.clone(), &z, w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_val_val_ref_val(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. The first two [`Float`]s are taken by value and
    /// the last two by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_val_ref_ref(y.clone(), &z, &w, Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_val_ref_ref(y.clone(), &z, &w, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_val_ref_ref(y.clone(), &z, &w, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_val_val_ref_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. The second [`Float`] is taken by reference and
    /// the others by value. An [`Ordering`] is also returned, indicating whether the rounded diff
    /// is less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_ref_val_val(&y, z.clone(), w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_ref_val_val(&y, z.clone(), w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_round_val_ref_val_val(&y, z.clone(), w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_val_ref_val_val(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. The second and fourth [`Float`]s are taken by
    /// reference and the others by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_val_ref(&y, z.clone(), &w, Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_val_ref(&y, z.clone(), &w, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_val_ref(&y, z.clone(), &w, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_val_ref_val_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. The second and third [`Float`]s are taken by
    /// reference and the others by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_ref_val(&y, &z, w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_ref_val(&y, &z, w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_ref_val(&y, &z, w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_val_ref_ref_val(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. The first [`Float`] is taken by value and the
    /// others by reference. An [`Ordering`] is also returned, indicating whether the rounded diff
    /// is less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_round_val_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_val_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of one pair of [`Float`]s from the product of another pair, rounding
    /// the result with the specified rounding mode; the products are not rounded before the final
    /// subtraction, so there is a single rounding. All four [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_sub_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (diff, o) = x.mul_sub_mul_round_ref_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_round_ref_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(diff.to_string(), "7.5594760792050195");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.mul_sub_mul_round_ref_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(diff.to_string(), "7.5594760792050186");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_ref_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign(y.clone(), z.clone(), w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign(y.clone(), z.clone(), w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign(y.clone(), z.clone(), w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_val_ref(y.clone(), z.clone(), &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_val_ref(y.clone(), z.clone(), &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_val_ref(y.clone(), z.clone(), &w, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign_val_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by reference and the others by value.
    /// An [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_ref_val(y.clone(), &z, w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_ref_val(y.clone(), &z, w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_ref_val(y.clone(), &z, w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign_val_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_ref_ref(y.clone(), &z, &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_ref_ref(y.clone(), &z, &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_val_ref_ref(y.clone(), &z, &w, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign_val_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_val_val(&y, z.clone(), w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_val_val(&y, z.clone(), w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_val_val(&y, z.clone(), w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign_ref_val_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by value and the others by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_val_ref(&y, z.clone(), &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_val_ref(&y, z.clone(), &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_val_ref(&y, z.clone(), &w, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign_ref_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_ref_val(&y, &z, w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_ref_val(&y, &z, w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_ref_val(&y, &z, w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign_ref_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_ref_ref(&y, &z, &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_ref_ref(&y, &z, &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050195");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_round_assign_ref_ref_ref(&y, &z, &w, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_round_assign_ref_ref_ref(y, z, w, prec, rm)
    }
}

impl Float {
    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The [`Float`]s and the [`Rational`] are all
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 5, Ceiling);
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 5, Nearest);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 20, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 20, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round(
        self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, &z, &w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The [`Float`]s are taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
    ///     y.clone(),
    ///     z.clone(),
    ///     &w,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
    ///     y.clone(),
    ///     z.clone(),
    ///     &w,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
    ///     y.clone(),
    ///     z.clone(),
    ///     &w,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
    ///     y.clone(),
    ///     z.clone(),
    ///     &w,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
    ///     y.clone(),
    ///     z.clone(),
    ///     &w,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
    ///     y.clone(),
    ///     z.clone(),
    ///     &w,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, &z, w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The third [`Float`] is taken by reference and
    /// the other operands by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
    ///     y.clone(),
    ///     &z,
    ///     w.clone(),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
    ///     y.clone(),
    ///     &z,
    ///     w.clone(),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
    ///     y.clone(),
    ///     &z,
    ///     w.clone(),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
    ///     y.clone(),
    ///     &z,
    ///     w.clone(),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
    ///     y.clone(),
    ///     &z,
    ///     w.clone(),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
    ///     y.clone(),
    ///     &z,
    ///     w.clone(),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, z, &w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The first two [`Float`]s are taken by value and
    /// the third [`Float`] and the [`Rational`] by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_ref(
    ///     y.clone(),
    ///     &z,
    ///     &w,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_ref(
    ///     y.clone(),
    ///     &z,
    ///     &w,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_ref(
    ///     y.clone(),
    ///     &z,
    ///     &w,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_val_ref_ref(
    ///     y.clone(),
    ///     &z,
    ///     &w,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, z, w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The second [`Float`] is taken by reference and
    /// the other operands by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
    ///     &y,
    ///     z.clone(),
    ///     w.clone(),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
    ///     &y,
    ///     z.clone(),
    ///     w.clone(),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
    ///     &y,
    ///     z.clone(),
    ///     w.clone(),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
    ///     &y,
    ///     z.clone(),
    ///     w.clone(),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
    ///     &y,
    ///     z.clone(),
    ///     w.clone(),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
    ///     &y,
    ///     z.clone(),
    ///     w.clone(),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, &z, &w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The second [`Float`] and the [`Rational`] are
    /// taken by reference and the other operands by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_ref(
    ///     &y,
    ///     z.clone(),
    ///     &w,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_ref(
    ///     &y,
    ///     z.clone(),
    ///     &w,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_ref(
    ///     &y,
    ///     z.clone(),
    ///     &w,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_val_ref(
    ///     &y,
    ///     z.clone(),
    ///     &w,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, &z, w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The second and third [`Float`]s are taken by
    /// reference and the other operands by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded diff is less than, equal to, or greater than the exact diff. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Floor);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_ref_val(
    ///     &y,
    ///     &z,
    ///     w.clone(),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_ref_val(
    ///     &y,
    ///     &z,
    ///     w.clone(),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Floor);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_ref_val(
    ///     &y,
    ///     &z,
    ///     w.clone(),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.clone().mul_sub_mul_rational_prec_round_val_ref_ref_val(
    ///     &y,
    ///     &z,
    ///     w.clone(),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, z, &w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The first [`Float`] is taken by value and the
    /// other operands by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, z, w, true, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// subtraction, so there is a single rounding. The [`Float`]s and the [`Rational`] are all
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_sub_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_sub_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(diff.to_string(), "4.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950699");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(self, y, z, w, true, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, &z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by reference
    /// and the others by value. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_val_ref(
    ///         y.clone(),
    ///         z.clone(),
    ///         &w,
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_val_ref(
    ///         y.clone(),
    ///         z.clone(),
    ///         &w,
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_val_ref(
    ///         y.clone(),
    ///         z.clone(),
    ///         &w,
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, &z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_ref_val(
    ///         y.clone(),
    ///         &z,
    ///         w.clone(),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_ref_val(
    ///         y.clone(),
    ///         &z,
    ///         w.clone(),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_ref_val(
    ///         y.clone(),
    ///         &z,
    ///         w.clone(),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_val_val(
    ///         &y,
    ///         z.clone(),
    ///         w.clone(),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_val_val(
    ///         &y,
    ///         z.clone(),
    ///         w.clone(),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_val_val(
    ///         &y,
    ///         z.clone(),
    ///         w.clone(),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, &z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, &z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by value and
    /// the others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, z, &w, true, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded diff is less than,
    /// equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.25");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, z, w, true, prec, rm);
        *self = s;
        o
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The [`Float`]s and the [`Rational`] are all taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec(y.clone(), z.clone(), w.clone(), 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec(y.clone(), z.clone(), w.clone(), 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec(
        self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The [`Float`]s are taken by value and the [`Rational`] by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal
    /// to, or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_val_val_val_ref(y.clone(), z.clone(), &w, 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_val_val_val_ref(y.clone(), z.clone(), &w, 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_val_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The third [`Float`] is taken by reference and the other operands by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded diff is less than,
    /// equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_val_val_ref_val(y.clone(), &z, w.clone(), 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_val_val_ref_val(y.clone(), &z, w.clone(), 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_val_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The first two [`Float`]s are taken by value and the third [`Float`]
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_val_ref_ref(y.clone(), &z, &w, 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_val_ref_ref(y.clone(), &z, &w, 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_val_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The second [`Float`] is taken by reference and the other operands by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded diff is less than,
    /// equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_val_ref_val_val(&y, z.clone(), w.clone(), 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_prec_val_ref_val_val(&y, z.clone(), w.clone(), 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_val_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The second [`Float`] and the [`Rational`] are taken by reference and
    /// the other operands by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_ref_val_ref(&y, z.clone(), &w, 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_ref_val_ref(&y, z.clone(), &w, 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_val_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The second and third [`Float`]s are taken by reference and the other
    /// operands by value. An [`Ordering`] is also returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_ref_ref_val(&y, &z, w.clone(), 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_ref_ref_val(&y, &z, w.clone(), 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_val_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The first [`Float`] is taken by value and the other operands by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_prec_val_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_val_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final subtraction, so there
    /// is a single rounding. The [`Float`]s and the [`Rational`] are all taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_ref_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(diff.to_string(), "4.00");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_prec_ref_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(diff.to_string(), "4.0950623");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign(y.clone(), z.clone(), w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign(y.clone(), z.clone(), w.clone(), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by reference and the
    /// others by value. An [`Ordering`] is returned, indicating whether the rounded diff is less
    /// than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_val_ref_ref(y.clone(), &z, &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_val_ref_ref(y.clone(), &z, &w, 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_val_ref(&y, z.clone(), &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_val_ref(&y, z.clone(), &w, 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_ref_val(&y, &z, w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_ref_val(&y, &z, w.clone(), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_ref_ref(&y, &z, &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_prec_assign_ref_ref_ref(&y, &z, &w, 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950623");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_prec_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The [`Float`]s and the [`Rational`] are all taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded diff is less than, equal to, or greater than
    /// the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round(y.clone(), z.clone(), w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round(y.clone(), z.clone(), w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round(y.clone(), z.clone(), w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round(
        self,
        y: Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The [`Float`]s are taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_val_ref(y.clone(), z.clone(), &w, Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_val_ref(y.clone(), z.clone(), &w, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_val_ref(y.clone(), z.clone(), &w, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_val_val_val_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The third [`Float`] is taken by reference and the other operands by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_ref_val(y.clone(), &z, w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_ref_val(y.clone(), &z, w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_ref_val(y.clone(), &z, w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_val_val_ref_val(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The first two [`Float`]s are taken by value and the third [`Float`] and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// diff is less than, equal to, or greater than the exact diff. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_ref_ref(y.clone(), &z, &w, Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_ref_ref(y.clone(), &z, &w, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_val_ref_ref(y.clone(), &z, &w, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_val_val_ref_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The second [`Float`] is taken by reference and the other operands by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_val_val(&y, z.clone(), w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_val_val(&y, z.clone(), w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_val_val(&y, z.clone(), w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_val_ref_val_val(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The second [`Float`] and the [`Rational`] are taken by reference and the other
    /// operands by value. An [`Ordering`] is also returned, indicating whether the rounded diff is
    /// less than, equal to, or greater than the exact diff. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_val_ref(&y, z.clone(), &w, Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_val_ref(&y, z.clone(), &w, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_val_ref(&y, z.clone(), &w, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_val_ref_val_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The second and third [`Float`]s are taken by reference and the other operands by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded diff is less than,
    /// equal to, or greater than the exact diff. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_ref_val(&y, &z, w.clone(), Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_ref_val(&y, &z, w.clone(), Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) =
    ///     x.clone()
    ///         .mul_sub_mul_rational_round_val_ref_ref_val(&y, &z, w.clone(), Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_val_ref_ref_val(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The first [`Float`] is taken by value and the other operands by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded diff is less than, equal to,
    /// or greater than the exact diff. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_round_val_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_round_val_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x
    ///     .clone()
    ///     .mul_sub_mul_rational_round_val_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_val_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Subtracts the product of a [`Float`] and a [`Rational`] from the product of two [`Float`]s,
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final subtraction, so there is a single
    /// rounding. The [`Float`]s and the [`Rational`] are all taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded diff is less than, equal to, or greater than
    /// the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul`](malachite_base::num::arithmetic::traits::MulSubMul::mul_sub_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_round_ref_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(diff.to_string(), "4.0950630266438379");
    /// assert_eq!(o, Less);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_round_ref_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    ///
    /// let (diff, o) = x.mul_sub_mul_rational_round_ref_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(diff.to_string(), "4.0950630266438388");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded diff is less than, equal to, or greater than the exact diff.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign(y.clone(), z.clone(), w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign(y.clone(), z.clone(), w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign(y.clone(), z.clone(), w.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_val_ref(y.clone(), z.clone(), &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_val_ref(y.clone(), z.clone(), &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_val_ref(y.clone(), z.clone(), &w, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign_val_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by reference and the others by value.
    /// An [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_ref_val(y.clone(), &z, w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_ref_val(y.clone(), &z, w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_ref_val(y.clone(), &z, w.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign_val_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_ref_ref(y.clone(), &z, &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_ref_ref(y.clone(), &z, &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_val_ref_ref(y.clone(), &z, &w, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign_val_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_val_val(&y, z.clone(), w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_val_val(&y, z.clone(), w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_val_val(&y, z.clone(), w.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign_ref_val_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by value and the others by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_val_ref(&y, z.clone(), &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_val_ref(&y, z.clone(), &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_val_ref(&y, z.clone(), &w, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign_ref_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded diff is less than, equal to, or
    /// greater than the exact diff. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_ref_val(&y, &z, w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_ref_val(&y, &z, w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_ref_val(&y, &z, w.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign_ref_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded diff is less than, equal to, or greater than the
    /// exact diff. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy-zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_sub_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_sub_mul_assign`](malachite_base::num::arithmetic::traits::MulSubMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_ref_ref(&y, &z, &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438379");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_ref_ref(&y, &z, &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_sub_mul_rational_round_assign_ref_ref_ref(&y, &z, &w, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_sub_mul_rational_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(y, z, w, prec, rm)
    }
}

impl MulSubMul<Self, Self, Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking all four by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(y, z, w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec(y, z, w, prec).0
    }
}

impl MulSubMul<Self, Self, &Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the first three by value and the fourth by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(y, z, &w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_val_val_val_ref(y, z, w, prec)
            .0
    }
}

impl MulSubMul<Self, &Self, Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the third by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(y, &z, w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_val_val_ref_val(y, z, w, prec)
            .0
    }
}

impl MulSubMul<Self, &Self, &Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the first two by value and the last two by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(y, &z, &w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_val_val_ref_ref(y, z, w, prec)
            .0
    }
}

impl MulSubMul<&Self, Self, Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the second by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(&y, z, w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_val_ref_val_val(y, z, w, prec)
            .0
    }
}

impl MulSubMul<&Self, Self, &Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the second and fourth by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(&y, z, &w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_val_ref_val_ref(y, z, w, prec)
            .0
    }
}

impl MulSubMul<&Self, &Self, Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the second and third by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(&y, &z, w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_val_ref_ref_val(y, z, w, prec)
            .0
    }
}

impl MulSubMul<&Self, &Self, &Rational> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the first by value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(x.mul_sub_mul(&y, &z, &w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_val_ref_ref_ref(y, z, w, prec)
            .0
    }
}

impl MulSubMul<&Float, &Float, &Rational> for &Float {
    type Output = Float;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking all four by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// assert_eq!(&x.mul_sub_mul(&y, &z, &w).to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Float, z: &Float, w: &Rational) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_ref_ref_ref_ref(y, z, w, prec)
            .0
    }
}

impl MulSubMulAssign<Self, Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(y, z, w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign(y, z, w, prec);
    }
}

impl MulSubMulAssign<Self, Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(y, z, &w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign_val_val_ref(y, z, w, prec);
    }
}

impl MulSubMulAssign<Self, &Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(y, &z, w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign_val_ref_val(y, z, w, prec);
    }
}

impl MulSubMulAssign<Self, &Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(y, &z, &w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign_val_ref_ref(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(&y, z, w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign_ref_val_val(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(&y, z, &w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign_ref_val_ref(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, &Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(&y, &z, w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign_ref_ref_val(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, &Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the diff is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(22, 7);
    /// x.mul_sub_mul_assign(&y, &z, &w);
    /// assert_eq!(x.to_string(), "4.0950630266438388");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_sub_mul_rational_prec_assign_ref_ref_ref(y, z, w, prec);
    }
}

impl MulSubMul<Self, Self, Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking all four by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(y, z, w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec(y, z, w, prec).0
    }
}

impl MulSubMul<Self, Self, &Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the first three by value and the fourth by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(y, z, &w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_val_val_val_ref(y, z, w, prec).0
    }
}

impl MulSubMul<Self, &Self, Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the third by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(y, &z, w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_val_val_ref_val(y, z, w, prec).0
    }
}

impl MulSubMul<Self, &Self, &Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the first two by value and the last two by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(y, &z, &w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_val_val_ref_ref(y, z, w, prec).0
    }
}

impl MulSubMul<&Self, Self, Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the second by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(&y, z, w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_val_ref_val_val(y, z, w, prec).0
    }
}

impl MulSubMul<&Self, Self, &Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the second and fourth by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(&y, z, &w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_val_ref_val_ref(y, z, w, prec).0
    }
}

impl MulSubMul<&Self, &Self, Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the second and third by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(&y, &z, w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_val_ref_ref_val(y, z, w, prec).0
    }
}

impl MulSubMul<&Self, &Self, &Self> for Float {
    type Output = Self;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking the first by value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(x.mul_sub_mul(&y, &z, &w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_val_ref_ref_ref(y, z, w, prec).0
    }
}

impl MulSubMul<&Float, &Float, &Float> for &Float {
    type Output = Float;
    /// Subtracts the product of one pair of [`Float`]s from the product of another pair with a
    /// single rounding, taking all four by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity, the second
    ///   product's sign counting as flipped.
    /// - If both products are infinite, the result is their common infinity if their signs differ,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply to $xy$ and $-zw$.
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_sub_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(&x.mul_sub_mul(&y, &z, &w).to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Float, z: &Float, w: &Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_ref_ref_ref_ref(y, z, w, prec).0
    }
}

impl MulSubMulAssign<Self, Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(y, z, w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign(y, z, w, prec);
    }
}

impl MulSubMulAssign<Self, Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(y, z, &w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign_val_val_ref(y, z, w, prec);
    }
}

impl MulSubMulAssign<Self, &Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(y, &z, w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign_val_ref_val(y, z, w, prec);
    }
}

impl MulSubMulAssign<Self, &Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(y, &z, &w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign_val_ref_ref(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(&y, z, w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign_ref_val_val(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(&y, z, &w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign_ref_val_ref(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, &Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(&y, &z, w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign_ref_ref_val(y, z, w, prec);
    }
}

impl MulSubMulAssign<&Self, &Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and subtracts the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// diff is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy-zw+\varepsilon.
    /// $$
    /// - If $xy-zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy-zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy-zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_sub_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_sub_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_sub_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_sub_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_sub_mul_assign(&y, &z, &w);
    /// assert_eq!(x.to_string(), "7.5594760792050186");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_sub_mul_prec_assign_ref_ref_ref(y, z, w, prec);
    }
}

/// Subtracts the product of one pair of primitive floats from the product of another pair with a
/// single rounding, using emulated [`Float`] arithmetic.
///
/// The products are not rounded before the subtraction, so the result is the true value of $xy-zw$
/// rounded once to the nearest representable value. No standard-library counterpart exists.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, LN_2, PI, SQRT_2};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::mul_sub_mul::*;
///
/// assert_eq!(
///     NiceFloat(primitive_float_mul_sub_mul(PI, E, SQRT_2, LN_2)),
///     NiceFloat(7.559476079205019)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_mul_sub_mul<T: PrimitiveFloat>(x: T, y: T, z: T, w: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_float_float_to_float_fn(Float::mul_sub_mul_prec, x, y, z, w)
}

/// Subtracts the product of a primitive float and a [`Rational`] from the product of two primitive
/// floats, with a single rounding, using emulated [`Float`] arithmetic.
///
/// The [`Rational`] enters its product exactly, the products are not rounded before the
/// subtraction, and the result is the true value of $xy-zw$ rounded once to the nearest
/// representable value.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `w.significant_bits()`.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, PI, SQRT_2};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::mul_sub_mul::*;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_mul_sub_mul_rational(
///         PI,
///         E,
///         SQRT_2,
///         &Rational::from_signeds(22, 7)
///     )),
///     NiceFloat(4.095063026643839)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_mul_sub_mul_rational<T: PrimitiveFloat>(x: T, y: T, z: T, w: &Rational) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_float_to_float_fn(
        |x, y, z, prec| x.mul_sub_mul_rational_prec_val_val_val_ref(y, z, w, prec),
        x,
        y,
        z,
    )
}
