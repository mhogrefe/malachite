// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::NaturalModPowerOf2EvaluateManyData;
use crate::natural_polynomial::modular::mod_div_rem::to_words;
use crate::natural_polynomial::modular::subproduct_tree::NaturalPowerOf2Modulus;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
use malachite_base::polynomial::modular::ModEvaluateManyData;
use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    PowerOf2Modulus, evaluate_many_fast_precomputed, subproduct_tree, subproduct_tree_inverses,
};

fn precompute_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    pow: u64,
) -> ModEvaluateManyData<T, u64> {
    let tree = subproduct_tree(&to_words::<T>(xs), &PowerOf2Modulus(pow));
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

fn precompute(xs: &[Natural], pow: u64) -> NaturalModPowerOf2EvaluateManyData {
    for x in xs {
        assert!(
            x.significant_bits() <= pow,
            "every value must be reduced mod 2^pow, but {x} >= 2^{pow}"
        );
    }
    if pow <= Limb::WIDTH {
        NaturalModPowerOf2EvaluateManyData::Limb(precompute_word(xs, pow))
    } else if pow <= DoubleLimb::WIDTH {
        NaturalModPowerOf2EvaluateManyData::DoubleLimb(precompute_word(xs, pow))
    } else {
        let tree = subproduct_tree(xs, &NaturalPowerOf2Modulus(pow));
        let inverses = subproduct_tree_inverses(&tree, xs.len(), &NaturalPowerOf2Modulus(pow));
        NaturalModPowerOf2EvaluateManyData::Natural(ModEvaluateManyData {
            tree,
            inverses,
            modulus: pow,
        })
    }
}

fn evaluate_precomputed_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    poly: &[Natural],
    data: &ModEvaluateManyData<T, u64>,
) -> Vec<Natural>
where
    Natural: From<T>,
{
    if data.modulus == 0 {
        return vec![Natural::ZERO; data.len()];
    }
    evaluate_many_fast_precomputed(
        &to_words::<T>(poly),
        &data.tree,
        &data.inverses,
        data.len(),
        &PowerOf2Modulus(data.modulus),
    )
    .into_iter()
    .map(Natural::from)
    .collect()
}

fn evaluate_precomputed(
    p: &NaturalPolynomial,
    pow: u64,
    data: &NaturalModPowerOf2EvaluateManyData,
) -> Vec<Natural> {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    let modulus = match data {
        NaturalModPowerOf2EvaluateManyData::Limb(data) => data.modulus,
        NaturalModPowerOf2EvaluateManyData::DoubleLimb(data) => data.modulus,
        NaturalModPowerOf2EvaluateManyData::Natural(data) => data.modulus,
    };
    assert_eq!(
        pow, modulus,
        "the data was precomputed for a different power of 2"
    );
    match data {
        NaturalModPowerOf2EvaluateManyData::Limb(data) => {
            evaluate_precomputed_word(&p.coefficients, data)
        }
        NaturalModPowerOf2EvaluateManyData::DoubleLimb(data) => {
            evaluate_precomputed_word(&p.coefficients, data)
        }
        NaturalModPowerOf2EvaluateManyData::Natural(data) => evaluate_many_fast_precomputed(
            &p.coefficients,
            &data.tree,
            &data.inverses,
            data.len(),
            &NaturalPowerOf2Modulus(pow),
        ),
    }
}

impl ModPowerOf2EvaluateManyPrecomputed<Natural> for NaturalPolynomial {
    type Output = Natural;
    type Data = NaturalModPowerOf2EvaluateManyData;

    /// Precomputes the data for evaluating [`NaturalPolynomial`]s at each value in `xs` modulo
    /// $2^k$ with [`mod_power_of_2_evaluate_many_precomputed`](
    /// ModPowerOf2EvaluateManyPrecomputed::mod_power_of_2_evaluate_many_precomputed). The values
    /// must already be reduced modulo $2^k$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree, with the inverse of each
    /// product's reversal as a power series alongside it, which exists modulo $2^k$ because the
    /// product is monic. When $2^k$ fits in one or two limbs, the data is held in words.
    ///
    /// # Worst-case complexity
    /// $T(l, k) = O(lk \log^2 (lk) \log\log (lk))$
    ///
    /// $M(l, k) = O(lk \log l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $l$ is `xs.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any value in `xs` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, 100);
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(100, &data)
    ///         .to_debug_string(),
    ///     "[7, 15, 33, 61]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_tree_alloc` and `_fmpz_mod_poly_tree_build` from
    /// `fmpz_mod_poly/tree.c`, FLINT 3.6.0, with the modulus $2^k$, except that the inverses of the
    /// reversals of the products are also computed.
    #[inline]
    fn precompute_mod_power_of_2_evaluate_many_data(xs: &[Natural], pow: u64) -> Self::Data {
        precompute(xs, pow)
    }

