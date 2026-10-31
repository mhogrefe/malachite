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
use crate::natural_polynomial::arithmetic::mod_div_rem::{
    assert_reduced, from_words, remainder, to_words, trimmed,
};
use crate::natural_polynomial::arithmetic::mod_inverse_truncated::mod_inverse_truncated_dispatch;
use crate::natural_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_ref_ref;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{ModInverse, ModIsReduced, ModMul};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{
    ModDivAssignRemPrecomputed, ModDivRemPrecomputed, ModPolynomialData, Polynomial,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_div_rem_precomputed::{
    mod_div_rem_precomputed_vec, mod_polynomial_inverse_vec, mod_quotient_precomputed_vec,
};

// The inverse modulo `m` of the reversal of the polynomial with coefficients `ys`, as a power
// series truncated to `ys.len()` coefficients, where `ys` is nonempty and its last coefficient has
// the inverse `lead_inverse` modulo `m`.
//
// This is equivalent to `fmpz_mod_poly_inv_series` applied to `fmpz_mod_poly_reverse` of the
// divisor, as in the preparation of `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
fn reversed_inverse(ys: &[Natural], m: &Natural, lead_inverse: &Natural) -> Vec<Natural> {
    let ys_reversed: Vec<Natural> = ys.iter().rev().cloned().collect();
    mod_inverse_truncated_dispatch(&ys_reversed, ys.len(), m, lead_inverse)
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, where `ys`
// is no longer than `xs`, `xs` has at most `2 * ys.len() - 2` coefficients, and `inverse` is the
// inverse of the reversal of `ys` truncated to `ys.len()` coefficients. Reversed, the quotient is
// the product of the reversed top of `xs` and `inverse`, truncated.
//
// This is equivalent to `_fmpz_mod_poly_div_newton_n_preinv` from
// `fmpz_mod_poly/div_newton_n_preinv.c`, FLINT 3.6.0.
crate_test_fn! {mod_quotient_preinv(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    m: &Natural,
) -> Vec<Natural> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_reversed: Vec<Natural> = xs[xs.len() - q_len..].iter().rev().cloned().collect();
    let mut qs = mod_mul_truncated_ref_ref(
        &xs_reversed,
        &inverse[..min(q_len, inverse.len())],
        u64::exact_from(q_len),
        m,
    )
    .into_coefficients_asc();
    qs.resize(q_len, Natural::ZERO);
    qs.reverse();
    qs
}}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`,
// reduced, under the conditions of `mod_quotient_preinv`: the quotient, and the remainder from it
// with one truncated product.
//
// This is equivalent to `_fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
crate_test_fn! {mod_div_rem_preinv(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    m: &Natural,
) -> (Vec<Natural>, Vec<Natural>) {
    let qs = mod_quotient_preinv(xs, ys, inverse, m);
    let rs = remainder(xs, ys, &qs, m);
    (qs, rs)
}}

// Divides the polynomial with coefficients `rs` by one with `y_len` coefficients, at least 2 and no
// more than `rs.len()`, until it has at most `2 * y_len - 2` coefficients, the most that the
// divisor's precomputed inverse can divide at once, returning the number left. The dividend is
// divided that many coefficients at a time, from the top, by `divide`, which returns the quotient
// and remainder of such a block; each remainder takes the place of the coefficients it came from,
// and each quotient is written to the matching part of `qs`. At least `y_len` coefficients are
// left.
pub(crate) fn reduce_to_window(
    rs: &mut [Natural],
    qs: &mut [Natural],
    y_len: usize,
    divide: impl Fn(&[Natural]) -> (Vec<Natural>, Vec<Natural>),
) -> usize {
    let window = (y_len << 1) - 2;
    let mut len = rs.len();
    while len > window {
        let start = len - window;
        let (q, r) = divide(&rs[start..len]);
        for (out, x) in qs[start..].iter_mut().zip(q) {
            *out = x;
        }
        for (out, x) in rs[start..].iter_mut().zip(r) {
            *out = x;
        }
        len = start + y_len - 1;
    }
    len
}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`,
// reduced, where `ys` has at least 2 coefficients and is no longer than `xs`, and `inverse` is the
// inverse of the reversal of `ys` truncated to `ys.len()` coefficients. A dividend too long for
// `inverse` is first reduced a block at a time.
fn mod_div_rem_blocks(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    m: &Natural,
) -> (Vec<Natural>, Vec<Natural>) {
    if xs.len() <= (ys.len() << 1) - 2 {
        return mod_div_rem_preinv(xs, ys, inverse, m);
    }
    let mut qs = vec![Natural::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys.len(), |xs| {
        mod_div_rem_preinv(xs, ys, inverse, m)
    });
    let (q, r) = mod_div_rem_preinv(&rs[..len], ys, inverse, m);
    for (out, x) in qs.iter_mut().zip(q) {
        *out = x;
    }
    (qs, r)
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, as for
// `mod_div_rem_blocks`, without the remainder of the last block.
pub(crate) fn mod_quotient_blocks(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    m: &Natural,
) -> Vec<Natural> {
    if xs.len() <= (ys.len() << 1) - 2 {
        return mod_quotient_preinv(xs, ys, inverse, m);
    }
    let mut qs = vec![Natural::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys.len(), |xs| {
        mod_div_rem_preinv(xs, ys, inverse, m)
    });
    for (out, x) in qs
        .iter_mut()
        .zip(mod_quotient_preinv(&rs[..len], ys, inverse, m))
    {
        *out = x;
    }
    qs
}

// The data for dividing by `ys`, nonempty, modulo `m`, which fits in `T`, computed in `T`, or
// `None` if the leading coefficient of `ys` is not invertible.
fn word_inverse<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    ys: &[Natural],
    m: &Natural,
) -> Option<Vec<Natural>>
where
    Natural: From<T>,
{
    mod_polynomial_inverse_vec(&to_words::<T>(ys), T::exact_from(m))
        .map(|inverse| from_words(inverse).coefficients)
}

// The data for dividing by `q` modulo `m`. When `m` fits in one or two limbs, it is computed in
// word arithmetic.
//
// This is equivalent to preparing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that modulo 1 the data is empty.
pub(crate) fn precompute_helper(q: &NaturalPolynomial, m: &Natural) -> ModPolynomialData<Natural> {
    assert!(
        q.mod_is_reduced(m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
    if *m == 1u32 {
        return ModPolynomialData {
            inverse: Vec::new(),
        };
    }
    assert_ne!(*q, NaturalPolynomial::ZERO, "division by zero");
    let ys = &q.coefficients;
    let bits = m.significant_bits();
    let inverse = if bits <= Limb::WIDTH {
        word_inverse::<Limb>(ys, m)
    } else if bits <= DoubleLimb::WIDTH {
        word_inverse::<DoubleLimb>(ys, m)
    } else {
        ys.last()
            .unwrap()
            .mod_inverse(m)
            .map(|lead_inverse| reversed_inverse(ys, m, &lead_inverse))
    };
    ModPolynomialData {
        inverse: inverse
            .expect("the leading coefficient of the divisor must be invertible modulo m"),
    }
}

// What dividing one polynomial by another modulo `m` with precomputed data comes to, after the
// checks that every such division makes.
pub(crate) enum PrecomputedDivision<'a> {
    // Modulo 1, every polynomial is 0, and so are the quotient and remainder.
    Zero,
    // The dividend is shorter than the divisor, so the quotient is 0 and the remainder is the
    // dividend.
    Short,
    // The modulus fits in a limb, or in two: the coefficients of the dividend and the divisor, to
    // be divided in word arithmetic.
    Limb(&'a [Natural], &'a [Natural]),
    DoubleLimb(&'a [Natural], &'a [Natural]),
    // The divisor is a constant, whose inverse is given, so the remainder is 0.
    Constant(&'a Natural),
    // Otherwise: the coefficients of the dividend and the divisor.
    General(&'a [Natural], &'a [Natural]),
}

// Checks that `p` and `q` are reduced modulo `m` and that `q` is nonzero unless `m` is 1, and finds
// which case dividing `p` by `q` with `data` is.
pub(crate) fn precomputed_division<'a>(
    p: &'a NaturalPolynomial,
    q: &'a NaturalPolynomial,
    m: &Natural,
    data: &'a ModPolynomialData<Natural>,
) -> PrecomputedDivision<'a> {
    assert_reduced(p, q, m);
    if *m == 1u32 {
        return PrecomputedDivision::Zero;
    }
    assert_ne!(*q, NaturalPolynomial::ZERO, "division by zero");
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    let bits = m.significant_bits();
    if xs.len() < ys.len() {
        PrecomputedDivision::Short
    } else if bits <= Limb::WIDTH {
        PrecomputedDivision::Limb(xs, ys)
    } else if bits <= DoubleLimb::WIDTH {
        PrecomputedDivision::DoubleLimb(xs, ys)
    } else if ys.len() == 1 {
        PrecomputedDivision::Constant(&data.inverse[0])
    } else {
        PrecomputedDivision::General(xs, ys)
    }
}

// The product of the polynomial with coefficients `xs` and the constant `c` modulo `m`.
pub(crate) fn mod_mul_constant(xs: &[Natural], c: &Natural, m: &Natural) -> NaturalPolynomial {
    trimmed(xs.iter().map(|x| x.mod_mul(c, m)).collect())
}

// The quotient and remainder modulo `m`, which fits in `T`, of the polynomials with coefficients
// `xs` and `ys`, with `inverse` precomputed, computed in `T`.
pub(crate) fn div_rem_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    m: &Natural,
) -> (NaturalPolynomial, NaturalPolynomial)
where
    Natural: From<T>,
{
    let (qs, rs) = mod_div_rem_precomputed_vec(
        &to_words::<T>(xs),
        &to_words::<T>(ys),
        &to_words::<T>(inverse),
        T::exact_from(m),
    );
    (from_words(qs), from_words(rs))
}

// The quotient modulo `m`, which fits in `T`, of the polynomials with coefficients `xs` and `ys`,
// with `inverse` precomputed, computed in `T`.
pub(crate) fn quotient_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    inverse: &[Natural],
    m: &Natural,
) -> NaturalPolynomial
where
    Natural: From<T>,
{
    from_words(mod_quotient_precomputed_vec(
        &to_words::<T>(xs),
        &to_words::<T>(ys),
        &to_words::<T>(inverse),
        T::exact_from(m),
    ))
}

// The quotient and remainder of `p` by `q` modulo `m`, with `data` precomputed from `q` and `m`.
//
// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
// be divided.
pub(crate) fn div_rem_precomputed_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
    data: &ModPolynomialData<Natural>,
) -> (NaturalPolynomial, NaturalPolynomial) {
    match precomputed_division(p, q, m, data) {
        PrecomputedDivision::Zero => (NaturalPolynomial::ZERO, NaturalPolynomial::ZERO),
        PrecomputedDivision::Short => (NaturalPolynomial::ZERO, p.clone()),
        PrecomputedDivision::Limb(xs, ys) => div_rem_word::<Limb>(xs, ys, &data.inverse, m),
        PrecomputedDivision::DoubleLimb(xs, ys) => {
            div_rem_word::<DoubleLimb>(xs, ys, &data.inverse, m)
        }
        PrecomputedDivision::Constant(c) => (
            mod_mul_constant(&p.coefficients, c, m),
            NaturalPolynomial::ZERO,
        ),
        PrecomputedDivision::General(xs, ys) => {
            let (qs, rs) = mod_div_rem_blocks(xs, ys, &data.inverse, m);
            (trimmed(qs), trimmed(rs))
        }
    }
}

impl ModDivRemPrecomputed<Self, Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q.clone(), m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &Self, m: &Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by value, the divisor by
    /// value, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q.clone(), m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, &other, &m, data)
    }
}

impl ModDivRemPrecomputed<Self, &Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q.clone(), &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &Self, m: &&Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by value, the divisor by
    /// value, and the modulus by reference. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q.clone(), &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, &other, m, data)
    }
}

impl ModDivRemPrecomputed<&Self, Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &&Self, m: &Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by value, the divisor by
    /// reference, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, other, &m, data)
    }
}

impl ModDivRemPrecomputed<&Self, &Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &&Self, m: &&Natural) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by value, the divisor by
    /// reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, other, m, data)
    }
}

impl ModDivRemPrecomputed<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q.clone(), m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// value, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q.clone(), m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        div_rem_precomputed_helper(self, &other, &m, data)
    }
}

impl ModDivRemPrecomputed<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q.clone(), &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(
        other: &NaturalPolynomial,
        m: &&Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// value, and the modulus by reference. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q.clone(), &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        div_rem_precomputed_helper(self, &other, m, data)
    }
}

impl ModDivRemPrecomputed<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(
        other: &&NaturalPolynomial,
        m: &Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// reference, and the modulus by value. The coefficients of both must already be reduced modulo
    /// $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, m.clone(), &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: &NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        div_rem_precomputed_helper(self, other, &m, data)
    }
}

impl ModDivRemPrecomputed<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Precomputes the data for dividing [`NaturalPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `fmpz_mod_poly_reverse` and
    /// `fmpz_mod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(
        other: &&NaturalPolynomial,
        m: &&Natural,
    ) -> ModPolynomialData<Natural> {
        precompute_helper(other, m)
    }

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the dividend by reference, the divisor by
    /// reference, and the modulus by reference. The coefficients of both must already be reduced
    /// modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, &m, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        div_rem_precomputed_helper(self, other, m, data)
    }
}

