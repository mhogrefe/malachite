// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::polynomial::modular::traits::{CheckedModExtendedGcd, ModExtendedGcd};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::hgcd::{div_rem, mul, sub_assign, sub_from, xgcd_hgcd};
use crate::unsigned_polynomial::modular::mod_gcd::{GcdModulus, assert_reduced};
use crate::unsigned_polynomial::modular::subproduct_tree::Modulus;
use alloc::vec;
use alloc::vec::Vec;

/// A GCD $g$, not normalized, of the polynomials $a$ and $b$ with coefficients `xs` and `ys`,
/// together with cofactors $s$ and $t$ such that $sa + tb = g$, where `xs` is at least as long as
/// `ys` and neither is empty or ends with a zero, by Euclid's algorithm; or `None` if a leading
/// coefficient met along the way has no inverse. A constant `ys` is returned as $g$, with $s = 0$
/// and $t = 1$.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute extended
/// GCDs of `NaturalPolynomial`s.
///
/// This is equivalent to `_gr_poly_xgcd_euclidean` from `gr_poly/xgcd_euclidean.c`, FLINT 3.6.0.
#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub fn xgcd_euclidean<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    m: &M,
) -> Option<(Vec<C>, Vec<C>, Vec<C>)> {
    if ys.len() == 1 {
        return Some((ys.to_vec(), Vec::new(), vec![m.one()]));
    }
    let r = div_rem(xs, ys, m)?.1;
    if r.is_empty() {
        return Some((ys.to_vec(), Vec::new(), vec![m.one()]));
    }
    let mut d = ys.to_vec();
    let mut u = Vec::new();
    let mut v1 = vec![m.one()];
    let mut v3 = r;
    while !v3.is_empty() {
        let (q, r) = div_rem(&d, &v3, m)?;
        sub_assign(&mut u, &mul(&v1, &q, m), m);
        core::mem::swap(&mut u, &mut v1);
        d = core::mem::replace(&mut v3, r);
    }
    let mut w = mul(xs, &u, m);
    sub_from(&mut w, &d, m);
    let t = div_rem(&w, ys, m).unwrap().0;
    Some((d, u, t))
}

fn scale<C: Clone, M: GcdModulus<C>>(xs: &mut [C], c: &C, m: &M) {
    for x in xs {
        *x = m.mul(x, c);
    }
}

/// The monic GCD $g$ of the polynomials $a$ and $b$ with coefficients `xs` and `ys`, neither ending
/// with a zero, together with cofactors $s$ and $t$ such that $sa + tb = g$, by Euclid's algorithm
/// or the half-GCD algorithm; or `None` if a leading coefficient met along the way has no inverse.
/// The GCD of two empty polynomials is empty, with empty cofactors.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute extended
/// GCDs of `NaturalPolynomial`s.
///
/// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0.
#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub fn checked_monic_extended_gcd<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    m: &M,
) -> Option<(Vec<C>, Vec<C>, Vec<C>)> {
    if xs.len() < ys.len() {
        return checked_monic_extended_gcd(ys, xs, m).map(|(g, s, t)| (g, t, s));
    }
    if xs.is_empty() {
        return Some((Vec::new(), Vec::new(), Vec::new()));
    }
    if ys.is_empty() {
        let inverse = m.inverse(xs.last().unwrap())?;
        let mut g = xs.to_vec();
        scale(&mut g, &inverse, m);
        return Some((g, vec![inverse], Vec::new()));
    }
    if ys.len() == 1 {
        let inverse = m.inverse(&ys[0])?;
        return Some((vec![m.one()], Vec::new(), vec![inverse]));
    }
    let cutoffs = m.gcd_cutoffs();
    let (mut g, mut s, mut t) = if ys.len() < cutoffs.xgcd_hgcd {
        xgcd_euclidean(xs, ys, m)?
    } else {
        xgcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.xgcd_hgcd, m)?
    };
    let inverse = m.inverse(g.last().unwrap())?;
    scale(&mut g, &inverse, m);
    scale(&mut s, &inverse, m);
    scale(&mut t, &inverse, m);
    Some((g, s, t))
}

#[allow(clippy::type_complexity)]
pub(crate) fn checked_mod_extended_gcd_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    assert_reduced(p, q, m);
    if m == T::ONE {
        return Some((
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
            UnsignedPolynomial::ZERO,
        ));
    }
    checked_monic_extended_gcd(&p.coefficients, &q.coefficients, &Modulus(m)).map(|(g, s, t)| {
        (
            UnsignedPolynomial::from_coefficients_asc(g),
            UnsignedPolynomial::from_coefficients_asc(s),
            UnsignedPolynomial::from_coefficients_asc(t),
        )
    })
}

