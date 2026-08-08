// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2020 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{DivExact, ExtendedGcd, Gcd};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;

// Each impl below computes the same quantities, differing only in which operands are consumed. The
// quotients are those of _fmpq_gcd_cofactors from fmpq/gcd_cofactors.c, FLINT 3.6.0, computed the
// same way: with a = ±p/q and b = ±r/s canonical, g = gcd(p, r)/lcm(q, s), a/g = ±(p/gcd(p,
// r))(s/gcd(q, s)), and b/g = ±(r/gcd(p, r))(q/gcd(q, s)).

impl ExtendedGcd for Rational {
    type Gcd = Self;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Rational`]s $a$ and $b$, and also the
    /// integer coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. Both [`Rational`]s
    /// are taken by value.
    ///
    /// The integer combinations of two rationals are exactly the integer multiples of their GCD, so
    /// integer Bézout coefficients exist. They are the coefficients of the integer
    /// [`extended_gcd`](malachite_base::num::arithmetic::traits::ExtendedGcd) applied to the
    /// coprime integer quotients $a/g$ and $b/g$, and inherit its normalization.
    ///
    /// $f(a, b) = (g, x, y)$, where $g = \gcd(a, b) \geq 0$ and $ax + by = g$, and $f(0, 0) = (0,
    /// 0, 0)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// // -1 * 2/3 + -1 * -3/4 = 1/12
    /// assert_eq!(
    ///     Rational::from_signeds(2, 3)
    ///         .extended_gcd(Rational::from_signeds(-3, 4))
    ///         .to_debug_string(),
    ///     "(1/12, -1, -1)"
    /// );
    /// ```
    ///
    /// This is fmpq_gcd_cofactors from fmpq/gcd_cofactors.c, FLINT 3.6.0, except that FLINT returns
    /// the quotients $a/g$ and $b/g$ themselves, while this function takes one more step and
    /// returns Bézout coefficients for them.
    fn extended_gcd(self, other: Self) -> (Self, Integer, Integer) {
        let ng = (&self.numerator).gcd(&other.numerator);
        if ng == 0u32 {
            // Both numerators are zero, so both rationals are.
            return (Self::ZERO, Integer::ZERO, Integer::ZERO);
        }
        let dg = (&self.denominator).gcd(&other.denominator);
        let dxbar = (&self.denominator).div_exact(&dg);
        let dybar = other.denominator.div_exact(dg);
        let x_over_g =
            Integer::from_sign_and_abs(self.sign, self.numerator.div_exact(&ng) * &dybar);
        let y_over_g =
            Integer::from_sign_and_abs(other.sign, other.numerator.div_exact(&ng) * dxbar);
        let g = Self {
            sign: true,
            denominator: self.denominator * dybar,
            numerator: ng,
        };
        let (one, u, v) = x_over_g.extended_gcd(y_over_g);
        debug_assert!(one == 1u32);
        (g, u, v)
    }
}

impl ExtendedGcd<&Self> for Rational {
    type Gcd = Self;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Rational`]s $a$ and $b$, and also the
    /// integer coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. The first
    /// [`Rational`] is taken by value and the second by reference.
    ///
    /// The integer combinations of two rationals are exactly the integer multiples of their GCD, so
    /// integer Bézout coefficients exist. They are the coefficients of the integer
    /// [`extended_gcd`](malachite_base::num::arithmetic::traits::ExtendedGcd) applied to the
    /// coprime integer quotients $a/g$ and $b/g$, and inherit its normalization.
    ///
    /// $f(a, b) = (g, x, y)$, where $g = \gcd(a, b) \geq 0$ and $ax + by = g$, and $f(0, 0) = (0,
    /// 0, 0)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// // -1 * 2/3 + -1 * -3/4 = 1/12
    /// assert_eq!(
    ///     Rational::from_signeds(2, 3)
    ///         .extended_gcd(&Rational::from_signeds(-3, 4))
    ///         .to_debug_string(),
    ///     "(1/12, -1, -1)"
    /// );
    /// ```
    ///
    /// This is fmpq_gcd_cofactors from fmpq/gcd_cofactors.c, FLINT 3.6.0, except that FLINT returns
    /// the quotients $a/g$ and $b/g$ themselves, while this function takes one more step and
    /// returns Bézout coefficients for them.
    fn extended_gcd(self, other: &Self) -> (Self, Integer, Integer) {
        let ng = (&self.numerator).gcd(&other.numerator);
        if ng == 0u32 {
            // Both numerators are zero, so both rationals are.
            return (Self::ZERO, Integer::ZERO, Integer::ZERO);
        }
        let dg = (&self.denominator).gcd(&other.denominator);
        let dxbar = (&self.denominator).div_exact(&dg);
        let dybar = (&other.denominator).div_exact(dg);
        let x_over_g =
            Integer::from_sign_and_abs(self.sign, self.numerator.div_exact(&ng) * &dybar);
        let y_over_g =
            Integer::from_sign_and_abs(other.sign, (&other.numerator).div_exact(&ng) * dxbar);
        let g = Self {
            sign: true,
            denominator: self.denominator * dybar,
            numerator: ng,
        };
        let (one, u, v) = x_over_g.extended_gcd(y_over_g);
        debug_assert!(one == 1u32);
        (g, u, v)
    }
}

