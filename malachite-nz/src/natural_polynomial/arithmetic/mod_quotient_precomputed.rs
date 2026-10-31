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

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_div_rem::trimmed;
use crate::natural_polynomial::arithmetic::mod_div_rem_precomputed::{
    PrecomputedDivision, mod_mul_constant, mod_quotient_blocks, precompute_helper,
    precomputed_division, quotient_word,
};
use crate::platform::{DoubleLimb, Limb};
use core::mem::take;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ModPolynomialData, ModQuotientPrecomputed, ModQuotientPrecomputedAssign,
};

// The quotient of `p` by `q` modulo `m`, with `data` precomputed from `q` and `m`.
//
// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
// divided.
pub(crate) fn quotient_precomputed_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
    data: &ModPolynomialData<Natural>,
) -> NaturalPolynomial {
    match precomputed_division(p, q, m, data) {
        PrecomputedDivision::Zero | PrecomputedDivision::Short => NaturalPolynomial::ZERO,
        PrecomputedDivision::Limb(xs, ys) => quotient_word::<Limb>(xs, ys, &data.inverse, m),
        PrecomputedDivision::DoubleLimb(xs, ys) => {
            quotient_word::<DoubleLimb>(xs, ys, &data.inverse, m)
        }
        PrecomputedDivision::Constant(c) => mod_mul_constant(&p.coefficients, c, m),
        PrecomputedDivision::General(xs, ys) => {
            trimmed(mod_quotient_blocks(xs, ys, &data.inverse, m))
        }
    }
}

impl ModQuotientPrecomputed<Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_quotient_precomputed(q, m, &data).to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &Self, m: &Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by value, the divisor by value,
    /// and the modulus by value. The coefficients of both must already be reduced modulo $m$, and
    /// the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(p.mod_quotient_precomputed(q, m, &data).to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        quotient_precomputed_helper(&self, &other, &m, data)
    }
}

impl ModQuotientPrecomputed<Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q.clone(), &m, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &Self, m: &&Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by value, the divisor by value,
    /// and the modulus by reference. The coefficients of both must already be reduced modulo $m$,
    /// and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q.clone(), &m, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        quotient_precomputed_helper(&self, &other, m, data)
    }
}

impl ModQuotientPrecomputed<&Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, m.clone(), &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &&Self, m: &Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by value, the divisor by
    /// reference, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, m.clone(), &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        quotient_precomputed_helper(&self, other, &m, data)
    }
}

impl ModQuotientPrecomputed<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(other: &&Self, m: &&Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by value, the divisor by
    /// reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        quotient_precomputed_helper(&self, other, m, data)
    }
}

impl ModQuotientPrecomputed<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q, m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// value, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q, m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        quotient_precomputed_helper(self, &other, &m, data)
    }
}

impl ModQuotientPrecomputed<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q.clone(), &m, &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(
        other: &NaturalPolynomial,
        m: &&Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// value, and the modulus by reference. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q.clone(), &m, &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        quotient_precomputed_helper(self, &other, m, data)
    }
}

impl ModQuotientPrecomputed<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, m.clone(), &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(
        other: &&NaturalPolynomial,
        m: &Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// reference, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, m.clone(), &data)
    ///         .to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: &NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        quotient_precomputed_helper(self, other, &m, data)
    }
}

impl ModQuotientPrecomputed<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_quotient_precomputed`](ModQuotientPrecomputed::mod_quotient_precomputed) or
    /// [`mod_quotient_precomputed_assign`](
    /// ModQuotientPrecomputedAssign::mod_quotient_precomputed_assign). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $m$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `other` is greater than or equal to `m`, or if
    /// `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModQuotientPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_quotient_data(
        other: &&NaturalPolynomial,
        m: &&Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient, using data
    /// precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_quotient_precomputed(&q, &m, &data).to_string(),
    ///     "5*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        quotient_precomputed_helper(self, other, m, data)
    }
}

impl ModQuotientPrecomputedAssign<Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $m$, taking the divisor by value
    /// and the modulus by value. The coefficients of both must already be reduced modulo $m$, and
    /// the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(q.clone(), m.clone(), &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(q, m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed_assign(
        &mut self,
        other: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = quotient_precomputed_helper(&take(self), &other, &m, data);
    }
}

impl ModQuotientPrecomputedAssign<Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $m$, taking the divisor by value
    /// and the modulus by reference. The coefficients of both must already be reduced modulo $m$,
    /// and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(q.clone(), &m, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(q, &m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed_assign(
        &mut self,
        other: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = quotient_precomputed_helper(&take(self), &other, m, data);
    }
}

impl ModQuotientPrecomputedAssign<&Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $m$, taking the divisor by
    /// reference and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(&q, m.clone(), &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(&q, m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed_assign(
        &mut self,
        other: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = quotient_precomputed_helper(&take(self), other, &m, data);
    }
}

impl ModQuotientPrecomputedAssign<&Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $m$, taking the divisor by
    /// reference and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data). When
    /// $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_quotient_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(&q, &m, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_quotient_precomputed_assign(&q, &m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_quotient_precomputed_assign(
        &mut self,
        other: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = quotient_precomputed_helper(&take(self), other, m, data);
    }
}