impl ModDivAssignRemPrecomputed<Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, returning the remainder,
    /// using data precomputed from the divisor and $m$, taking the divisor by value and the modulus
    /// by value. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::{ModDivAssignRemPrecomputed, ModDivRemPrecomputed};
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(q.clone(), m.clone(), &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(q, m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_assign_rem_precomputed(
        &mut self,
        other: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), &other, &m, data);
        *self = q;
        r
    }
}

impl ModDivAssignRemPrecomputed<Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, returning the remainder,
    /// using data precomputed from the divisor and $m$, taking the divisor by value and the modulus
    /// by reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::{ModDivAssignRemPrecomputed, ModDivRemPrecomputed};
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(q.clone(), &m, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(q, &m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_assign_rem_precomputed(
        &mut self,
        other: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), &other, m, data);
        *self = q;
        r
    }
}

impl ModDivAssignRemPrecomputed<&Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, returning the remainder,
    /// using data precomputed from the divisor and $m$, taking the divisor by reference and the
    /// modulus by value. The coefficients of both must already be reduced modulo $m$, and the
    /// leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::{ModDivAssignRemPrecomputed, ModDivRemPrecomputed};
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(&q, m.clone(), &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(&q, m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_assign_rem_precomputed(
        &mut self,
        other: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), other, &m, data);
        *self = q;
        r
    }
}

impl ModDivAssignRemPrecomputed<&Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, returning the remainder,
    /// using data precomputed from the divisor and $m$, taking the divisor by reference and the
    /// modulus by reference. The coefficients of both must already be reduced modulo $m$, and the
    /// leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). When $m$
    /// fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise,
    /// with the inverse of the reversed divisor that the data holds, a dividend no more than twice
    /// as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top.
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
    /// use malachite_base::polynomial::{ModDivAssignRemPrecomputed, ModDivRemPrecomputed};
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(&q, &m, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(&q, &m, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem_newton_n_preinv` from
    /// `fmpz_mod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length
    /// may be divided.
    #[inline]
    fn mod_div_assign_rem_precomputed(
        &mut self,
        other: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), other, m, data);
        *self = q;
        r
    }
}
