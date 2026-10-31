// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011, 2013 William Hart
//
//      Copyright © 2013 Martin Lee
//
//      Copyright © 2024 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModDivAssignRemPrecomputed, ModDivRemPrecomputed, ModPolynomialData};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem::{
    assert_reduced, leading_inverse, remainder,
};
use crate::unsigned_polynomial::arithmetic::mod_inverse_truncated::mod_inverse_truncated_dispatch;
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_vector::arithmetic::mod_dot::ModData;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::iter::once;
use core::mem::take;

// The most nonzero coefficients, the leading one included, that a divisor may have for
// `mod_div_rem_sparse` to divide by it.
//
// This is `MAX_NZ` from `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
const SPARSE_MAX_NONZERO: usize = 6;

// The length above which `mod_div_rem_preinv` tries `mod_div_rem_sparse`.
//
// This is the cutoff in `_nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
const SPARSE_MIN_LENGTH: usize = 20;

// The inverse modulo `m` of the reversal of the polynomial with coefficients `ys`, as a power
// series truncated to `ys.len()` coefficients, where `ys` is nonempty and its last coefficient is
// invertible modulo `m`, whose inverse is `lead_inverse`.
//
// This is equivalent to `nmod_poly_inv_series` applied to `nmod_poly_reverse` of the divisor, as in
// the preparation of `Binv` for `nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
pub(crate) fn reversed_inverse<T: PrimitiveUnsigned>(ys: &[T], m: T, lead_inverse: T) -> Vec<T> {
    let ys_reversed: Vec<T> = ys.iter().rev().copied().collect();
    mod_inverse_truncated_dispatch(&ys_reversed, ys.len(), m, lead_inverse)
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, where `ys` is no
// longer than `xs`, `xs` has at most `2 * ys.len() - 2` coefficients, and `inverse` is the inverse
// of the reversal of `ys` truncated to `ys.len()` coefficients. Reversed, the quotient is the
// product of the reversed top of `xs` and `inverse`, truncated.
//
// This is equivalent to `_nmod_poly_div_newton_n_preinv` from `nmod_poly/div_newton_n_preinv.c`,
// FLINT 3.6.0.
crate_test_fn! {mod_quotient_preinv<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    m: T,
) -> Vec<T> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_reversed: Vec<T> = xs[xs.len() - q_len..].iter().rev().copied().collect();
    let mut qs = vec![T::ZERO; q_len];
    mod_mul_truncated_to_out(&mut qs, &xs_reversed, &inverse[..min(q_len, inverse.len())], m);
    qs.reverse();
    qs
}}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`, where
// `ys` has at least 2 coefficients and `xs` has one more, so that the quotient has 2 coefficients;
// `lead_inverse` is the inverse of the last coefficient of `ys`. Both quotient coefficients are
// found first, and then each remainder coefficient is a sum of a dividend coefficient and two
// products, accumulated exactly and reduced once.
//
// This is equivalent to `_nmod_poly_divrem_q1_preinv1` from `nmod_poly/divrem_basecase.c`, FLINT
// 3.6.0, except that the sums are accumulated in as many words as their bound needs, as in the
// `nmod_vec` dot products, rather than with variants for each size of modulus.
crate_test_fn! {mod_div_rem_basecase<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    lead_inverse: T,
    m: T,
) -> (Vec<T>, Vec<T>) {
    let n = ys.len() - 1;
    let q_1 = xs[n + 1].mod_mul(lead_inverse, m);
    // The negatives of the quotient's coefficients, so that the remainder is a sum.
    let neg_q_0 = q_1.mod_mul(ys[n - 1], m).mod_sub(xs[n], m).mod_mul(lead_inverse, m);
    let neg_q_1 = q_1.mod_neg(m);
    let d = ModData::new(m, 3);
    // Pairs of divisor coefficients that multiply the two quotient coefficients in each remainder
    // coefficient; below the divisor's constant term, the coefficient is 0.
    let zero = T::ZERO;
    let terms = xs[..n]
        .iter()
        .zip(once(&zero).chain(&ys[..n - 1]))
        .zip(&ys[..n]);
    let rs = match d.words {
        1 => terms
            .map(|((&x, &y_1), &y_0)| {
                x.wrapping_add(neg_q_1.wrapping_mul(y_1))
                    .wrapping_add(neg_q_0.wrapping_mul(y_0))
                    % m
            })
            .collect(),
        2 => terms
            .map(|((&x, &y_1), &y_0)| d.reduce_2_sum(x, neg_q_1, y_1, neg_q_0, y_0))
            .collect(),
        // When three words might be needed, the sum still fits in two if `1 + neg_q_0 + neg_q_1`
        // does: each of the three terms is less than `m` times its multiplier.
        _ if neg_q_0
            .checked_add(neg_q_1)
            .and_then(|s| s.checked_add(T::ONE))
            .is_some() =>
        {
            terms
                .map(|((&x, &y_1), &y_0)| d.reduce_2_sum(x, neg_q_1, y_1, neg_q_0, y_0))
                .collect()
        }
        _ => terms
            .map(|((&x, &y_1), &y_0)| {
                let r = d.reduce_2_sum(x, neg_q_1, y_1, T::ZERO, T::ZERO);
                d.reduce_2_sum(r, neg_q_0, y_0, T::ZERO, T::ZERO)
            })
            .collect(),
    };
    (vec![neg_q_0.mod_neg(m), q_1], rs)
}}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`, as for
// `mod_div_rem_basecase`, when `ys` has at most `SPARSE_MAX_NONZERO` nonzero coefficients, or
// `None` when it has more. Each step subtracts only the divisor's few nonzero terms, with the
// divisor made monic by scaling by `lead_inverse`; modulo 2, subtraction is exclusive or, and when
// every lower coefficient of the monic divisor is 1, the terms need no multiplication.
//
// This is equivalent to `_nmod_poly_divrem_try_sparse` from `nmod_poly/divrem_newton_n_preinv.c`,
// FLINT 3.6.0, except that small moduli use the same reduction as large ones rather than Barrett
// reduction.
crate_test_fn! {mod_div_rem_sparse<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    lead_inverse: T,
    m: T,
) -> Option<(Vec<T>, Vec<T>)> {
    let n = ys.len() - 1;
    let mut exponents = Vec::with_capacity(SPARSE_MAX_NONZERO);
    let mut coefficients = Vec::with_capacity(SPARSE_MAX_NONZERO);
    for (i, &y) in ys[..n].iter().enumerate() {
        if y != T::ZERO {
            if exponents.len() + 2 > SPARSE_MAX_NONZERO {
                return None;
            }
            exponents.push(i);
            coefficients.push(m - y);
        }
    }
    let lead = ys[n];
    if lead != T::ONE {
        for c in &mut coefficients {
            *c = c.mod_mul(lead_inverse, m);
        }
    }
    let all_ones = !coefficients.is_empty() && coefficients.iter().all(|&c| m - c == T::ONE);
    let mut rs = xs.to_vec();
    let mut qs = vec![T::ZERO; xs.len() - n];
    if m == T::TWO {
        for i in (n..xs.len()).rev() {
            let c = rs[i];
            qs[i - n] = c;
            for &j in exponents.iter().rev() {
                rs[j + i - n] ^= c;
            }
        }
    } else if all_ones {
        for i in (n..xs.len()).rev() {
            let c = rs[i];
            qs[i - n] = c;
            for &j in exponents.iter().rev() {
                rs[j + i - n].mod_sub_assign(c, m);
            }
        }
    } else {
        for i in (n..xs.len()).rev() {
            let c = rs[i];
            qs[i - n] = c;
            for (&j, &k) in exponents.iter().zip(&coefficients).rev() {
                let r = &mut rs[j + i - n];
                *r = r.mod_add(c.mod_mul(k, m), m);
            }
        }
    }
    if lead != T::ONE {
        for q in &mut qs {
            *q = q.mod_mul(lead_inverse, m);
        }
    }
    rs.truncate(n);
    Some((qs, rs))
}}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`, where
// `ys` has at least 2 coefficients and is no longer than `xs`, `xs` has at most `2 * ys.len() - 2`
// coefficients, and `inverse` is the inverse of the reversal of `ys` truncated to `ys.len()`
// coefficients. A quotient of 2 coefficients is found one term at a time, and a long, sparse
// divisor is divided by directly; otherwise the quotient is found with `inverse` and the remainder
// from it.
//
// This is equivalent to `_nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
crate_test_fn! {mod_div_rem_preinv<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    m: T,
) -> (Vec<T>, Vec<T>) {
    if xs.len() == ys.len() + 1 {
        return mod_div_rem_basecase(xs, ys, inverse[0], m);
    }
    if ys.len() > SPARSE_MIN_LENGTH
        && let Some(result) = mod_div_rem_sparse(xs, ys, inverse[0], m)
    {
        return result;
    }
    let qs = mod_quotient_preinv(xs, ys, inverse, m);
    let rs = remainder(xs, ys, &qs, m);
    (qs, rs)
}}

