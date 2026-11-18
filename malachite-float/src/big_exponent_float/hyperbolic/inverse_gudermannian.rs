// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::bracket::{
    Bracket, ZIV_GUARD_BITS, floor_bracket, rational_exponent_bound, ziv_loop_certified_from,
};
use crate::big_exponent_float::hyperbolic::gudermannian::{GdTarget, odd_function_near_identity};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::{NaN as NaNTrait, NegativeOne, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The logarithm of an enclosure `y` at Ziv level `w`: the logarithm at the lower end, with the
// upper end raised by (hi - lo) / lo >= ln(hi / lo). When the lower end is not above 1, so that its
// logarithm is not positive, the result is the dummy bracket [-1, 1], which Ziv's loop never
// accepts.
//
// This is equivalent to `lnOfEncl` from `AzFloat/LnEncl.lean`, Azurite, with the correctly rounded
// logarithm in place of Azurite's bracket.
fn ln_of_encl(y: &Bracket, w: u64) -> Bracket {
    if y.0 <= 1u32 {
        // The working precision exceeds the number of leading zeros of the argument, so the lower
        // end of the enclosure of (1 + sin a) / cos a is above 1.
        fail_on_untested_path("ln_of_encl, lower end not above 1");
        return (BigExponentFloat::NEGATIVE_ONE, BigExponentFloat::ONE);
    }
    let prec = w + 12;
    let (lo, mut hi) = floor_bracket(y.0.ln_prec_round_ref(prec, Floor).0, prec);
    let width =
        y.1.sub_prec_round_ref_ref(&y.0, 64, Ceiling)
            .0
            .div_prec_round_val_ref(&y.0, 64, Ceiling)
            .0;
    hi.add_prec_round_assign(width, prec, Ceiling);
    (lo, hi)
}

// The bracket of the inverse Gudermannian of a at Ziv level `w`, for 0 < a < pi / 2: ln((1 + sin a)
// / cos a).
//
// This is equivalent to `gdInvBracket`, `gdInvRatBracket`, `gdInvOfSinCos`, and `gdInvNum` from
// `AzFloat/GdInvBracket.lean`, Azurite. Azurite halves 1 + sin a before the division and doubles
// the quotient, and gives a dummy bracket when its enclosure of the cosine contains 0; the
// correctly rounded cosine used here is positive.
fn gd_inv_bracket(target: &GdTarget, w: u64) -> Bracket {
    let prec = w + 12;
    let (s, c) = target.sin_cos(prec);
    let lo = BigExponentFloat::ONE
        .add_prec_round(s.0, prec, Floor)
        .0
        .div_prec_round(c.1, prec, Floor)
        .0;
    let hi = BigExponentFloat::ONE
        .add_prec_round(s.1, prec, Ceiling)
        .0
        .div_prec_round(c.0, prec, Ceiling)
        .0;
    ln_of_encl(&(lo, hi), w)
}

// The first working precision of Ziv's loop at target precision `prec`, for a < 2^e. The logarithm
// has absolute accuracy and the result is about a for a small a, so the loop starts with as many
// more bits as a has leading zeros.
//
// An exponent too small for the sum to fit in a `u64` belongs to an argument that takes the
// tiny-argument path, which does not use this value.
//
// This is equivalent to `leadingZeroBits` from `AzFloat/LnP1.lean`, Azurite.
fn gd_inv_start(e: &Integer, prec: u64) -> u64 {
    let start = prec + ZIV_GUARD_BITS;
    if *e < 0u32 {
        start.saturating_add(u64::saturating_from(&-e))
    } else {
        start
    }
}

// The inverse Gudermannian of x, rounded to `prec` bits with `rm`.
//
// This is equivalent to `gdInvPrecRound` from `AzFloat/GdInv.lean` and `gdInvInDomain` from
// `AzFloat/GdInvBracket.lean`, Azurite.
pub(crate) fn inverse_gudermannian_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(NaN | Infinity { .. }) => (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Zero) => (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => {
            let x_prec = significand_prec(significand);
            let a = x.abs();
            // |x| < pi / 2 when |x| < 1; and otherwise exactly when |x| is at most pi / 2 rounded
            // down to the precision of x, the largest such float below pi / 2.
            if *exponent > 1u32
                || *exponent == 1u32 && a > BigExponentFloat::pi_over_2_prec_round(x_prec, Floor).0
            {
                return (BigExponentFloat::NAN, Equal);
            }
            assert_ne!(rm, Exact, "Inexact inverse Gudermannian function");
            let target = GdTarget::BigExponentFloat(&a, exponent);
            odd_function_near_identity(
                *sign,
                &target,
                true,
                max(prec, x_prec) + ZIV_GUARD_BITS,
                prec,
                rm,
                &|rm| {
                    ziv_loop_certified_from(
                        &|w| gd_inv_bracket(&target, w),
                        prec,
                        rm,
                        gd_inv_start(exponent, prec),
                    )
                },
            )
        }
    }
}

