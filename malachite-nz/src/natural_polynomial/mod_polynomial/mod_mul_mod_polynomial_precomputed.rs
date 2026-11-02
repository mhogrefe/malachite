// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::mod_polynomial::mod_mul_mod_polynomial::*;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::modular::traits::ModMul;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModMulModPolynomialPrecomputed, ModMulModPolynomialPrecomputedAssign,
};
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::ModRemPrecomputed;

// The product of `a` and `b` modulo `f`, with coefficients modulo $m$, with `data` precomputed from
// `f` and $m$.
//
// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`, FLINT
// 3.6.0.
pub(crate) fn mod_mul_mod_polynomial_precomputed_helper(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    m: &Natural,
    data: &ModPolynomialData<Natural>,
) -> NaturalPolynomial {
    if assert_mul_mod_polynomial_arguments(a, b, f, m, false) {
        return NaturalPolynomial::ZERO;
    }
    let product = a.mod_mul(b, m);
    if product.len() < f.len() {
        product
    } else {
        (&product).mod_rem_precomputed(f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<Self, Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by value, `f` by value, and the modulus by value. Both factors must already be reduced
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
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, f.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: Self,
        m: Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, &other, &f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<Self, Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by value, `f` by value, and the modulus by reference. Both factors must already be reduced
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
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, f.clone(), &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: Self,
        m: &Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, &other, &f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<Self, &Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by value, `f` by reference, and the modulus by value. Both factors must already be reduced
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
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, &f, m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, &f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: &Self,
        m: Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, &other, f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<Self, &Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by value, `f` by reference, and the modulus by reference. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: Self,
        f: &Self,
        m: &Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, &other, f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&Self, Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by reference, `f` by value, and the modulus by value. Both factors must already be reduced
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
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, f.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: Self,
        m: Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, other, &f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&Self, Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by reference, `f` by value, and the modulus by reference. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, f.clone(), &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: Self,
        m: &Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, other, &f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&Self, &Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by reference, `f` by reference, and the modulus by value. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, &f, m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, &f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: &Self,
        m: Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, other, f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&Self, &Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by value, the second factor
    /// by reference, `f` by reference, and the modulus by reference. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     a.mod_mul_mod_polynomial_precomputed(&b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &Self,
        f: &Self,
        m: &Natural,
        data: &Self::Data,
    ) -> Self {
        mod_mul_mod_polynomial_precomputed_helper(&self, other, f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<NaturalPolynomial, NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by value, `f` by value, and the modulus by value. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, f.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: NaturalPolynomial,
        f: NaturalPolynomial,
        m: Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, &other, &f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<NaturalPolynomial, NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by value, `f` by value, and the modulus by reference. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, f.clone(), &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: NaturalPolynomial,
        f: NaturalPolynomial,
        m: &Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, &other, &f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<NaturalPolynomial, &NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by value, `f` by reference, and the modulus by value. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, &f, m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, &f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: NaturalPolynomial,
        f: &NaturalPolynomial,
        m: Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, &other, f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<NaturalPolynomial, &NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by value, `f` by reference, and the modulus by reference. Both factors must already
    /// be reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: NaturalPolynomial,
        f: &NaturalPolynomial,
        m: &Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, &other, f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&NaturalPolynomial, NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by reference, `f` by value, and the modulus by value. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, f.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &NaturalPolynomial,
        f: NaturalPolynomial,
        m: Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, other, &f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&NaturalPolynomial, NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by reference, `f` by value, and the modulus by reference. Both factors must already
    /// be reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, f.clone(), &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &NaturalPolynomial,
        f: NaturalPolynomial,
        m: &Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, other, &f, m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&NaturalPolynomial, &NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by reference, `f` by reference, and the modulus by value. Both factors must already
    /// be reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, &f, m.clone(), &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, &f, m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &NaturalPolynomial,
        f: &NaturalPolynomial,
        m: Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, other, f, &m, data)
    }
}

impl ModMulModPolynomialPrecomputed<&NaturalPolynomial, &NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo $m$,
    /// using data precomputed from $f$ and $m$, taking the first factor by reference, the second
    /// factor by reference, `f` by reference, and the modulus by reference. Both factors must
    /// already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients of
    /// all three must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::mod_polynomial::traits::ModMulModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+4"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul_mod_polynomial_precomputed(&b, &f, &m, &data)
    ///         .to_string(),
    ///     "2*x^2+2*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: &NaturalPolynomial,
        f: &NaturalPolynomial,
        m: &Natural,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_mul_mod_polynomial_precomputed_helper(self, other, f, m, data)
    }
}

impl ModMulModPolynomialPrecomputedAssign<Self, Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// value, `f` by value, and the modulus by value. Both factors must already be reduced modulo
    /// $f$, with degrees less than that of $f$, and the coefficients of all three must be reduced
    /// modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, f.clone(), m.clone(), &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, f, m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: Self,
        m: Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, &other, &f, &m, data);
    }
}

impl ModMulModPolynomialPrecomputedAssign<Self, Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// value, `f` by value, and the modulus by reference. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, f.clone(), &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, f, &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: Self,
        m: &Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, &other, &f, m, data);
    }
}

impl ModMulModPolynomialPrecomputedAssign<Self, &Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// value, `f` by reference, and the modulus by value. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, &f, m.clone(), &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, &f, m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: &Self,
        m: Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, &other, f, &m, data);
    }
}

impl ModMulModPolynomialPrecomputedAssign<Self, &Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// value, `f` by reference, and the modulus by reference. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, &f, &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(b, &f, &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Self,
        f: &Self,
        m: &Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, &other, f, m, data);
    }
}

impl ModMulModPolynomialPrecomputedAssign<&Self, Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// reference, `f` by value, and the modulus by value. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, f.clone(), m.clone(), &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, f, m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: Self,
        m: Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, other, &f, &m, data);
    }
}

impl ModMulModPolynomialPrecomputedAssign<&Self, Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// reference, `f` by value, and the modulus by reference. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, f.clone(), &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, f, &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: Self,
        m: &Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, other, &f, m, data);
    }
}

impl ModMulModPolynomialPrecomputedAssign<&Self, &Self, Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// reference, `f` by reference, and the modulus by value. Both factors must already be reduced
    /// modulo $f$, with degrees less than that of $f$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, &f, m.clone(), &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, &f, m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: &Self,
        m: Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, other, f, &m, data);
    }
}

impl ModMulModPolynomialPrecomputedAssign<&Self, &Self, &Natural> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $m$, in place, using data precomputed from $f$ and $m$, taking the other factor by
    /// reference, `f` by reference, and the modulus by reference. Both factors must already be
    /// reduced modulo $f$, with degrees less than that of $f$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::modular::traits::ModDivRemPrecomputed), as for division; with it
    /// the product is reduced modulo $f$ with one truncated product for the quotient and one for
    /// the remainder.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &m);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, &f, &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+4");
    ///
    /// let mut a = NaturalPolynomial::from_str("2*x^2+3").unwrap();
    /// let b = NaturalPolynomial::from_str("5*x+1").unwrap();
    /// a.mod_mul_mod_polynomial_precomputed_assign(&b, &f, &m, &data);
    /// assert_eq!(a.to_string(), "2*x^2+2*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod_preinv` from `fmpz_mod_poly/mulmod_preinv.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: &Self,
        f: &Self,
        m: &Natural,
        data: &Self::Data,
    ) {
        *self = mod_mul_mod_polynomial_precomputed_helper(self, other, f, m, data);
    }
}
