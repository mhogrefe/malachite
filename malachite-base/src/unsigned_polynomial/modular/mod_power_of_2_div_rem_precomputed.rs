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

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::polynomial::modular::ModPolynomialData;
use crate::polynomial::modular::traits::{
    ModPowerOf2DivAssignRemPrecomputed, ModPowerOf2DivRemPrecomputed,
};
#[cfg(doc)]
use crate::polynomial::modular::traits::{
    ModPowerOf2QuotientPrecomputed, ModPowerOf2RemPrecomputed,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_div_rem_precomputed::{
    PrecomputedDivision, reduce_to_window,
};
use crate::unsigned_polynomial::modular::mod_power_of_2_div_rem::{
    assert_reduced, leading_inverse, reduced, remainder,
};
use crate::unsigned_polynomial::modular::mod_power_of_2_mul::mask_coefficients;
use crate::unsigned_polynomial::series::mod_power_of_2_inverse_truncated::*;
use crate::unsigned_polynomial::series::mod_power_of_2_mul_truncated::mul_truncated_wrapping;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::iter::once;
use core::mem::take;

// The most nonzero coefficients, the leading one included, that a divisor may have for
// `mod_power_of_2_div_rem_sparse` to divide by it.
//
// This is `MAX_NZ` from `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
const SPARSE_MAX_NONZERO: usize = 6;

// The length above which `mod_power_of_2_div_rem_preinv` tries `mod_power_of_2_div_rem_sparse`.
//
// This is the cutoff in `_nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0.
pub(crate) const SPARSE_MIN_LENGTH: usize = 20;

// The inverse modulo $2^k$, where $k$ is `pow`, which is positive, of the reversal of the
// polynomial with coefficients `ys`, as a power series truncated to `ys.len()` coefficients,
// reduced, where `ys` is nonempty and its last coefficient is odd, with inverse `lead_inverse`.
//
// This is equivalent to `nmod_poly_inv_series` applied to `nmod_poly_reverse` of the divisor, as in
// the preparation of `Binv` for `nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$.
fn reversed_inverse<T: PrimitiveUnsigned>(ys: &[T], pow: u64, lead_inverse: T) -> Vec<T> {
    let ys_reversed: Vec<T> = ys.iter().rev().copied().collect();
    let mut inverse =
        mod_power_of_2_inverse_truncated_dispatch(&ys_reversed, ys.len(), pow, lead_inverse);
    mask_coefficients(&mut inverse, pow);
    inverse
}

// The quotient of the polynomials with coefficients `xs` and `ys`, correct modulo $2^k$, where $k$
// is `pow`, but not reduced, where `ys` is no longer than `xs`, `xs` has at most `2 * ys.len() - 2`
// coefficients, and `inverse` is the inverse of the reversal of `ys` truncated to `ys.len()`
// coefficients. Reversed, the quotient is the product of the reversed top of `xs` and `inverse`,
// truncated.
//
// This is equivalent to `_nmod_poly_div_newton_n_preinv` from `nmod_poly/div_newton_n_preinv.c`,
// FLINT 3.6.0, with the modulus $2^k$, in wrapping arithmetic.
crate_test_fn! {mod_power_of_2_quotient_preinv<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    pow: u64,
) -> Vec<T> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_reversed: Vec<T> = xs[xs.len() - q_len..].iter().rev().copied().collect();
    let mut qs = vec![T::ZERO; q_len];
    mul_truncated_wrapping(&mut qs, &xs_reversed, &inverse[..min(q_len, inverse.len())], pow);
    qs.reverse();
    qs
}}

