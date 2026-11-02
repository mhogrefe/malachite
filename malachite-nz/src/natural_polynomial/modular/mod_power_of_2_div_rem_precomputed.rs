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
use crate::natural_polynomial::modular::mod_div_rem::{from_words, to_words, trimmed};
use crate::natural_polynomial::modular::mod_div_rem_precomputed::reduce_to_window;
use crate::natural_polynomial::modular::mod_power_of_2_div_rem::{assert_reduced, remainder};
use crate::natural_polynomial::series::mod_power_of_2_inverse_truncated::*;
use crate::natural_polynomial::series::mod_power_of_2_mul_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::take;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::{
    ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2DivAssignRemPrecomputed, ModPowerOf2DivRemPrecomputed,
};
use malachite_base::unsigned_polynomial::modular::mod_power_of_2_div_rem_precomputed::{
    mod_power_of_2_div_rem_precomputed_vec, mod_power_of_2_polynomial_inverse_vec,
    mod_power_of_2_quotient_precomputed_vec,
};

// The inverse modulo $2^k$, where $k$ is `pow`, of the reversal of the polynomial with coefficients
// `ys`, as a power series truncated to `ys.len()` coefficients, where `ys` is nonempty and its last
// coefficient has the inverse `lead_inverse` modulo $2^k$.
//
// This is equivalent to `fmpz_mod_poly_inv_series` applied to `fmpz_mod_poly_reverse` of the
// divisor, as in the preparation of `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$.
fn reversed_inverse(ys: &[Natural], pow: u64, lead_inverse: &Natural) -> Vec<Natural> {
    let ys_reversed: Vec<Natural> = ys.iter().rev().cloned().collect();
    mod_power_of_2_inverse_truncated_dispatch(&ys_reversed, ys.len(), pow, lead_inverse)
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, where `ys` is no longer than `xs`, `xs` has at most `2 * ys.len() - 2`
// coefficients, and `inverse` is the inverse of the reversal of `ys` truncated to `ys.len()`
// coefficients.
//
// This is equivalent to `_fmpz_mod_poly_div_newton_n_preinv` from
// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$.
crate_test_fn! {mod_power_of_2_quotient_preinv(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    pow: u64,
) -> Vec<Natural> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_reversed: Vec<Natural> = xs[xs.len() - q_len..].iter().rev().cloned().collect();
    let mut qs = mod_power_of_2_mul_truncated_ref_ref(
        &xs_reversed,
        &inverse[..min(q_len, inverse.len())],
        u64::exact_from(q_len),
        pow,
    )
    .into_coefficients_asc();
    qs.resize(q_len, Natural::ZERO);
    qs.reverse();
    qs
}}

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients
// `xs` and `ys`, reduced, under the conditions of `mod_power_of_2_quotient_preinv`.
//
// This is equivalent to `_fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$.
crate_test_fn! {mod_power_of_2_div_rem_preinv(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    pow: u64,
) -> (Vec<Natural>, Vec<Natural>) {
    let qs = mod_power_of_2_quotient_preinv(xs, ys, inverse, pow);
    let rs = remainder(xs, ys, &qs, pow);
    (qs, rs)
}}

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `ys` has at least 2 coefficients and is no longer than `xs`, and
// `inverse` is the inverse of the reversal of `ys` truncated to `ys.len()` coefficients. A dividend
// too long for `inverse` is first reduced a block at a time.
fn mod_power_of_2_div_rem_blocks(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    pow: u64,
) -> (Vec<Natural>, Vec<Natural>) {
    if xs.len() <= (ys.len() << 1) - 2 {
        return mod_power_of_2_div_rem_preinv(xs, ys, inverse, pow);
    }
    let mut qs = vec![Natural::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys.len(), |xs| {
        mod_power_of_2_div_rem_preinv(xs, ys, inverse, pow)
    });
    let (q, r) = mod_power_of_2_div_rem_preinv(&rs[..len], ys, inverse, pow);
    for (out, x) in qs.iter_mut().zip(q) {
        *out = x;
    }
    (qs, r)
}