fn mod_extended_gcd_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> (
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
) {
    checked_mod_extended_gcd_helper(p, q, m)
        .expect("a leading coefficient met by Euclid's algorithm is not invertible modulo m")
}

impl<T: PrimitiveUnsigned> ModExtendedGcd<Self, T> for UnsignedPolynomial<T> {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// taking both by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](crate::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.mod_extended_gcd(q, 7);
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(self, other: Self, m: T) -> (Self, Self, Self) {
        mod_extended_gcd_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModExtendedGcd<&Self, T> for UnsignedPolynomial<T> {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// taking the first by value and the second by reference. The coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](crate::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.mod_extended_gcd(&q, 7);
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(self, other: &Self, m: T) -> (Self, Self, Self) {
        mod_extended_gcd_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModExtendedGcd<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Gcd = UnsignedPolynomial<T>;
    type Cofactor = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// taking the first by reference and the second by value. The coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](crate::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).mod_extended_gcd(q, 7);
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
    ) -> (
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    ) {
        mod_extended_gcd_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModExtendedGcd<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Gcd = UnsignedPolynomial<T>;
    type Cofactor = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// taking both by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m},
    /// $$
    ///
    /// where $g$ is the monic polynomial that generates the same ideal of
    /// $(\mathbb{Z}/m\mathbb{Z})\[x\]$ as $a$ and $b$, or 0 if both are 0, as computed by
    /// [`ModGcd`](crate::polynomial::modular::traits::ModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then this function panics. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if a leading coefficient met along the way is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).mod_extended_gcd(&q, 7);
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT assumes the modulus prime.
    #[inline]
    fn mod_extended_gcd(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
    ) -> (
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    ) {
        mod_extended_gcd_helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModExtendedGcd<Self, T> for UnsignedPolynomial<T> {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking both by value. The coefficients of both must already be reduced modulo $m$.
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
    /// [`CheckedModGcd`](crate::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.checked_mod_extended_gcd(q, 7).unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_extended_gcd(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(self, other: Self, m: T) -> Option<(Self, Self, Self)> {
        checked_mod_extended_gcd_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModExtendedGcd<&Self, T> for UnsignedPolynomial<T> {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking the first by value and the second by reference. The coefficients of both must already
    /// be reduced modulo $m$.
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
    /// [`CheckedModGcd`](crate::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = p.checked_mod_extended_gcd(&q, 7).unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_extended_gcd(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(self, other: &Self, m: T) -> Option<(Self, Self, Self)> {
        checked_mod_extended_gcd_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModExtendedGcd<UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Gcd = UnsignedPolynomial<T>;
    type Cofactor = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking the first by reference and the second by value. The coefficients of both must already
    /// be reduced modulo $m$.
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
    /// [`CheckedModGcd`](crate::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).checked_mod_extended_gcd(q, 7).unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_extended_gcd(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
    ) -> Option<(
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    )> {
        checked_mod_extended_gcd_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModExtendedGcd<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Gcd = UnsignedPolynomial<T>;
    type Cofactor = UnsignedPolynomial<T>;

    /// Computes the monic GCD of two [`UnsignedPolynomial`]s modulo $m$, together with cofactors,
    /// returning `None` if a leading coefficient met along the way is not invertible modulo $m$,
    /// taking both by reference. The coefficients of both must already be reduced modulo $m$.
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
    /// [`CheckedModGcd`](crate::polynomial::modular::traits::CheckedModGcd).
    ///
    /// When $m$ is prime, $a$ and $b$ are nonzero, and $\deg g < \min(\deg a, \deg b)$, then $\deg
    /// s < \deg b - \deg g$ and $\deg t < \deg a - \deg g$. If $b$ is 0, the result is $(a/c, 1/c,
    /// 0)$, where $c$ is the leading coefficient of $a$; if both are 0, all three are 0.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Short polynomials use Euclid's algorithm and long ones the half-GCD algorithm. Both divide
    /// only by leading coefficients that are invertible modulo $m$; when $m$ is composite, a
    /// leading coefficient met along the way may not be, and then the result is `None`. When $m$ is
    /// prime, this never happens.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::CheckedModExtendedGcd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x^2+3").unwrap();
    /// let (g, s, t) = (&p).checked_mod_extended_gcd(&q, 7).unwrap();
    /// assert_eq!(g.to_string(), "1");
    /// assert_eq!(s.to_string(), "2*x+2");
    /// assert_eq!(t.to_string(), "5*x^2+5*x+2");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_extended_gcd(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_xgcd` from `nmod_poly/xgcd.c`, FLINT 3.6.0, except that
    /// Malachite returns `None` where FLINT assumes the modulus prime.
    #[inline]
    fn checked_mod_extended_gcd(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
    ) -> Option<(
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    )> {
        checked_mod_extended_gcd_helper(self, other, m)
    }
}