// The inverse Gudermannian of a `Rational` q, rounded to `prec` bits with `rm`.
//
// This is equivalent to `gdInvRatPrecRound` from `AzFloat/GdInvRat.lean` and `gdInvRatInDomain`
// from `AzFloat/GdInvBracket.lean`, Azurite.
pub(crate) fn inverse_gudermannian_rational_helper(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *q == 0u32 {
        return (BigExponentFloat::ZERO, Equal);
    }
    let a = q.abs();
    // |q| < pi / 2 when |q| <= 1, and not when |q| >= 2; in between, exactly when cos |q| is
    // positive. The cosine of a nonzero rational is irrational, so it is not 0, and its rounding
    // down to one bit is negative exactly when it is.
    if a >= 2u32 || a > 1u32 && BigExponentFloat::cos_rational_prec_round_ref(&a, 1, Floor).0 < 0u32
    {
        return (BigExponentFloat::NAN, Equal);
    }
    assert_ne!(rm, Exact, "Inexact inverse Gudermannian function");
    let bound = rational_exponent_bound(&a);
    let target = GdTarget::Rational(&a, &bound);
    odd_function_near_identity(
        *q > 0u32,
        &target,
        true,
        prec + ZIV_GUARD_BITS,
        prec,
        rm,
        &|rm| {
            ziv_loop_certified_from(
                &|w| gd_inv_bracket(&target, w),
                prec,
                rm,
                gd_inv_start(&bound, prec),
            )
        },
    )
}

impl BigExponentFloat {
    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_prec`] instead.
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
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.226192");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_gudermannian_helper(&self, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_prec_ref`] instead.
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
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.226192");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_gudermannian_helper(self, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN`
    /// it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::inverse_gudermannian_prec_round`] documentation for information
    /// on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_prec_assign`] instead.
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
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.inverse_gudermannian_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(
    ///     x.inverse_gudermannian_prec_round_assign(5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(
    ///     x.inverse_gudermannian_prec_round_assign(5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.inverse_gudermannian_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.2261906");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(
    ///     x.inverse_gudermannian_prec_round_assign(20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.226192");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.inverse_gudermannian_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.2261906");
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (mut g, o) = inverse_gudermannian_helper(self, prec, rm);
        swap(self, &mut g);
        o
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_prec_round`] instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec(5);
    /// assert_eq!(w.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec(20);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    ///
    /// let x = BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// let (w, o) = x.inverse_gudermannian_prec(20);
    /// assert_eq!(w.to_string(), "9.2835e-4342944820");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec(self, prec: u64) -> (Self, Ordering) {
        self.inverse_gudermannian_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_prec_round_ref`] instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_ref(5);
    /// assert_eq!(w.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.inverse_gudermannian_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    ///
    /// let x = BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// let (w, o) = x.inverse_gudermannian_prec_ref(20);
    /// assert_eq!(w.to_string(), "9.2835e-4342944820");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.inverse_gudermannian_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a
    /// [`BigExponentFloat`] in place, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::inverse_gudermannian_prec_round`] documentation for information
    /// on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_prec_round_assign`] instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.inverse_gudermannian_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.2");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.inverse_gudermannian_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.2261906");
    ///
    /// let mut x = BigExponentFloat::from(-1.0e10).exp_prec(10).0;
    /// assert_eq!(x.inverse_gudermannian_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "9.2835e-4342944820");
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_assign(&mut self, prec: u64) -> Ordering {
        self.inverse_gudermannian_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::inverse_gudermannian_rational_prec_round),
    /// this function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_rational_prec`] instead.
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
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.6396217");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn inverse_gudermannian_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_gudermannian_rational_helper(&x, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::inverse_gudermannian_rational_prec_round),
    /// this function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_rational_prec_ref`] instead.
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
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.6396217");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_gudermannian_rational_helper(x, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::inverse_gudermannian_rational_prec_round),
    /// this function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_rational_prec_round`] instead.
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
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    /// );
    /// assert_eq!(w.to_string(), "0.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::inverse_gudermannian_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::inverse_gudermannian_rational_prec_round),
    /// this function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::inverse_gudermannian_rational_prec_round_ref`] instead.
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
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    /// );
    /// assert_eq!(w.to_string(), "0.62");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::inverse_gudermannian_rational_prec_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::inverse_gudermannian_rational_prec_round_ref(x, prec, Nearest)
    }
}
