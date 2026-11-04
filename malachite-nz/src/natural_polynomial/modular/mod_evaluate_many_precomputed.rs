// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::NaturalModEvaluateManyData;
use crate::natural_polynomial::modular::mod_div_rem::to_words;
use crate::natural_polynomial::modular::subproduct_tree::NaturalModulus;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::ModIsReduced;
use malachite_base::polynomial::modular::ModEvaluateManyData;
use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    Modulus, evaluate_many_fast_precomputed, subproduct_tree, subproduct_tree_inverses,
};

fn precompute_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    m: &Natural,
) -> ModEvaluateManyData<T, T> {
    let m = T::exact_from(m);
    let tree = subproduct_tree(&to_words::<T>(xs), &Modulus(m));
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

fn precompute(xs: &[Natural], m: &Natural) -> NaturalModEvaluateManyData {
    assert_ne!(*m, 0u32);
    for x in xs {
        assert!(x < m, "every value must be reduced mod m, but {x} >= {m}");
    }
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        NaturalModEvaluateManyData::Limb(precompute_word(xs, m))
    } else if bits <= DoubleLimb::WIDTH {
        NaturalModEvaluateManyData::DoubleLimb(precompute_word(xs, m))
    } else {
        let modulus = NaturalModulus::new(m);
        let tree = subproduct_tree(xs, &modulus);
        let inverses = subproduct_tree_inverses(&tree, xs.len(), &modulus);
        NaturalModEvaluateManyData::Natural(ModEvaluateManyData {
            tree,
            inverses,
            modulus: m.clone(),
        })
    }
}

fn evaluate_precomputed_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    poly: &[Natural],
    m: &Natural,
    data: &ModEvaluateManyData<T, T>,
) -> Vec<Natural>
where
    Natural: From<T>,
{
    assert_eq!(
        *m,
        Natural::from(data.modulus),
        "the data was precomputed for a different modulus"
    );
    if data.modulus == T::ONE {
        return vec![Natural::ZERO; data.len()];
    }
    evaluate_many_fast_precomputed(
        &to_words::<T>(poly),
        &data.tree,
        &data.inverses,
        data.len(),
        &Modulus(data.modulus),
    )
    .into_iter()
    .map(Natural::from)
    .collect()
}

fn evaluate_precomputed(
    p: &NaturalPolynomial,
    m: &Natural,
    data: &NaturalModEvaluateManyData,
) -> Vec<Natural> {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    match data {
        NaturalModEvaluateManyData::Limb(data) => {
            evaluate_precomputed_word(&p.coefficients, m, data)
        }
        NaturalModEvaluateManyData::DoubleLimb(data) => {
            evaluate_precomputed_word(&p.coefficients, m, data)
        }
        NaturalModEvaluateManyData::Natural(data) => {
            assert_eq!(
                *m, data.modulus,
                "the data was precomputed for a different modulus"
            );
            evaluate_many_fast_precomputed(
                &p.coefficients,
                &data.tree,
                &data.inverses,
                data.len(),
                &NaturalModulus::new(m),
            )
        }
    }
}

impl ModEvaluateManyPrecomputed<Natural, Natural> for NaturalPolynomial {
    type Output = Natural;
    type Data = NaturalModEvaluateManyData;

