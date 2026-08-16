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

use crate::InnerFloat::NaN;
use crate::{Float, emulate_float_float_to_float_fn, float_nan};
use core::cmp::Ordering::{self, Equal, Greater};
use core::cmp::max;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};

// This is mpfr_dim from dim.c, MPFR 4.2.2, with the result's precision passed explicitly. The
// positive difference is x - y if x > y, and +0 otherwise (a definition choice: negative values are
// representable, but the function returns zero for them); NaN if either input is NaN. The
// comparison treats zeros of both signs as equal and infinities as their usual extremes, so
// dim(Infinity, Infinity) is +0.

impl Float {
    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the specified precision and with the specified rounding mode.
    /// Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact positive difference.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::positive_difference_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::positive_difference_round`] instead. If both of these things
    /// are true, consider using [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the positive difference is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_prec_round(Float::from(1u32), 10, Floor);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) = Float::from(10u32).positive_difference_prec_round(Float::from(7u32), 1, Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (d, o) =
    ///     Float::from(10u32).positive_difference_prec_round(Float::from(7u32), 1, Ceiling);
    /// assert_eq!(d.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn positive_difference_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if matches!(self.partial_cmp(&other), Some(Greater)) {
            self.sub_prec_round(other, prec, rm)
        } else if matches!(self, Self(NaN)) || matches!(other, Self(NaN)) {
            (float_nan!(), Equal)
        } else {
            (Self::ZERO, Equal)
        }
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the specified precision and with the specified rounding mode. The
    /// first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::positive_difference_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::positive_difference_round`] instead. If both of these things
    /// are true, consider using [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the positive difference is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) =
    ///     Float::from(3u32).positive_difference_prec_round_val_ref(&Float::from(1u32), 10, Floor);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) =
    ///     Float::from(10u32).positive_difference_prec_round_val_ref(&Float::from(7u32), 1, Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (d, o) = Float::from(10u32).positive_difference_prec_round_val_ref(
    ///     &Float::from(7u32),
    ///     1,
    ///     Ceiling,
    /// );
    /// assert_eq!(d.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn positive_difference_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if matches!(self.partial_cmp(other), Some(Greater)) {
            self.sub_prec_round_val_ref(other, prec, rm)
        } else if matches!(self, Self(NaN)) || matches!(other, Self(NaN)) {
            (float_nan!(), Equal)
        } else {
            (Self::ZERO, Equal)
        }
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the specified precision and with the specified rounding mode. The
    /// first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::positive_difference_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::positive_difference_round`] instead. If both of these things
    /// are true, consider using [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the positive difference is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) =
    ///     Float::from(3u32).positive_difference_prec_round_ref_val(Float::from(1u32), 10, Floor);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) =
    ///     Float::from(10u32).positive_difference_prec_round_ref_val(Float::from(7u32), 1, Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (d, o) = Float::from(10u32).positive_difference_prec_round_ref_val(
    ///     Float::from(7u32),
    ///     1,
    ///     Ceiling,
    /// );
    /// assert_eq!(d.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn positive_difference_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if matches!((*self).partial_cmp(&other), Some(Greater)) {
            self.sub_prec_round_ref_val(other, prec, rm)
        } else if matches!(self, Self(NaN)) || matches!(other, Self(NaN)) {
            (float_nan!(), Equal)
        } else {
            (Self::ZERO, Equal)
        }
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the specified precision and with the specified rounding mode.
    /// Both [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded result is less than, equal to, or greater than the exact positive difference.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::positive_difference_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::positive_difference_round`] instead. If both of these things
    /// are true, consider using [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the positive difference is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) =
    ///     Float::from(3u32).positive_difference_prec_round_ref_ref(&Float::from(1u32), 10, Floor);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) =
    ///     Float::from(10u32).positive_difference_prec_round_ref_ref(&Float::from(7u32), 1, Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (d, o) = Float::from(10u32).positive_difference_prec_round_ref_ref(
    ///     &Float::from(7u32),
    ///     1,
    ///     Ceiling,
    /// );
    /// assert_eq!(d.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn positive_difference_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if matches!((*self).partial_cmp(other), Some(Greater)) {
            self.sub_prec_round_ref_ref(other, prec, rm)
        } else if matches!(self, Self(NaN)) || matches!(other, Self(NaN)) {
            (float_nan!(), Equal)
        } else {
            (Self::ZERO, Equal)
        }
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the specified precision. Both [`Float`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded result is
    /// less than, equal to, or greater than the exact positive difference. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using
    /// [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_prec(Float::from(1u32), 10);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.positive_difference_prec_round(other, prec, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the specified precision. The first [`Float`]
    /// is taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact positive
    /// difference. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using
    /// [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_prec_val_ref(&Float::from(1u32), 10);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.positive_difference_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the specified precision. The first [`Float`]
    /// is taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded result is less than, equal to, or greater than the exact positive
    /// difference. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using
    /// [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_prec_ref_val(Float::from(1u32), 10);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.positive_difference_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the specified precision. Both [`Float`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded result
    /// is less than, equal to, or greater than the exact positive difference. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using
    /// [`Float::positive_difference`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_prec_ref_ref(&Float::from(1u32), 10);
    /// assert_eq!(d.to_string(), "2.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.positive_difference_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the maximum of the precisions of the inputs, with the specified
    /// rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// positive difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::positive_difference`] instead.
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
    /// Panics if `rm` is `Exact` and the positive difference is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_round(Float::from(1u32), Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn positive_difference_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.positive_difference_prec_round(other, prec, rm)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the maximum of the precisions of the inputs, with the specified
    /// rounding mode. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::positive_difference`] instead.
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
    /// Panics if `rm` is `Exact` and the positive difference is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_round_val_ref(&Float::from(1u32), Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn positive_difference_round_val_ref(
        self,
        other: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.positive_difference_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the maximum of the precisions of the inputs, with the specified
    /// rounding mode. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::positive_difference`] instead.
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
    /// Panics if `rm` is `Exact` and the positive difference is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_round_ref_val(Float::from(1u32), Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn positive_difference_round_ref_val(
        &self,
        other: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.positive_difference_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the maximum of the precisions of the inputs, with the specified
    /// rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// positive difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::positive_difference`] instead.
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
    /// Panics if `rm` is `Exact` and the positive difference is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_round_ref_ref(&Float::from(1u32), Floor);
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn positive_difference_round_ref_ref(
        &self,
        other: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.positive_difference_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the maximum of the precisions of the inputs.
    /// Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded result is less than, equal to, or greater than the exact positive difference.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::positive_difference_round`] instead.
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
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference(Float::from(1u32));
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) = Float::from(3u32).positive_difference(Float::from(5u32));
    /// assert_eq!(d.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference(self, other: Self) -> (Self, Ordering) {
        self.positive_difference_round(other, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the maximum of the precisions of the inputs.
    /// The first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::positive_difference_round`] instead.
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
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_val_ref(&Float::from(1u32));
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_val_ref(&Float::from(5u32));
    /// assert_eq!(d.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference_val_ref(self, other: &Self) -> (Self, Ordering) {
        self.positive_difference_round_val_ref(other, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the maximum of the precisions of the inputs.
    /// The first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::positive_difference_round`] instead.
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
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_ref_val(Float::from(1u32));
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_ref_val(Float::from(5u32));
    /// assert_eq!(d.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference_ref_val(&self, other: Self) -> (Self, Ordering) {
        self.positive_difference_round_ref_val(other, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s — $x-y$ if $x>y$, and $+0.0$ otherwise
    /// — rounding the result to the nearest value of the maximum of the precisions of the inputs.
    /// Both [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded result is less than, equal to, or greater than the exact positive difference.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::positive_difference_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::positive_difference_round`] instead.
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
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_ref_ref(&Float::from(1u32));
    /// assert_eq!(d.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (d, o) = Float::from(3u32).positive_difference_ref_ref(&Float::from(5u32));
    /// assert_eq!(d.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn positive_difference_ref_ref(&self, other: &Self) -> (Self, Ordering) {
        self.positive_difference_round_ref_ref(other, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the positive difference is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign(Float::from(1u32), 10, Floor),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0000");
    /// ```
    pub fn positive_difference_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        if matches!((*self).partial_cmp(&other), Some(Greater)) {
            self.sub_prec_round_assign(other, prec, rm)
        } else if matches!(self, Self(NaN)) || matches!(other, Self(NaN)) {
            *self = float_nan!();
            Equal
        } else {
            *self = Self::ZERO;
            Equal
        }
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the positive difference is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// let y = Float::from(1u32);
    /// assert_eq!(
    ///     x.positive_difference_prec_round_assign_ref(&y, 10, Floor),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0000");
    /// ```
    pub fn positive_difference_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        if matches!((*self).partial_cmp(other), Some(Greater)) {
            self.sub_prec_round_assign_ref(other, prec, rm)
        } else if matches!(self, Self(NaN)) || matches!(other, Self(NaN)) {
            *self = float_nan!();
            Equal
        } else {
            *self = Self::ZERO;
            Equal
        }
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the nearest value of the specified precision. The
    /// [`Float`] on the right-hand side is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// positive difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// assert_eq!(
    ///     x.positive_difference_prec_assign(Float::from(1u32), 10),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0000");
    /// ```
    #[inline]
    pub fn positive_difference_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.positive_difference_prec_round_assign(other, prec, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the nearest value of the specified precision. The
    /// [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded result is less than, equal to, or greater than the exact
    /// positive difference. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// assert_eq!(
    ///     x.positive_difference_prec_assign_ref(&Float::from(1u32), 10),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0000");
    /// ```
    #[inline]
    pub fn positive_difference_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.positive_difference_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the maximum of the precisions of the inputs, with the
    /// specified rounding mode. The [`Float`] on the right-hand side is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
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
    /// Panics if `rm` is `Exact` and the positive difference is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// assert_eq!(
    ///     x.positive_difference_round_assign(Float::from(1u32), Floor),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    pub fn positive_difference_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.positive_difference_prec_round_assign(other, prec, rm)
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the maximum of the precisions of the inputs, with the
    /// specified rounding mode. The [`Float`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded result is less than, equal to,
    /// or greater than the exact positive difference. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p+1}$.
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
    /// Panics if `rm` is `Exact` and the positive difference is not exactly representable with the
    /// output precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// assert_eq!(
    ///     x.positive_difference_round_assign_ref(&Float::from(1u32), Floor),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    pub fn positive_difference_round_assign_ref(
        &mut self,
        other: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.positive_difference_prec_round_assign_ref(other, prec, rm)
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the nearest value of the maximum of the precisions of
    /// the inputs. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded result is less than, equal to, or greater than the
    /// exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
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
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// assert_eq!(x.positive_difference_assign(Float::from(1u32)), Equal);
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    #[inline]
    pub fn positive_difference_assign(&mut self, other: Self) -> Ordering {
        self.positive_difference_round_assign(other, Nearest)
    }

    /// Computes the positive difference of two [`Float`]s in place — $x-y$ if $x>y$, and $+0.0$
    /// otherwise — rounding the result to the nearest value of the maximum of the precisions of
    /// the inputs. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded result is less than, equal to, or greater than
    /// the exact positive difference. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is the positive difference, `mpfr_dim` and C's `fdim`. Zero is returned for $x\leq y$
    /// as a matter of definition — negative values are representable, but the function chooses
    /// $+0.0$ instead — so this is not a saturating subtraction. The comparison treats zeros of
    /// both signs as equal and infinities as their usual extremes.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$
    /// - $f(x,y,p)=+0.0$ if $x\leq y$, including $f(\pm0.0,\pm0.0,p)$ and $f(\infty,\infty,p)$
    /// - $f(\infty,y,p)=\infty$ if $y$ is not `NaN` and $y\neq\infty$
    /// - $f(x,-\infty,p)=\infty$ if $x$ is not `NaN` and $x\neq-\infty$
    ///
    /// Overflow and underflow are as for subtraction:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// $$
    /// f(x,y,p) = \begin{cases} x-y+\varepsilon & x>y \\\ +0.0 & \text{otherwise,} \end{cases}
    /// $$
    /// - If $x\leq y$ or the exact difference is representable, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 (x-y)\rfloor-p}$.
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
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3u32);
    /// assert_eq!(x.positive_difference_assign_ref(&Float::from(1u32)), Equal);
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    #[inline]
    pub fn positive_difference_assign_ref(&mut self, other: &Self) -> Ordering {
        self.positive_difference_round_assign_ref(other, Nearest)
    }
}

/// Computes the positive difference of two primitive floats — $x-y$ if $x>y$, and $+0.0$
/// otherwise — using emulated [`Float`] arithmetic.
///
/// This is C's `fdim`, which the standard library does not provide. For finite operands the result
/// equals `x - y` when `x > y` (the primitive subtraction is already correctly rounded) and a
/// positive zero otherwise; a NaN input gives NaN.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::positive_difference::*;
///
/// assert_eq!(
///     NiceFloat(primitive_float_positive_difference(3.0, 1.0)),
///     NiceFloat(2.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_positive_difference(1.0, 3.0)),
///     NiceFloat(0.0)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_positive_difference<T: PrimitiveFloat>(x: T, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(Float::positive_difference_prec, x, y)
}
