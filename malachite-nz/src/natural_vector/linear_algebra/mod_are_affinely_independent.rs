// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::common_dimension;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::One;
use malachite_base::num::linear_algebra::traits::{
    ModAreAffinelyIndependent, ModAreLinearlyIndependent,
};
use malachite_base::num::modular::traits::{ModIsReduced, ModSub};
use malachite_base::vector::Vector;

// Checks the points' dimensions and reducedness, and returns the answer if it is already known: no
// points, or one, or any points modulo 1, are affinely independent, and more than d + 1 points of
// dimension d are not.
fn known_answer(xs: &[NaturalVector], m: &Natural) -> Option<bool> {
    assert_ne!(*m, 0u32, "the modulus must be nonzero");
    let d = common_dimension(xs, "points");
    assert!(
        xs.iter().all(|x| x.mod_is_reduced(m)),
        "each point must be reduced modulo {m}"
    );
    if xs.len() <= 1 || *m == 1u32 {
        Some(true)
    } else if xs.len() > d + 1 {
        Some(false)
    } else {
        None
    }
}

crate_test_fn! {
#[allow(dead_code)]
mod_are_affinely_independent_by_homogeneous_coordinates(xs: &[NaturalVector], m: &Natural) -> bool {
    if let Some(independent) = known_answer(xs, m) {
        return independent;
    }
    let ys: Vec<NaturalVector> = xs
        .iter()
        .map(|x| {
            let mut y = x.clone();
            y.push(Natural::ONE);
            y
        })
        .collect();
    NaturalVector::mod_are_linearly_independent(&ys, m)
}}

crate_test_fn! {
#[allow(dead_code)]
mod_are_affinely_independent_by_subtraction(xs: &[NaturalVector], m: &Natural) -> bool {
    if let Some(independent) = known_answer(xs, m) {
        return independent;
    }
    // the simplest point: the one with the most zero elements
    let (j, p) = xs
        .iter()
        .enumerate()
        .min_by_key(|(_, x)| x.elements_ref().iter().filter(|e| **e != 0u32).count())
        .unwrap();
    let differences: Vec<NaturalVector> = xs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != j)
        .map(|(_, x)| x.clone().mod_sub(p, m))
        .collect();
    NaturalVector::mod_are_linearly_independent(&differences, m)
}}

impl ModAreAffinelyIndependent<Natural> for NaturalVector {
    /// Determines whether points, given as [`NaturalVector`]s, are affinely independent modulo $m$:
    /// whether no nontrivial linear combination of them, with coefficients in
    /// $\mathbb{Z}/m\mathbb{Z}$ that sum to zero, is zero modulo $m$. The modulus is taken by
    /// value.
    ///
    /// The points must all have the same dimension and must already be reduced modulo $m$. The
    /// modulus need not be prime, and every set of points is affinely independent modulo 1.
    ///
    /// Points $p_0, \ldots, p_r$ are affinely independent exactly when the vectors $(p_i, 1)$, in
    /// homogeneous coordinates, are linearly independent, and exactly when the differences $p_i -
    /// p_j$, for any fixed $j$ and every $i \neq j$, are linearly independent. The differences are
    /// taken from the simplest point, the one with the most zero elements, and their linear
    /// independence is determined by [`ModAreLinearlyIndependent`]. No points, or one, are affinely
    /// independent; more than $d + 1$ points of dimension $d$ are not.
    ///
    /// # Worst-case complexity
    /// $T(r, d, n) = O((r + d)^3 n \log n \log\log n)$
    ///
    /// $M(r, d, n) = O((r + d)^2 n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $r$ is `xs.len()`, $d$ is the dimension of the
    /// points, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the points do not all have the same dimension, or if any element
    /// of any point is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAreAffinelyIndependent;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = |s| NaturalVector::from_str(s).unwrap();
    /// assert!(!NaturalVector::mod_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(1, 1)"), v("(2, 2)")],
    ///     Natural::from(7u32)
    /// ));
    /// // (2, 0) - (0, 0) is dependent modulo 4
    /// assert!(!NaturalVector::mod_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(2, 0)")],
    ///     Natural::from(4u32)
    /// ));
    /// ```
    #[inline]
    fn mod_are_affinely_independent(xs: &[Self], m: Natural) -> bool {
        mod_are_affinely_independent_by_subtraction(xs, &m)
    }
}

impl ModAreAffinelyIndependent<&Natural> for NaturalVector {
    /// Determines whether points, given as [`NaturalVector`]s, are affinely independent modulo $m$:
    /// whether no nontrivial linear combination of them, with coefficients in
    /// $\mathbb{Z}/m\mathbb{Z}$ that sum to zero, is zero modulo $m$. The modulus is taken by
    /// reference.
    ///
    /// The points must all have the same dimension and must already be reduced modulo $m$. The
    /// modulus need not be prime, and every set of points is affinely independent modulo 1.
    ///
    /// Points $p_0, \ldots, p_r$ are affinely independent exactly when the vectors $(p_i, 1)$, in
    /// homogeneous coordinates, are linearly independent, and exactly when the differences $p_i -
    /// p_j$, for any fixed $j$ and every $i \neq j$, are linearly independent. The differences are
    /// taken from the simplest point, the one with the most zero elements, and their linear
    /// independence is determined by [`ModAreLinearlyIndependent`]. No points, or one, are affinely
    /// independent; more than $d + 1$ points of dimension $d$ are not.
    ///
    /// # Worst-case complexity
    /// $T(r, d, n) = O((r + d)^3 n \log n \log\log n)$
    ///
    /// $M(r, d, n) = O((r + d)^2 n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $r$ is `xs.len()`, $d$ is the dimension of the
    /// points, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the points do not all have the same dimension, or if any element
    /// of any point is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAreAffinelyIndependent;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = |s| NaturalVector::from_str(s).unwrap();
    /// assert!(!NaturalVector::mod_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(1, 1)"), v("(2, 2)")],
    ///     &Natural::from(7u32)
    /// ));
    /// // (2, 0) - (0, 0) is dependent modulo 4
    /// assert!(!NaturalVector::mod_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(2, 0)")],
    ///     &Natural::from(4u32)
    /// ));
    /// ```
    #[inline]
    fn mod_are_affinely_independent(xs: &[Self], m: &Natural) -> bool {
        mod_are_affinely_independent_by_subtraction(xs, m)
    }
}