// The quotient modulo $2^k$ of the polynomials with coefficients `xs` and `ys`, reduced, as for
// `mod_power_of_2_div_rem_blocks`, without the remainder of the last block.
pub(crate) fn mod_power_of_2_quotient_blocks(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    pow: u64,
) -> Vec<Natural> {
    if xs.len() <= (ys.len() << 1) - 2 {
        return mod_power_of_2_quotient_preinv(xs, ys, inverse, pow);
    }
    let mut qs = vec![Natural::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys.len(), |xs| {
        mod_power_of_2_div_rem_preinv(xs, ys, inverse, pow)
    });
    for (out, x) in qs
        .iter_mut()
        .zip(mod_power_of_2_quotient_preinv(&rs[..len], ys, inverse, pow))
    {
        *out = x;
    }
    qs
}

// The data for dividing by `ys`, nonempty, modulo $2^k$, where $k$ is `pow`, which is no greater
// than `T::WIDTH`, computed in `T`, or `None` if the leading coefficient of `ys` is even.
fn word_inverse<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    ys: &[Natural],
    pow: u64,
) -> Option<Vec<Natural>>
where
    Natural: From<T>,
{
    mod_power_of_2_polynomial_inverse_vec(&to_words::<T>(ys), pow)
        .map(|inverse| from_words(inverse).coefficients)
}

// The data for dividing by `q` modulo $2^k$, where $k$ is `pow`. When $2^k$ fits in one or two
// limbs, it is computed in word arithmetic.
//
// This is equivalent to preparing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that modulo
// 1 the data is empty.
pub(crate) fn precompute_helper(q: &NaturalPolynomial, pow: u64) -> ModPolynomialData<Natural> {
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return ModPolynomialData {
            inverse: Vec::new(),
        };
    }
    assert_ne!(*q, NaturalPolynomial::ZERO, "division by zero");
    let ys = &q.coefficients;
    let inverse = if pow <= Limb::WIDTH {
        word_inverse::<Limb>(ys, pow)
    } else if pow <= DoubleLimb::WIDTH {
        word_inverse::<DoubleLimb>(ys, pow)
    } else {
        ys.last()
            .unwrap()
            .mod_power_of_2_inverse(pow)
            .map(|lead_inverse| reversed_inverse(ys, pow, &lead_inverse))
    };
    ModPolynomialData {
        inverse: inverse.expect("the leading coefficient of the divisor must be odd"),
    }
}

// What dividing one polynomial by another modulo $2^k$ with precomputed data comes to, after the
// checks that every such division makes.
pub(crate) enum PrecomputedDivision<'a> {
    // Modulo 1, every polynomial is 0, and so are the quotient and remainder.
    Zero,
    // The dividend is shorter than the divisor, so the quotient is 0 and the remainder is the
    // dividend.
    Short,
    // $2^k$ fits in a limb, or in two: the coefficients of the dividend and the divisor, to be
    // divided in word arithmetic.
    Limb(&'a [Natural], &'a [Natural]),
    DoubleLimb(&'a [Natural], &'a [Natural]),
    // The divisor is a constant, whose inverse is given, so the remainder is 0.
    Constant(&'a Natural),
    // Otherwise: the coefficients of the dividend and the divisor.
    General(&'a [Natural], &'a [Natural]),
}

// Checks that `p` and `q` are reduced modulo $2^k$, where $k$ is `pow`, and that `q` is nonzero
// unless $k$ is 0, and finds which case dividing `p` by `q` with `data` is.
pub(crate) fn precomputed_division<'a>(
    p: &'a NaturalPolynomial,
    q: &'a NaturalPolynomial,
    pow: u64,
    data: &'a ModPolynomialData<Natural>,
) -> PrecomputedDivision<'a> {
    assert_reduced(p, q, pow);
    if pow == 0 {
        return PrecomputedDivision::Zero;
    }
    assert_ne!(*q, NaturalPolynomial::ZERO, "division by zero");
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if xs.len() < ys.len() {
        PrecomputedDivision::Short
    } else if pow <= Limb::WIDTH {
        PrecomputedDivision::Limb(xs, ys)
    } else if pow <= DoubleLimb::WIDTH {
        PrecomputedDivision::DoubleLimb(xs, ys)
    } else if ys.len() == 1 {
        PrecomputedDivision::Constant(&data.inverse[0])
    } else {
        PrecomputedDivision::General(xs, ys)
    }
}

