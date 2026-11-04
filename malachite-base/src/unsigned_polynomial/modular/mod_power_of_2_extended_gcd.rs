// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_extended_gcd::checked_monic_extended_gcd;
use crate::unsigned_polynomial::modular::mod_power_of_2_gcd::assert_reduced_mod_power_of_2;
use crate::unsigned_polynomial::modular::subproduct_tree::PowerOf2Modulus;

#[allow(clippy::type_complexity)]
fn checked_mod_power_of_2_extended_gcd_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    assert_reduced_mod_power_of_2(p, q, pow);
    if pow == 0 {
        return Some((
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
        ));
    }
    checked_monic_extended_gcd(&p.coefficients, &q.coefficients, &PowerOf2Modulus(pow)).map(
        |(g, s, t)| {
            (
                UnsignedPolynomial::from_coefficients_asc(g),
                UnsignedPolynomial::from_coefficients_asc(s),
                UnsignedPolynomial::from_coefficients_asc(t),
            )
        },
    )
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2ExtendedGcd<Self> for UnsignedPolynomial<T> {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, together with cofactors,
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
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd).
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = p.checked_mod_power_of_2_extended_gcd(q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
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

impl<T: PrimitiveUnsigned> CheckedModPowerOf2ExtendedGcd<&Self> for UnsignedPolynomial<T> {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, together with cofactors,
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
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd).
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = p.checked_mod_power_of_2_extended_gcd(&q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
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

impl<T: PrimitiveUnsigned> CheckedModPowerOf2ExtendedGcd<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Gcd = UnsignedPolynomial<T>;
    type Cofactor = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, together with cofactors,
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
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd).
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = (&p).checked_mod_power_of_2_extended_gcd(q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_extended_gcd(q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_xgcd` from
    /// `nmod_poly/xgcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_extended_gcd(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<(
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    )> {
        checked_mod_power_of_2_extended_gcd_helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2ExtendedGcd<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Gcd = UnsignedPolynomial<T>;
    type Cofactor = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $2^k$, together with cofactors,
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
    /// [`CheckedModPowerOf2Gcd`](crate::polynomial::modular::traits::CheckedModPowerOf2Gcd).
    ///
    /// If $a$ and $b$ are nonzero and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg
    /// g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is
    /// the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $2^k$, which are the odd ones; when
    /// one met along the way is even, the result is `None`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+x^2+3*x+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// let (g, s, t) = (&p).checked_mod_power_of_2_extended_gcd(&q, 3).unwrap();
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "7");
    /// assert_eq!(t.to_string(), "x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_extended_gcd(&q, 2).is_none());
    /// ```
    ///
    /// FLINT has no version modulo a power of 2; this follows `nmod_poly_xgcd` from
    /// `nmod_poly/xgcd.c`, FLINT 3.6.0, returning `None` where a leading coefficient is even.
    #[inline]
    fn checked_mod_power_of_2_extended_gcd(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<(
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    )> {
        checked_mod_power_of_2_extended_gcd_helper(self, other, pow)
    }
}
