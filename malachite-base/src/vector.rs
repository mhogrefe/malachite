// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::ExactFrom;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::take;

/// What every vector type has in common: a vector of a fixed dimension, stored as its elements in
/// order.
///
/// The functions here are the ones whose meaning does not depend on what the elements are. Each
/// implementation documents its own complexity and gives its own examples.
pub trait Vector: Sized {
    /// The type of an element.
    type Element;

    /// What [`pivot`](Self::pivot) returns.
    ///
    /// This is a reference to an [`Element`](Self::Element) when the vector holds elements that
    /// are expensive to copy, and an [`Element`](Self::Element) itself when one is cheap to copy.
    type ElementOutput<'a>
    where
        Self: 'a;

    /// Converts a slice to a vector, cloning the elements.
    ///
    /// The vector's dimension is the length of the slice. Every slice is a valid vector, so this
    /// cannot fail; the empty slice gives the 0-dimensional vector.
    fn from_elements(xs: &[Self::Element]) -> Self
    where
        Self::Element: Clone;

    /// Converts a [`Vec`] to a vector, taking ownership of it.
    ///
    /// The vector's dimension is the length of the [`Vec`]. Every [`Vec`] is a valid vector, so
    /// this cannot fail, and nothing is copied or allocated.
    fn from_owned_elements(xs: Vec<Self::Element>) -> Self;

    /// Returns the zero vector of a given dimension: a vector of `dimension` zeros.
    fn zero(dimension: u64) -> Self;

    /// Returns a standard basis vector: the vector of a given dimension whose element at `index` is
    /// 1 and whose other elements are 0.
    fn standard_basis_vector(dimension: u64, index: u64) -> Self;

    /// Appends an element to the end of a vector, increasing its dimension by 1.
    fn push(&mut self, x: Self::Element);

    /// Sets the dimension of a vector, removing elements from the end if the new dimension is
    /// smaller, and appending zeros if it is larger.
    fn set_dimension(&mut self, dimension: u64);

    /// Returns a vector's elements as a [`Vec`], cloning them.
    fn to_elements(&self) -> Vec<Self::Element>
    where
        Self::Element: Clone;

    /// Returns a vector's elements as a [`Vec`], taking ownership of the vector.
    ///
    /// The [`Vec`] is what [`from_owned_elements`](Self::from_owned_elements) would take back.
    fn into_elements(self) -> Vec<Self::Element>;

    /// Returns a reference to a vector's elements, as a slice.
    fn elements_ref(&self) -> &[Self::Element];

    /// Returns the dimension of a vector: the number of its elements.
    fn dimension(&self) -> u64;

    /// Returns the pivot of a vector: its first nonzero element.
    ///
    /// This is the element that leads the vector when it is a row of a matrix in echelon form.
    /// Returns `None` if every element is zero, which includes the 0-dimensional vector.
    fn pivot(&self) -> Option<Self::ElementOutput<'_>>;

    /// Returns the index of the pivot of a vector: the position of its first nonzero element.
    ///
    /// Indices start at 0, as they do for [`Index`](core::ops::Index). Returns `None` if every
    /// element is zero, which includes the 0-dimensional vector. When it returns `Some(i)`,
    /// [`pivot`](Self::pivot) is the element at `i`.
    fn pivot_index(&self) -> Option<u64>;

    /// Returns the index of an element of largest height: the first one, when several share it.
    ///
    /// The height of an integer is its absolute value, and the height of a rational number $p/q$
    /// in lowest terms is $\max(|p|, q)$. Returns `None` for the 0-dimensional vector.
    fn height_index(&self) -> Option<u64>;

    /// Returns the number of significant bits of a vector's height, together with whether any of
    /// its elements is negative.
    ///
    /// The bit count is the largest bit length of any element's height, or 0 for the 0-dimensional
    /// vector; it is what `Height::height_significant_bits` returns.
    fn max_bits(&self) -> (u64, bool);

    /// Returns the number of significant bits of a vector's $\ell^1$ norm, the sum of the absolute
    /// values of its elements, together with the number of significant bits of its height.
    ///
    /// The counts are what `L1Norm::l1_norm_significant_bits` and `Height::height_significant_bits`
    /// return, computed together in one pass where possible. Both are 0 for the 0-dimensional
    /// vector.
    fn sum_max_bits(&self) -> (u64, u64);
}

/// Selects coordinates of a vector by index: the result's element $j$ is the original's element
/// `indices[j]`.
///
/// The indices may repeat and may appear in any order, so this covers projection onto some of the
/// coordinates (strictly increasing indices), permutation, and duplication of coordinates. The
/// result's dimension is the number of indices.
pub trait SelectCoordinates {
    type Output;

    fn select_coordinates(self, indices: &[u64]) -> Self::Output;
}

/// Selects coordinates of a vector by index, in place: afterwards, element $j$ of the vector is
/// what element `indices[j]` was before.
pub trait SelectCoordinatesAssign {
    fn select_coordinates_assign(&mut self, indices: &[u64]);
}

fn check_index(index: u64, dimension: usize) -> usize {
    let index = usize::exact_from(index);
    assert!(
        index < dimension,
        "index {index} is out of range for a vector of dimension {dimension}"
    );
    index
}

// The elements of `xs` at `indices`, cloned. Panics if an index is out of range.
#[doc(hidden)]
#[inline]
pub fn select_elements<T: Clone>(xs: &[T], indices: &[u64]) -> Vec<T> {
    indices
        .iter()
        .map(|&i| xs[check_index(i, xs.len())].clone())
        .collect()
}

// Replaces `xs` with its elements at `indices`, cloning as few as possible. Panics if an index is
// out of range.
//
// When the indices are strictly increasing, the selected elements are swapped into place and the
// rest truncated away, so nothing is cloned or allocated. Otherwise each element is moved out at
// its last use and cloned at any earlier one, so an element is cloned once for each extra time it
// is selected, which is as few clones as any method needs.
#[doc(hidden)]
pub fn select_elements_in_place<T: Clone + Default>(xs: &mut Vec<T>, indices: &[u64]) {
    let n = xs.len();
    if indices.windows(2).all(|w| w[0] < w[1]) {
        if let Some(&last) = indices.last() {
            check_index(last, n);
        }
        for (j, &i) in indices.iter().enumerate() {
            let i = usize::exact_from(i);
            if i != j {
                xs.swap(j, i);
            }
        }
        xs.truncate(indices.len());
        return;
    }
    let mut last_use = vec![usize::MAX; n];
    for (j, &i) in indices.iter().enumerate() {
        last_use[check_index(i, n)] = j;
    }
    let mut selected = Vec::with_capacity(indices.len());
    for (j, &i) in indices.iter().enumerate() {
        let i = usize::exact_from(i);
        selected.push(if last_use[i] == j {
            take(&mut xs[i])
        } else {
            xs[i].clone()
        });
    }
    *xs = selected;
}
