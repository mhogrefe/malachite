// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::{max, Ordering};
use core::ops::{Sub, SubAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;

impl Float {
    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any other [`Float`], whenever
    /// this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::sub_round`] instead. If both of these things are true, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 5, RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 5, RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 5, RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 20, RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 20, RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round(Float::from(E), 20, RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec_round(
        mut self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is taken by value and the second by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded difference is less than,
    /// equal to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// other [`Float`], whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_val_ref`] instead. If both of these things are true,
    /// consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(
    ///     &Float::from(E),
    ///     5,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(
    ///     &Float::from(E),
    ///     5,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(
    ///     &Float::from(E),
    ///     5,
    ///     RoundingMode::Nearest
    /// );
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(
    ///     &Float::from(E),
    ///     20,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(
    ///     &Float::from(E),
    ///     20,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_val_ref(
    ///     &Float::from(E),
    ///     20,
    ///     RoundingMode::Nearest
    /// );
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_val_ref(
        mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is taken by reference and the second by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded difference is less than,
    /// equal to, or greater than the exact difference. Although `NaN`s are not comparable to any
    /// other [`Float`], whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_ref_val`] instead. If both of these things are true,
    /// consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(
    ///     Float::from(E),
    ///     5,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(
    ///     Float::from(E),
    ///     5,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(
    ///     Float::from(E),
    ///     5,
    ///     RoundingMode::Nearest
    /// );
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(
    ///     Float::from(E),
    ///     20,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(
    ///     Float::from(E),
    ///     20,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_val(
    ///     Float::from(E),
    ///     20,
    ///     RoundingMode::Nearest
    /// );
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_ref_val(
        &self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.add_prec_round_ref_val(-other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any other [`Float`], whenever
    /// this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,\infty,p,m)=f(-\infty,-\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p,m)=0.0$
    /// - $f(-0.0,0.0,p,m)=-0.0$
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p,m)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_ref_ref`] instead. If both of these things are true,
    /// consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(
    ///     &Float::from(E),
    ///     5,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(
    ///     &Float::from(E),
    ///     5,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(sum.to_string(), "0.44");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(
    ///     &Float::from(E),
    ///     5,
    ///     RoundingMode::Nearest
    /// );
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(
    ///     &Float::from(E),
    ///     20,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(
    ///     &Float::from(E),
    ///     20,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(sum.to_string(), "0.4233112");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_round_ref_ref(
    ///     &Float::from(E),
    ///     20,
    ///     RoundingMode::Nearest
    /// );
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec_round_ref_ref(
        &self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.add_prec_round_ref_ref_helper(other, prec, rm, true)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function returns a
    /// `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec(Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec(Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec(self, other: Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec_val_ref(self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round_val_ref(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec_ref_val(&self, other: Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round_ref_val(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`Float`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded difference is less than, equal to, or greater than the exact
    /// difference. Although `NaN`s are not comparable to any other [`Float`], whenever this
    /// function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,\infty,p)=f(-\infty,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,p)=0.0$
    /// - $f(-0.0,0.0,p)=-0.0$
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,p)=f(-0.0,-0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,p)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,p)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "0.42");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "0.4233108");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_prec_ref_ref(&self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.sub_prec_round_ref_ref(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded difference is less than, equal to, or greater than the exact difference. Although
    /// `NaN`s are not comparable to any other [`Float`], whenever this function returns a `NaN` it
    /// also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the maximum precision of the inputs is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_round(Float::from(-E), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round(Float::from(-E), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round(Float::from(-E), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_round(self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        Float::sub_prec_round(self, other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. The
    /// [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any other [`Float`], whenever
    /// this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_val_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the maximum precision of the inputs is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_round_val_ref(&Float::from(-E), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_val_ref(&Float::from(-E), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_val_ref(&Float::from(-E), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_round_val_ref(self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_val_ref(other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. The
    /// [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any other [`Float`], whenever
    /// this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_ref_val`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the maximum precision of the inputs is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_val(Float::from(-E), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_val(Float::from(-E), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_val(Float::from(-E), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_round_ref_val(&self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_val(other, prec, rm)
    }

    /// Subtracts two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded difference is less than, equal to, or greater than the exact difference. Although
    /// `NaN`s are not comparable to any other [`Float`], whenever this function returns a `NaN` it
    /// also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,\infty,m)=f(-\infty,-\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0,m)=0.0$
    /// - $f(-0.0,0.0,m)=-0.0$
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,0.0,m)=f(-0.0,-0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_ref_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the maximum precision of the inputs is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_ref(&Float::from(-E), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_ref(&Float::from(-E), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI).sub_round_ref_ref(&Float::from(-E), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_round_ref_ref(&self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_ref(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any other [`Float`], whenever this function sets the [`Float`] to `NaN` it
    /// also returns `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_assign`] instead. If
    /// you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::sub_round_assign`] instead. If both of these things are true,
    /// consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(Float::from(E), 5, RoundingMode::Floor),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(Float::from(E), 5, RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "0.44");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(Float::from(E), 5, RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(Float::from(E), 20, RoundingMode::Floor),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.4233108");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(Float::from(E), 20, RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "0.4233112");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign(Float::from(E), 20, RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_round_assign(&mut self, other: Float, prec: u64, rm: RoundingMode) -> Ordering {
        self.add_prec_round_assign_helper(other, prec, rm, true)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded difference
    /// is less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any other [`Float`], whenever this function sets the [`Float`] to `NaN` it
    /// also returns `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::sub_round_assign`] instead. If both of these things are
    /// true, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 5, RoundingMode::Floor),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 5, RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "0.44");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 5, RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 20, RoundingMode::Floor),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.4233108");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 20, RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "0.4233112");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_prec_round_assign_ref(&Float::from(E), 20, RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_round_assign_ref(
        &mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.add_prec_round_assign_ref_helper(other, prec, rm, true)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Float`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign(Float::from(E), 5), Ordering::Less);
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign(Float::from(E), 20), Ordering::Less);
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_assign(&mut self, other: Float, prec: u64) -> Ordering {
        self.sub_prec_round_assign(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Float`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_round_assign_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign_ref(&Float::from(E), 5), Ordering::Less);
    /// assert_eq!(x.to_string(), "0.42");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_prec_assign_ref(&Float::from(E), 20), Ordering::Less);
    /// assert_eq!(x.to_string(), "0.4233108");
    /// ```
    #[inline]
    pub fn sub_prec_assign_ref(&mut self, other: &Float, prec: u64) -> Ordering {
        self.sub_prec_round_assign_ref(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any other [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::sub_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::sub_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `-=`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the maximum precision of the inputs is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign(Float::from(-E), RoundingMode::Floor), Ordering::Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign(Float::from(-E), RoundingMode::Ceiling), Ordering::Greater);
    /// assert_eq!(x.to_string(), "5.859874482048839");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign(Float::from(-E), RoundingMode::Nearest), Ordering::Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    /// ```
    #[inline]
    pub fn sub_round_assign(&mut self, other: Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`]
    /// is returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::sub_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the maximum precision of the inputs is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign_ref(&Float::from(-E), RoundingMode::Floor), Ordering::Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_round_assign_ref(&Float::from(-E), RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "5.859874482048839");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_round_assign_ref(&Float::from(-E), RoundingMode::Nearest), Ordering::Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    /// ```
    #[inline]
    pub fn sub_round_assign_ref(&mut self, other: &Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign_ref(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded difference is less
    /// than, equal to, or greater than the exact difference. Although `NaN`s are not comparable to
    /// any other [`Float`], whenever this function returns a `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec`] instead.
    /// If you know that your target precision is the precision of the [`Float`] input, consider
    /// using [`Float::sub_rational_round`] instead. If both of these things are true, consider
    /// using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round(
        mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_rational_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any other [`Float`], whenever this function returns a `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_val_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_val_ref`] instead. If both of these things are
    /// true, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Floor
    ///     );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Ceiling
    ///     );
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Nearest
    ///     );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Floor
    ///     );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Ceiling
    ///     );
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Nearest
    ///     );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_val_ref(
        mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.sub_rational_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any other [`Float`], whenever this function returns a `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_ref_val`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_ref_val`] instead. If both of these things are
    /// true, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Floor
    ///     );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Ceiling
    ///     );
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Nearest
    ///     );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Floor
    ///     );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Ceiling
    ///     );
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Nearest
    ///     );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.sub_rational_prec_round_ref_val_helper(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded difference is
    /// less than, equal to, or greater than the exact difference. Although `NaN`s are not
    /// comparable to any other [`Float`], whenever this function returns a `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_ref_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_ref_ref`] instead. If both of these things are
    /// true, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Floor
    ///     );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Ceiling
    ///     );
    /// assert_eq!(sum.to_string(), "2.9");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Nearest
    ///     );
    /// assert_eq!(sum.to_string(), "2.8");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Floor
    ///     );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Ceiling
    ///     );
    /// assert_eq!(sum.to_string(), "2.808262");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Nearest
    ///     );
    /// assert_eq!(sum.to_string(), "2.808258");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.sub_rational_prec_round_ref_ref_helper(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`],  whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round`] instead. If you know that your target precision is the
    /// precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec(Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec(Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec(self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`],  whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_val_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_val_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_val_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round_val_ref(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference and the [`Rational`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`],  whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_ref_val`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_val(Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_val(Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round_ref_val(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded difference is less than, equal
    /// to, or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`],  whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_ref_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "1.62");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI).sub_rational_prec_ref_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "1.641592");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.sub_rational_prec_round_ref_ref(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] and the [`Rational`] are both are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the precision of the [`Float`] input is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round(Rational::from_unsigneds(1u8, 3), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round(Rational::from_unsigneds(1u8, 3), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "2.8082593202564601");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round(Rational::from_unsigneds(1u8, 3), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_round(self, other: Rational, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_val_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the precision of the [`Float`] input is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "2.8082593202564601");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_ref_val`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the precision of the [`Float`] input is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "2.8082593202564601");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Subtracts a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] and the [`Rational`] are both are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded difference is less than, equal to, or greater
    /// than the exact difference. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(x,0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x,m)=f(-0.0,x,m)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_ref_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the precision of the [`Float`] input is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), RoundingMode::Floor);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), RoundingMode::Ceiling);
    /// assert_eq!(sum.to_string(), "2.8082593202564601");
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (sum, o) = Float::from(PI)
    ///     .sub_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), RoundingMode::Nearest);
    /// assert_eq!(sum.to_string(), "2.8082593202564596");
    /// assert_eq!(o, Ordering::Less);
    /// ```
    #[inline]
    pub fn sub_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sub_rational_prec_assign`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::sub_rational_round_assign`] instead. If both of these things are
    /// true, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Floor
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Ceiling
    ///     ),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Nearest
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Floor
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Ceiling
    ///     ),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808262");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Nearest
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.sub_rational_prec_round_assign_helper(other, prec, rm)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded difference is less than, equal to,
    /// or greater than the exact difference. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Ordering::Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$.
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sub_rational_prec_assign_ref`] instead. If you know that your target precision is
    /// the precision of the [`Float`] input, consider using
    /// [`Float::sub_rational_round_assign_ref`] instead. If both of these things are true, consider
    /// using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but `prec` is too small for an exact subtraction.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Floor
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Ceiling
    ///     ),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "2.9");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         5,
    ///         RoundingMode::Nearest
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Floor
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Ceiling
    ///     ),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "2.808262");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         RoundingMode::Nearest
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.808258");
    /// ```
    #[inline]
    pub fn sub_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.sub_rational_prec_round_assign_ref_helper(other, prec, rm)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded difference is less than, equal to, or greater than the exact
    /// difference. Although `NaN`s are not comparable to any other [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_rational_prec_assign(Rational::exact_from(1.5), 5), Ordering::Less);
    /// assert_eq!(x.to_string(), "1.62");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_rational_prec_assign(Rational::exact_from(1.5), 20), Ordering::Less);
    /// assert_eq!(x.to_string(), "1.641592");
    /// ```
    #[inline]
    pub fn sub_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.sub_rational_prec_round_assign(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded difference is less than, equal to, or greater than
    /// the exact difference. Although `NaN`s are not comparable to any other [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Ordering::Equal`.
    ///
    /// If the difference is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sub_rational_prec_val_ref`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_round_assign_ref`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_rational_prec_assign_ref(&Rational::exact_from(1.5), 5), Ordering::Less);
    /// assert_eq!(x.to_string(), "1.62");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.sub_rational_prec_assign_ref(&Rational::exact_from(1.5), 20), Ordering::Less);
    /// assert_eq!(x.to_string(), "1.641592");
    /// ```
    #[inline]
    pub fn sub_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        self.sub_rational_prec_round_assign_ref(other, prec, RoundingMode::Nearest)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is returned, indicating
    /// whether the rounded difference is less than, equal to, or greater than the exact difference.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::sub_rational_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the precision of the input [`Float`] is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign(Rational::from_unsigneds(1u8, 3), RoundingMode::Floor),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8082593202564596");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign(Rational::from_unsigneds(1u8, 3), RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "2.8082593202564601");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign(Rational::from_unsigneds(1u8, 3), RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8082593202564596");
    /// ```
    #[inline]
    pub fn sub_rational_round_assign(&mut self, other: Rational, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign(other, prec, rm)
    }

    /// Subtracts a [`Rational`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded difference is less than, equal to, or greater than the exact
    /// difference. Although `NaN`s are not comparable to any other [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Ordering::Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x-y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x-y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x-y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::sub_rational_round_val_ref`] documentation for information on special
    /// cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sub_rational_prec_round_assign_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `-=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `RoundingMode::Exact` but the precision of the input [`Float`] is not high
    /// enough to represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         RoundingMode::Floor
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8082593202564596");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         RoundingMode::Ceiling
    ///     ),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(x.to_string(), "2.8082593202564601");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.sub_rational_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         RoundingMode::Nearest
    ///     ),
    ///     Ordering::Less
    /// );
    /// assert_eq!(x.to_string(), "2.8082593202564596");
    /// ```
    #[inline]
    pub fn sub_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign_ref(other, prec, rm)
    }
}

impl Sub<Float> for Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking both by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sub_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::sub_round`].
    /// If you want both of these things, consider using [`Float::sub_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) - Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(Float::from(1.5) - Float::NEGATIVE_INFINITY, Float::INFINITY);
    /// assert!((Float::INFINITY - Float::INFINITY).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(Float::from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(Float::from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    ///
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round(other, prec, RoundingMode::Nearest).0
    }
}

impl<'a> Sub<&'a Float> for Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking the first by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_val_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_val_ref`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) - &Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(Float::from(1.5) - &Float::NEGATIVE_INFINITY, Float::INFINITY);
    /// assert!((Float::INFINITY - &Float::INFINITY).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(Float::from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(Float::from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &'a Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_val_ref(other, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a> Sub<Float> for &'a Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking the first by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_ref_val`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_ref_val`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) - Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(&Float::from(1.5) - Float::NEGATIVE_INFINITY, Float::INFINITY);
    /// assert!((&Float::INFINITY - Float::INFINITY).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(&Float::from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(&Float::from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_val(other, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a, 'b> Sub<&'a Float> for &'b Float {
    type Output = Float;

    /// Subtracts two [`Float`]s, taking both by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,\infty)=f(-\infty,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=-\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(x,\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,-0.0)=0.0$
    /// - $f(-0.0,0.0)=-0.0$
    /// - $f(0.0,0.0)=f(-0.0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(0.0,x)=f(-0.0,x)=-x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sub_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::sub_round`].
    /// If you want both of these things, consider using [`Float::sub_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) - &Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(&Float::from(1.5) - &Float::NEGATIVE_INFINITY, Float::INFINITY);
    /// assert!((&Float::INFINITY - &Float::INFINITY).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(&Float::from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(&Float::from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &'a Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_ref_ref(other, prec, RoundingMode::Nearest)
            .0
    }
}

impl SubAssign<Float> for Float {
    /// Subtracts a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side
    /// by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases: See the `-` documenation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_assign`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x -= Float::INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::from(2.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= Float::from(-2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= Float::from(2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= Float::from(-2.5);
    /// assert_eq!(x, 1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign(other, prec, RoundingMode::Nearest);
    }
}

impl<'a> SubAssign<&'a Float> for Float {
    /// Subtracts a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side
    /// by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// difference is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases: See the `-` documenation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_round_assign`]. If you want both of these things, consider using
    /// [`Float::sub_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x -= &Float::INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::from(2.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x -= &Float::from(-2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= &Float::from(2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x -= &Float::from(-2.5);
    /// assert_eq!(x, 1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.sub_prec_round_assign_ref(other, prec, RoundingMode::Nearest);
    }
}

impl Sub<Rational> for Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec`] instead. If you want to specify the output precision, consider
    /// using [`Float::sub_rational_round`]. If you want both of these things, consider using
    /// [`Float::sub_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN - Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(Float::INFINITY - Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY - Rational::exact_from(1.5), Float::NEGATIVE_INFINITY);
    ///
    /// assert_eq!(Float::from(2.5) - Rational::exact_from(1.5), 1.0);
    /// assert_eq!(Float::from(2.5) - Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(Float::from(-2.5) - Rational::exact_from(1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) - Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round(other, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a> Sub<&'a Rational> for Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking the first by value and the second by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_val_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_val_ref`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN - &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(Float::INFINITY - &Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY - &Rational::exact_from(1.5), Float::NEGATIVE_INFINITY);
    ///
    /// assert_eq!(Float::from(2.5) - &Rational::exact_from(1.5), 1.0);
    /// assert_eq!(Float::from(2.5) - &Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(Float::from(-2.5) - &Rational::exact_from(1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) - &Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_val_ref(other, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a> Sub<Rational> for &'a Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking the first by reference and the second by
    /// value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_ref_val`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_ref_val`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN - Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(&Float::INFINITY - Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(&Float::NEGATIVE_INFINITY - Rational::exact_from(1.5), Float::NEGATIVE_INFINITY);
    ///
    /// assert_eq!(&Float::from(2.5) - Rational::exact_from(1.5), 1.0);
    /// assert_eq!(&Float::from(2.5) - Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(&Float::from(-2.5) - Rational::exact_from(1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) - Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_val(other, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a, 'b> Sub<&'a Rational> for &'b Float {
    type Output = Float;

    /// Subtracts a [`Float`] by a [`Rational`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(x,0)=x$
    /// - $f(0.0,x)=f(-0.0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_ref_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_ref_ref`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN - Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(&Float::INFINITY - Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(&Float::NEGATIVE_INFINITY - Rational::exact_from(1.5), Float::NEGATIVE_INFINITY);
    ///
    /// assert_eq!(&Float::from(2.5) - &Rational::exact_from(1.5), 1.0);
    /// assert_eq!(&Float::from(2.5) - &Rational::exact_from(-1.5), 4.0);
    /// assert_eq!(&Float::from(-2.5) - &Rational::exact_from(1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) - &Rational::exact_from(-1.5), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_ref_ref(other, prec, RoundingMode::Nearest)
            .0
    }
}

impl SubAssign<Rational> for Float {
    /// Subtracts a [`Rational`] by a [`Float`] in place, taking the [`Rational`] by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `-` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::sub_rational_round_assign`]. If you want both of these things,
    /// consider using [`Float::sub_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x -= Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= Rational::exact_from(-1.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= Rational::exact_from(1.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= Rational::exact_from(-1.5);
    /// assert_eq!(x, -1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Rational) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign(other, prec, RoundingMode::Nearest);
    }
}

impl<'a> SubAssign<&'a Rational> for Float {
    /// Subtracts a [`Rational`] by a [`Float`] in place, taking the [`Rational`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `-` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sub_rational_prec_assign_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::sub_rational_round_assign_ref`]. If you want both of
    /// these things, consider using [`Float::sub_rational_prec_round_assign_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x -= &Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(2.5);
    /// x -= &Rational::exact_from(-1.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= &Rational::exact_from(1.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x -= &Rational::exact_from(-1.5);
    /// assert_eq!(x, -1.0);
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Rational) {
        let prec = self.significant_bits();
        self.sub_rational_prec_round_assign_ref(other, prec, RoundingMode::Nearest);
    }
}

impl Sub<Float> for Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(Rational::exact_from(1.5) - Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(Rational::exact_from(1.5) - Float::NEGATIVE_INFINITY, Float::INFINITY);
    ///
    /// assert_eq!(Rational::exact_from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(Rational::exact_from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = other.significant_bits();
        -other
            .sub_rational_prec_round(self, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a> Sub<&'a Float> for Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking the [`Rational`] by value and the [`Float`]
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(Rational::exact_from(1.5) - &Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(Rational::exact_from(1.5) - &Float::NEGATIVE_INFINITY, Float::INFINITY);
    ///
    /// assert_eq!(Rational::exact_from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(Rational::exact_from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(Rational::exact_from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        -other
            .sub_rational_prec_round_ref_val(self, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a> Sub<Float> for &'a Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking the [`Rational`] by value and the [`Float`]
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) - Float::NAN).is_nan());
    /// assert_eq!(&Rational::exact_from(1.5) - Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(&Rational::exact_from(1.5) - Float::NEGATIVE_INFINITY, Float::INFINITY);
    ///
    /// assert_eq!(&Rational::exact_from(1.5) - Float::from(2.5), -1.0);
    /// assert_eq!(&Rational::exact_from(1.5) - Float::from(-2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - Float::from(2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: Float) -> Float {
        let prec = other.significant_bits();
        -other
            .sub_rational_prec_round_val_ref(self, prec, RoundingMode::Nearest)
            .0
    }
}

impl<'a, 'b> Sub<&'a Float> for &'b Rational {
    type Output = Float;

    /// Subtracts a [`Rational`] by a [`Float`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the difference
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x-y+\epsilon.
    /// $$
    /// - If $x-y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x-y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x-y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=-\infty$
    /// - $f(x,-\infty)=\infty$
    /// - $f(0,0.0)=-0.0$
    /// - $f(0,-0.0)=0.0$
    /// - $f(x,0.0)=f(x,-0.0)=x$
    /// - $f(0,x)=-x$
    /// - $f(x,x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) - &Float::NAN).is_nan());
    /// assert_eq!(&Rational::exact_from(1.5) - &Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(&Rational::exact_from(1.5) - &Float::NEGATIVE_INFINITY, Float::INFINITY);
    ///
    /// assert_eq!(&Rational::exact_from(1.5) - &Float::from(2.5), -1.0);
    /// assert_eq!(&Rational::exact_from(1.5) - &Float::from(-2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - &Float::from(2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) - &Float::from(-2.5), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        -other
            .sub_rational_prec_round_ref_ref(self, prec, RoundingMode::Nearest)
            .0
    }
}
