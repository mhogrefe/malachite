// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::{NaturalMatrix, common_dimension};
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::{ModPowerOf2AreLinearlyIndependent, ModRank};
use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the vectors' dimensions and reducedness, and returns the answer if it is already known: no
// vectors, or any vectors modulo 1, are linearly independent, and more vectors than their dimension
// are not. Otherwise, returns the number of vectors and their dimension.
fn known_answer(xs: &[NaturalVector], pow: u64) -> Result<bool, (u64, u64)> {
    let d = common_dimension(xs, "vectors");
    assert!(
        xs.iter().all(|x| x.mod_power_of_2_is_reduced(pow)),
        "each vector must be reduced modulo 2^{pow}"
    );
    let r = xs.len();
    if r == 0 || pow == 0 {
        Ok(true)
    } else if r > d {
        Ok(false)
    } else {
        Err((u64::exact_from(r), u64::exact_from(d)))
    }
}

crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_are_linearly_independent_by_natural_rank(xs: &[NaturalVector], pow: u64) -> bool {
    let (r, d) = match known_answer(xs, pow) {
        Ok(independent) => return independent,
        Err(shape) => shape,
    };
    let elements: Vec<Natural> = xs
        .iter()
        .flat_map(|x| x.elements.iter().map(|e| Natural::from(e.odd())))
        .collect();
    NaturalMatrix::from_elements(r, d, &elements).mod_rank(Natural::TWO) == r
}}

crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_are_linearly_independent_by_word_rank(xs: &[NaturalVector], pow: u64) -> bool {
    let (r, d) = match known_answer(xs, pow) {
        Ok(independent) => return independent,
        Err(shape) => shape,
    };
    let elements: Vec<u64> = xs
        .iter()
        .flat_map(|x| x.elements.iter().map(|e| u64::from(e.odd())))
        .collect();
    UnsignedMatrix::from_elements(r, d, &elements).mod_rank(2) == r
}}

impl ModPowerOf2AreLinearlyIndependent for NaturalVector {
    /// Determines whether [`NaturalVector`]s are linearly independent modulo $2^k$: whether no
    /// nontrivial linear combination of them, with coefficients in $\mathbb{Z}/2^k\mathbb{Z}$, is
    /// zero modulo $2^k$.
    ///
    /// The vectors must all have the same dimension and must already be reduced modulo $2^k$. Since
    /// 2 is the only prime factor of $2^k$, the vectors are linearly independent modulo $2^k$ for
    /// $k > 0$ exactly when they are linearly independent modulo 2, which is when the matrix whose
    /// rows are the vectors reduced modulo 2 has full rank. An empty slice is linearly independent,
    /// and so is every slice modulo 1, where the only coefficient is 0.
    ///
    /// # Worst-case complexity
    /// $T(r, d) = O(rd\min(r, d))$
    ///
    /// $M(r, d) = O(rd)$
    ///
    /// where $T$ is time, $M$ is additional memory, $r$ is `xs.len()`, and $d$ is the dimension of
    /// the vectors.
    ///
    /// # Panics
    /// Panics if the vectors do not all have the same dimension, or if any element of any vector is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2AreLinearlyIndependent;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = |s| NaturalVector::from_str(s).unwrap();
    /// // 2 * (2, 0) = (0, 0) modulo 4
    /// assert!(!NaturalVector::mod_power_of_2_are_linearly_independent(
    ///     &[v("(2, 0)")],
    ///     2
    /// ));
    /// assert!(NaturalVector::mod_power_of_2_are_linearly_independent(
    ///     &[v("(2, 1)")],
    ///     2
    /// ));
    /// assert!(NaturalVector::mod_power_of_2_are_linearly_independent(
    ///     &[v("(1, 2)"), v("(3, 1)")],
    ///     2
    /// ));
    /// ```
    #[inline]
    fn mod_power_of_2_are_linearly_independent(xs: &[Self], pow: u64) -> bool {
        mod_power_of_2_are_linearly_independent_by_word_rank(xs, pow)
    }
}
