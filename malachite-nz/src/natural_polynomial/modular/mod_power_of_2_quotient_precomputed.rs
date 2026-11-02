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
use crate::natural_polynomial::modular::mod_div_rem::trimmed;
use crate::natural_polynomial::modular::mod_power_of_2_div_rem_precomputed::{
    PrecomputedDivision, mod_power_of_2_mul_constant, mod_power_of_2_quotient_blocks,
    precompute_helper, precomputed_division, quotient_word,
};
use crate::platform::{DoubleLimb, Limb};
use core::mem::take;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2QuotientPrecomputed, ModPowerOf2QuotientPrecomputedAssign,
};

// The quotient of `p` by `q` modulo $2^k$, where $k$ is `pow`, with `data` precomputed from `q` and
// $k$.
//
// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
// dividend of any length may be divided.
pub(crate) fn quotient_precomputed_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
    data: &ModPolynomialData<Natural>,
) -> NaturalPolynomial {
    match precomputed_division(p, q, pow, data) {
        PrecomputedDivision::Zero | PrecomputedDivision::Short => NaturalPolynomial::ZERO,
        PrecomputedDivision::Limb(xs, ys) => quotient_word::<Limb>(xs, ys, &data.inverse, pow),
        PrecomputedDivision::DoubleLimb(xs, ys) => {
            quotient_word::<DoubleLimb>(xs, ys, &data.inverse, pow)
        }
        PrecomputedDivision::Constant(c) => mod_power_of_2_mul_constant(&p.coefficients, c, pow),
        PrecomputedDivision::General(xs, ys) => {
            trimmed(mod_power_of_2_quotient_blocks(xs, ys, &data.inverse, pow))
        }
    }
}

impl ModPowerOf2QuotientPrecomputed<Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_quotient_precomputed`](
    /// ModPowerOf2QuotientPrecomputed::mod_power_of_2_quotient_precomputed) or
    /// [`mod_power_of_2_quotient_precomputed_assign`](
    /// ModPowerOf2QuotientPrecomputedAssign::mod_power_of_2_quotient_precomputed_assign). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $2^k$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `other` is greater than or equal to $2^k$, or if `other` is 0
    /// (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_quotient_data(
        other: &Self,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient, using data
    /// precomputed from the divisor and $k$, taking the dividend by value and the divisor by value.
    /// The coefficients of both must already be reduced modulo $2^k$, and the leading coefficient
    /// of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_quotient_data`](
    /// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0). If `data` was not precomputed from `other` and `pow`, the
    /// result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_quotient_precomputed(
        self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        quotient_precomputed_helper(&self, &other, pow, data)
    }
}

impl ModPowerOf2QuotientPrecomputed<&Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_quotient_precomputed`](
    /// ModPowerOf2QuotientPrecomputed::mod_power_of_2_quotient_precomputed) or
    /// [`mod_power_of_2_quotient_precomputed_assign`](
    /// ModPowerOf2QuotientPrecomputedAssign::mod_power_of_2_quotient_precomputed_assign). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $2^k$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `other` is greater than or equal to $2^k$, or if `other` is 0
    /// (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_quotient_data(
        other: &&Self,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient, using data
    /// precomputed from the divisor and $k$, taking the dividend by value and the divisor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_quotient_data`](
    /// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0). If `data` was not precomputed from `other` and `pow`, the
    /// result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_quotient_precomputed(
        self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        quotient_precomputed_helper(&self, other, pow, data)
    }
}

impl ModPowerOf2QuotientPrecomputed<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_quotient_precomputed`](
    /// ModPowerOf2QuotientPrecomputed::mod_power_of_2_quotient_precomputed) or
    /// [`mod_power_of_2_quotient_precomputed_assign`](
    /// ModPowerOf2QuotientPrecomputedAssign::mod_power_of_2_quotient_precomputed_assign). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $2^k$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `other` is greater than or equal to $2^k$, or if `other` is 0
    /// (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_quotient_data(
        other: &NaturalPolynomial,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient, using data
    /// precomputed from the divisor and $k$, taking the dividend by reference and the divisor by
    /// value. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_quotient_data`](
    /// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0). If `data` was not precomputed from `other` and `pow`, the
    /// result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(q.clone(), 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_quotient_precomputed(
        self,
        other: NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        quotient_precomputed_helper(self, &other, pow, data)
    }
}

impl ModPowerOf2QuotientPrecomputed<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_quotient_precomputed`](
    /// ModPowerOf2QuotientPrecomputed::mod_power_of_2_quotient_precomputed) or
    /// [`mod_power_of_2_quotient_precomputed_assign`](
    /// ModPowerOf2QuotientPrecomputedAssign::mod_power_of_2_quotient_precomputed_assign). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has, and the same data serves the division, quotient, and
    /// remainder traits alike. When $2^k$ fits in one or two limbs, it is computed in word
    /// arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `other.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `other` is greater than or equal to $2^k$, or if `other` is 0
    /// (unless $k$ is 0) or its leading coefficient is even.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_quotient_data(
        other: &&NaturalPolynomial,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient, using data
    /// precomputed from the divisor and $k$, taking the dividend by reference and the divisor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_quotient_data`](
    /// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0). If `data` was not precomputed from `other` and `pow`, the
    /// result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2QuotientPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x^3+7*x+3"
    /// );
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_quotient_precomputed(&q, 3, &data)
    ///         .to_string(),
    ///     "3*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_quotient_precomputed(
        self,
        other: &NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        quotient_precomputed_helper(self, other, pow, data)
    }
}

impl ModPowerOf2QuotientPrecomputedAssign<Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $k$, taking the divisor by value.
    /// The coefficients of both must already be reduced modulo $2^k$, and the leading coefficient
    /// of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_quotient_data`](
    /// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0). If `data` was not precomputed from `other` and `pow`, the
    /// result is meaningless, and the function may panic.
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
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_power_of_2_quotient_precomputed_assign(q.clone(), 3, &data);
    /// assert_eq!(p.to_string(), "3*x^3+7*x+3");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_power_of_2_quotient_precomputed_assign(q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x+7");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_quotient_precomputed_assign(
        &mut self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = quotient_precomputed_helper(&take(self), &other, pow, data);
    }
}

impl ModPowerOf2QuotientPrecomputedAssign<&Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, replacing the dividend by
    /// the quotient, using data precomputed from the divisor and $k$, taking the divisor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_quotient_data`](
    /// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product, and a longer one is first divided that
    /// many coefficients at a time, from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0). If `data` was not precomputed from `other` and `pow`, the
    /// result is meaningless, and the function may panic.
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
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, 3);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// p.mod_power_of_2_quotient_precomputed_assign(&q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x^3+7*x+3");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// p.mod_power_of_2_quotient_precomputed_assign(&q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x+7");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_div_newton_n_preinv` from
    /// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_quotient_precomputed_assign(
        &mut self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = quotient_precomputed_helper(&take(self), other, pow, data);
    }
}