// The quotient and remainder of the polynomials with coefficients `xs` and `ys`, correct modulo
// $2^k$ but not reduced, where `ys` has at least 2 coefficients and `xs` has one more, so that the
// quotient has 2 coefficients; `lead_inverse` is an inverse modulo $2^k$ of the last coefficient of
// `ys`. Both quotient coefficients are found first, and then each remainder coefficient is a sum of
// a dividend coefficient and two products, in wrapping arithmetic.
//
// This is equivalent to `_nmod_poly_divrem_q1_preinv1` from `nmod_poly/divrem_basecase.c`, FLINT
// 3.6.0, with the modulus $2^k$, in wrapping arithmetic.
crate_test_fn! {mod_power_of_2_div_rem_basecase<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    lead_inverse: T,
) -> (Vec<T>, Vec<T>) {
    let n = ys.len() - 1;
    let q_1 = xs[n + 1].wrapping_mul(lead_inverse);
    // The negatives of the quotient's coefficients, so that the remainder is a sum.
    let neg_q_0 = q_1
        .wrapping_mul(ys[n - 1])
        .wrapping_sub(xs[n])
        .wrapping_mul(lead_inverse);
    let neg_q_1 = q_1.wrapping_neg();
    let zero = T::ZERO;
    let rs = xs[..n]
        .iter()
        .zip(once(&zero).chain(&ys[..n - 1]))
        .zip(&ys[..n])
        .map(|((&x, &y_1), &y_0)| {
            x.wrapping_add(neg_q_1.wrapping_mul(y_1))
                .wrapping_add(neg_q_0.wrapping_mul(y_0))
        })
        .collect();
    (vec![neg_q_0.wrapping_neg(), q_1], rs)
}}

// The quotient and remainder of the polynomials with coefficients `xs` and `ys`, correct modulo
// $2^k$ but not reduced, as for `mod_power_of_2_div_rem_basecase`, when `ys` has at most
// `SPARSE_MAX_NONZERO` nonzero coefficients, or `None` when it has more. Each step subtracts only
// the divisor's few nonzero terms, with the divisor made monic by scaling by `lead_inverse`.
//
// This is equivalent to `_nmod_poly_divrem_try_sparse` from `nmod_poly/divrem_newton_n_preinv.c`,
// FLINT 3.6.0, with the modulus $2^k$, in wrapping arithmetic, so that the special cases for the
// modulus 2 and for lower coefficients that are all 1 are not needed.
crate_test_fn! {mod_power_of_2_div_rem_sparse<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    lead_inverse: T,
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
            coefficients.push(y.wrapping_neg());
        }
    }
    let lead = ys[n];
    if lead != T::ONE {
        for c in &mut coefficients {
            *c = c.wrapping_mul(lead_inverse);
        }
    }
    let mut rs = xs.to_vec();
    let mut qs = vec![T::ZERO; xs.len() - n];
    for i in (n..xs.len()).rev() {
        let c = rs[i];
        qs[i - n] = c;
        for (&j, &k) in exponents.iter().zip(&coefficients).rev() {
            let r = &mut rs[j + i - n];
            *r = r.wrapping_add(c.wrapping_mul(k));
        }
    }
    if lead != T::ONE {
        for q in &mut qs {
            *q = q.wrapping_mul(lead_inverse);
        }
    }
    rs.truncate(n);
    Some((qs, rs))
}}

// The quotient and remainder of the polynomials with coefficients `xs` and `ys`, correct modulo
// $2^k$, where $k$ is `pow`, but not reduced, where `ys` has at least 2 coefficients and is no
// longer than `xs`, `xs` has at most `2 * ys.len() - 2` coefficients, and `inverse` is the inverse
// of the reversal of `ys` truncated to `ys.len()` coefficients. A quotient of 2 coefficients is
// found one term at a time, and a long, sparse divisor is divided by directly; otherwise the
// quotient is found with `inverse` and the remainder from it.
//
// This is equivalent to `_nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, in wrapping
// arithmetic.
crate_test_fn! {mod_power_of_2_div_rem_preinv<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    pow: u64,
) -> (Vec<T>, Vec<T>) {
    if xs.len() == ys.len() + 1 {
        return mod_power_of_2_div_rem_basecase(xs, ys, inverse[0]);
    }
    if ys.len() > SPARSE_MIN_LENGTH
        && let Some(result) = mod_power_of_2_div_rem_sparse(xs, ys, inverse[0])
    {
        return result;
    }
    let qs = mod_power_of_2_quotient_preinv(xs, ys, inverse, pow);
    let rs = remainder(xs, ys, &qs, pow);
    (qs, rs)
}}

