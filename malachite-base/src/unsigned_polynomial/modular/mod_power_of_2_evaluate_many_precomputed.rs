// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::polynomial::modular::ModEvaluateManyData;
use crate::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::subproduct_tree::{
    PowerOf2Modulus, evaluate_many_fast_precomputed, subproduct_tree, subproduct_tree_inverses,
};
use alloc::vec;
use alloc::vec::Vec;

fn precompute<T: PrimitiveUnsigned>(xs: &[T], pow: u64) -> ModEvaluateManyData<T, u64> {
    assert!(pow <= T::WIDTH);
    for &x in xs {
        assert!(
            x.significant_bits() <= pow,
            "every value must be reduced mod 2^pow, but {x} >= 2^{pow}"
        );
    }
    let tree = subproduct_tree(xs, &PowerOf2Modulus(pow));
    // Modulo 1 every polynomial is 0, and no remainders are taken.
    let inverses = if pow == 0 {
        Vec::new()
    } else {
        subproduct_tree_inverses(&tree, xs.len(), &PowerOf2Modulus(pow))
    };
    ModEvaluateManyData {
        tree,
        inverses,
        modulus: pow,
    }
}

fn evaluate_precomputed<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    pow: u64,
    data: &ModEvaluateManyData<T, u64>,
) -> Vec<T> {
    assert_eq!(
        pow, data.modulus,
        "the data was precomputed for a different power of 2"
    );
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return vec![T::ZERO; data.len()];
    }
    evaluate_many_fast_precomputed(
        &p.coefficients,
        &data.tree,
        &data.inverses,
        data.len(),
        &PowerOf2Modulus(pow),
    )
}

impl<T: PrimitiveUnsigned> ModPowerOf2EvaluateManyPrecomputed<T> for UnsignedPolynomial<T> {
    type Output = T;
    type Data = ModEvaluateManyData<T, u64>;

    /// Precomputes the data for evaluating [`UnsignedPolynomial`]s at each value in `xs` modulo
    /// $2^k$ with [`mod_power_of_2_evaluate_many_precomputed`](
    /// ModPowerOf2EvaluateManyPrecomputed::mod_power_of_2_evaluate_many_precomputed). The values
    /// must already be reduced modulo $2^k$, and $k$ may be at most the width of the type.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree. Alongside each product is
    /// the inverse of its reversal as a power series, which exists modulo $2^k$ because the product
    /// is monic, and with which a remainder by the product takes one truncated multiplication.
    ///
    /// # Worst-case complexity
    /// $T(l) = O(l \log^2 l)$
    ///
    /// $M(l) = O(l \log l)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $l$ is `xs.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any value in `xs` is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(
    ///     &[0, 1, 2, 3],
    ///     4,
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 16
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(4, &data),
    ///     &[7, 15, 1, 13]
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(4, &data),
    ///     &[1, 2, 9, 12]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_tree_alloc` and `_nmod_poly_tree_build` from
    /// `nmod_poly/tree.c`, FLINT 3.6.0, with the modulus $2^k$, except that the inverses of the
    /// reversals of the products are also computed.
    #[inline]
    fn precompute_mod_power_of_2_evaluate_many_data(
        xs: &[T],
        pow: u64,
    ) -> ModEvaluateManyData<T, u64> {
        precompute(xs, pow)
    }

