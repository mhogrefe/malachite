// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_extended_gcd::ExtendedGcdOp;
use crate::natural_polynomial::modular::mod_gcd::checked_mod_power_of_2_gcd_operation;
use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;

fn checked_mod_power_of_2_extended_gcd_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    checked_mod_power_of_2_gcd_operation::<ExtendedGcdOp>(p, q, pow)
}

impl CheckedModPowerOf2ExtendedGcd<Self> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is even, taking both by value.
    /// The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{2^k}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// `checked_mod_power_of_2_gcd`.
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = p.checked_mod_power_of_2_extended_gcd(q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_extended_gcd(q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_xgcd` from
    /// `nmod_poly/xgcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_extended_gcd(
        self,
        other: Self,
        pow: u64,
    ) -> Option<(Self, Self, Self)> {
        checked_mod_power_of_2_extended_gcd_helper(&self, &other, pow)
    }
}

impl CheckedModPowerOf2ExtendedGcd<&Self> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is even, taking the first by
    /// value and the second by reference. The coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{2^k}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// `checked_mod_power_of_2_gcd`.
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = p.checked_mod_power_of_2_extended_gcd(&q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_extended_gcd(&q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_xgcd` from
    /// `nmod_poly/xgcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_extended_gcd(
        self,
        other: &Self,
        pow: u64,
    ) -> Option<(Self, Self, Self)> {
        checked_mod_power_of_2_extended_gcd_helper(&self, other, pow)
    }
}

impl CheckedModPowerOf2ExtendedGcd<NaturalPolynomial> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is even, taking the first by
    /// reference and the second by value. The coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{2^k}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// `checked_mod_power_of_2_gcd`.
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = (&p).checked_mod_power_of_2_extended_gcd(q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_extended_gcd(q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_xgcd` from
    /// `nmod_poly/xgcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_extended_gcd(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
        checked_mod_power_of_2_extended_gcd_helper(self, &other, pow)
    }
}

impl CheckedModPowerOf2ExtendedGcd<&NaturalPolynomial> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $2^k$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is even, taking both by
    /// reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{2^k}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/2^k\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// `checked_mod_power_of_2_gcd`.
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $2^k$, which are the odd ones; when one met
    /// along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n (k + \log n) \log n \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n (k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = (&p).checked_mod_power_of_2_extended_gcd(&q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_extended_gcd(&q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_xgcd` from
    /// `nmod_poly/xgcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_extended_gcd(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
        checked_mod_power_of_2_extended_gcd_helper(self, other, pow)
    }
}
