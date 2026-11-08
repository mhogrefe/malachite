// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::natural_vector::modular::mod_generalized_cross::{
    generalized_cross_dimension, square_with_zero_last_row,
};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::GeneralizedCross;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::ModPowerOf2Cofactor;
use malachite_base::num::modular::traits::{
    ModPowerOf2, ModPowerOf2GeneralizedCross, ModPowerOf2IsReduced,
};

// Panics unless the `n - 1` vectors all have dimension `n` and are reduced modulo `2^pow`, and
// returns `n`.
fn check(xs: &[NaturalVector], pow: u64) -> usize {
    let n = generalized_cross_dimension(xs);
    assert!(
        xs.iter().all(|x| x.mod_power_of_2_is_reduced(pow)),
        "each vector must be reduced modulo 2^{pow}"
    );
    n
}

crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_generalized_cross_by_cofactors(xs: &[NaturalVector], pow: u64) -> Vec<Natural> {
    let n = check(xs, pow);
    let a = square_with_zero_last_row(xs, n);
    let last = u64::exact_from(n - 1);
    (0..=last).map(|i| (&a).mod_power_of_2_cofactor(last, i, pow)).collect()
}}

// The generalized cross product over the integers, reduced modulo `2^pow`.
crate_test_fn! {
mod_power_of_2_generalized_cross_by_integers(xs: &[NaturalVector], pow: u64) -> Vec<Natural> {
    check(xs, pow);
    let ys: Vec<IntegerVector> = xs.iter().map(IntegerVector::from).collect();
    IntegerVector::generalized_cross(&ys)
        .elements
        .into_iter()
        .map(|e| e.mod_power_of_2(pow))
        .collect()
}}

impl ModPowerOf2GeneralizedCross for NaturalVector {
    /// Computes the generalized cross product of $n - 1$ [`NaturalVector`]s of dimension $n$ modulo
    /// $2^k$.
    ///
    /// $$
    /// f(v_0, \ldots, v_{n - 2}, k)_i = (-1)^{n - 1 + i} \det A_{(i)} \bmod 2^k,
    /// $$
    ///
    /// where $A$ is the matrix whose rows are the vectors and $A_{(i)}$ is $A$ without column $i$.
    /// The result $w$ satisfies $w \cdot x \equiv \det \begin{pmatrix} A \\ x \end{pmatrix}
    /// \pmod{2^k}$ for every $x$, so it is perpendicular to each vector modulo $2^k$. For $n = 2$
    /// it rotates the vector a quarter turn counterclockwise, $(a, b) \mapsto (-b, a) \bmod 2^k$;
    /// for $n = 3$ it is the cross product; and no vectors give $(1 \bmod 2^k)$.
    ///
    /// The product is found over the integers, by
    /// [`IntegerVector::generalized_cross`](IntegerVector), and reduced modulo $2^k$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`generalized_cross`](GeneralizedCross::generalized_cross) for
    /// [`IntegerVector`]s with the same elements.
    ///
    /// # Panics
    /// Panics if the vectors do not all have dimension one more than their number, or if any
    /// element of any vector is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::modular::traits::ModPowerOf2GeneralizedCross;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = |s| NaturalVector::from_str(s).unwrap();
    /// assert_eq!(
    ///     NaturalVector::mod_power_of_2_generalized_cross(&[v("(2, 0, 1)"), v("(0, 2, 3)")], 2)
    ///         .to_string(),
    ///     "(2, 2, 0)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::mod_power_of_2_generalized_cross(&[v("(3, 4)")], 3).to_string(),
    ///     "(4, 3)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::mod_power_of_2_generalized_cross(&[], 0).to_string(),
    ///     "(0)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_generalized_cross(xs: &[Self], pow: u64) -> Self {
        Self {
            elements: mod_power_of_2_generalized_cross_by_integers(xs, pow),
        }
    }
}