// The quotient and remainder of the polynomials with coefficients `xs` and `ys`, correct modulo
// $2^k$, where $k$ is `pow`, but not reduced, where `ys` has at least 2 coefficients and is no
// longer than `xs`, and `inverse` is the inverse of the reversal of `ys` truncated to `ys.len()`
// coefficients. A dividend too long for `inverse` is first reduced a block at a time.
pub(crate) fn mod_power_of_2_div_rem_blocks<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    pow: u64,
) -> (Vec<T>, Vec<T>) {
    if xs.len() <= (ys.len() << 1) - 2 {
        return mod_power_of_2_div_rem_preinv(xs, ys, inverse, pow);
    }
    let mut qs = vec![T::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys.len(), |xs| {
        mod_power_of_2_div_rem_preinv(xs, ys, inverse, pow)
    });
    let (q, r) = mod_power_of_2_div_rem_preinv(&rs[..len], ys, inverse, pow);
    qs[..q.len()].copy_from_slice(&q);
    (qs, r)
}

// The quotient of the polynomials with coefficients `xs` and `ys`, correct modulo $2^k$ but not
// reduced, as for `mod_power_of_2_div_rem_blocks`, without the remainder of the last block.
pub(crate) fn mod_power_of_2_quotient_blocks<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    pow: u64,
) -> Vec<T> {
    if xs.len() <= (ys.len() << 1) - 2 {
        return mod_power_of_2_quotient_preinv(xs, ys, inverse, pow);
    }
    let mut qs = vec![T::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let len = reduce_to_window(&mut rs, &mut qs, ys.len(), |xs| {
        mod_power_of_2_div_rem_preinv(xs, ys, inverse, pow)
    });
    let q = mod_power_of_2_quotient_preinv(&rs[..len], ys, inverse, pow);
    qs[..q.len()].copy_from_slice(&q);
    qs
}

/// The inverse modulo $2^k$ of the reversal of the polynomial with coefficients `ys`, as a power
/// series truncated to `ys.len()` coefficients, or `None` if `ys` is empty or its last coefficient
/// is even. The coefficients must be reduced modulo $2^k$, where $k$ is `pow`, which must be
/// positive and no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a power of 2 with precomputed data.
#[doc(hidden)]
pub fn mod_power_of_2_polynomial_inverse_vec<T: PrimitiveUnsigned>(
    ys: &[T],
    pow: u64,
) -> Option<Vec<T>> {
    let lead_inverse = leading_inverse(ys, pow)?;
    Some(reversed_inverse(ys, pow, lead_inverse))
}

/// The coefficients of the quotient and remainder modulo $2^k$ of the polynomials with coefficients
/// `xs` and `ys`, without zeros at the end, where `inverse` is the result of
/// `mod_power_of_2_polynomial_inverse_vec` for `ys` and `pow`. The coefficients must be reduced
/// modulo $2^k$, where $k$ is `pow`, which must be positive and no greater than `T::WIDTH`, `ys`
/// must be nonempty and no longer than `xs`, and neither `xs` nor `ys` may end with a zero.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a power of 2 with precomputed data.
#[doc(hidden)]
pub fn mod_power_of_2_div_rem_precomputed_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    pow: u64,
) -> (Vec<T>, Vec<T>) {
    let (qs, rs) = if ys.len() == 1 {
        (
            xs.iter().map(|&x| x.wrapping_mul(inverse[0])).collect(),
            Vec::new(),
        )
    } else {
        mod_power_of_2_div_rem_blocks(xs, ys, inverse, pow)
    };
    (reduced(qs, pow).coefficients, reduced(rs, pow).coefficients)
}

/// The coefficients of the quotient modulo $2^k$ of the polynomials with coefficients `xs` and
/// `ys`, without zeros at the end, under the same conditions as
/// `mod_power_of_2_div_rem_precomputed_vec`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a power of 2 with precomputed data.
#[doc(hidden)]
pub fn mod_power_of_2_quotient_precomputed_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    inverse: &[T],
    pow: u64,
) -> Vec<T> {
    let qs = if ys.len() == 1 {
        xs.iter().map(|&x| x.wrapping_mul(inverse[0])).collect()
    } else {
        mod_power_of_2_quotient_blocks(xs, ys, inverse, pow)
    };
    reduced(qs, pow).coefficients
}

