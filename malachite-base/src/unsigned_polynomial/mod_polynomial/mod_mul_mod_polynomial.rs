// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::{ModIsReduced, ModMul};
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::traits::{ModMulModPolynomial, ModMulModPolynomialAssign};
use crate::polynomial::modular::traits::ModRem;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Checks that `a`, `b`, and `f` are reduced modulo $m$, that `f` is nonzero, and, if `check_lead`
// is set, has an invertible leading coefficient, unless $m$ is 1, and that `a` and `b` are shorter
// than `f`, and returns whether $m$ is 1.
pub(crate) fn assert_mul_mod_polynomial_arguments<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    b: &UnsignedPolynomial<T>,
    f: &UnsignedPolynomial<T>,
    m: T,
    check_lead: bool,
) -> bool {
    assert!(
        a.mod_is_reduced(&m),
        "self must be reduced mod m, but {a} has a coefficient >= {m}"
    );
    assert!(
        b.mod_is_reduced(&m),
        "other must be reduced mod m, but {b} has a coefficient >= {m}"
    );
    assert!(
        f.mod_is_reduced(&m),
        "f must be reduced mod m, but {f} has a coefficient >= {m}"
    );
    if m == T::ONE {
        return true;
    }
    assert_ne!(*f, UnsignedPolynomial::ZERO, "division by zero");
    assert!(
        !check_lead || f.coefficients.last().unwrap().mod_inverse(m).is_some(),
        "the leading coefficient of f must be invertible modulo m"
    );
    assert!(
        a.len() < f.len(),
        "self must be reduced mod f, but {a} has a degree >= that of {f}"
    );
    assert!(
        b.len() < f.len(),
        "other must be reduced mod f, but {b} has a degree >= that of {f}"
    );
    false
}

// The product of `a` and `b` modulo `f`, with coefficients modulo $m$.
//
// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that both
// factors must be reduced modulo $f$.
pub(crate) fn mod_mul_mod_polynomial_helper<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    b: &UnsignedPolynomial<T>,
    f: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    if assert_mul_mod_polynomial_arguments(a, b, f, m, true) {
        return UnsignedPolynomial::ZERO;
    }
    let product = (a).mod_mul(b, m);
    if product.len() < f.len() {
        product
    } else {
        (&product).mod_rem(f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<Self, Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by value, the second factor by value, and `f` by value. Both
    /// factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(a.mod_mul_mod_polynomial(b, f, 7).to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(self, other: Self, f: Self, m: T) -> Self {
        mod_mul_mod_polynomial_helper(&self, &other, &f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<Self, &Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by value, the second factor by value, and `f` by reference.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(a.mod_mul_mod_polynomial(b, &f, 7).to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(self, other: Self, f: &Self, m: T) -> Self {
        mod_mul_mod_polynomial_helper(&self, &other, f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<&Self, Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by value, the second factor by reference, and `f` by value.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(a.mod_mul_mod_polynomial(&b, f, 7).to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(self, other: &Self, f: Self, m: T) -> Self {
        mod_mul_mod_polynomial_helper(&self, other, &f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<&Self, &Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by value, the second factor by reference, and `f` by reference.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(a.mod_mul_mod_polynomial(&b, &f, 7).to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(self, other: &Self, f: &Self, m: T) -> Self {
        mod_mul_mod_polynomial_helper(&self, other, f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by reference, the second factor by value, and `f` by value.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!((&a).mod_mul_mod_polynomial(b, f, 7).to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(
        self,
        other: UnsignedPolynomial<T>,
        f: UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_helper(self, &other, &f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by reference, the second factor by value, and `f` by reference.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!((&a).mod_mul_mod_polynomial(b, &f, 7).to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(
        self,
        other: UnsignedPolynomial<T>,
        f: &UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_helper(self, &other, f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<&UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by reference, the second factor by reference, and `f` by value.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!((&a).mod_mul_mod_polynomial(&b, f, 7).to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(
        self,
        other: &UnsignedPolynomial<T>,
        f: UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_helper(self, other, &f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomial<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the first factor by reference, the second factor by reference, and `f` by
    /// reference. Both factors must already be reduced modulo $f$, with degrees less than that of
    /// $f$, and the coefficients of all three must be reduced modulo $m$; the leading coefficient
    /// of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial(&b, &f, 7).to_string(),
    ///     "2*x^2+4"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial(
        self,
        other: &UnsignedPolynomial<T>,
        f: &UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_helper(self, other, f, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialAssign<Self, Self, T> for UnsignedPolynomial<T> {
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, taking the other factor by value and `f` by value. Both factors must
    /// already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients of
    /// all three must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_assign(b, f, 7);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial_assign(&mut self, other: Self, f: Self, m: T) {
        *self = mod_mul_mod_polynomial_helper(self, &other, &f, m);
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialAssign<Self, &Self, T> for UnsignedPolynomial<T> {
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, taking the other factor by value and `f` by reference. Both factors
    /// must already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients
    /// of all three must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_assign(b, &f, 7);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial_assign(&mut self, other: Self, f: &Self, m: T) {
        *self = mod_mul_mod_polynomial_helper(self, &other, f, m);
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialAssign<&Self, Self, T> for UnsignedPolynomial<T> {
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, taking the other factor by reference and `f` by value. Both factors
    /// must already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients
    /// of all three must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_assign(&b, f, 7);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial_assign(&mut self, other: &Self, f: Self, m: T) {
        *self = mod_mul_mod_polynomial_helper(self, other, &f, m);
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialAssign<&Self, &Self, T> for UnsignedPolynomial<T> {
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, taking the other factor by reference and `f` by reference. Both
    /// factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $f$ must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModRem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `other`, or `f` is greater than or
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_assign(&b, &f, 7);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, except that
    /// both factors must be reduced modulo $f$.
    #[inline]
    fn mod_mul_mod_polynomial_assign(&mut self, other: &Self, f: &Self, m: T) {
        *self = mod_mul_mod_polynomial_helper(self, other, f, m);
    }
}
