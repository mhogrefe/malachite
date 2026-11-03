// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_div_rem::to_words;
use crate::natural_polynomial::modular::mod_evaluate::{horner_mod, precompute_mod_mul_data};
use crate::natural_polynomial::modular::subproduct_tree::{
    NaturalModulus, mod_evaluate_many_tree_preferred,
};
use crate::platform::{DoubleLimb, Limb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::ModIsReduced;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::ModEvaluateMany;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::evaluate_many_fast;

// The values modulo `m` of the polynomial with coefficients `coefficients` at the points `xs`,
// computed in `T`, which `m` must fit in, by the evaluation of `UnsignedPolynomial`s.
fn mod_evaluate_many_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    coefficients: &[Natural],
    xs: &[Natural],
    m: &Natural,
) -> Vec<Natural>
where
    Natural: From<T>,
{
    (&UnsignedPolynomial::from_coefficients_asc(to_words::<T>(coefficients)))
        .mod_evaluate_many(&to_words::<T>(xs), T::exact_from(m))
        .into_iter()
        .map(Natural::from)
        .collect()
}

fn mod_evaluate_many(p: &NaturalPolynomial, xs: &[Natural], m: &Natural) -> Vec<Natural> {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    for x in xs {
        assert!(*x < *m, "x must be reduced mod m, but {x} >= {m}");
    }
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        return mod_evaluate_many_word::<Limb>(&p.coefficients, xs, m);
    } else if bits <= DoubleLimb::WIDTH {
        return mod_evaluate_many_word::<DoubleLimb>(&p.coefficients, xs, m);
    }
    let Some((leading, rest)) = p.coefficients.split_last() else {
        return vec![Natural::ZERO; xs.len()];
    };
    if mod_evaluate_many_tree_preferred(p.coefficients.len(), xs.len()) {
        return evaluate_many_fast(&p.coefficients, xs, &NaturalModulus::new(m));
    }
    let data = precompute_mod_mul_data(m);
    xs.iter()
        .map(|x| {
            if rest.is_empty() || *x == 0u32 {
                p.coefficients[0].clone()
            } else {
                horner_mod(leading.clone(), rest, x, m, &data)
            }
        })
        .collect()
}

impl ModEvaluateMany<Natural, &Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at each of several [`Natural`]s, modulo a [`Natural`],
    /// taking the modulus by reference. The coefficients and the values must already be reduced
    /// modulo `m`.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`mod_evaluate`](malachite_base::polynomial::modular::traits::ModEvaluate::mod_evaluate) at
    /// each value, but the polynomial is checked once. When `m` fits in one or two limbs, the
    /// evaluation is done in words. Otherwise, Horner's rule is used at each value, with the data
    /// for modular multiplication by `m` computed once, or, for a long enough polynomial, the
    /// polynomial is reduced modulo the products of the linear factors $x - x_j$ in a subproduct
    /// tree, from the top down.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(\min(nkb \log b \log\log b, N \log^2 N \log\log N))$
    ///
    /// $M(n, k, b) = O((n + k \log k)b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `xs.len()`, $b$ is
    /// `m.significant_bits()`, and $N$ is $(n + k)b$.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or any value in `xs` is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateMany;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     (&p).mod_evaluate_many(&xs, &Natural::from(13u32)),
    ///     [7u32, 2, 7, 9].map(Natural::from)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz_vec` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that the values must be reduced,
    /// and that the choice between evaluating at each value and the subproduct tree depends on the
    /// number of values as well as on the polynomial's length.
    #[inline]
    fn mod_evaluate_many(self, xs: &[Natural], m: &Natural) -> Vec<Natural> {
        mod_evaluate_many(self, xs, m)
    }
}

impl ModEvaluateMany<Natural, Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at each of several [`Natural`]s, modulo a [`Natural`],
    /// taking the modulus by value. The coefficients and the values must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`mod_evaluate`](malachite_base::polynomial::modular::traits::ModEvaluate::mod_evaluate) at
    /// each value, but the polynomial is checked once. When `m` fits in one or two limbs, the
    /// evaluation is done in words. Otherwise, Horner's rule is used at each value, with the data
    /// for modular multiplication by `m` computed once, or, for a long enough polynomial, the
    /// polynomial is reduced modulo the products of the linear factors $x - x_j$ in a subproduct
    /// tree, from the top down.
    ///
    /// # Worst-case complexity
    /// $T(n, k, b) = O(\min(nkb \log b \log\log b, N \log^2 N \log\log N))$
    ///
    /// $M(n, k, b) = O((n + k \log k)b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `xs.len()`, $b$ is
    /// `m.significant_bits()`, and $N$ is $(n + k)b$.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or any value in `xs` is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateMany;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     (&p).mod_evaluate_many(&xs, Natural::from(13u32)),
    ///     [7u32, 2, 7, 9].map(Natural::from)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz_vec` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that the values must be reduced,
    /// and that the choice between evaluating at each value and the subproduct tree depends on the
    /// number of values as well as on the polynomial's length.
    #[inline]
    fn mod_evaluate_many(self, xs: &[Natural], m: Natural) -> Vec<Natural> {
        mod_evaluate_many(self, xs, &m)
    }
}
