// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2013 William Hart
//
//      Copyright © 2013 Martin Lee
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{
    ModPolynomialData, ModPowerOf2RemPrecomputed, ModPowerOf2RemPrecomputedAssign,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem_precomputed::PrecomputedDivision;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::reduced;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem_precomputed::{
    mod_power_of_2_div_rem_blocks, precompute_helper, precomputed_division,
};
use core::mem::take;

// The remainder of `p` by `q` modulo $2^k$, where $k$ is `pow`, with `data` precomputed from `q`
// and $k$.
//
// This is equivalent to the remainder of `nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
// divided.
pub(crate) fn rem_precomputed_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
    data: &ModPolynomialData<T>,
) -> UnsignedPolynomial<T> {
    match precomputed_division(p, q, pow, data) {
        PrecomputedDivision::Zero | PrecomputedDivision::Constant(_) => UnsignedPolynomial::ZERO,
        PrecomputedDivision::Short => p.clone(),
        PrecomputedDivision::General(xs, ys) => reduced(
            mod_power_of_2_div_rem_blocks(xs, ys, &data.inverse, pow).1,
            pow,
        ),
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2RemPrecomputed<Self> for UnsignedPolynomial<T> {
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_rem_precomputed`](
    /// ModPowerOf2RemPrecomputed::mod_power_of_2_rem_precomputed) or
    /// [`mod_power_of_2_rem_precomputed_assign`](
    /// ModPowerOf2RemPrecomputedAssign::mod_power_of_2_rem_precomputed_assign). The coefficients of
    /// `other` must already be reduced modulo $2^k$, and its leading coefficient must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModPowerOf2DivRemPrecomputed`](crate::polynomial::ModPowerOf2DivRemPrecomputed),
    /// [`ModPowerOf2QuotientPrecomputed`](crate::polynomial::ModPowerOf2QuotientPrecomputed), and
    /// [`ModPowerOf2RemPrecomputed`] alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `other` is greater than
    /// or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(q, 3, &data).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_rem_data(other: &Self, pow: u64) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the remainder, using data
    /// precomputed from the divisor and $k$, taking both by value. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_rem_data`](
    /// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data). It holds the inverse of the
    /// reversed divisor as a power series, with which a dividend no more than twice as long as the
    /// divisor takes one truncated product for the quotient and one for the remainder, and a longer
    /// one is divided that many coefficients at a time, from the top. A long divisor with few
    /// nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// result is reduced once at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0). If `data` was not
    /// precomputed from `other` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(q, 3, &data).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to the remainder computed by `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_rem_precomputed(
        self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Self {
        rem_precomputed_helper(&self, &other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2RemPrecomputed<&Self> for UnsignedPolynomial<T> {
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_rem_precomputed`](
    /// ModPowerOf2RemPrecomputed::mod_power_of_2_rem_precomputed) or
    /// [`mod_power_of_2_rem_precomputed_assign`](
    /// ModPowerOf2RemPrecomputedAssign::mod_power_of_2_rem_precomputed_assign). The coefficients of
    /// `other` must already be reduced modulo $2^k$, and its leading coefficient must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModPowerOf2DivRemPrecomputed`](crate::polynomial::ModPowerOf2DivRemPrecomputed),
    /// [`ModPowerOf2QuotientPrecomputed`](crate::polynomial::ModPowerOf2QuotientPrecomputed), and
    /// [`ModPowerOf2RemPrecomputed`] alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `other` is greater than
    /// or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(&q, 3, &data).to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(&q, 3, &data).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_rem_data(other: &&Self, pow: u64) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the remainder, using data
    /// precomputed from the divisor and $k$, taking the first by value and the second by reference.
    /// The coefficients of both must already be reduced modulo $2^k$, and the leading coefficient
    /// of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_rem_data`](
    /// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data). It holds the inverse of the
    /// reversed divisor as a power series, with which a dividend no more than twice as long as the
    /// divisor takes one truncated product for the quotient and one for the remainder, and a longer
    /// one is divided that many coefficients at a time, from the top. A long divisor with few
    /// nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// result is reduced once at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0). If `data` was not
    /// precomputed from `other` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(&q, 3, &data).to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_rem_precomputed(&q, 3, &data).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to the remainder computed by `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_rem_precomputed(
        self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Self {
        rem_precomputed_helper(&self, other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2RemPrecomputed<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_rem_precomputed`](
    /// ModPowerOf2RemPrecomputed::mod_power_of_2_rem_precomputed) or
    /// [`mod_power_of_2_rem_precomputed_assign`](
    /// ModPowerOf2RemPrecomputedAssign::mod_power_of_2_rem_precomputed_assign). The coefficients of
    /// `other` must already be reduced modulo $2^k$, and its leading coefficient must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModPowerOf2DivRemPrecomputed`](crate::polynomial::ModPowerOf2DivRemPrecomputed),
    /// [`ModPowerOf2QuotientPrecomputed`](crate::polynomial::ModPowerOf2QuotientPrecomputed), and
    /// [`ModPowerOf2RemPrecomputed`] alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `other` is greater than
    /// or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(q, 3, &data).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_rem_data(
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the remainder, using data
    /// precomputed from the divisor and $k$, taking the first by reference and the second by value.
    /// The coefficients of both must already be reduced modulo $2^k$, and the leading coefficient
    /// of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_rem_data`](
    /// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data). It holds the inverse of the
    /// reversed divisor as a power series, with which a dividend no more than twice as long as the
    /// divisor takes one truncated product for the quotient and one for the remainder, and a longer
    /// one is divided that many coefficients at a time, from the top. A long divisor with few
    /// nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// result is reduced once at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0). If `data` was not
    /// precomputed from `other` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(q, 3, &data).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to the remainder computed by `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_rem_precomputed(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        rem_precomputed_helper(self, &other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2RemPrecomputed<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_rem_precomputed`](
    /// ModPowerOf2RemPrecomputed::mod_power_of_2_rem_precomputed) or
    /// [`mod_power_of_2_rem_precomputed_assign`](
    /// ModPowerOf2RemPrecomputedAssign::mod_power_of_2_rem_precomputed_assign). The coefficients of
    /// `other` must already be reduced modulo $2^k$, and its leading coefficient must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModPowerOf2DivRemPrecomputed`](crate::polynomial::ModPowerOf2DivRemPrecomputed),
    /// [`ModPowerOf2QuotientPrecomputed`](crate::polynomial::ModPowerOf2QuotientPrecomputed), and
    /// [`ModPowerOf2RemPrecomputed`] alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `other` is greater than
    /// or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_rem_data(
        other: &&UnsignedPolynomial<T>,
        pow: u64,
    ) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the remainder, using data
    /// precomputed from the divisor and $k$, taking both by reference. The coefficients of both
    /// must already be reduced modulo $2^k$, and the leading coefficient of the divisor must be
    /// odd.
    ///
    /// $$
    /// f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_rem_data`](
    /// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data). It holds the inverse of the
    /// reversed divisor as a power series, with which a dividend no more than twice as long as the
    /// divisor takes one truncated product for the quotient and one for the remainder, and a longer
    /// one is divided that many coefficients at a time, from the top. A long divisor with few
    /// nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// result is reduced once at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0). If `data` was not
    /// precomputed from `other` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2RemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "6"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_rem_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to the remainder computed by `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_rem_precomputed(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        rem_precomputed_helper(self, other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2RemPrecomputedAssign<Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, replacing the dividend
    /// by the remainder, using data precomputed from the divisor and $k$, taking the divisor by
    /// value. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_rem_data`](
    /// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data). It holds the inverse of the
    /// reversed divisor as a power series, with which a dividend no more than twice as long as the
    /// divisor takes one truncated product for the quotient and one for the remainder, and a longer
    /// one is divided that many coefficients at a time, from the top. A long divisor with few
    /// nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// result is reduced once at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0). If `data` was not
    /// precomputed from `other` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_power_of_2_rem_precomputed_assign(q.clone(), 3, &data);
    /// assert_eq!(p.to_string(), "6");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// p.mod_power_of_2_rem_precomputed_assign(q, 3, &data);
    /// assert_eq!(p.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to the remainder computed by `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_rem_precomputed_assign(
        &mut self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) {
        *self = rem_precomputed_helper(&take(self), &other, pow, data);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2RemPrecomputedAssign<&Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, replacing the dividend
    /// by the remainder, using data precomputed from the divisor and $k$, taking the divisor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_rem_data`](
    /// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data). It holds the inverse of the
    /// reversed divisor as a power series, with which a dividend no more than twice as long as the
    /// divisor takes one truncated product for the quotient and one for the remainder, and a longer
    /// one is divided that many coefficients at a time, from the top. A long divisor with few
    /// nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// result is reduced once at the end.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0). If `data` was not
    /// precomputed from `other` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_rem_data(&q, 3);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_power_of_2_rem_precomputed_assign(&q, 3, &data);
    /// assert_eq!(p.to_string(), "6");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// p.mod_power_of_2_rem_precomputed_assign(&q, 3, &data);
    /// assert_eq!(p.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to the remainder computed by `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_rem_precomputed_assign(
        &mut self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) {
        *self = rem_precomputed_helper(&take(self), other, pow, data);
    }
}