// The product of the polynomial with coefficients `xs` and the constant `c` modulo $2^k$, where $k$
// is `pow`.
pub(crate) fn mod_power_of_2_mul_constant(
    xs: &[Natural],
    c: &Natural,
    pow: u64,
) -> NaturalPolynomial {
    trimmed(xs.iter().map(|x| x.mod_power_of_2_mul(c, pow)).collect())
}

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, which is no greater than `T::WIDTH`,
// of the polynomials with coefficients `xs` and `ys`, with `inverse` precomputed, computed in `T`.
pub(crate) fn div_rem_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    pow: u64,
) -> (NaturalPolynomial, NaturalPolynomial)
where
    Natural: From<T>,
{
    let (qs, rs) = mod_power_of_2_div_rem_precomputed_vec(
        &to_words::<T>(xs),
        &to_words::<T>(ys),
        &to_words::<T>(inverse),
        pow,
    );
    (from_words(qs), from_words(rs))
}

// The quotient modulo $2^k$, where $k$ is `pow`, which is no greater than `T::WIDTH`, of the
// polynomials with coefficients `xs` and `ys`, with `inverse` precomputed, computed in `T`.
pub(crate) fn quotient_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    pow: u64,
) -> NaturalPolynomial
where
    Natural: From<T>,
{
    from_words(mod_power_of_2_quotient_precomputed_vec(
        &to_words::<T>(xs),
        &to_words::<T>(ys),
        &to_words::<T>(inverse),
        pow,
    ))
}

// The quotient and remainder of `p` by `q` modulo $2^k$, where $k$ is `pow`, with `data`
// precomputed from `q` and $k$.
//
// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
// dividend of any length may be divided.
pub(crate) fn div_rem_precomputed_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
    data: &ModPolynomialData<Natural>,
) -> (NaturalPolynomial, NaturalPolynomial) {
    match precomputed_division(p, q, pow, data) {
        PrecomputedDivision::Zero => (NaturalPolynomial::ZERO, NaturalPolynomial::ZERO),
        PrecomputedDivision::Short => (NaturalPolynomial::ZERO, p.clone()),
        PrecomputedDivision::Limb(xs, ys) => div_rem_word::<Limb>(xs, ys, &data.inverse, pow),
        PrecomputedDivision::DoubleLimb(xs, ys) => {
            div_rem_word::<DoubleLimb>(xs, ys, &data.inverse, pow)
        }
        PrecomputedDivision::Constant(c) => (
            mod_power_of_2_mul_constant(&p.coefficients, c, pow),
            NaturalPolynomial::ZERO,
        ),
        PrecomputedDivision::General(xs, ys) => {
            let (qs, rs) = mod_power_of_2_div_rem_blocks(xs, ys, &data.inverse, pow);
            (trimmed(qs), trimmed(rs))
        }
    }
}

impl ModPowerOf2DivRemPrecomputed<Self> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(
        other: &Self,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $k$, taking the dividend by value and the divisor by
    /// value. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, &other, pow, data)
    }
}

impl ModPowerOf2DivRemPrecomputed<&Self> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(
        other: &&Self,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $k$, taking the dividend by value and the divisor by
    /// reference. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, other, pow, data)
    }
}

impl ModPowerOf2DivRemPrecomputed<NaturalPolynomial> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(
        other: &NaturalPolynomial,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $k$, taking the dividend by reference and the divisor
    /// by value. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        div_rem_precomputed_helper(self, &other, pow, data)
    }
}

impl ModPowerOf2DivRemPrecomputed<&NaturalPolynomial> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `fmpz_mod_poly_reverse` and `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(
        other: &&NaturalPolynomial,
        pow: u64,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, pow)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $k$, taking the dividend by reference and the divisor
    /// by reference. The coefficients of both must already be reduced modulo $2^k$, and the leading
    /// coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: &NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        div_rem_precomputed_helper(self, other, pow, data)
    }
}

impl ModPowerOf2DivAssignRemPrecomputed<Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, returning the remainder,
    /// using data precomputed from the divisor and $k$, taking the divisor by value. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(p.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_assign_rem_precomputed(
        &mut self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), &other, pow, data);
        *self = q;
        r
    }
}

impl ModPowerOf2DivAssignRemPrecomputed<&Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, returning the remainder,
    /// using data precomputed from the divisor and $k$, taking the divisor by reference. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). When $2^k$ fits in
    /// one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(&q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(&q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_assign_rem_precomputed(
        &mut self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), other, pow, data);
        *self = q;
        r
    }
}
