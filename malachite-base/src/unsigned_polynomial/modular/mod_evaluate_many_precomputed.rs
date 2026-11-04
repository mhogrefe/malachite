// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModIsReduced;
use crate::polynomial::modular::ModEvaluateManyData;
use crate::polynomial::modular::traits::ModEvaluateManyPrecomputed;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::subproduct_tree::{
    Modulus, evaluate_many_fast_precomputed, subproduct_tree, subproduct_tree_inverses,
};
use alloc::vec;
use alloc::vec::Vec;

fn precompute<T: PrimitiveUnsigned>(xs: &[T], m: T) -> ModEvaluateManyData<T, T> {
    assert_ne!(m, T::ZERO);
    for &x in xs {
        assert!(x < m, "every value must be reduced mod m, but {x} >= {m}");
    }
    let tree = subproduct_tree(xs, &Modulus(m));
    // Modulo 1 every polynomial is 0, and no remainders are taken.
    let inverses = if m == T::ONE {
        Vec::new()
    } else {
        subproduct_tree_inverses(&tree, xs.len(), &Modulus(m))
    };
    ModEvaluateManyData {
        tree,
        inverses,
        modulus: m,
    }
}

fn evaluate_precomputed<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    m: T,
    data: &ModEvaluateManyData<T, T>,
) -> Vec<T> {
    assert_eq!(
        m, data.modulus,
        "the data was precomputed for a different modulus"
    );
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    if m == T::ONE {
        return vec![T::ZERO; data.len()];
    }
    evaluate_many_fast_precomputed(
        &p.coefficients,
        &data.tree,
        &data.inverses,
        data.len(),
        &Modulus(m),
    )
}

impl<T: PrimitiveUnsigned> ModEvaluateManyPrecomputed<T> for UnsignedPolynomial<T> {
    type Output = T;
    type Data = ModEvaluateManyData<T, T>;

    /// Precomputes the data for evaluating [`UnsignedPolynomial`]s at each value in `xs` modulo $m$
    /// with [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed). The values must already be
    /// reduced modulo $m$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree. Alongside each product is
    /// the inverse of its reversal as a power series, with which a remainder by the product takes
    /// one truncated multiplication. The same data serves every polynomial and both ways of calling
    /// [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log^2 k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `xs.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any value in `xs` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data =
    ///     UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[0, 1, 2, 3, 4, 5], &13);
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, 61, 99, and 147, mod 13
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(13, &data),
    ///     &[7, 2, 7, 9, 8, 4]
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+1").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(13, &data),
    ///     &[1, 2, 9, 2, 0, 9]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_tree_alloc` and `_nmod_poly_tree_build` from
    /// `nmod_poly/tree.c`, FLINT 3.6.0, except that the inverses of the reversals of the products
    /// are also computed.
    #[inline]
    fn precompute_mod_evaluate_many_data(xs: &[T], m: &T) -> ModEvaluateManyData<T, T> {
        precompute(xs, *m)
    }

    /// Evaluates an [`UnsignedPolynomial`] modulo $m$ at each value the data was precomputed from,
    /// taking the polynomial by value, and returns the values in the same order. The coefficients
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $n$ is its length, and the $x_j$ are the $k$
    /// values the data was precomputed from.
    ///
    /// The data should be obtained with [`precompute_mod_evaluate_many_data`](
    /// ModEvaluateManyPrecomputed::precompute_mod_evaluate_many_data). The polynomial is reduced
    /// modulo the products of the tree's top level, and each remainder is reduced modulo the two
    /// products below it, down to the linear factors, where the remainders are the values.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n \log n + k \log^2 k)$
    ///
    /// $M(n, k) = O(n + k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is the number of
    /// values the data was precomputed from.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to `m`, or if `m` is not the
    /// modulus the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data =
    ///     UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[0, 1, 2, 3, 4, 5], &13);
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, 61, 99, and 147, mod 13
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(13, &data),
    ///     &[7, 2, 7, 9, 8, 4]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_evaluate_nmod_vec_fast_precomp` from
    /// `nmod_poly/evaluate_nmod_vec.c`, FLINT 3.6.0, except that each remainder by a product whose
    /// dividend is at most twice as long uses the product's precomputed inverse.
    #[inline]
    fn mod_evaluate_many_precomputed(self, m: T, data: &ModEvaluateManyData<T, T>) -> Vec<T> {
        evaluate_precomputed(&self, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModEvaluateManyPrecomputed<T> for &UnsignedPolynomial<T> {
    type Output = T;
    type Data = ModEvaluateManyData<T, T>;

    /// Precomputes the data for evaluating [`UnsignedPolynomial`]s at each value in `xs` modulo $m$
    /// with [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed). The values must already be
    /// reduced modulo $m$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree. Alongside each product is
    /// the inverse of its reversal as a power series, with which a remainder by the product takes
    /// one truncated multiplication. The same data serves every polynomial and both ways of calling
    /// [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(k) = O(k \log^2 k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $k$ is `xs.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any value in `xs` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data =
    ///     UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[0, 1, 2, 3, 4, 5], &13);
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, 61, 99, and 147, mod 13
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(13, &data),
    ///     &[7, 2, 7, 9, 8, 4]
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+1").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(13, &data),
    ///     &[1, 2, 9, 2, 0, 9]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_tree_alloc` and `_nmod_poly_tree_build` from
    /// `nmod_poly/tree.c`, FLINT 3.6.0, except that the inverses of the reversals of the products
    /// are also computed.
    #[inline]
    fn precompute_mod_evaluate_many_data(xs: &[T], m: &T) -> ModEvaluateManyData<T, T> {
        precompute(xs, *m)
    }

    /// Evaluates an [`UnsignedPolynomial`] modulo $m$ at each value the data was precomputed from,
    /// taking the polynomial by reference, and returns the values in the same order. The
    /// coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $n$ is its length, and the $x_j$ are the $k$
    /// values the data was precomputed from.
    ///
    /// The data should be obtained with [`precompute_mod_evaluate_many_data`](
    /// ModEvaluateManyPrecomputed::precompute_mod_evaluate_many_data). The polynomial is reduced
    /// modulo the products of the tree's top level, and each remainder is reduced modulo the two
    /// products below it, down to the linear factors, where the remainders are the values.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n \log n + k \log^2 k)$
    ///
    /// $M(n, k) = O(n + k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is the number of
    /// values the data was precomputed from.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to `m`, or if `m` is not the
    /// modulus the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data =
    ///     UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[0, 1, 2, 3, 4, 5], &13);
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, 61, 99, and 147, mod 13
    /// assert_eq!(
    ///     (&p).mod_evaluate_many_precomputed(13, &data),
    ///     &[7, 2, 7, 9, 8, 4]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_evaluate_nmod_vec_fast_precomp` from
    /// `nmod_poly/evaluate_nmod_vec.c`, FLINT 3.6.0, except that each remainder by a product whose
    /// dividend is at most twice as long uses the product's precomputed inverse.
    #[inline]
    fn mod_evaluate_many_precomputed(self, m: T, data: &ModEvaluateManyData<T, T>) -> Vec<T> {
        evaluate_precomputed(self, m, data)
    }
}
