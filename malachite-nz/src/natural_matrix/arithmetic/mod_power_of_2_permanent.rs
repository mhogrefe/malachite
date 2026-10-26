// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::{Natural, TWICE_WIDTH};
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Permanent};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::{
    ModPowerOf2Ring, permanent_structured,
};

// Returns the permanent modulo 2^pow of the square matrix `a`, whose elements are reduced modulo
// 2^pow: on single limbs when 2^pow fits in one, on double limbs when it fits in two, and otherwise
// on `Natural`s, by the same algorithm as for `UnsignedMatrix`s.
fn permanent_helper(a: &NaturalMatrix, pow: u64) -> Natural {
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    if pow <= Limb::WIDTH {
        Natural::from(to_limb_matrix(a).mod_power_of_2_permanent(pow))
    } else if pow <= TWICE_WIDTH {
        let n = u64::exact_from(a.height);
        let elements: Vec<DoubleLimb> = a.elements.iter().map(DoubleLimb::exact_from).collect();
        Natural::from(
            UnsignedMatrix::from_owned_elements(n, n, elements).mod_power_of_2_permanent(pow),
        )
    } else {
        permanent_structured(&ModPowerOf2Ring::new(pow), a.elements.clone(), a.height)
    }
}

impl ModPowerOf2Permanent for NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`] modulo $2^k$, taking it by value. The
    /// elements must already be reduced modulo $2^k$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. It is computed as for an
    /// [`UnsignedMatrix`] (see [`mod_power_of_2_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2Permanent)), on single limbs when $2^k$
    /// fits in one, on double limbs when it fits in two, and otherwise on [`Natural`]s, where
    /// Glynn's formula, computed modulo $2^{k+n-1}$, is always available.
    ///
    /// $$
    /// f(A, k) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(2^n n(n + k) \log (n + k) \log\log (n + k))$ for a dense matrix of dimension at
    /// most 63
    ///
    /// $M(n, k) = O(n^2 (n + k))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if any element is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Permanent;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.clone().mod_power_of_2_permanent(3).to_string(), "2");
    /// assert_eq!(a.clone().mod_power_of_2_permanent(100).to_string(), "10");
    ///
    /// let a = NaturalMatrix::from_str(
    ///     "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
    ///     5, 6, 1267650600228229401496703205373]",
    /// )
    /// .unwrap();
    /// assert_eq!(
    ///     a.clone().mod_power_of_2_permanent(100).to_string(),
    ///     "1267650600228229401496703205294"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_permanent(self, pow: u64) -> Natural {
        permanent_helper(&self, pow)
    }
}

impl ModPowerOf2Permanent for &NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`] modulo $2^k$, taking it by reference.
    /// The elements must already be reduced modulo $2^k$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. It is computed as for an
    /// [`UnsignedMatrix`] (see [`mod_power_of_2_permanent`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2Permanent)), on single limbs when $2^k$
    /// fits in one, on double limbs when it fits in two, and otherwise on [`Natural`]s, where
    /// Glynn's formula, computed modulo $2^{k+n-1}$, is always available.
    ///
    /// $$
    /// f(A, k) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(2^n n(n + k) \log (n + k) \log\log (n + k))$ for a dense matrix of dimension at
    /// most 63
    ///
    /// $M(n, k) = O(n^2 (n + k))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if any element is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Permanent;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_permanent(3).to_string(), "2");
    /// assert_eq!((&a).mod_power_of_2_permanent(100).to_string(), "10");
    ///
    /// let a = NaturalMatrix::from_str(
    ///     "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
    ///     5, 6, 1267650600228229401496703205373]",
    /// )
    /// .unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_permanent(100).to_string(),
    ///     "1267650600228229401496703205294"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_permanent(self, pow: u64) -> Natural {
        permanent_helper(self, pow)
    }
}
