// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ExtendedGcd, NegAssign, UnsignedAbs};

impl ExtendedGcd for Integer {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. Both [`Integer`]s are
    /// taken by value.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(a, ak) = (-a, -1, 0)$ if $a < 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(bk, b) = (-b, 0, -1)$ if $b < 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(|a|, |b|)$,
    ///   where $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq
    ///   \lfloor a/g \rfloor$.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(3)
    ///         .extended_gcd(Integer::from(5))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     Integer::from(240)
    ///         .extended_gcd(Integer::from(46))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-111)
    ///         .extended_gcd(Integer::from(300))
    ///         .to_debug_string(),
    ///     "(3, 27, 10)"
    /// );
    /// ```
    fn extended_gcd(self, other: Integer) -> (Natural, Integer, Integer) {
        let a_sign = self.sign;
        let b_sign = other.sign;
        let (gcd, mut x, mut y) = self.unsigned_abs().extended_gcd(other.unsigned_abs());
        if !a_sign {
            x.neg_assign();
        }
        if !b_sign {
            y.neg_assign();
        }
        (gcd, x, y)
    }
}

impl ExtendedGcd<&Integer> for Integer {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. The first [`Integer`] is
    /// taken by value and the second by reference.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(a, ak) = (-a, -1, 0)$ if $a < 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(bk, b) = (-b, 0, -1)$ if $b < 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(|a|, |b|)$,
    ///   where $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq
    ///   \lfloor a/g \rfloor$.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(3)
    ///         .extended_gcd(&Integer::from(5))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     Integer::from(240)
    ///         .extended_gcd(&Integer::from(46))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-111)
    ///         .extended_gcd(&Integer::from(300))
    ///         .to_debug_string(),
    ///     "(3, 27, 10)"
    /// );
    /// ```
    fn extended_gcd(self, other: &Integer) -> (Natural, Integer, Integer) {
        let a_sign = self.sign;
        let (gcd, mut x, mut y) = self.unsigned_abs().extended_gcd(other.unsigned_abs_ref());
        if !a_sign {
            x.neg_assign();
        }
        if !other.sign {
            y.neg_assign();
        }
        (gcd, x, y)
    }
}

impl ExtendedGcd<Integer> for &Integer {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. The first [`Integer`] is
    /// taken by reference and the second by value.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(a, ak) = (-a, -1, 0)$ if $a < 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(bk, b) = (-b, 0, -1)$ if $b < 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(|a|, |b|)$,
    ///   where $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq
    ///   \lfloor a/g \rfloor$.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(3))
    ///         .extended_gcd(Integer::from(5))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(240))
    ///         .extended_gcd(Integer::from(46))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-111))
    ///         .extended_gcd(Integer::from(300))
    ///         .to_debug_string(),
    ///     "(3, 27, 10)"
    /// );
    /// ```
    fn extended_gcd(self, other: Integer) -> (Natural, Integer, Integer) {
        let b_sign = other.sign;
        let (gcd, mut x, mut y) = self.unsigned_abs_ref().extended_gcd(other.unsigned_abs());
        if !self.sign {
            x.neg_assign();
        }
        if !b_sign {
            y.neg_assign();
        }
        (gcd, x, y)
    }
}

impl ExtendedGcd<&Integer> for &Integer {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. Both [`Integer`]s are
    /// taken by reference.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(a, ak) = (-a, -1, 0)$ if $a < 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(bk, b) = (-b, 0, -1)$ if $b < 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(|a|, |b|)$,
    ///   where $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq
    ///   \lfloor a/g \rfloor$.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(3))
    ///         .extended_gcd(&Integer::from(5))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(240))
    ///         .extended_gcd(&Integer::from(46))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-111))
    ///         .extended_gcd(&Integer::from(300))
    ///         .to_debug_string(),
    ///     "(3, 27, 10)"
    /// );
    /// ```
    fn extended_gcd(self, other: &Integer) -> (Natural, Integer, Integer) {
        let (gcd, mut x, mut y) = self
            .unsigned_abs_ref()
            .extended_gcd(other.unsigned_abs_ref());
        if !self.sign {
            x.neg_assign();
        }
        if !other.sign {
            y.neg_assign();
        }
        (gcd, x, y)
    }
}