impl ExtendedGcd<Rational> for &Rational {
    type Gcd = Rational;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Rational`]s $a$ and $b$, and also the
    /// integer coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. The first
    /// [`Rational`] is taken by reference and the second by value.
    ///
    /// The integer combinations of two rationals are exactly the integer multiples of their GCD, so
    /// integer Bézout coefficients exist. They are the coefficients of the integer
    /// [`extended_gcd`](malachite_base::num::arithmetic::traits::ExtendedGcd) applied to the
    /// coprime integer quotients $a/g$ and $b/g$, and inherit its normalization.
    ///
    /// $f(a, b) = (g, x, y)$, where $g = \gcd(a, b) \geq 0$ and $ax + by = g$, and $f(0, 0) = (0,
    /// 0, 0)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// // -1 * 2/3 + -1 * -3/4 = 1/12
    /// assert_eq!(
    ///     (&Rational::from_signeds(2, 3))
    ///         .extended_gcd(Rational::from_signeds(-3, 4))
    ///         .to_debug_string(),
    ///     "(1/12, -1, -1)"
    /// );
    /// ```
    ///
    /// This is fmpq_gcd_cofactors from fmpq/gcd_cofactors.c, FLINT 3.6.0, except that FLINT returns
    /// the quotients $a/g$ and $b/g$ themselves, while this function takes one more step and
    /// returns Bézout coefficients for them.
    fn extended_gcd(self, other: Rational) -> (Rational, Integer, Integer) {
        let ng = (&self.numerator).gcd(&other.numerator);
        if ng == 0u32 {
            // Both numerators are zero, so both rationals are.
            return (Rational::ZERO, Integer::ZERO, Integer::ZERO);
        }
        let dg = (&self.denominator).gcd(&other.denominator);
        let dxbar = (&self.denominator).div_exact(&dg);
        let dybar = other.denominator.div_exact(dg);
        let x_over_g =
            Integer::from_sign_and_abs(self.sign, (&self.numerator).div_exact(&ng) * &dybar);
        let y_over_g =
            Integer::from_sign_and_abs(other.sign, other.numerator.div_exact(&ng) * dxbar);
        let g = Rational {
            sign: true,
            denominator: &self.denominator * dybar,
            numerator: ng,
        };
        let (one, u, v) = x_over_g.extended_gcd(y_over_g);
        debug_assert!(one == 1u32);
        (g, u, v)
    }
}

impl ExtendedGcd<&Rational> for &Rational {
    type Gcd = Rational;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Rational`]s $a$ and $b$, and also the
    /// integer coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. Both [`Rational`]s
    /// are taken by reference.
    ///
    /// The integer combinations of two rationals are exactly the integer multiples of their GCD, so
    /// integer Bézout coefficients exist. They are the coefficients of the integer
    /// [`extended_gcd`](malachite_base::num::arithmetic::traits::ExtendedGcd) applied to the
    /// coprime integer quotients $a/g$ and $b/g$, and inherit its normalization.
    ///
    /// $f(a, b) = (g, x, y)$, where $g = \gcd(a, b) \geq 0$ and $ax + by = g$, and $f(0, 0) = (0,
    /// 0, 0)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// // -1 * 2/3 + -1 * -3/4 = 1/12
    /// assert_eq!(
    ///     (&Rational::from_signeds(2, 3))
    ///         .extended_gcd(&Rational::from_signeds(-3, 4))
    ///         .to_debug_string(),
    ///     "(1/12, -1, -1)"
    /// );
    /// ```
    ///
    /// This is fmpq_gcd_cofactors from fmpq/gcd_cofactors.c, FLINT 3.6.0, except that FLINT returns
    /// the quotients $a/g$ and $b/g$ themselves, while this function takes one more step and
    /// returns Bézout coefficients for them.
    fn extended_gcd(self, other: &Rational) -> (Rational, Integer, Integer) {
        let ng = (&self.numerator).gcd(&other.numerator);
        if ng == 0u32 {
            // Both numerators are zero, so both rationals are.
            return (Rational::ZERO, Integer::ZERO, Integer::ZERO);
        }
        let dg = (&self.denominator).gcd(&other.denominator);
        let dxbar = (&self.denominator).div_exact(&dg);
        let dybar = (&other.denominator).div_exact(dg);
        let x_over_g =
            Integer::from_sign_and_abs(self.sign, (&self.numerator).div_exact(&ng) * &dybar);
        let y_over_g =
            Integer::from_sign_and_abs(other.sign, (&other.numerator).div_exact(&ng) * dxbar);
        let g = Rational {
            sign: true,
            denominator: &self.denominator * dybar,
            numerator: ng,
        };
        let (one, u, v) = x_over_g.extended_gcd(y_over_g);
        debug_assert!(one == 1u32);
        (g, u, v)
    }
}
