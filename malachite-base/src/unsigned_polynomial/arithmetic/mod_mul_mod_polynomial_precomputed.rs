// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModMul;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{
    ModMulModPolynomialPrecomputed, ModMulModPolynomialPrecomputedAssign, ModPolynomialData,
    ModRemPrecomputed, Polynomial,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_mul_mod_polynomial::*;

// The product of `a` and `b` modulo `f`, with coefficients modulo $m$, with `data` precomputed from
// `f` and $m$.
//
// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT 3.6.0.
pub(crate) fn mod_mul_mod_polynomial_precomputed_helper<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    b: &UnsignedPolynomial<T>,
    f: &UnsignedPolynomial<T>,
    m: T,
    data: &ModPolynomialData<T>,
) -> UnsignedPolynomial<T> {
    if assert_mul_mod_polynomial_arguments(a, b, f, m, false) {
        return UnsignedPolynomial::ZERO;
    }
    let product = (a).mod_mul(b, m);
    if product.len() < f.len() {
        product
    } else {
        (&product).mod_rem_precomputed(f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputed<Self, Self, T> for UnsignedPolynomial<T> {
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by value, the second
    /// factor by value, and `f` by value. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo $m$;
    /// the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, f.clone(), 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: Self,
        m: T,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, &other, &f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputed<Self, &Self, T>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by value, the second
    /// factor by value, and `f` by reference. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo $m$;
    /// the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: &Self,
        m: T,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, &other, f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputed<&Self, Self, T>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by value, the second
    /// factor by reference, and `f` by value. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo $m$;
    /// the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, f.clone(), 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: Self,
        m: T,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, other, &f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputed<&Self, &Self, T>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by value, the second
    /// factor by reference, and `f` by reference. Both factors must already be reduced modulo $f$,
    /// with degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: &Self,
        m: T,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, other, f, m, data)
    }
}

impl<T: PrimitiveUnsigned>
    ModMulModPolynomialPrecomputed<UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by reference, the
    /// second factor by value, and `f` by value. Both factors must already be reduced modulo $f$,
    /// with degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, f.clone(), 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: UnsignedPolynomial<T>,
        f: UnsignedPolynomial<T>,
        m: T,
        data: &Self::Data,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_precomputed_helper(self, &other, &f, m, data)
    }
}

impl<T: PrimitiveUnsigned>
    ModMulModPolynomialPrecomputed<UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by reference, the
    /// second factor by value, and `f` by reference. Both factors must already be reduced modulo
    /// $f$, with degrees less than that of $f$, and the coefficients of all three must be reduced
    /// modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: UnsignedPolynomial<T>,
        f: &UnsignedPolynomial<T>,
        m: T,
        data: &Self::Data,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_precomputed_helper(self, &other, f, m, data)
    }
}

impl<T: PrimitiveUnsigned>
    ModMulModPolynomialPrecomputed<&UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by reference, the
    /// second factor by reference, and `f` by value. Both factors must already be reduced modulo
    /// $f$, with degrees less than that of $f$, and the coefficients of all three must be reduced
    /// modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, f.clone(), 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &UnsignedPolynomial<T>,
        f: UnsignedPolynomial<T>,
        m: T,
        data: &Self::Data,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_precomputed_helper(self, other, &f, m, data)
    }
}

impl<T: PrimitiveUnsigned>
    ModMulModPolynomialPrecomputed<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the first factor by reference, the
    /// second factor by reference, and `f` by reference. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModDivRemPrecomputed, ModMulModPolynomialPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &UnsignedPolynomial<T>,
        f: &UnsignedPolynomial<T>,
        m: T,
        data: &Self::Data,
    ) -> UnsignedPolynomial<T> {
        mod_mul_mod_polynomial_precomputed_helper(self, other, f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputedAssign<Self, Self, T>
    for UnsignedPolynomial<T>
{
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// value and `f` by value. Both factors must already be reduced modulo $f$, with degrees less
    /// than that of $f$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, f.clone(), 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, f, 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: Self,
        m: T,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, &other, &f, m, data);
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputedAssign<Self, &Self, T>
    for UnsignedPolynomial<T>
{
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// value and `f` by reference. Both factors must already be reduced modulo $f$, with degrees
    /// less than that of $f$, and the coefficients of all three must be reduced modulo $m$; the
    /// leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, &f, 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, &f, 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: &Self,
        m: T,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, &other, f, m, data);
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputedAssign<&Self, Self, T>
    for UnsignedPolynomial<T>
{
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// reference and `f` by value. Both factors must already be reduced modulo $f$, with degrees
    /// less than that of $f$, and the coefficients of all three must be reduced modulo $m$; the
    /// leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, f.clone(), 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, f, 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: Self,
        m: T,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, other, &f, m, data);
    }
}

impl<T: PrimitiveUnsigned> ModMulModPolynomialPrecomputedAssign<&Self, &Self, T>
    for UnsignedPolynomial<T>
{
    /// Multiplies a [`UnsignedPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// reference and `f` by reference. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo $m$;
    /// the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for division; with
    /// it the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
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
    /// equal to `m`, or if `f` is 0 (unless $m$ is 1), or if `self` or `other` is not shorter than
    /// `f`. If `data` was not precomputed from `f` and `m`, the result is meaningless, and the
    /// function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, &f, 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("2*x^2+3").unwrap();
    /// let b = UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, &f, 7, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mulmod_preinv` from `nmod_poly/mulmod_preinv.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: &Self,
        m: T,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, other, f, m, data);
    }
}
