// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::linear_algebra::traits::{
    ModPowerOf2AreAffinelyIndependent, ModPowerOf2AreLinearlyIndependent,
};
use crate::num::modular::traits::{ModPowerOf2IsReduced, ModPowerOf2Sub};
use crate::unsigned_matrix::common_dimension;
use crate::unsigned_vector::UnsignedVector;
use crate::vector::Vector;
use alloc::vec::Vec;

// Checks the points' dimensions and reducedness, and returns the answer if it is already known: no
// points, or one, or any points modulo 1, are affinely independent, and more than d + 1 points of
// dimension d are not.
fn known_answer<T: PrimitiveUnsigned>(xs: &[UnsignedVector<T>], pow: u64) -> Option<bool> {
    let d = common_dimension(xs, "points");
    assert!(
        xs.iter().all(|x| x.mod_power_of_2_is_reduced(pow)),
        "each point must be reduced modulo 2^{pow}"
    );
    if xs.len() <= 1 || pow == 0 {
        Some(true)
    } else if xs.len() > d + 1 {
        Some(false)
    } else {
        None
    }
}

crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_are_affinely_independent_by_homogeneous_coordinates<T: PrimitiveUnsigned>(
    xs: &[UnsignedVector<T>],
    pow: u64,
) -> bool {
    if let Some(independent) = known_answer(xs, pow) {
        return independent;
    }
    let ys: Vec<UnsignedVector<T>> = xs
        .iter()
        .map(|x| {
            let mut y = x.clone();
            y.push(T::ONE);
            y
        })
        .collect();
    UnsignedVector::mod_power_of_2_are_linearly_independent(&ys, pow)
}}

crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_are_affinely_independent_by_subtraction<T: PrimitiveUnsigned>(
    xs: &[UnsignedVector<T>],
    pow: u64,
) -> bool {
    if let Some(independent) = known_answer(xs, pow) {
        return independent;
    }
    // the simplest point: the one with the most zero elements
    let (j, p) = xs
        .iter()
        .enumerate()
        .min_by_key(|(_, x)| x.elements.iter().filter(|&&e| e != T::ZERO).count())
        .unwrap();
    let differences: Vec<UnsignedVector<T>> = xs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != j)
        .map(|(_, x)| x.clone().mod_power_of_2_sub(p, pow))
        .collect();
    UnsignedVector::mod_power_of_2_are_linearly_independent(&differences, pow)
}}

crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_are_affinely_independent_by_reduction_mod_2<T: PrimitiveUnsigned>(
    xs: &[UnsignedVector<T>],
    pow: u64,
) -> bool {
    if let Some(independent) = known_answer(xs, pow) {
        return independent;
    }
    // Since pow > 0, the differences of the points are linearly independent modulo 2^pow exactly
    // when they are modulo 2, so the points are affinely independent modulo 2^pow exactly when
    // their parities are affinely independent modulo 2.
    let parities: Vec<UnsignedVector<T>> = xs
        .iter()
        .map(|x| {
            UnsignedVector::from_owned_elements(x.elements.iter().map(|&e| T::from(e.odd())).collect())
        })
        .collect();
    mod_power_of_2_are_affinely_independent_by_subtraction(&parities, 1)
}}

impl<T: PrimitiveUnsigned> ModPowerOf2AreAffinelyIndependent for UnsignedVector<T> {
    /// Determines whether points, given as [`UnsignedVector`]s, are affinely independent modulo
    /// $2^k$: whether no nontrivial linear combination of them, with coefficients in
    /// $\mathbb{Z}/2^k\mathbb{Z}$ that sum to zero, is zero modulo $2^k$.
    ///
    /// The points must all have the same dimension and must already be reduced modulo $2^k$. Every
    /// set of points is affinely independent modulo 1.
    ///
    /// Points $p_0, \ldots, p_r$ are affinely independent exactly when the vectors $(p_i, 1)$, in
    /// homogeneous coordinates, are linearly independent, and exactly when the differences $p_i -
    /// p_j$, for any fixed $j$ and every $i \neq j$, are linearly independent. The differences are
    /// taken from the simplest point, the one with the most zero elements, and their linear
    /// independence is determined by [`ModPowerOf2AreLinearlyIndependent`]. No points, or one, are
    /// affinely independent; more than $d + 1$ points of dimension $d$ are not.
    ///
    /// # Worst-case complexity
    /// $T(r, d) = O(rd\min(r, d))$
    ///
    /// $M(r, d) = O(rd)$
    ///
    /// where $T$ is time, $M$ is additional memory, $r$ is `xs.len()`, and $d$ is the dimension of
    /// the points.
    ///
    /// # Panics
    /// Panics if the points do not all have the same dimension, or if any element of any point is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2AreAffinelyIndependent;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = |s| UnsignedVector::<u8>::from_str(s).unwrap();
    /// assert!(UnsignedVector::mod_power_of_2_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(1, 0)"), v("(0, 1)")],
    ///     3
    /// ));
    /// // (2, 0) - (0, 0) is dependent modulo 4
    /// assert!(!UnsignedVector::mod_power_of_2_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(2, 0)")],
    ///     2
    /// ));
    /// ```
    #[inline]
    fn mod_power_of_2_are_affinely_independent(xs: &[Self], pow: u64) -> bool {
        mod_power_of_2_are_affinely_independent_by_subtraction(xs, pow)
    }
}
