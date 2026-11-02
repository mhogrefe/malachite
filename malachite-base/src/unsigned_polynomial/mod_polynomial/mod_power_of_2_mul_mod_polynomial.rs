// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::traits::{
    ModPowerOf2MulModPolynomial, ModPowerOf2MulModPolynomialAssign,
};
use crate::polynomial::modular::traits::ModPowerOf2Rem;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Checks that `a`, `b`, and `f` are reduced modulo $2^k$, that `f` is nonzero, and, if `check_lead`
// is set, has an odd leading coefficient, unless $k$ is 0, and that `a` and `b` are shorter than
// `f`, and returns whether $k$ is 0.
pub(crate) fn assert_mul_mod_polynomial_arguments<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    b: &UnsignedPolynomial<T>,
    f: &UnsignedPolynomial<T>,
    pow: u64,
    check_lead: bool,
) -> bool {
    assert!(pow <= T::WIDTH);
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {a} has a coefficient >= 2^{pow}"
    );
    assert!(
        b.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {b} has a coefficient >= 2^{pow}"
    );
    assert!(
        f.mod_power_of_2_is_reduced(pow),
        "f must be reduced mod 2^pow, but {f} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return true;
    }
    assert_ne!(*f, UnsignedPolynomial::ZERO, "division by zero");
    assert!(
        !check_lead || f.coefficients.last().unwrap().odd(),
        "the leading coefficient of f must be odd"
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

// The product of `a` and `b` modulo `f`, with coefficients modulo $2^k$.
//
// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the modulus
// $2^k$, except that both factors must be reduced modulo $f$.
pub(crate) fn mod_power_of_2_mul_mod_polynomial_helper<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    b: &UnsignedPolynomial<T>,
    f: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    if assert_mul_mod_polynomial_arguments(a, b, f, pow, true) {
        return UnsignedPolynomial::ZERO;
    }
    let product = (a).mod_power_of_2_mul(b, pow);
    if product.len() < f.len() {
        product
    } else {
        (&product).mod_power_of_2_rem(f, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomial<Self, Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by value, and `f` by value. Both
    /// factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: Self, f: Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, &other, &f, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomial<Self, &Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by value, and `f` by reference.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(b, &f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: Self, f: &Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, &other, f, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomial<&Self, Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by reference, and `f` by value.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(&b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: &Self, f: Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, other, &f, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomial<&Self, &Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by reference, and `f` by
    /// reference. Both factors must already be reduced modulo $f$, with degrees less than that of
    /// $f$, and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(&b, &f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: &Self, f: &Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, other, f, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomial<UnsignedPolynomial<T>, UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by value, and `f` by value.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: UnsignedPolynomial<T>,
        f: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_mul_mod_polynomial_helper(self, &other, &f, pow)
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2MulModPolynomial<UnsignedPolynomial<T>, &UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by value, and `f` by
    /// reference. Both factors must already be reduced modulo $f$, with degrees less than that of
    /// $f$, and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(b, &f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: UnsignedPolynomial<T>,
        f: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_mul_mod_polynomial_helper(self, &other, f, pow)
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2MulModPolynomial<&UnsignedPolynomial<T>, UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by reference, and `f` by
    /// value. Both factors must already be reduced modulo $f$, with degrees less than that of $f$,
    /// and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of
    /// $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(&b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: &UnsignedPolynomial<T>,
        f: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_mul_mod_polynomial_helper(self, other, &f, pow)
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2MulModPolynomial<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by reference, and `f` by
    /// reference. Both factors must already be reduced modulo $f$, with degrees less than that of
    /// $f$, and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(&b, &f, 3)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: &UnsignedPolynomial<T>,
        f: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_mul_mod_polynomial_helper(self, other, f, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomialAssign<Self, Self> for UnsignedPolynomial<T> {
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by value and `f` by value. Both factors must
    /// already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients of
    /// all three must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(b, f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: Self, f: Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, &other, &f, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomialAssign<Self, &Self>
    for UnsignedPolynomial<T>
{
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by value and `f` by reference. Both factors
    /// must already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients
    /// of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(b, &f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: Self, f: &Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, &other, f, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomialAssign<&Self, Self>
    for UnsignedPolynomial<T>
{
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by reference and `f` by value. Both factors
    /// must already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients
    /// of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(&b, f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: &Self, f: Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, other, &f, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulModPolynomialAssign<&Self, &Self>
    for UnsignedPolynomial<T>
{
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by reference and `f` by reference. Both
    /// factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `other`, or `f`
    /// is greater than or equal to $2^k$, or if `f` is 0 (unless $k$ is 0) or its leading
    /// coefficient is even, or if `self` or `other` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(&b, &f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod` from `nmod_poly/mulmod.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: &Self, f: &Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, other, f, pow);
    }
}