    /// Precomputes the data for evaluating [`NaturalPolynomial`]s at each value in `xs` modulo $m$
    /// with [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed). The values must already be
    /// reduced modulo $m$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree, with the inverse of each
    /// product's reversal as a power series alongside it. When $m$ fits in one or two limbs, the
    /// data is held in words. The same data serves every polynomial and every way of calling
    /// [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(k, b) = O(kb \log^2 (kb) \log\log (kb))$
    ///
    /// $M(k, b) = O(kb \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any value in `xs` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(13u32);
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(
    ///     &[0u32, 1, 2, 3].map(Natural::from),
    ///     &m,
    /// );
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(m, &data).to_debug_string(),
    ///     "[7, 2, 7, 9]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_tree_alloc` and `_fmpz_mod_poly_tree_build` from
    /// `fmpz_mod_poly/tree.c`, FLINT 3.6.0, except that the inverses of the reversals of the
    /// products are also computed.
    #[inline]
    fn precompute_mod_evaluate_many_data(xs: &[Natural], m: &Natural) -> Self::Data {
        precompute(xs, m)
    }

    /// Evaluates a [`NaturalPolynomial`] modulo $m$ at each value the data was precomputed from,
    /// taking the polynomial by value and the modulus by value, and returns the values in the same
    /// order. The coefficients must already be reduced modulo $m$.
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
    /// $T(n, k, b) = O(N \log^2 N \log\log N)$
    ///
    /// $M(n, k, b) = O(N)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is the number of
    /// values the data was precomputed from, $b$ is `m.significant_bits()`, and $N$ is $(n + k)b$.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to `m`, or if `m` is not the
    /// modulus the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 2^127 - 1
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let xs = [Natural::from_str("18446744073709551616").unwrap(), Natural::from(5u32)];
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
    /// let p = NaturalPolynomial::from_str("1267650600228229401496703205379*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(m, &data).to_debug_string(),
    ///     "[2535301200474905547067115962375, 31691265005705735037417580134481]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that each remainder by a product
    /// whose dividend is at most twice as long uses the product's precomputed inverse.
    #[inline]
    fn mod_evaluate_many_precomputed(self, m: Natural, data: &Self::Data) -> Vec<Natural> {
        evaluate_precomputed(&self, &m, data)
    }
}

impl ModEvaluateManyPrecomputed<Natural, &Natural> for NaturalPolynomial {
    type Output = Natural;
    type Data = NaturalModEvaluateManyData;

    /// Precomputes the data for evaluating [`NaturalPolynomial`]s at each value in `xs` modulo $m$
    /// with [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed). The values must already be
    /// reduced modulo $m$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree, with the inverse of each
    /// product's reversal as a power series alongside it. When $m$ fits in one or two limbs, the
    /// data is held in words. The same data serves every polynomial and every way of calling
    /// [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(k, b) = O(kb \log^2 (kb) \log\log (kb))$
    ///
    /// $M(k, b) = O(kb \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any value in `xs` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(13u32);
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(
    ///     &[0u32, 1, 2, 3].map(Natural::from),
    ///     &m,
    /// );
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(&m, &data).to_debug_string(),
    ///     "[7, 2, 7, 9]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_tree_alloc` and `_fmpz_mod_poly_tree_build` from
    /// `fmpz_mod_poly/tree.c`, FLINT 3.6.0, except that the inverses of the reversals of the
    /// products are also computed.
    #[inline]
    fn precompute_mod_evaluate_many_data(xs: &[Natural], m: &&Natural) -> Self::Data {
        precompute(xs, m)
    }

    /// Evaluates a [`NaturalPolynomial`] modulo $m$ at each value the data was precomputed from,
    /// taking the polynomial by value and the modulus by reference, and returns the values in the
    /// same order. The coefficients must already be reduced modulo $m$.
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
    /// $T(n, k, b) = O(N \log^2 N \log\log N)$
    ///
    /// $M(n, k, b) = O(N)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is the number of
    /// values the data was precomputed from, $b$ is `m.significant_bits()`, and $N$ is $(n + k)b$.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to `m`, or if `m` is not the
    /// modulus the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 2^127 - 1
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let xs = [Natural::from_str("18446744073709551616").unwrap(), Natural::from(5u32)];
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
    /// let p = NaturalPolynomial::from_str("1267650600228229401496703205379*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(&m, &data).to_debug_string(),
    ///     "[2535301200474905547067115962375, 31691265005705735037417580134481]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that each remainder by a product
    /// whose dividend is at most twice as long uses the product's precomputed inverse.
    #[inline]
    fn mod_evaluate_many_precomputed(self, m: &Natural, data: &Self::Data) -> Vec<Natural> {
        evaluate_precomputed(&self, m, data)
    }
}

impl ModEvaluateManyPrecomputed<Natural, Natural> for &NaturalPolynomial {
    type Output = Natural;
    type Data = NaturalModEvaluateManyData;

