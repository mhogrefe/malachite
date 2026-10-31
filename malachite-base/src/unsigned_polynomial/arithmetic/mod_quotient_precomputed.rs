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
use crate::polynomial::{ModPolynomialData, ModQuotientPrecomputed, ModQuotientPrecomputedAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem_precomputed::{
    PrecomputedDivision, mod_mul_constant, mod_quotient_blocks, precompute_helper,
    precomputed_division,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use core::mem::take;

// The quotient of `p` by `q` modulo `m`, with `data` precomputed from `q` and `m`. A dividend of at
// most `2 * q.len() - 2` coefficients takes one truncated multiplication; a longer one is first
// reduced a block at a time.
//
// This is equivalent to `nmod_poly_div_newton_n_preinv` from `nmod_poly/div_newton_n_preinv.c`,
// FLINT 3.6.0, except that a dividend of any length may be divided.
pub(crate) fn quotient_precomputed_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
    data: &ModPolynomialData<T>,
) -> UnsignedPolynomial<T> {
    match precomputed_division(p, q, m, data) {
        PrecomputedDivision::Zero | PrecomputedDivision::Short => UnsignedPolynomial::ZERO,
        PrecomputedDivision::Constant(c) => mod_mul_constant(p, c, m),
        PrecomputedDivision::General(xs, ys) => {
            from_coefficients_trimmed(mod_quotient_blocks(xs, ys, &data.inverse, m))
        }
    }
}

impl<T: PrimitiveUnsigned> ModQuotientPrecomputed<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed),
    /// [`ModQuotientPrecomputed`], and [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed)
    /// alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q.clone(), 7, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_quotient_precomputed(q, 7, &data).to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &Self, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking both by value. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). It
    /// holds the inverse of the reversed divisor as a power series, with which a dividend no more
    /// than twice as long as the divisor takes one truncated product, and a longer one is first
    /// divided that many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1). If `data` was not precomputed from `other` and
    /// `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q.clone(), 7, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_quotient_precomputed(q, 7, &data).to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_newton_n_preinv` from
    /// `nmod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
    /// divided.
    #[inline]
    fn mod_quotient_precomputed(self, other: Self, m: T, data: &ModPolynomialData<T>) -> Self {
        quotient_precomputed_helper(&self, &other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModQuotientPrecomputed<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed),
    /// [`ModQuotientPrecomputed`], and [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed)
    /// alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &&Self, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the first by value and the second by reference.
    /// The coefficients of both must already be reduced modulo $m$, and the leading coefficient of
    /// the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). It
    /// holds the inverse of the reversed divisor as a power series, with which a dividend no more
    /// than twice as long as the divisor takes one truncated product, and a longer one is first
    /// divided that many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1). If `data` was not precomputed from `other` and
    /// `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_newton_n_preinv` from
    /// `nmod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
    /// divided.
    #[inline]
    fn mod_quotient_precomputed(self, other: &Self, m: T, data: &ModPolynomialData<T>) -> Self {
        quotient_precomputed_helper(&self, other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModQuotientPrecomputed<UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed),
    /// [`ModQuotientPrecomputed`], and [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed)
    /// alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q.clone(), 7, &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q, 7, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &UnsignedPolynomial<T>, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the first by reference and the second by value.
    /// The coefficients of both must already be reduced modulo $m$, and the leading coefficient of
    /// the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). It
    /// holds the inverse of the reversed divisor as a power series, with which a dividend no more
    /// than twice as long as the divisor takes one truncated product, and a longer one is first
    /// divided that many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1). If `data` was not precomputed from `other` and
    /// `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q.clone(), 7, &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q, 7, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_newton_n_preinv` from
    /// `nmod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
    /// divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        quotient_precomputed_helper(self, &other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModQuotientPrecomputed<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed),
    /// [`ModQuotientPrecomputed`], and [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed)
    /// alike.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &&UnsignedPolynomial<T>, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking both by reference. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). It
    /// holds the inverse of the reversed divisor as a power series, with which a dividend no more
    /// than twice as long as the divisor takes one truncated product, and a longer one is first
    /// divided that many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1). If `data` was not precomputed from `other` and
    /// `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, 7, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_newton_n_preinv` from
    /// `nmod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
    /// divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        quotient_precomputed_helper(self, other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModQuotientPrecomputedAssign<Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $m$, taking the divisor by value.
    /// The coefficients of both must already be reduced modulo $m$, and the leading coefficient of
    /// the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). It
    /// holds the inverse of the reversed divisor as a power series, with which a dividend no more
    /// than twice as long as the divisor takes one truncated product, and a longer one is first
    /// divided that many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1). If `data` was not precomputed from `other` and
    /// `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModQuotientPrecomputed, ModQuotientPrecomputedAssign};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(q.clone(), 7, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(q, 7, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_newton_n_preinv` from
    /// `nmod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
    /// divided.
    #[inline]
    fn mod_quotient_precomputed_assign(&mut self, other: Self, m: T, data: &ModPolynomialData<T>) {
        *self = quotient_precomputed_helper(&take(self), &other, m, data);
    }
}

impl<T: PrimitiveUnsigned> ModQuotientPrecomputedAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $m$, taking the divisor by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). It
    /// holds the inverse of the reversed divisor as a power series, with which a dividend no more
    /// than twice as long as the divisor takes one truncated product, and a longer one is first
    /// divided that many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1). If `data` was not precomputed from `other` and
    /// `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{ModQuotientPrecomputed, ModQuotientPrecomputedAssign};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &7);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(&q, 7, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(&q, 7, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_div_newton_n_preinv` from
    /// `nmod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
    /// divided.
    #[inline]
    fn mod_quotient_precomputed_assign(&mut self, other: &Self, m: T, data: &ModPolynomialData<T>) {
        *self = quotient_precomputed_helper(&take(self), other, m, data);
    }
}