// The data for dividing by `q` modulo $2^k$, where $k$ is `pow`.
//
// This is equivalent to preparing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$.
pub(crate) fn precompute_helper<T: PrimitiveUnsigned>(
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> ModPolynomialData<T> {
    assert!(pow <= T::WIDTH);
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return ModPolynomialData {
            inverse: Vec::new(),
        };
    }
    assert_ne!(*q, UnsignedPolynomial::ZERO, "division by zero");
    let lead_inverse = leading_inverse(&q.coefficients, pow)
        .expect("the leading coefficient of the divisor must be odd");
    ModPolynomialData {
        inverse: reversed_inverse(&q.coefficients, pow, lead_inverse),
    }
}

// Checks that `p` and `q` are reduced modulo $2^k$, where $k$ is `pow`, and that `q` is nonzero
// unless $k$ is 0, and finds which case dividing `p` by `q` with `data` is.
pub(crate) fn precomputed_division<'a, T: PrimitiveUnsigned>(
    p: &'a UnsignedPolynomial<T>,
    q: &'a UnsignedPolynomial<T>,
    pow: u64,
    data: &ModPolynomialData<T>,
) -> PrecomputedDivision<'a, T> {
    assert!(pow <= T::WIDTH);
    assert_reduced(p, q, pow);
    if pow == 0 {
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

// The product of `p` and the constant `c` modulo $2^k$, where $k$ is `pow`.
pub(crate) fn mod_power_of_2_mul_constant<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    pow: u64,
) -> UnsignedPolynomial<T> {
    reduced(
        p.coefficients.iter().map(|&x| x.wrapping_mul(c)).collect(),
        pow,
    )
}

