// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::exponential::traits::{TreeTMinus1, TreeTMinus1Assign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl Float {
    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never overflows or underflows: $T_{-1}(x)\geq 1$, and for $2^{-2^{30}}$, the
    /// positive [`Float`] closest to zero, $T_{-1}(x)$ is only about $7.4\cdot10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tree_t_minus_1_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::tree_t_minus_1_round`] instead. If both of these things are true, consider
    /// using [`Float::tree_t_minus_1`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "2.1532898");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (w, o) = (-self).lambert_w_minus_1_prec_round(prec, -rm);
        (-w, o.reverse())
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never overflows or underflows: $T_{-1}(x)\geq 1$, and for $2^{-2^{30}}$, the
    /// positive [`Float`] closest to zero, $T_{-1}(x)$ is only about $7.4\cdot10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tree_t_minus_1_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::tree_t_minus_1_round_ref`] instead. If both of these things are true,
    /// consider using `(&Float).tree_t_minus_1()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "2.1532898");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (w, o) = (-self).lambert_w_minus_1_prec_round(prec, -rm);
        (-w, o.reverse())
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`],
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never overflows or underflows: $T_{-1}(x)\geq 1$, and for $2^{-2^{30}}$, the
    /// positive [`Float`] closest to zero, $T_{-1}(x)$ is only about $7.4\cdot10^8$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::tree_t_minus_1`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec(5);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec(20);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        self.tree_t_minus_1_prec_round(prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`],
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never overflows or underflows: $T_{-1}(x)\geq 1$, and for $2^{-2^{30}}$, the
    /// positive [`Float`] closest to zero, $T_{-1}(x)$ is only about $7.4\cdot10^8$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using `(&Float).tree_t_minus_1()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_ref(5);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_prec_ref(20);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.tree_t_minus_1_prec_round_ref(prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`],
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\infty$
    /// - $f(x,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::tree_t_minus_1_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::tree_t_minus_1`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_round(Floor);
    /// assert_eq!(w.to_string(), "2.1532923641103496491690991500910");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_round(Ceiling);
    /// assert_eq!(w.to_string(), "2.1532923641103496491690991500941");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_round(Nearest);
    /// assert_eq!(w.to_string(), "2.1532923641103496491690991500941");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.tree_t_minus_1_prec_round(prec, rm)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`],
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\infty$
    /// - $f(x,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::tree_t_minus_1_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).tree_t_minus_1()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_round_ref(Floor);
    /// assert_eq!(w.to_string(), "2.1532923641103496491690991500910");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_round_ref(Ceiling);
    /// assert_eq!(w.to_string(), "2.1532923641103496491690991500941");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_primitive_float_prec(0.25, 100)
    ///     .0
    ///     .tree_t_minus_1_round_ref(Nearest);
    /// assert_eq!(w.to_string(), "2.1532923641103496491690991500941");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.tree_t_minus_1_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`] in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::tree_t_minus_1_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::tree_t_minus_1_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::tree_t_minus_1_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::tree_t_minus_1_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.25");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.1532898");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.1532936");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.1532936");
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        self.neg_assign();
        let o = self.lambert_w_minus_1_prec_round_assign(prec, -rm);
        self.neg_assign();
        o.reverse()
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`] in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::tree_t_minus_1_prec_round`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_prec_round_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::tree_t_minus_1_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "2.1532936");
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        self.tree_t_minus_1_prec_round_assign(prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`] in
    /// place, rounding the result with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::tree_t_minus_1_prec_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::tree_t_minus_1_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::tree_t_minus_1_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "2.1532923641103496491690991500910");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.1532923641103496491690991500941");
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// assert_eq!(x.tree_t_minus_1_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "2.1532923641103496491690991500941");
    /// ```
    #[inline]
    pub fn tree_t_minus_1_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.tree_t_minus_1_prec_round_assign(prec, rm)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never underflows, since $T_{-1}(x)\geq 1$. It does not overflow either:
    /// $T_{-1}(x)$ grows like $-\ln x$ as $x$ approaches zero, so an overflow would take an
    /// argument that is too close to zero to fit in memory.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::tree_t_minus_1_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) =
    ///     Float::tree_t_minus_1_rational_prec_round(Rational::from_unsigneds(1u8, 5), 5, Floor);
    /// assert_eq!(w.to_string(), "2.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::tree_t_minus_1_rational_prec_round(Rational::from_unsigneds(1u8, 5), 5, Ceiling);
    /// assert_eq!(w.to_string(), "2.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::tree_t_minus_1_rational_prec_round(Rational::from_unsigneds(1u8, 5), 5, Nearest);
    /// assert_eq!(w.to_string(), "2.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::tree_t_minus_1_rational_prec_round(Rational::from_unsigneds(1u8, 5), 20, Floor);
    /// assert_eq!(w.to_string(), "2.5426407");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "2.5426445");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "2.5426407");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (w, o) = Self::lambert_w_minus_1_rational_prec_round(-x, prec, -rm);
        (-w, o.reverse())
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded value is less than, equal to, or greater
    /// than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p+1}$.
    /// - If $T_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never underflows, since $T_{-1}(x)\geq 1$. It does not overflow either:
    /// $T_{-1}(x)$ grows like $-\ln x$ as $x$ approaches zero, so an overflow would take an
    /// argument that is too close to zero to fit in memory.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::tree_t_minus_1_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "2.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "2.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "2.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "2.5426407");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "2.5426445");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "2.5426407");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (w, o) = Self::lambert_w_minus_1_rational_prec_round(-x, prec, -rm);
        (-w, o.reverse())
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never underflows, since $T_{-1}(x)\geq 1$. It does not overflow either:
    /// $T_{-1}(x)$ grows like $-\ln x$ as $x$ approaches zero, so an overflow would take an
    /// argument that is too close to zero to fit in memory.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (w, o) = Float::tree_t_minus_1_rational_prec(Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(w.to_string(), "2.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec(Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(w.to_string(), "2.5426407");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::tree_t_minus_1_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// This function never underflows, since $T_{-1}(x)\geq 1$. It does not overflow either:
    /// $T_{-1}(x)$ grows like $-\ln x$ as $x$ approaches zero, so an overflow would take an
    /// argument that is too close to zero to fit in memory.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(w.to_string(), "2.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::tree_t_minus_1_rational_prec_ref(&Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(w.to_string(), "2.5426407");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::tree_t_minus_1_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl TreeTMinus1 for Float {
    type Output = Self;

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`], taking
    /// it by value.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\infty$
    /// - $f(x)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::tree_t_minus_1_prec`]. If you want both of these things, consider
    /// using [`Float::tree_t_minus_1_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_base::num::exponential::traits::TreeTMinus1;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.tree_t_minus_1().is_nan());
    /// assert_eq!(Float::ZERO.tree_t_minus_1(), Float::INFINITY);
    /// assert!(Float::ONE.tree_t_minus_1().is_nan());
    /// assert_eq!(
    ///     Float::from_primitive_float_prec(0.25, 100)
    ///         .0
    ///         .tree_t_minus_1()
    ///         .to_string(),
    ///     "2.1532923641103496491690991500941"
    /// );
    /// ```
    #[inline]
    fn tree_t_minus_1(self) -> Self {
        let prec = self.significant_bits();
        self.tree_t_minus_1_prec(prec).0
    }
}

