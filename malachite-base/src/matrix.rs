// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;
use core::ops::Range;

/// What every matrix type has in common: a matrix of a fixed height and width, stored as its
/// elements in row-major order.
///
/// The functions here are the ones whose meaning does not depend on what the elements are. Each
/// implementation documents its own complexity and gives its own examples.
///
/// Heights, widths, and indices are [`u64`]s, as the dimensions and indices of
/// [`Vector`](crate::vector::Vector)s are; only [`Index`](core::ops::Index) takes [`usize`]s.
pub trait Matrix: Sized {
    /// The type of an element.
    type Element;

    /// Creates a matrix with the given height and width from a slice of its elements in row-major
    /// order: the first row, then the second, and so on.
    fn from_elements(height: u64, width: u64, xs: &[Self::Element]) -> Self
    where
        Self::Element: Clone;

    /// Creates a matrix with the given height and width from a [`Vec`] of its elements in row-major
    /// order, taking ownership of the [`Vec`].
    fn from_owned_elements(height: u64, width: u64, xs: Vec<Self::Element>) -> Self;

    /// Creates a matrix with the given height and width from a slice of its elements in
    /// column-major order: the first column, then the second, and so on.
    fn from_elements_column_major(height: u64, width: u64, xs: &[Self::Element]) -> Self
    where
        Self::Element: Clone;

    /// Creates a matrix with the given height and width from a [`Vec`] of its elements in
    /// column-major order, taking ownership of the [`Vec`].
    fn from_owned_elements_column_major(height: u64, width: u64, xs: Vec<Self::Element>) -> Self;

    /// Returns the zero matrix of a given height and width: every element is 0.
    fn zero(height: u64, width: u64) -> Self;

    /// Returns the identity matrix of a given size: the square matrix whose diagonal elements are 1
    /// and whose other elements are 0.
    fn identity(size: u64) -> Self;

    /// Returns a standard basis matrix, also called a matrix unit: the matrix of a given height and
    /// width whose element in row `i` and column `j` is 1 and whose other elements are 0.
    fn standard_basis_matrix(height: u64, width: u64, i: u64, j: u64) -> Self;

    /// Returns a matrix's elements in row-major order as a [`Vec`], cloning them.
    fn to_elements(&self) -> Vec<Self::Element>
    where
        Self::Element: Clone;

    /// Returns a matrix's elements in row-major order as a [`Vec`], taking ownership of the matrix.
    ///
    /// The [`Vec`] is what [`from_owned_elements`](Self::from_owned_elements) would take back,
    /// together with the height and width.
    fn into_elements(self) -> Vec<Self::Element>;

    /// Returns a reference to a matrix's elements in row-major order, as a slice.
    fn elements_ref(&self) -> &[Self::Element];

    /// Returns the height of a matrix: the number of its rows.
    fn height(&self) -> u64;

    /// Returns the width of a matrix: the number of its columns.
    fn width(&self) -> u64;

    /// Determines whether a matrix is square, that is, whether its height equals its width.
    fn is_square(&self) -> bool;

    /// Determines whether every element of a matrix is zero. This includes every matrix with no
    /// elements.
    fn is_zero(&self) -> bool;

    /// Determines whether a matrix is an identity matrix: a square matrix whose diagonal elements
    /// are 1 and whose other elements are 0. This includes the 0-by-0 matrix.
    fn is_identity(&self) -> bool;

    /// Determines whether a matrix is a standard basis matrix, returning the row and column of its
    /// 1 if it is.
    ///
    /// A standard basis matrix has a single element equal to 1 and every other element 0, as
    /// [`standard_basis_matrix`](Self::standard_basis_matrix) builds. Returns `None` for every
    /// other matrix, including the zero matrices and the matrices with no elements. When it returns
    /// `Some((i, j))`, the matrix equals `standard_basis_matrix(self.height(), self.width(), i,
    /// j)`.
    fn standard_basis_index(&self) -> Option<(u64, u64)>;

    /// Determines whether a matrix is symmetric: square, and equal to its transpose.
    fn is_symmetric(&self) -> bool;

    /// Determines whether a matrix is skew-symmetric: square, and equal to the negative of its
    /// transpose. The diagonal of a skew-symmetric matrix is zero, and a matrix whose elements
    /// cannot be negative is skew-symmetric only if it is a square zero matrix.
    fn is_skew_symmetric(&self) -> bool;

    /// Determines whether a matrix is diagonal: every element off the diagonal, that is, in row $i$
    /// and column $j$ with $i \neq j$, is zero. The matrix need not be square.
    fn is_diagonal(&self) -> bool;

    /// Determines whether a matrix is upper triangular: every element below the diagonal, that is,
    /// in row $i$ and column $j$ with $i > j$, is zero. The matrix need not be square.
    fn is_upper_triangular(&self) -> bool;

    /// Determines whether a matrix is lower triangular: every element above the diagonal, that is,
    /// in row $i$ and column $j$ with $i < j$, is zero. The matrix need not be square.
    fn is_lower_triangular(&self) -> bool;

    /// Sets every element of a matrix to zero, keeping its height and width.
    fn set_zero(&mut self);

    /// Returns the transpose of a matrix, whose rows are the columns of the original.
    fn transpose(&self) -> Self;

    /// Replaces a matrix with its transpose, whose rows are the columns of the original.
    fn transpose_assign(&mut self);

    /// Returns the window of a matrix made of the given ranges of rows and columns, as a new
    /// matrix, cloning its elements.
    ///
    /// The element of the window in row $i$ and column $j$ is the element of the matrix in row
    /// `rows.start` $+ i$ and column `columns.start` $+ j$.
    fn window(&self, rows: Range<u64>, columns: Range<u64>) -> Self
    where
        Self::Element: Clone;

    /// Returns the window of a matrix made of the given ranges of rows and columns, as a new
    /// matrix, taking ownership of the matrix and reusing its storage.
    ///
    /// The result equals what [`window`](Self::window) returns.
    fn into_window(self, rows: Range<u64>, columns: Range<u64>) -> Self;
}
