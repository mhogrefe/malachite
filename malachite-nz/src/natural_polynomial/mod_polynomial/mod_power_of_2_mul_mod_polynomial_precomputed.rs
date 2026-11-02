// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::mod_polynomial::mod_power_of_2_mul_mod_polynomial::*;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::modular::traits::ModPowerOf2Mul;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModPowerOf2MulModPolynomialPrecomputed, ModPowerOf2MulModPolynomialPrecomputedAssign,
};
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::ModPowerOf2RemPrecomputed;

// The product of `a` and `b` modulo `f`, with coefficients modulo $2^k$, with `data` precomputed
// from `f` and $k$.
//
// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`, FLINT
// 3.6.0, with the modulus $2^k$.
pub(crate) fn mod_power_of_2_mul_mod_polynomial_precomputed_helper(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    pow: u64,
    data: &ModPolynomialData<Natural>,
) -> NaturalPolynomial {
    if assert_mul_mod_polynomial_arguments(a, b, f, pow, false) {
        return NaturalPolynomial::ZERO;
    }
    let product = a.mod_power_of_2_mul(b, pow);
    if product.len() < f.len() {
        product
    } else {
        (&product).mod_power_of_2_rem_precomputed(f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<Self, Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by value, the second
    /// factor by value, and `f` by value. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(b, f.clone(), 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(b, f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: Self,
        pow: u64,
        data: &Self::Data,
    ) -> Self {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(&self, &other, &f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<Self, &Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by value, the second
    /// factor by value, and `f` by reference. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(b, &f, 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(b, &f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: &Self,
        pow: u64,
        data: &Self::Data,
    ) -> Self {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(&self, &other, f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<&Self, Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by value, the second
    /// factor by reference, and `f` by value. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(&b, f.clone(), 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(&b, f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: Self,
        pow: u64,
        data: &Self::Data,
    ) -> Self {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(&self, other, &f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<&Self, &Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by value, the second
    /// factor by reference, and `f` by reference. Both factors must already be reduced modulo $f$,
    /// with degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(&b, &f, 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial_precomputed(&b, &f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: &Self,
        pow: u64,
        data: &Self::Data,
    ) -> Self {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(&self, other, f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<NaturalPolynomial, NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by reference, the
    /// second factor by value, and `f` by value. Both factors must already be reduced modulo $f$,
    /// with degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(b, f.clone(), 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(b, f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: NaturalPolynomial,
        f: NaturalPolynomial,
        pow: u64,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, &other, &f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<NaturalPolynomial, &NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by reference, the
    /// second factor by value, and `f` by reference. Both factors must already be reduced modulo
    /// $f$, with degrees less than that of $f$, and the coefficients of all three must be reduced
    /// modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(b, &f, 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(b, &f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: NaturalPolynomial,
        f: &NaturalPolynomial,
        pow: u64,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, &other, f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<&NaturalPolynomial, NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by reference, the
    /// second factor by reference, and `f` by value. Both factors must already be reduced modulo
    /// $f$, with degrees less than that of $f$, and the coefficients of all three must be reduced
    /// modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(&b, f.clone(), 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(&b, f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: &NaturalPolynomial,
        f: NaturalPolynomial,
        pow: u64,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, other, &f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputed<&NaturalPolynomial, &NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the first factor by reference, the
    /// second factor by reference, and `f` by reference. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(&b, &f, 3, &data)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial_precomputed(&b, &f, 3, &data)
    ///         .to_string(),
    ///     "2*x^2+3*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: &NaturalPolynomial,
        f: &NaturalPolynomial,
        pow: u64,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, other, f, pow, data)
    }
}

impl ModPowerOf2MulModPolynomialPrecomputedAssign<Self, Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, using data precomputed from $f$ and $k$, taking the other factor by
    /// value and `f` by value. Both factors must already be reduced modulo $f$, with degrees less
    /// than that of $f$, and the coefficients of all three must be reduced modulo $2^k$; the
    /// leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(b, f.clone(), 3, &data);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(b, f, 3, &data);
    /// assert_eq!(a.to_string(), "2*x^2+3*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: Self,
        pow: u64,
        data: &Self::Data,
    ) {
        *self = mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, &other, &f, pow, data);
    }
}

impl ModPowerOf2MulModPolynomialPrecomputedAssign<Self, &Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, using data precomputed from $f$ and $k$, taking the other factor by
    /// value and `f` by reference. Both factors must already be reduced modulo $f$, with degrees
    /// less than that of $f$, and the coefficients of all three must be reduced modulo $2^k$; the
    /// leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(b, &f, 3, &data);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(b, &f, 3, &data);
    /// assert_eq!(a.to_string(), "2*x^2+3*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: &Self,
        pow: u64,
        data: &Self::Data,
    ) {
        *self = mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, &other, f, pow, data);
    }
}

impl ModPowerOf2MulModPolynomialPrecomputedAssign<&Self, Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, using data precomputed from $f$ and $k$, taking the other factor by
    /// reference and `f` by value. Both factors must already be reduced modulo $f$, with degrees
    /// less than that of $f$, and the coefficients of all three must be reduced modulo $2^k$; the
    /// leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(&b, f.clone(), 3, &data);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(&b, f, 3, &data);
    /// assert_eq!(a.to_string(), "2*x^2+3*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: Self,
        pow: u64,
        data: &Self::Data,
    ) {
        *self = mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, other, &f, pow, data);
    }
}

impl ModPowerOf2MulModPolynomialPrecomputedAssign<&Self, &Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, using data precomputed from $f$ and $k$, taking the other factor by
    /// reference and `f` by reference. Both factors must already be reduced modulo $f$, with
    /// degrees less than that of $f$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed), as for division;
    /// with it the product is reduced modulo $f$ with one truncated product for the quotient and
    /// one for the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0), or if `self` or `other` is not shorter than `f`. If `data`
    /// was not precomputed from `f` and `pow`, the result is meaningless, and the function may
    /// panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(&b, &f, 3, &data);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_precomputed_assign(&b, &f, 3, &data);
    /// assert_eq!(a.to_string(), "2*x^2+3*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: &Self,
        pow: u64,
        data: &Self::Data,
    ) {
        *self = mod_power_of_2_mul_mod_polynomial_precomputed_helper(self, other, f, pow, data);
    }
}
