// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::exponential::lambert_w0::{LambertTarget, lambert_w_neg};
use core::cmp::Ordering::{self, *};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::{NaN as NaNTrait, NegativeInfinity};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

// W_{-1}(x), rounded to `prec` bits with `rm`.
//
// This is equivalent to `lambertWm1PrecRound` from `AzFloat/LambertW.lean`, Azurite.
pub(crate) fn lambert_w_minus_1_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(NaN | Infinity { .. } | Finite { sign: true, .. }) => {
            (BigExponentFloat::NAN, Equal)
        }
        BigExponentFloat(Zero) => (BigExponentFloat::NEGATIVE_INFINITY, Equal),
        BigExponentFloat(Finite { sign: false, .. }) => {
            lambert_w_neg(false, &LambertTarget::BigExponentFloat(x), prec, rm)
        }
    }
}

// W_{-1}(q) for a `Rational` q, rounded to `prec` bits with `rm`.
//
// This is equivalent to `lambertWm1RatPrecRound` from `AzFloat/LambertW.lean`, Azurite.
pub(crate) fn lambert_w_minus_1_rational_helper(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match q.sign() {
        Equal => (BigExponentFloat::NEGATIVE_INFINITY, Equal),
        Greater => (BigExponentFloat::NAN, Equal),
        Less => lambert_w_neg(false, &LambertTarget::Rational(q), prec, rm),
    }
}

impl BigExponentFloat {
    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p+1}$.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_prec`] instead.
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
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "-2.25");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "-2.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "-2.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "-2.1532936");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "-2.153290");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "-2.1532936");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        lambert_w_minus_1_helper(&self, prec, rm)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p+1}$.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_prec_ref`] instead.
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
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "-2.25");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "-2.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "-2.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "-2.1532936");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "-2.153290");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "-2.1532936");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        lambert_w_minus_1_helper(self, prec, rm)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p+1}$.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::lambert_w_minus_1_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_prec_assign`] instead.
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
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "-2.25");
    ///
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "-2.12");
    ///
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "-2.12");
    ///
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "-2.1532936");
    ///
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "-2.153290");
    ///
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "-2.1532936");
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (mut w, o) = lambert_w_minus_1_helper(self, prec, rm);
        swap(self, &mut w);
        o
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_prec_round`] instead.
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
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec(5);
    /// assert_eq!(w.to_string(), "-2.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec(20);
    /// assert_eq!(w.to_string(), "-2.1532936");
    /// assert_eq!(o, Less);
    ///
    /// let x = -BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// let (w, o) = x.lambert_w_minus_1_prec(20);
    /// assert_eq!(w.to_string(), "-1.00000e10");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        self.lambert_w_minus_1_prec_round(prec, Nearest)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_prec_round_ref`] instead.
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
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_ref(5);
    /// assert_eq!(w.to_string(), "-2.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::from(-0.25).lambert_w_minus_1_prec_ref(20);
    /// assert_eq!(w.to_string(), "-2.1532936");
    /// assert_eq!(o, Less);
    ///
    /// let x = -BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// let (w, o) = x.lambert_w_minus_1_prec_ref(20);
    /// assert_eq!(w.to_string(), "-1.00000e10");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.lambert_w_minus_1_prec_round_ref(prec, Nearest)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_{-1}(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::lambert_w_minus_1_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_prec_round_assign`] instead.
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
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "-2.12");
    ///
    /// let mut x = BigExponentFloat::from(-0.25);
    /// assert_eq!(x.lambert_w_minus_1_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "-2.1532936");
    ///
    /// let mut x = -BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// assert_eq!(x.lambert_w_minus_1_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "-1.00000e10");
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        self.lambert_w_minus_1_prec_round_assign(prec, Nearest)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p+1}$.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_rational_prec`] instead.
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
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "-2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "-2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "-2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "-2.5426445");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "-2.542641");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round(
    ///     Rational::from_signeds(-1i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "-2.542641");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn lambert_w_minus_1_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        lambert_w_minus_1_rational_helper(&x, prec, rm)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p+1}$.
    /// - If $W_{-1}(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_rational_prec_ref`] instead.
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
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "-2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "-2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "-2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "-2.5426445");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "-2.542641");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "-2.542641");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        lambert_w_minus_1_rational_helper(x, prec, rm)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_rational_prec_round`] instead.
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
    ///     BigExponentFloat::lambert_w_minus_1_rational_prec(Rational::from_signeds(-1i8, 5), 5);
    /// assert_eq!(w.to_string(), "-2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::lambert_w_minus_1_rational_prec(Rational::from_signeds(-1i8, 5), 20);
    /// assert_eq!(w.to_string(), "-2.542641");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::lambert_w_minus_1_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_{-1}(x)$ is the solution $w \leq -1$ of $we^w = x$, which exists when $-1/e \leq x < 0$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_{-1}(x)+\varepsilon.
    /// $$
    /// - If $W_{-1}(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $W_{-1}(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(x,p)=\text{NaN}$ if $x>0$ or $x<-1/e$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref`] instead.
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
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     5,
    /// );
    /// assert_eq!(w.to_string(), "-2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_ref(
    ///     &Rational::from_signeds(-1i8, 5),
    ///     20,
    /// );
    /// assert_eq!(w.to_string(), "-2.542641");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::lambert_w_minus_1_rational_prec_round_ref(x, prec, Nearest)
    }
}
