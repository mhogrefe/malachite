// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::float::arithmetic::cos::round_bracket;
use core::cmp::{Ordering, min};
use malachite_base::num::arithmetic::traits::{Abs, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{DottieNumber, One};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

impl Float {
    /// Returns an approximation of the Dottie number, the unique real fixed point of the cosine,
    /// with the given precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than or greater than the exact
    /// value of the constant. (Since the constant is irrational, the rounded value is never equal
    /// to the exact value.)
    ///
    /// $$
    /// x = d+\varepsilon, \quad \text{where } \cos d = d.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental (if $d$ were algebraic, $\cos d$ would be
    /// transcendental by the Lindemann-Weierstrass theorem, and could not equal $d$).
    ///
    /// The output has precision `prec`.
    ///
    /// The root of $x - \cos x$ is found by Newton's method with the working precision doubled at
    /// each step, and the final iterate is certified by bounding the residual $x - \cos x$ with a
    /// correctly rounded cosine, so that the result is correctly rounded rather than merely the
    /// fixed point of a rounded cosine.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (dottie_number, o) = Float::dottie_number_prec_round(100, Floor);
    /// assert_eq!(
    ///     dottie_number.to_string(),
    ///     "0.73908513321516064165531208767346"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// let (dottie_number, o) = Float::dottie_number_prec_round(100, Ceiling);
    /// assert_eq!(
    ///     dottie_number.to_string(),
    ///     "0.73908513321516064165531208767425"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    pub fn dottie_number_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact, "Inexact Dottie number");
        let mut w = prec + 10;
        let mut increment = Limb::WIDTH;
        // Newton's method on f(x) = x - cos x, whose derivative 1 + sin x is about 1.67 at the
        // root, from a double-precision seed; convergence is quadratic, so each step runs at twice
        // the number of bits the previous iterate got right, and the whole iteration costs little
        // more than its last step.
        let mut x = Self::from(f64::DOTTIE_NUMBER);
        let mut correct = 50;
        loop {
            while correct + 2 < w {
                let p = min(correct << 1, w);
                let (s, c, _, _) = x.sin_cos_prec_ref(p);
                let t = x.sub_prec_ref_val(c, p).0;
                let u = s.add_prec(Self::ONE, p).0;
                x.sub_prec_assign(t.div_round(u, Nearest).0, p);
                // the step's error is dominated by the rounding of its cosine
                correct = p - 2;
            }
            // Certification: c = cos x rounded to nearest is within 2^(-w-1) of cos x (c < 1, so
            // its ulp is 2^-w), so the residual x - cos x is within that of x - c, computed
            // exactly; and by the mean value theorem |d - x| <= |x - cos x| / min(1 + sin) over [x,
            // d], where 1 + sin >= 1.6 on [0.7, 0.8] (sin 0.7 > 0.64), so the bracket [x - e, x +
            // e] with e = (|x - c| + 2^(-w-1)) * 5/8 contains d.
            assert!(x > 0.7f64 && x < 0.8f64);
            let c = x.cos_prec_ref(w).0;
            let xr = Rational::exact_from(&x);
            let e = ((&xr - Rational::exact_from(&c)).abs()
                + Rational::power_of_2(-i64::exact_from(w) - 1))
                * const { Rational::const_from_unsigneds(5, 8) };
            if let Some(result) = round_bracket(&(&xr - &e), &(xr + e), prec, rm) {
                return result;
            }
            w += increment;
            increment = w >> 1;
        }
    }

    /// Returns an approximation of the Dottie number, the unique real fixed point of the cosine,
    /// with the given precision and rounded to the nearest [`Float`] of that precision. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than or greater
    /// than the exact value of the constant. (Since the constant is irrational, the rounded value
    /// is never equal to the exact value.)
    ///
    /// $$
    /// x = d+\varepsilon, \quad \text{where } \cos d = d.
    /// $$
    /// - $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (dottie_number, o) = Float::dottie_number_prec(1);
    /// assert_eq!(dottie_number.to_string(), "0.50");
    /// assert_eq!(o, Less);
    ///
    /// let (dottie_number, o) = Float::dottie_number_prec(10);
    /// assert_eq!(dottie_number.to_string(), "0.73926");
    /// assert_eq!(o, Greater);
    ///
    /// let (dottie_number, o) = Float::dottie_number_prec(100);
    /// assert_eq!(
    ///     dottie_number.to_string(),
    ///     "0.73908513321516064165531208767425"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn dottie_number_prec(prec: u64) -> (Self, Ordering) {
        Self::dottie_number_prec_round(prec, Nearest)
    }
}