    /// Evaluates a [`NaturalPolynomial`] modulo $2^k$ at each value the data was precomputed from,
    /// taking the polynomial by value, and returns the values in the same order. The coefficients
    /// must already be reduced modulo $2^k$.
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
    /// $T(n, l, k) = O(N \log^2 N \log\log N)$
    ///
    /// $M(n, l, k) = O(N)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $l$ is the number of
    /// values the data was precomputed from, $k$ is `pow`, and $N$ is $(n + l)k$.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$, or if `pow` is not
    /// the exponent the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, 100);
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(100, &data)
    ///         .to_debug_string(),
    ///     "[7, 15, 33, 61]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, with the modulus $2^k$, except that each
    /// remainder by a product whose dividend is at most twice as long uses the product's
    /// precomputed inverse.
    #[inline]
    fn mod_power_of_2_evaluate_many_precomputed(self, pow: u64, data: &Self::Data) -> Vec<Natural> {
        evaluate_precomputed(&self, pow, data)
    }
}

impl ModPowerOf2EvaluateManyPrecomputed<Natural> for &NaturalPolynomial {
    type Output = Natural;
    type Data = NaturalModPowerOf2EvaluateManyData;

    /// Precomputes the data for evaluating [`NaturalPolynomial`]s at each value in `xs` modulo
    /// $2^k$ with [`mod_power_of_2_evaluate_many_precomputed`](
    /// ModPowerOf2EvaluateManyPrecomputed::mod_power_of_2_evaluate_many_precomputed). The values
    /// must already be reduced modulo $2^k$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree, with the inverse of each
    /// product's reversal as a power series alongside it, which exists modulo $2^k$ because the
    /// product is monic. When $2^k$ fits in one or two limbs, the data is held in words.
    ///
    /// # Worst-case complexity
    /// $T(l, k) = O(lk \log^2 (lk) \log\log (lk))$
    ///
    /// $M(l, k) = O(lk \log l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $l$ is `xs.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any value in `xs` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, 100);
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// assert_eq!(
    ///     p.mod_power_of_2_evaluate_many_precomputed(100, &data)
    ///         .to_debug_string(),
    ///     "[7, 15, 33, 61]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_tree_alloc` and `_fmpz_mod_poly_tree_build` from
    /// `fmpz_mod_poly/tree.c`, FLINT 3.6.0, with the modulus $2^k$, except that the inverses of the
    /// reversals of the products are also computed.
    #[inline]
    fn precompute_mod_power_of_2_evaluate_many_data(xs: &[Natural], pow: u64) -> Self::Data {
        precompute(xs, pow)
    }

    /// Evaluates a [`NaturalPolynomial`] modulo $2^k$ at each value the data was precomputed from,
    /// taking the polynomial by reference, and returns the values in the same order. The
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
    /// $T(n, l, k) = O(N \log^2 N \log\log N)$
    ///
    /// $M(n, l, k) = O(N)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $l$ is the number of
    /// values the data was precomputed from, $k$ is `pow`, and $N$ is $(n + l)k$.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$, or if `pow` is not
    /// the exponent the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, 100);
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate_many_precomputed(100, &data)
    ///         .to_debug_string(),
    ///     "[7, 15, 33, 61]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, with the modulus $2^k$, except that each
    /// remainder by a product whose dividend is at most twice as long uses the product's
    /// precomputed inverse.
    #[inline]
    fn mod_power_of_2_evaluate_many_precomputed(self, pow: u64, data: &Self::Data) -> Vec<Natural> {
        evaluate_precomputed(self, pow, data)
    }
}
