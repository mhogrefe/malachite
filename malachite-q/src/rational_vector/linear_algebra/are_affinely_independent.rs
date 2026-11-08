// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::common_dimension;
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::One;
use malachite_base::num::linear_algebra::traits::{AreAffinelyIndependent, AreLinearlyIndependent};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::vector::Vector;

crate_test_fn! {
#[allow(dead_code)]
are_affinely_independent_by_homogeneous_coordinates(xs: &[RationalVector]) -> bool {
    let d = common_dimension(xs, "points");
    if xs.len() <= 1 {
        return true;
    } else if xs.len() > d + 1 {
        return false;
    }
    let ys: Vec<RationalVector> = xs
        .iter()
        .map(|x| {
            let mut y = x.clone();
            y.push(<RationalVector as Vector>::Element::ONE);
            y
        })
        .collect();
    RationalVector::are_linearly_independent(&ys)
}}

crate_test_fn! {
#[allow(dead_code)]
are_affinely_independent_by_subtraction(xs: &[RationalVector]) -> bool {
    let d = common_dimension(xs, "points");
    if xs.len() <= 1 {
        return true;
    } else if xs.len() > d + 1 {
        return false;
    }
    // the simplest point: the one whose elements have the fewest bits in all
    let (j, p) = xs
        .iter()
        .enumerate()
        .min_by_key(|(_, x)| x.significant_bits())
        .unwrap();
    let differences: Vec<RationalVector> = xs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != j)
        .map(|(_, x)| x - p)
        .collect();
    RationalVector::are_linearly_independent(&differences)
}}

impl AreAffinelyIndependent for RationalVector {
    /// Determines whether points, given as [`RationalVector`]s, are affinely independent over the
    /// rationals: whether no nontrivial linear combination of them, with rational coefficients that
    /// sum to zero, is zero.
    ///
    /// The points must all have the same dimension.
    ///
    /// Points $p_0, \ldots, p_r$ are affinely independent exactly when the vectors $(p_i, 1)$, in
    /// homogeneous coordinates, are linearly independent, and exactly when the differences $p_i -
    /// p_j$, for any fixed $j$ and every $i \neq j$, are linearly independent. The vectors in
    /// homogeneous coordinates are used, and their linear independence is determined by
    /// [`AreLinearlyIndependent`]. No points, or one, are affinely independent; more than $d + 1$
    /// points of dimension $d$ are not.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of
    /// [`are_linearly_independent`](AreLinearlyIndependent::are_linearly_independent) for the
    /// points in homogeneous coordinates.
    ///
    /// # Panics
    /// Panics if the points do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::AreAffinelyIndependent;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = |s| RationalVector::from_str(s).unwrap();
    /// assert!(!RationalVector::are_affinely_independent(&[
    ///     v("(0, 0)"),
    ///     v("(1/2, 1/3)"),
    ///     v("(1, 2/3)")
    /// ]));
    /// assert!(RationalVector::are_affinely_independent(&[
    ///     v("(0, 0)"),
    ///     v("(1/2, 0)"),
    ///     v("(0, 1/3)")
    /// ]));
    /// ```
    #[inline]
    fn are_affinely_independent(xs: &[Self]) -> bool {
        are_affinely_independent_by_homogeneous_coordinates(xs)
    }
}