impl TreeTMinus1 for &Float {
    type Output = Float;

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`], taking
    /// it by reference.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\infty$
    /// - $f(x)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::tree_t_minus_1_prec_ref`]. If you want both of these things,
    /// consider using [`Float::tree_t_minus_1_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_base::num::exponential::traits::TreeTMinus1;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).tree_t_minus_1().is_nan());
    /// assert_eq!((&Float::ZERO).tree_t_minus_1(), Float::INFINITY);
    /// assert!((&Float::ONE).tree_t_minus_1().is_nan());
    /// assert_eq!(
    ///     (&Float::from_primitive_float_prec(0.25, 100).0)
    ///         .tree_t_minus_1()
    ///         .to_string(),
    ///     "2.1532923641103496491690991500941"
    /// );
    /// ```
    #[inline]
    fn tree_t_minus_1(self) -> Float {
        self.tree_t_minus_1_prec_ref(self.significant_bits()).0
    }
}

impl TreeTMinus1Assign for Float {
    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Float`] in
    /// place.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::tree_t_minus_1`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::tree_t_minus_1_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::tree_t_minus_1_prec_assign`]. If you want both of these things,
    /// consider using [`Float::tree_t_minus_1_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_base::num::exponential::traits::TreeTMinus1Assign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.tree_t_minus_1_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.tree_t_minus_1_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::ONE;
    /// x.tree_t_minus_1_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_primitive_float_prec(0.25, 100).0;
    /// x.tree_t_minus_1_assign();
    /// assert_eq!(x.to_string(), "2.1532923641103496491690991500941");
    /// ```
    #[inline]
    fn tree_t_minus_1_assign(&mut self) {
        let prec = self.significant_bits();
        self.tree_t_minus_1_prec_assign(prec);
    }
}

/// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a primitive float.
///
/// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
/// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq 1/e$.
///
/// $$
/// f(x) = T_{-1}(x)+\varepsilon.
/// $$
/// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\infty$
/// - $f(x)=\text{NaN}$ if $x<0$ or $x>1/e$
///
/// Underflow is not possible, since $T_{-1}(x)\geq 1$, and neither is overflow, since $T_{-1}(x)$
/// only grows like $-\ln x$ as $x$ approaches zero.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::exponential::tree_t_minus_1::primitive_float_tree_t_minus_1;
///
/// assert!(primitive_float_tree_t_minus_1(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_tree_t_minus_1(0.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert!(primitive_float_tree_t_minus_1(-1.0f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_tree_t_minus_1(0.25f32)),
///     NiceFloat(2.1532924)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_tree_t_minus_1<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::tree_t_minus_1_prec, x)
}

/// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Rational`],
/// returning the result as a primitive float.
///
/// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
/// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq 1/e$.
///
/// $$
/// f(x) = T_{-1}(x)+\varepsilon.
/// $$
/// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |T_{-1}(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=\infty$
/// - $f(x)=\text{NaN}$ if $x<0$ or $x>1/e$
///
/// Underflow is not possible, since $T_{-1}(x)\geq 1$, and neither is overflow, since $T_{-1}(x)$
/// only grows like $-\ln x$ as $x$ approaches zero.
///
/// # Worst-case complexity
/// $T(n) = O(n^{3/2} \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::exponential::tree_t_minus_1::*;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_tree_t_minus_1_rational::<f64>(
///         &Rational::ZERO
///     )),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_tree_t_minus_1_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 5)
///     )),
///     NiceFloat(2.5426413577735265)
/// );
/// assert!(primitive_float_tree_t_minus_1_rational::<f64>(&Rational::NEGATIVE_ONE).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_tree_t_minus_1_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::tree_t_minus_1_rational_prec_ref, x)
}
