// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::linear_algebra::traits::{
    ModAreAffinelyIndependent, ModAreLinearlyIndependent, ModHowellForm,
};
use crate::num::modular::traits::{ModIsReduced, ModSub};
use crate::unsigned_matrix::{UnsignedMatrix, common_dimension};
use crate::unsigned_vector::UnsignedVector;
use crate::vector::Vector;
use alloc::vec::Vec;

// Checks the points' dimensions and reducedness, and returns the answer if it is already known: no
// points, or one, or any points modulo 1, are affinely independent, and more than d + 1 points of
// dimension d are not.
fn known_answer<T: PrimitiveUnsigned>(xs: &[UnsignedVector<T>], m: T) -> Option<bool> {
    assert_ne!(m, T::ZERO, "the modulus must be nonzero");
    let d = common_dimension(xs, "points");
    assert!(
        xs.iter().all(|x| x.mod_is_reduced(&m)),
        "each point must be reduced modulo {m}"
    );
    if xs.len() <= 1 || m == T::ONE {
        Some(true)
    } else if xs.len() > d + 1 {
        Some(false)
    } else {
        None
    }
}

crate_test_fn! {
#[allow(dead_code)]
mod_are_affinely_independent_by_homogeneous_coordinates<T: PrimitiveUnsigned>(
    xs: &[UnsignedVector<T>],
    m: T,
) -> bool
where
    UnsignedMatrix<T>: ModHowellForm<T, Output = UnsignedMatrix<T>>,
{
    if let Some(independent) = known_answer(xs, m) {
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
    UnsignedVector::mod_are_linearly_independent(&ys, m)
}}

crate_test_fn! {
#[allow(dead_code)]
mod_are_affinely_independent_by_subtraction<T: PrimitiveUnsigned>(
    xs: &[UnsignedVector<T>],
    m: T,
) -> bool
where
    UnsignedMatrix<T>: ModHowellForm<T, Output = UnsignedMatrix<T>>,
{
    if let Some(independent) = known_answer(xs, m) {
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
        .map(|(_, x)| x.clone().mod_sub(p, m))
        .collect();
    UnsignedVector::mod_are_linearly_independent(&differences, m)
}}

impl<T: PrimitiveUnsigned> ModAreAffinelyIndependent<T> for UnsignedVector<T>
where
    UnsignedMatrix<T>: ModHowellForm<T, Output = UnsignedMatrix<T>>,
{
    /// Determines whether points, given as [`UnsignedVector`]s, are affinely independent modulo
    /// $m$: whether no nontrivial linear combination of them, with coefficients in
    /// $\mathbb{Z}/m\mathbb{Z}$ that sum to zero, is zero modulo $m$.
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
    /// $T(r, d) = O((r + d)^3)$
    ///
    /// $M(r, d) = O((r + d)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $r$ is `xs.len()`, and $d$ is the dimension of
    /// the points.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the points do not all have the same dimension, or if any element
    /// of any point is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAreAffinelyIndependent;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = |s| UnsignedVector::<u8>::from_str(s).unwrap();
    /// // three points on a line
    /// assert!(!UnsignedVector::mod_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(1, 1)"), v("(2, 2)")],
    ///     7
    /// ));
    /// assert!(UnsignedVector::mod_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(1, 0)"), v("(0, 1)")],
    ///     7
    /// ));
    /// // (2, 0) - (0, 0) is dependent modulo 4
    /// assert!(!UnsignedVector::mod_are_affinely_independent(
    ///     &[v("(0, 0)"), v("(2, 0)")],
    ///     4
    /// ));
    /// assert!(UnsignedVector::mod_are_affinely_independent(
    ///     &[v("(1, 1)")],
    ///     4
    /// ));
    /// ```
    #[inline]
    fn mod_are_affinely_independent(xs: &[Self], m: T) -> bool {
        mod_are_affinely_independent_by_subtraction(xs, m)
    }
}
