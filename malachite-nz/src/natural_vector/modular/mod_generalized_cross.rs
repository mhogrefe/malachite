// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::GeneralizedCross;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::ModCofactor;
use malachite_base::num::modular::traits::{Mod, ModGeneralizedCross, ModIsReduced};

// Panics unless the `n - 1` vectors all have dimension `n`, and returns `n`.
pub(crate) fn generalized_cross_dimension(xs: &[NaturalVector]) -> usize {
    let n = xs.len() + 1;
    assert!(
        xs.iter().all(|x| x.elements.len() == n),
        "the generalized cross product of {} vectors is only defined for vectors of dimension {n}",
        xs.len()
    );
    n
}

// The `n`-by-`n` matrix whose first `n - 1` rows are the vectors and whose last row is zero.
// Element `i` of the generalized cross product of the vectors is its cofactor in row `n - 1` and
// column `i`.
pub(crate) fn square_with_zero_last_row(xs: &[NaturalVector], n: usize) -> NaturalMatrix {
    let mut elements = Vec::with_capacity(n * n);
    for x in xs {
        elements.extend_from_slice(&x.elements);
    }
    elements.resize(n * n, Natural::ZERO);
    let n = u64::exact_from(n);
    NaturalMatrix::from_owned_elements(n, n, elements)
}

// Panics unless `m` is nonzero and the `n - 1` vectors all have dimension `n` and are reduced
// modulo `m`, and returns `n`.
fn check(xs: &[NaturalVector], m: &Natural) -> usize {
    assert_ne!(*m, 0u32, "the modulus must be nonzero");
    let n = generalized_cross_dimension(xs);
    assert!(
        xs.iter().all(|x| x.mod_is_reduced(m)),
        "each vector must be reduced modulo {m}"
    );
    n
}

crate_test_fn! {
#[allow(dead_code)]
mod_generalized_cross_by_cofactors(xs: &[NaturalVector], m: &Natural) -> Vec<Natural> {
    let n = check(xs, m);
    let a = square_with_zero_last_row(xs, n);
    let last = u64::exact_from(n - 1);
    (0..=last).map(|i| (&a).mod_cofactor(last, i, m)).collect()
}}

// The generalized cross product over the integers, reduced modulo `m`.
crate_test_fn! {
mod_generalized_cross_by_integers(xs: &[NaturalVector], m: &Natural) -> Vec<Natural> {
    check(xs, m);
    let ys: Vec<IntegerVector> = xs.iter().map(IntegerVector::from).collect();
    let m = Integer::from(m);
    IntegerVector::generalized_cross(&ys)
        .elements
        .into_iter()
        .map(|e| Natural::exact_from(e.mod_op(&m)))
        .collect()
}}

impl ModGeneralizedCross<Natural> for NaturalVector {
    /// Computes the generalized cross product of $n - 1$ [`NaturalVector`]s of dimension $n$ modulo
    /// $m$, taking the modulus by value.
    ///
    /// $$
    /// f(v_0, \ldots, v_{n - 2}, m)_i = (-1)^{n - 1 + i} \det A_{(i)} \bmod m,
    /// $$
    ///
    /// where $A$ is the matrix whose rows are the vectors and $A_{(i)}$ is $A$ without column $i$.
    /// The result $w$ satisfies $w \cdot x \equiv \det \begin{pmatrix} A \\ x \end{pmatrix}
    /// \pmod{m}$ for every $x$, so it is perpendicular to each vector modulo $m$. The modulus may
    /// be composite. For $n = 2$ it rotates the vector a quarter turn counterclockwise, $(a, b)
    /// \mapsto (-b, a) \bmod m$; for $n = 3$ it is the cross product; and no vectors give $(1 \bmod
    /// m)$.
    ///
    /// The product is found over the integers, by
    /// [`IntegerVector::generalized_cross`](IntegerVector), and reduced modulo $m$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`generalized_cross`](GeneralizedCross::generalized_cross) for
    /// [`IntegerVector`]s with the same elements.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors do not all have dimension one more than their number,
    /// or if any element of any vector is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::modular::traits::ModGeneralizedCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = |s| NaturalVector::from_str(s).unwrap();
    /// assert_eq!(
    ///     NaturalVector::mod_generalized_cross(
    ///         &[v("(1, 2, 3)"), v("(4, 5, 6)")],
    ///         Natural::from(7u32)
    ///     )
    ///     .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::mod_generalized_cross(&[v("(3, 4)")], Natural::from(7u32)).to_string(),
    ///     "(3, 3)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::mod_generalized_cross(&[], Natural::from(5u32)).to_string(),
    ///     "(1)"
    /// );
    /// ```
    #[inline]
    fn mod_generalized_cross(xs: &[Self], m: Natural) -> Self {
        Self {
            elements: mod_generalized_cross_by_integers(xs, &m),
        }
    }
}

impl ModGeneralizedCross<&Natural> for NaturalVector {
    /// Computes the generalized cross product of $n - 1$ [`NaturalVector`]s of dimension $n$ modulo
    /// $m$, taking the modulus by reference.
    ///
    /// $$
    /// f(v_0, \ldots, v_{n - 2}, m)_i = (-1)^{n - 1 + i} \det A_{(i)} \bmod m,
    /// $$
    ///
    /// where $A$ is the matrix whose rows are the vectors and $A_{(i)}$ is $A$ without column $i$.
    /// The result $w$ satisfies $w \cdot x \equiv \det \begin{pmatrix} A \\ x \end{pmatrix}
    /// \pmod{m}$ for every $x$, so it is perpendicular to each vector modulo $m$. The modulus may
    /// be composite. For $n = 2$ it rotates the vector a quarter turn counterclockwise, $(a, b)
    /// \mapsto (-b, a) \bmod m$; for $n = 3$ it is the cross product; and no vectors give $(1 \bmod
    /// m)$.
    ///
    /// The product is found over the integers, by
    /// [`IntegerVector::generalized_cross`](IntegerVector), and reduced modulo $m$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`generalized_cross`](GeneralizedCross::generalized_cross) for
    /// [`IntegerVector`]s with the same elements.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors do not all have dimension one more than their number,
    /// or if any element of any vector is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::modular::traits::ModGeneralizedCross;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = |s| NaturalVector::from_str(s).unwrap();
    /// assert_eq!(
    ///     NaturalVector::mod_generalized_cross(
    ///         &[v("(1, 2, 3)"), v("(4, 5, 6)")],
    ///         &Natural::from(7u32)
    ///     )
    ///     .to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::mod_generalized_cross(&[v("(3, 4)")], &Natural::from(7u32)).to_string(),
    ///     "(3, 3)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::mod_generalized_cross(&[], &Natural::from(5u32)).to_string(),
    ///     "(1)"
    /// );
    /// ```
    #[inline]
    fn mod_generalized_cross(xs: &[Self], m: &Natural) -> Self {
        Self {
            elements: mod_generalized_cross_by_integers(xs, m),
        }
    }
}
