// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "2.153290");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (w, o) = (-self).lambert_w_minus_1_prec_round(prec, -rm);
        (-w, o.reverse())
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
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
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "2.153290");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (w, o) = (-self).lambert_w_minus_1_prec_round(prec, -rm);
        (-w, o.reverse())
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a
    /// [`BigExponentFloat`] in place, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
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
    /// See the [`BigExponentFloat::tree_t_minus_1_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_prec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.25);
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = BigExponentFloat::from(0.25);
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.25");
    ///
    /// let mut x = BigExponentFloat::from(0.25);
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = BigExponentFloat::from(0.25);
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "2.153290");
    ///
    /// let mut x = BigExponentFloat::from(0.25);
    /// assert_eq!(x.tree_t_minus_1_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "2.1532936");
    ///
    /// let mut x = BigExponentFloat::from(0.25);
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

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec(5);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec(20);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    ///
    /// let x = BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// let (w, o) = x.tree_t_minus_1_prec(20);
    /// assert_eq!(w.to_string(), "1.00000e10");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        self.tree_t_minus_1_prec_round(prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_ref(5);
    /// assert_eq!(w.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(0.25).tree_t_minus_1_prec_ref(20);
    /// assert_eq!(w.to_string(), "2.1532936");
    /// assert_eq!(o, Greater);
    ///
    /// let x = BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// let (w, o) = x.tree_t_minus_1_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.00000e10");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.tree_t_minus_1_prec_round_ref(prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a
    /// [`BigExponentFloat`] in place, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::tree_t_minus_1_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.25);
    /// assert_eq!(x.tree_t_minus_1_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "2.12");
    ///
    /// let mut x = BigExponentFloat::from(0.25);
    /// assert_eq!(x.tree_t_minus_1_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "2.1532936");
    ///
    /// let mut x = BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// assert_eq!(x.tree_t_minus_1_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.00000e10");
    /// ```
    #[inline]
    pub fn tree_t_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        self.tree_t_minus_1_prec_round_assign(prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_rational_prec`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "2.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "2.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "2.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "2.542641");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "2.5426445");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "2.542641");
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
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_rational_prec_ref`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "2.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "2.62");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "2.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "2.542641");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "2.5426445");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "2.542641");
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
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_rational_prec_round`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) =
    ///     BigExponentFloat::tree_t_minus_1_rational_prec(Rational::from_unsigneds(1u8, 5), 5);
    /// assert_eq!(w.to_string(), "2.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::tree_t_minus_1_rational_prec(Rational::from_unsigneds(1u8, 5), 20);
    /// assert_eq!(w.to_string(), "2.542641");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::tree_t_minus_1_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $T_{-1}(x)$, the other real branch of the tree function $T$, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The tree function is $T(x) = -W(-x)$, where $W$ is the Lambert $W$ function. $T_{-1}(x) =
    /// -W_{-1}(-x)$ is the solution $t \geq 1$ of $te^{-t} = x$, which exists when $0 < x \leq
    /// 1/e$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = T_{-1}(x)+\varepsilon.
    /// $$
    /// - If $T_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $T_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |T_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$ or $x>1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::tree_t_minus_1_rational_prec_round_ref`] instead.
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
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     5,
    /// );
    /// assert_eq!(w.to_string(), "2.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::tree_t_minus_1_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(w.to_string(), "2.542641");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn tree_t_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::tree_t_minus_1_rational_prec_round_ref(x, prec, Nearest)
    }
}
