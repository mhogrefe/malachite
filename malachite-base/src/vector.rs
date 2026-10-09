// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;

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
}