// Divides the polynomial with coefficients `rs` by the one with coefficients `ys` modulo `m` until
// it has at most `2 * ys.len() - 2` coefficients, the most that `inverse` can divide at once,
// returning the number left; `ys` has at least 2 coefficients and is no longer than `rs`, and
// `inverse` is the inverse of the reversal of `ys` truncated to `ys.len()` coefficients. The
// dividend is divided that many coefficients at a time, from the top, each remainder taking the
// place of the coefficients it came from and each quotient written to the matching part of `qs`. At
// least `ys.len()` coefficients are left.
fn reduce_to_window<T: PrimitiveUnsigned>(
    rs: &mut [T],
    qs: &mut [T],
    ys: &[T],
    inverse: &[T],
    m: T,
) -> usize {
    let y_len = ys.len();
    let window = (y_len << 1) - 2;
    let mut len = rs.len();
    while len > window {
        let start = len - window;
        let (q, r) = mod_div_rem_preinv(&rs[start..len], ys, inverse, m);
        qs[start..start + q.len()].copy_from_slice(&q);
        rs[start..start + r.len()].copy_from_slice(&r);
        len = start + y_len - 1;
    }
    len
}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`, where
// `ys` has at least 2 coefficients and is no longer than `xs`, and `inverse` is the inverse of the
// reversal of `ys` truncated to `ys.len()` coefficients. A dividend too long for `inverse` is first
// reduced a block at a time.
pub(crate) fn mod_div_rem_blocks<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    m: T,
) -> (Vec<T>, Vec<T>) {
    let mut qs = vec![T::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys, inverse, m);
    let (q, r) = mod_div_rem_preinv(&rs[..len], ys, inverse, m);
    qs[..q.len()].copy_from_slice(&q);
    (qs, r)
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, as for
// `mod_div_rem_blocks`, without the remainder of the last block.
pub(crate) fn mod_quotient_blocks<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    m: T,
) -> Vec<T> {
    if xs.len() <= (ys.len() << 1) - 2 {
        return mod_quotient_preinv(xs, ys, inverse, m);
    }
    let mut qs = vec![T::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys, inverse, m);
    let q = mod_quotient_preinv(&rs[..len], ys, inverse, m);
    qs[..q.len()].copy_from_slice(&q);
    qs
}

// The data for dividing by `q` modulo `m`.
//
// This is equivalent to preparing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that modulo 1 the data is empty.
pub(crate) fn precompute_helper<T: PrimitiveUnsigned>(
    q: &UnsignedPolynomial<T>,
    m: T,
) -> ModPolynomialData<T> {
    assert!(
        q.mod_is_reduced(&m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
    if m == T::ONE {
        return ModPolynomialData {
            inverse: Vec::new(),
        };
    }
    assert_ne!(*q, UnsignedPolynomial::ZERO, "division by zero");
    let lead_inverse = leading_inverse(&q.coefficients, m)
        .expect("the leading coefficient of the divisor must be invertible modulo m");
    ModPolynomialData {
        inverse: reversed_inverse(&q.coefficients, m, lead_inverse),
    }
}

// What dividing one polynomial by another modulo `m` with precomputed data comes to, after the
// checks that every such division makes.
pub(crate) enum PrecomputedDivision<'a, T> {
    // Modulo 1, every polynomial is 0, and so are the quotient and remainder.
    Zero,
    // The dividend is shorter than the divisor, so the quotient is 0 and the remainder is the
    // dividend.
    Short,
    // The divisor is a constant, whose inverse is given, so the remainder is 0.
    Constant(T),
    // Otherwise: the coefficients of the dividend and the divisor.
    General(&'a [T], &'a [T]),
}

// Checks that `p` and `q` are reduced modulo `m` and that `q` is nonzero unless `m` is 1, and finds
// which case dividing `p` by `q` with `data` is.
pub(crate) fn precomputed_division<'a, T: PrimitiveUnsigned>(
    p: &'a UnsignedPolynomial<T>,
    q: &'a UnsignedPolynomial<T>,
    m: T,
    data: &ModPolynomialData<T>,
) -> PrecomputedDivision<'a, T> {
    assert_reduced(p, q, m);
    if m == T::ONE {
        return PrecomputedDivision::Zero;
    }
    assert_ne!(*q, UnsignedPolynomial::ZERO, "division by zero");
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if xs.len() < ys.len() {
        PrecomputedDivision::Short
    } else if ys.len() == 1 {
        PrecomputedDivision::Constant(data.inverse[0])
    } else {
        PrecomputedDivision::General(xs, ys)
    }
}

// The product of `p` and the constant `c` modulo `m`.
pub(crate) fn mod_mul_constant<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    m: T,
) -> UnsignedPolynomial<T> {
    from_coefficients_trimmed(p.coefficients.iter().map(|&x| x.mod_mul(c, m)).collect())
}

// The quotient and remainder of `p` by `q` modulo `m`, with `data` precomputed from `q` and `m`.
//
// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may be
// divided.
pub(crate) fn div_rem_precomputed_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
    data: &ModPolynomialData<T>,
) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
    match precomputed_division(p, q, m, data) {
        PrecomputedDivision::Zero => (UnsignedPolynomial::ZERO, UnsignedPolynomial::ZERO),
        PrecomputedDivision::Short => (UnsignedPolynomial::ZERO, p.clone()),
        PrecomputedDivision::Constant(c) => (mod_mul_constant(p, c, m), UnsignedPolynomial::ZERO),
        PrecomputedDivision::General(xs, ys) => {
            let (qs, rs) = mod_div_rem_blocks(xs, ys, &data.inverse, m);
            (from_coefficients_trimmed(qs), from_coefficients_trimmed(rs))
        }
    }
}

impl<T: PrimitiveUnsigned> ModDivRemPrecomputed<Self, T> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModDivRemPrecomputed`],
    /// [`ModQuotientPrecomputed`](crate::polynomial::ModQuotientPrecomputed), and
    /// [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed) alike.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q.clone(), 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &Self, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking both by value. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). It holds
    /// the inverse of the reversed divisor as a power series, with which a dividend no more than
    /// twice as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top. A
    /// long divisor with few nonzero coefficients is divided by directly, term by term.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q.clone(), 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, &other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModDivRemPrecomputed<&Self, T> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModDivRemPrecomputed`],
    /// [`ModQuotientPrecomputed`](crate::polynomial::ModQuotientPrecomputed), and
    /// [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed) alike.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &&Self, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the first by value and the second by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). It holds
    /// the inverse of the reversed divisor as a power series, with which a dividend no more than
    /// twice as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top. A
    /// long divisor with few nonzero coefficients is divided by directly, term by term.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: &Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModDivRemPrecomputed<UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModDivRemPrecomputed`],
    /// [`ModQuotientPrecomputed`](crate::polynomial::ModQuotientPrecomputed), and
    /// [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed) alike.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q.clone(), 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &UnsignedPolynomial<T>, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking the first by reference and the second by
    /// value. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). It holds
    /// the inverse of the reversed divisor as a power series, with which a dividend no more than
    /// twice as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top. A
    /// long divisor with few nonzero coefficients is divided by directly, term by term.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q.clone(), 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        div_rem_precomputed_helper(self, &other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModDivRemPrecomputed<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $m$ with
    /// [`mod_div_rem_precomputed`](ModDivRemPrecomputed::mod_div_rem_precomputed) or
    /// [`mod_div_assign_rem_precomputed`](
    /// ModDivAssignRemPrecomputed::mod_div_assign_rem_precomputed). The coefficients of `other`
    /// must already be reduced modulo $m$, and its leading coefficient must be invertible modulo
    /// $m$.
    ///
    /// The data is the inverse modulo $m$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModDivRemPrecomputed`],
    /// [`ModQuotientPrecomputed`](crate::polynomial::ModQuotientPrecomputed), and
    /// [`ModRemPrecomputed`](crate::polynomial::ModRemPrecomputed) alike.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with `nmod_poly_reverse` and
    /// `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_div_rem_data(other: &&UnsignedPolynomial<T>, m: &T) -> ModPolynomialData<T> {
        precompute_helper(other, *m)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $m$, taking both by reference. The coefficients of
    /// both must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). It holds
    /// the inverse of the reversed divisor as a power series, with which a dividend no more than
    /// twice as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top. A
    /// long divisor with few nonzero coefficients is divided by directly, term by term.
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
    /// use malachite_base::polynomial::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem_precomputed(&q, 7, &data);
    /// assert_eq!(s.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_div_rem_precomputed(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        div_rem_precomputed_helper(self, other, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModDivAssignRemPrecomputed<Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, returning the remainder,
    /// using data precomputed from the divisor and $m$, taking the divisor by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). It holds
    /// the inverse of the reversed divisor as a power series, with which a dividend no more than
    /// twice as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top. A
    /// long divisor with few nonzero coefficients is divided by directly, term by term.
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
    /// use malachite_base::polynomial::{ModDivAssignRemPrecomputed, ModDivRemPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(q.clone(), 7, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(q, 7, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_div_assign_rem_precomputed(
        &mut self,
        other: Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), &other, m, data);
        *self = q;
        r
    }
}

impl<T: PrimitiveUnsigned> ModDivAssignRemPrecomputed<&Self, T> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, returning the remainder,
    /// using data precomputed from the divisor and $m$, taking the divisor by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data). It holds
    /// the inverse of the reversed divisor as a power series, with which a dividend no more than
    /// twice as long as the divisor takes one truncated product for the quotient and one for the
    /// remainder, and a longer one is divided that many coefficients at a time, from the top. A
    /// long divisor with few nonzero coefficients is divided by directly, term by term.
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
    /// use malachite_base::polynomial::{ModDivAssignRemPrecomputed, ModDivRemPrecomputed};
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&q, &7);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(&q, 7, &data);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_div_assign_rem_precomputed(&q, 7, &data);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// assert_eq!(r.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, except that a dividend of any length may
    /// be divided.
    #[inline]
    fn mod_div_assign_rem_precomputed(
        &mut self,
        other: &Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), other, m, data);
        *self = q;
        r
    }
}