    /// Precomputes the data for evaluating [`NaturalPolynomial`]s at each value in `xs` modulo $m$
    /// with [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed). The values must already be
    /// reduced modulo $m$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree, with the inverse of each
    /// product's reversal as a power series alongside it. When $m$ fits in one or two limbs, the
    /// data is held in words. The same data serves every polynomial and every way of calling
    /// [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(k, b) = O(kb \log^2 (kb) \log\log (kb))$
    ///
    /// $M(k, b) = O(kb \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any value in `xs` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(13u32);
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(
    ///     &[0u32, 1, 2, 3].map(Natural::from),
    ///     &m,
    /// );
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(m, &data).to_debug_string(),
    ///     "[7, 2, 7, 9]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_tree_alloc` and `_fmpz_mod_poly_tree_build` from
    /// `fmpz_mod_poly/tree.c`, FLINT 3.6.0, except that the inverses of the reversals of the
    /// products are also computed.
    #[inline]
    fn precompute_mod_evaluate_many_data(xs: &[Natural], m: &Natural) -> Self::Data {
        precompute(xs, m)
    }

    /// Evaluates a [`NaturalPolynomial`] modulo $m$ at each value the data was precomputed from,
    /// taking the polynomial by reference and the modulus by value, and returns the values in the
    /// same order. The coefficients must already be reduced modulo $m$.
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
    /// $T(n, k, b) = O(N \log^2 N \log\log N)$
    ///
    /// $M(n, k, b) = O(N)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is the number of
    /// values the data was precomputed from, $b$ is `m.significant_bits()`, and $N$ is $(n + k)b$.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to `m`, or if `m` is not the
    /// modulus the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 2^127 - 1
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let xs = [Natural::from_str("18446744073709551616").unwrap(), Natural::from(5u32)];
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
    /// let p = NaturalPolynomial::from_str("1267650600228229401496703205379*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate_many_precomputed(m, &data)
    ///         .to_debug_string(),
    ///     "[2535301200474905547067115962375, 31691265005705735037417580134481]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that each remainder by a product
    /// whose dividend is at most twice as long uses the product's precomputed inverse.
    #[inline]
    fn mod_evaluate_many_precomputed(self, m: Natural, data: &Self::Data) -> Vec<Natural> {
        evaluate_precomputed(self, &m, data)
    }
}

impl ModEvaluateManyPrecomputed<Natural, &Natural> for &NaturalPolynomial {
    type Output = Natural;
    type Data = NaturalModEvaluateManyData;

    /// Precomputes the data for evaluating [`NaturalPolynomial`]s at each value in `xs` modulo $m$
    /// with [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed). The values must already be
    /// reduced modulo $m$.
    ///
    /// The data is the subproduct tree of the linear factors $x - x_j$: the products of $2^i$
    /// consecutive factors, for each $i$ up to the height of the tree, with the inverse of each
    /// product's reversal as a power series alongside it. When $m$ fits in one or two limbs, the
    /// data is held in words. The same data serves every polynomial and every way of calling
    /// [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(k, b) = O(kb \log^2 (kb) \log\log (kb))$
    ///
    /// $M(k, b) = O(kb \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any value in `xs` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(13u32);
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(
    ///     &[0u32, 1, 2, 3].map(Natural::from),
    ///     &m,
    /// );
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 13
    /// assert_eq!(
    ///     p.mod_evaluate_many_precomputed(&m, &data).to_debug_string(),
    ///     "[7, 2, 7, 9]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_tree_alloc` and `_fmpz_mod_poly_tree_build` from
    /// `fmpz_mod_poly/tree.c`, FLINT 3.6.0, except that the inverses of the reversals of the
    /// products are also computed.
    #[inline]
    fn precompute_mod_evaluate_many_data(xs: &[Natural], m: &&Natural) -> Self::Data {
        precompute(xs, m)
    }

    /// Evaluates a [`NaturalPolynomial`] modulo $m$ at each value the data was precomputed from,
    /// taking the polynomial by reference and the modulus by reference, and returns the values in
    /// the same order. The coefficients must already be reduced modulo $m$.
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
    /// $T(n, k, b) = O(N \log^2 N \log\log N)$
    ///
    /// $M(n, k, b) = O(N)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is the number of
    /// values the data was precomputed from, $b$ is `m.significant_bits()`, and $N$ is $(n + k)b$.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to `m`, or if `m` is not the
    /// modulus the data was precomputed for.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateManyPrecomputed;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // 2^127 - 1
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let xs = [Natural::from_str("18446744073709551616").unwrap(), Natural::from(5u32)];
    /// let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
    /// let p = NaturalPolynomial::from_str("1267650600228229401496703205379*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_evaluate_many_precomputed(&m, &data)
    ///         .to_debug_string(),
    ///     "[2535301200474905547067115962375, 31691265005705735037417580134481]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, except that each remainder by a product
    /// whose dividend is at most twice as long uses the product's precomputed inverse.
    #[inline]
    fn mod_evaluate_many_precomputed(self, m: &Natural, data: &Self::Data) -> Vec<Natural> {
        evaluate_precomputed(self, m, data)
    }
}