// The quotient and remainder of `p` by `q` modulo $2^k$, where $k$ is `pow`, with `data`
// precomputed from `q` and $k$.
//
// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a dividend
// of any length may be divided.
pub(crate) fn div_rem_precomputed_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
    data: &ModPolynomialData<T>,
) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
    match precomputed_division(p, q, pow, data) {
        PrecomputedDivision::Zero => (UnsignedPolynomial::ZERO, UnsignedPolynomial::ZERO),
        PrecomputedDivision::Short => (UnsignedPolynomial::ZERO, p.clone()),
        PrecomputedDivision::Constant(c) => (
            mod_power_of_2_mul_constant(p, c, pow),
            UnsignedPolynomial::ZERO,
        ),
        PrecomputedDivision::General(xs, ys) => {
            let (qs, rs) = mod_power_of_2_div_rem_blocks(xs, ys, &data.inverse, pow);
            (reduced(qs, pow), reduced(rs, pow))
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRemPrecomputed<Self> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModPowerOf2DivRemPrecomputed`],
    /// [`ModPowerOf2QuotientPrecomputed`], and [`ModPowerOf2RemPrecomputed`] alike.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(other: &Self, pow: u64) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// using data precomputed from the divisor and $k$, taking both by value. The coefficients of
    /// both must already be reduced modulo $2^k$, and the leading coefficient of the divisor must
    /// be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). It holds the inverse
    /// of the reversed divisor as a power series, with which a dividend no more than twice as long
    /// as the divisor takes one truncated product for the quotient and one for the remainder, and a
    /// longer one is divided that many coefficients at a time, from the top. A long divisor with
    /// few nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// results are reduced once at the end.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, &other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRemPrecomputed<&Self> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModPowerOf2DivRemPrecomputed`],
    /// [`ModPowerOf2QuotientPrecomputed`], and [`ModPowerOf2RemPrecomputed`] alike.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(other: &&Self, pow: u64) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// using data precomputed from the divisor and $k$, taking the first by value and the second by
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
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). It holds the inverse
    /// of the reversed divisor as a power series, with which a dividend no more than twice as long
    /// as the divisor takes one truncated product for the quotient and one for the remainder, and a
    /// longer one is divided that many coefficients at a time, from the top. A long divisor with
    /// few nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// results are reduced once at the end.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> (Self, Self) {
        div_rem_precomputed_helper(&self, other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRemPrecomputed<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModPowerOf2DivRemPrecomputed`],
    /// [`ModPowerOf2QuotientPrecomputed`], and [`ModPowerOf2RemPrecomputed`] alike.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// using data precomputed from the divisor and $k$, taking the first by reference and the
    /// second by value. The coefficients of both must already be reduced modulo $2^k$, and the
    /// leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). It holds the inverse
    /// of the reversed divisor as a power series, with which a dividend no more than twice as long
    /// as the divisor takes one truncated product for the quotient and one for the remainder, and a
    /// longer one is divided that many coefficients at a time, from the top. A long divisor with
    /// few nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// results are reduced once at the end.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        div_rem_precomputed_helper(self, &other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRemPrecomputed<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Precomputes the data for dividing [`UnsignedPolynomial`]s by `other` modulo $2^k$ with
    /// [`mod_power_of_2_div_rem_precomputed`](
    /// ModPowerOf2DivRemPrecomputed::mod_power_of_2_div_rem_precomputed) or
    /// [`mod_power_of_2_div_assign_rem_precomputed`](
    /// ModPowerOf2DivAssignRemPrecomputed::mod_power_of_2_div_assign_rem_precomputed). The
    /// coefficients of `other` must already be reduced modulo $2^k$, and its leading coefficient
    /// must be odd.
    ///
    /// The data is the inverse modulo $2^k$ of the reversal of `other`, as a power series truncated
    /// to as many terms as `other` has. The same data serves [`ModPowerOf2DivRemPrecomputed`],
    /// [`ModPowerOf2QuotientPrecomputed`], and [`ModPowerOf2RemPrecomputed`] alike.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to computing `Binv` for `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, with
    /// `nmod_poly_reverse` and `nmod_poly_inv_series`.
    #[inline]
    fn precompute_mod_power_of_2_div_rem_data(
        other: &&UnsignedPolynomial<T>,
        pow: u64,
    ) -> ModPolynomialData<T> {
        precompute_helper(other, pow)
    }

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// using data precomputed from the divisor and $k$, taking both by reference. The coefficients
    /// of both must already be reduced modulo $2^k$, and the leading coefficient of the divisor
    /// must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). It holds the inverse
    /// of the reversed divisor as a power series, with which a dividend no more than twice as long
    /// as the divisor takes one truncated product for the quotient and one for the remainder, and a
    /// longer one is divided that many coefficients at a time, from the top. A long divisor with
    /// few nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// results are reduced once at the end.
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
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem_precomputed(&q, 3, &data);
    /// assert_eq!(s.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        div_rem_precomputed_helper(self, other, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivAssignRemPrecomputed<Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, returning the remainder,
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
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). It holds the inverse
    /// of the reversed divisor as a power series, with which a dividend no more than twice as long
    /// as the divisor takes one truncated product for the quotient and one for the remainder, and a
    /// longer one is divided that many coefficients at a time, from the top. A long divisor with
    /// few nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// results are reduced once at the end.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(q.clone(), 3, &data);
    /// assert_eq!(p.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_assign_rem_precomputed(
        &mut self,
        other: Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), &other, pow, data);
        *self = q;
        r
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivAssignRemPrecomputed<&Self> for UnsignedPolynomial<T> {
    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, returning the remainder,
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
    /// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data). It holds the inverse
    /// of the reversed divisor as a power series, with which a dividend no more than twice as long
    /// as the divisor takes one truncated product for the quotient and one for the remainder, and a
    /// longer one is divided that many coefficients at a time, from the top. A long divisor with
    /// few nonzero coefficients is divided by directly, term by term. The arithmetic wraps, and the
    /// results are reduced once at the end.
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
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(&q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x^3+7*x+3");
    /// assert_eq!(r.to_string(), "6");
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem_precomputed(&q, 3, &data);
    /// assert_eq!(p.to_string(), "3*x+7");
    /// assert_eq!(r.to_string(), "2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem_newton_n_preinv` from
    /// `nmod_poly/divrem_newton_n_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except that a
    /// dividend of any length may be divided.
    #[inline]
    fn mod_power_of_2_div_assign_rem_precomputed(
        &mut self,
        other: &Self,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Self {
        let (q, r) = div_rem_precomputed_helper(&take(self), other, pow, data);
        *self = q;
        r
    }
}
