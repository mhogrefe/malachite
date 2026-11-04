// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_gcd::{
    GcdOperation, ThreePolynomials, checked_mod_gcd_operation,
};
use alloc::vec::Vec;
use malachite_base::polynomial::modular::traits::{CheckedModExtendedGcd, ModExtendedGcd};
use malachite_base::unsigned_polynomial::modular::mod_extended_gcd::checked_monic_extended_gcd;
use malachite_base::unsigned_polynomial::modular::mod_gcd::GcdModulus;

pub(crate) struct ExtendedGcdOp;

impl GcdOperation for ExtendedGcdOp {
    type Shape = ThreePolynomials;

    #[inline]
    fn run<C: Clone, M: GcdModulus<C>>(
        xs: &[C],
        ys: &[C],
        m: &M,
    ) -> Option<(Vec<C>, Vec<C>, Vec<C>)> {
        checked_monic_extended_gcd(xs, ys, m)
    }
}

fn checked_mod_extended_gcd_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    checked_mod_gcd_operation::<ExtendedGcdOp>(p, q, m)
}

fn mod_extended_gcd_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> (NaturalPolynomial, NaturalPolynomial, NaturalPolynomial) {
    checked_mod_extended_gcd_helper(p, q, m)
        .expect("a leading coefficient met by Euclid's algorithm is not invertible modulo m")
}

impl ModExtendedGcd<Self, Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking both polynomials by value and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.mod_extended_gcd(q, Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(self, other: Self, m: Natural) -> (Self, Self, Self) {
        mod_extended_gcd_helper(&self, &other, &m)
    }
}

impl ModExtendedGcd<Self, &Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking both polynomials by value and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.mod_extended_gcd(q, &Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(self, other: Self, m: &Natural) -> (Self, Self, Self) {
        mod_extended_gcd_helper(&self, &other, m)
    }
}

impl ModExtendedGcd<&Self, Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking the first polynomial by value, the second by reference, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.mod_extended_gcd(&q, Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(self, other: &Self, m: Natural) -> (Self, Self, Self) {
        mod_extended_gcd_helper(&self, other, &m)
    }
}

impl ModExtendedGcd<&Self, &Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking the first polynomial by value, the second by reference, and the modulus by reference.
    /// The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.mod_extended_gcd(&q, &Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(self, other: &Self, m: &Natural) -> (Self, Self, Self) {
        mod_extended_gcd_helper(&self, other, m)
    }
}

impl ModExtendedGcd<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking the first polynomial by reference, the second by value, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).mod_extended_gcd(q, Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(
        self,
        other: NaturalPolynomial,
        m: Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial, NaturalPolynomial) {
        mod_extended_gcd_helper(self, &other, &m)
    }
}

impl ModExtendedGcd<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking the first polynomial by reference, the second by value, and the modulus by reference.
    /// The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).mod_extended_gcd(q, &Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(
        self,
        other: NaturalPolynomial,
        m: &Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial, NaturalPolynomial) {
        mod_extended_gcd_helper(self, &other, m)
    }
}

impl ModExtendedGcd<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking both polynomials by reference and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).mod_extended_gcd(&q, Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(
        self,
        other: &NaturalPolynomial,
        m: Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial, NaturalPolynomial) {
        mod_extended_gcd_helper(self, other, &m)
    }
}

impl ModExtendedGcd<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// taking both polynomials by reference and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](malachite_base::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then this function panics. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).mod_extended_gcd(&q, &Natural::from(7u32));
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial, NaturalPolynomial) {
        mod_extended_gcd_helper(self, other, m)
    }
}

impl CheckedModExtendedGcd<Self, Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking both polynomials by value and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.checked_mod_extended_gcd(q, Natural::from(7u32)).unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_extended_gcd(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(self, other: Self, m: Natural) -> Option<(Self, Self, Self)> {
        checked_mod_extended_gcd_helper(&self, &other, &m)
    }
}

impl CheckedModExtendedGcd<Self, &Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking both polynomials by value and the modulus by reference. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.checked_mod_extended_gcd(q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     p.checked_mod_extended_gcd(q, &Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(self, other: Self, m: &Natural) -> Option<(Self, Self, Self)> {
        checked_mod_extended_gcd_helper(&self, &other, m)
    }
}

impl CheckedModExtendedGcd<&Self, Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking the first polynomial by value, the second by reference, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.checked_mod_extended_gcd(&q, Natural::from(7u32)).unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     p.checked_mod_extended_gcd(&q, Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(self, other: &Self, m: Natural) -> Option<(Self, Self, Self)> {
        checked_mod_extended_gcd_helper(&self, other, &m)
    }
}

impl CheckedModExtendedGcd<&Self, &Natural> for NaturalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking the first polynomial by value, the second by reference, and the modulus by reference.
    /// The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p
    ///     .checked_mod_extended_gcd(&q, &Natural::from(7u32))
    ///     .unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     p.checked_mod_extended_gcd(&q, &Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(self, other: &Self, m: &Natural) -> Option<(Self, Self, Self)> {
        checked_mod_extended_gcd_helper(&self, other, m)
    }
}

impl CheckedModExtendedGcd<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking the first polynomial by reference, the second by value, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p)
    ///     .checked_mod_extended_gcd(q, Natural::from(7u32))
    ///     .unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     (&p).checked_mod_extended_gcd(q, Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(
        self,
        other: NaturalPolynomial,
        m: Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
        checked_mod_extended_gcd_helper(self, &other, &m)
    }
}

impl CheckedModExtendedGcd<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking the first polynomial by reference, the second by value, and the modulus by reference.
    /// The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p)
    ///     .checked_mod_extended_gcd(q, &Natural::from(7u32))
    ///     .unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     (&p).checked_mod_extended_gcd(q, &Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(
        self,
        other: NaturalPolynomial,
        m: &Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
        checked_mod_extended_gcd_helper(self, &other, m)
    }
}

impl CheckedModExtendedGcd<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking both polynomials by reference and the modulus by value. The coefficients of both must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p)
    ///     .checked_mod_extended_gcd(&q, Natural::from(7u32))
    ///     .unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     (&p).checked_mod_extended_gcd(&q, Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(
        self,
        other: &NaturalPolynomial,
        m: Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
        checked_mod_extended_gcd_helper(self, other, &m)
    }
}

impl CheckedModExtendedGcd<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Gcd = NaturalPolynomial;
    type Cofactor = NaturalPolynomial;

    /// Computes the monic GCD of two [`NaturalPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking both polynomials by reference and the modulus by reference. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((g, s, t)) &
    ///         \text{if every leading coefficient met is invertible}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// where $sa + tb \equiv g \pmod{m}$ and $g$ is the monic polynomial that generates the same
    /// ideal of $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`CheckedModGcd`](malachite_base::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Short
    /// polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide only by
    /// leading coefficients that are invertible modulo $m$; when $m$ is composite, a leading
    /// coefficient met along the way may not be, and then the result is `None`. When $m$ is prime,
    /// this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (m + \log n) \log n \log (nm) \log\log (nm) + n m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p)
    ///     .checked_mod_extended_gcd(&q, &Natural::from(7u32))
    ///     .unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(
    ///     (&p).checked_mod_extended_gcd(&q, &Natural::from(6u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_xgcd` from `fmpz_mod_poly/xgcd.c`, FLINT 3.6.0, except
    /// that Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
        checked_mod_extended_gcd_helper(self, other, m)
    }
}