    /// Evaluates an [`UnsignedPolynomial`] modulo $2^k$ at each value the data was precomputed
    /// from, taking the polynomial by value, and returns the values in the same order. The
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod 2^k \right )_{j=0}^{l-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $n$ is its length, and the $x_j$ are the $l$
    /// values the data was precomputed from.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_evaluate_many_data`](
    /// ModPowerOf2EvaluateManyPrecomputed::precompute_mod_power_of_2_evaluate_many_data). The
    /// polynomial is reduced modulo the products of the tree's top level, and each remainder is
    /// reduced modulo the two products below it, down to the linear factors, where the remainders
    /// are the values.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n \log n + l \log^2 l)$
    ///
    /// $M(n, l) = O(n + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $l$ is the number of
    /// values the data was precomputed from.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$, or if `pow` is not
    /// the exponent the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(
    ///     &[0, 1, 2, 3],
    ///     4,
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 16
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(4, &data),
    ///     &[7, 15, 1, 13]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_evaluate_nmod_vec_fast_precomp` from
    /// `nmod_poly/evaluate_nmod_vec.c`, FLINT 3.6.0, with the modulus $2^k$, except that each
    /// remainder by a product whose dividend is at most twice as long uses the product's
    /// precomputed inverse.
    #[inline]
    fn mod_power_of_2_evaluate_many_precomputed(
        self,
        pow: u64,
        data: &ModEvaluateManyData<T, u64>,
    ) -> Vec<T> {
        evaluate_precomputed(&self, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EvaluateManyPrecomputed<T> for &UnsignedPolynomial<T> {
    type Output = T;
    type Data = ModEvaluateManyData<T, u64>;

    /// Precomputes the data for evaluating [`UnsignedPolynomial`]s at each value in `xs` modulo
    /// $2^k$ with [`mod_power_of_2_evaluate_many_precomputed`](
    /// ModPowerOf2EvaluateManyPrecomputed::mod_power_of_2_evaluate_many_precomputed). The values
    /// must already be reduced modulo $2^k$, and $k$ may be at most the width of the type.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree. Alongside each product is
    /// the inverse of its reversal as a power series, which exists modulo $2^k$ because the product
    /// is monic, and with which a remainder by the product takes one truncated multiplication.
    ///
    /// # Worst-case complexity
    /// $T(l) = O(l \log^2 l)$
    ///
    /// $M(l) = O(l \log l)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $l$ is `xs.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any value in `xs` is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(
    ///     &[0, 1, 2, 3],
    ///     4,
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 16
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(4, &data),
    ///     &[7, 15, 1, 13]
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+1").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(4, &data),
    ///     &[1, 2, 9, 12]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_tree_alloc` and `_nmod_poly_tree_build` from
    /// `nmod_poly/tree.c`, FLINT 3.6.0, with the modulus $2^k$, except that the inverses of the
    /// reversals of the products are also computed.
    #[inline]
    fn precompute_mod_power_of_2_evaluate_many_data(
        xs: &[T],
        pow: u64,
    ) -> ModEvaluateManyData<T, u64> {
        precompute(xs, pow)
    }

    /// Evaluates an [`UnsignedPolynomial`] modulo $2^k$ at each value the data was precomputed
    /// from, taking the polynomial by reference, and returns the values in the same order. The
    /// coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod 2^k \right )_{j=0}^{l-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $n$ is its length, and the $x_j$ are the $l$
    /// values the data was precomputed from.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_evaluate_many_data`](
    /// ModPowerOf2EvaluateManyPrecomputed::precompute_mod_power_of_2_evaluate_many_data). The
    /// polynomial is reduced modulo the products of the tree's top level, and each remainder is
    /// reduced modulo the two products below it, down to the linear factors, where the remainders
    /// are the values.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n \log n + l \log^2 l)$
    ///
    /// $M(n, l) = O(n + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $l$ is the number of
    /// values the data was precomputed from.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$, or if `pow` is not
    /// the exponent the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let data = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(
    ///     &[0, 1, 2, 3],
    ///     4,
    /// );
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 16
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate_many_precomputed(4, &data),
    ///     &[7, 15, 1, 13]
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_poly_evaluate_nmod_vec_fast_precomp` from
    /// `nmod_poly/evaluate_nmod_vec.c`, FLINT 3.6.0, with the modulus $2^k$, except that each
    /// remainder by a product whose dividend is at most twice as long uses the product's
    /// precomputed inverse.
    #[inline]
    fn mod_power_of_2_evaluate_many_precomputed(
        self,
        pow: u64,
        data: &ModEvaluateManyData<T, u64>,
    ) -> Vec<T> {
        evaluate_precomputed(self, pow, data)
    }
}
