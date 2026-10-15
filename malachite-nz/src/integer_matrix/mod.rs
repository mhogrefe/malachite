// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};
use core::mem::take;

/// A matrix whose elements are [`Integer`]s.
///
/// The elements are stored in one [`Vec`], `elements`, in row-major order: the element in row $i$
/// and column $j$, both counted from 0, is at index $iw + j$, where $w$ is the width. This is the
/// layout of FLINT's `fmpz_mat_t`, without the stride FLINT uses for windows into other matrices.
///
/// `elements` is public, but its length must stay equal to the height times the width; a matrix
/// whose `elements` has been resized is invalid, and functions on it may panic or return wrong
/// results. The dimensions are private, so that they can only change together with the elements.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeIntegerMatrix", into = "SerdeIntegerMatrix")
)]
pub struct IntegerMatrix {
    pub elements: Vec<Integer>,
    height: usize,
    width: usize,
}

// The serialized form of an `IntegerMatrix`: its dimensions and its elements in row-major order. A
// matrix whose number of elements is not the product of its dimensions is rejected.
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeIntegerMatrix {
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) elements: Vec<Integer>,
}

// The number of elements of a matrix with the given dimensions, panicking if it does not fit in a
// `usize`.
const fn element_count(height: usize, width: usize) -> usize {
    height
        .checked_mul(width)
        .expect("the matrix has too many elements to represent")
}

impl IntegerMatrix {
    // Returns true iff `self` is valid.
    //
    // To be valid, the number of elements must be the height times the width. All `IntegerMatrix`s
    // must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.height.checked_mul(self.width) == Some(self.elements.len())
    }

    /// Returns the number of rows of an [`IntegerMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// assert_eq!(
    ///     IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .height(),
    ///     2
    /// );
    /// assert_eq!(IntegerMatrix::from_str("[]_{0,4}").unwrap().height(), 0);
    /// ```
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the number of columns of an [`IntegerMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// assert_eq!(
    ///     IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .width(),
    ///     3
    /// );
    /// assert_eq!(IntegerMatrix::from_str("[]_{0,4}").unwrap().width(), 4);
    /// ```
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Creates an [`IntegerMatrix`] with the given height and width from a slice of its elements in
    /// row-major order: the first row, then the second, and so on.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if `xs.len()` is not `height * width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let m = IntegerMatrix::from_elements(2, 3, &[1, 2, 3, 4, 5, 6].map(Integer::from));
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// // A matrix with no rows can still have columns.
    /// assert_eq!(
    ///     IntegerMatrix::from_elements(0, 4, &[]).to_string(),
    ///     "[]_{0,4}"
    /// );
    /// ```
    #[inline]
    pub fn from_elements(height: usize, width: usize, xs: &[Integer]) -> Self {
        Self::from_owned_elements(height, width, xs.to_vec())
    }

    /// Creates an [`IntegerMatrix`] with the given height and width from a [`Vec`] of its elements
    /// in row-major order: the first row, then the second, and so on. The [`Vec`] becomes the
    /// matrix's storage.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `xs.len()` is not `height * width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let m = IntegerMatrix::from_owned_elements(
    ///     3,
    ///     2,
    ///     [1, 2, 3, 4, 5, 6].map(Integer::from).to_vec(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 2; 3, 4; 5, 6]");
    /// ```
    pub fn from_owned_elements(height: usize, width: usize, xs: Vec<Integer>) -> Self {
        assert_eq!(
            xs.len(),
            element_count(height, width),
            "the number of elements must be the height times the width"
        );
        Self {
            elements: xs,
            height,
            width,
        }
    }

    /// Creates an [`IntegerMatrix`] with the given height and width from a slice of its elements in
    /// column-major order: the first column, then the second, and so on.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if `xs.len()` is not `height * width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let m =
    ///     IntegerMatrix::from_elements_column_major(2, 3, &[1, 2, 3, 4, 5, 6].map(Integer::from));
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_elements_column_major(height: usize, width: usize, xs: &[Integer]) -> Self {
        assert_eq!(
            xs.len(),
            element_count(height, width),
            "the number of elements must be the height times the width"
        );
        let mut elements = Vec::with_capacity(xs.len());
        for i in 0..height {
            elements.extend(xs.iter().skip(i).step_by(height).cloned());
        }
        Self {
            elements,
            height,
            width,
        }
    }

    /// Creates an [`IntegerMatrix`] with the given height and width from a [`Vec`] of its elements
    /// in column-major order: the first column, then the second, and so on. The elements are moved,
    /// not copied.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
    ///
    /// # Panics
    /// Panics if `xs.len()` is not `height * width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let m = IntegerMatrix::from_owned_elements_column_major(
    ///     2,
    ///     3,
    ///     [1, 2, 3, 4, 5, 6].map(Integer::from).to_vec(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_owned_elements_column_major(height: usize, width: usize, xs: Vec<Integer>) -> Self {
        assert_eq!(
            xs.len(),
            element_count(height, width),
            "the number of elements must be the height times the width"
        );
        Self {
            elements: transposed_elements(xs, width, height),
            height,
            width,
        }
    }

    /// Creates an [`IntegerMatrix`] from a slice of its rows, each an [`IntegerVector`]. The height
    /// is the number of rows and the width is their common dimension; with no rows, the matrix is
    /// 0-by-0, and [`from_elements`](Self::from_elements) makes a matrix with no rows but some
    /// columns.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the rows do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let m = IntegerMatrix::from_rows(&[
    ///     IntegerVector::from_str("(1, 2, 3)").unwrap(),
    ///     IntegerVector::from_str("(4, 5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    /// ```
    pub fn from_rows(rows: &[IntegerVector]) -> Self {
        let width = common_dimension(rows, "rows");
        let mut elements = Vec::with_capacity(element_count(rows.len(), width));
        for row in rows {
            elements.extend_from_slice(&row.elements);
        }
        Self {
            elements,
            height: rows.len(),
            width,
        }
    }

    /// Creates an [`IntegerMatrix`] from a [`Vec`] of its rows, each an [`IntegerVector`], moving
    /// the elements rather than copying them. The height is the number of rows and the width is
    /// their common dimension; with no rows, the matrix is 0-by-0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if the rows do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let m = IntegerMatrix::from_owned_rows(vec![
    ///     IntegerVector::from_str("(1, 2, 3)").unwrap(),
    ///     IntegerVector::from_str("(4, 5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    /// ```
    pub fn from_owned_rows(rows: Vec<IntegerVector>) -> Self {
        let width = common_dimension(&rows, "rows");
        let height = rows.len();
        let mut elements = Vec::with_capacity(element_count(height, width));
        for row in rows {
            elements.extend(row.elements);
        }
        Self {
            elements,
            height,
            width,
        }
    }

    /// Creates an [`IntegerMatrix`] from a slice of its columns, each an [`IntegerVector`]. The
    /// width is the number of columns and the height is their common dimension; with no columns,
    /// the matrix is 0-by-0, and [`from_elements`](Self::from_elements) makes a matrix with no
    /// columns but some rows.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the columns do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let m = IntegerMatrix::from_columns(&[
    ///     IntegerVector::from_str("(1, 2)").unwrap(),
    ///     IntegerVector::from_str("(3, 4)").unwrap(),
    ///     IntegerVector::from_str("(5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_columns(columns: &[IntegerVector]) -> Self {
        let height = common_dimension(columns, "columns");
        let width = columns.len();
        let mut elements = Vec::with_capacity(element_count(height, width));
        for i in 0..height {
            elements.extend(columns.iter().map(|column| column.elements[i].clone()));
        }
        Self {
            elements,
            height,
            width,
        }
    }

    /// Creates an [`IntegerMatrix`] from a [`Vec`] of its columns, each an [`IntegerVector`],
    /// moving the elements rather than copying them. The width is the number of columns and the
    /// height is their common dimension; with no columns, the matrix is 0-by-0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if the columns do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let m = IntegerMatrix::from_owned_columns(vec![
    ///     IntegerVector::from_str("(1, 2)").unwrap(),
    ///     IntegerVector::from_str("(3, 4)").unwrap(),
    ///     IntegerVector::from_str("(5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_owned_columns(columns: Vec<IntegerVector>) -> Self {
        let height = common_dimension(&columns, "columns");
        let width = columns.len();
        let elements = columns
            .into_iter()
            .flat_map(|column| column.elements)
            .collect();
        Self {
            elements: transposed_elements(elements, width, height),
            height,
            width,
        }
    }
}

// Moves the elements of a `height`-by-`width` matrix, given in row-major order, into the row-major
// order of its transpose.
pub(crate) fn transposed_elements(
    mut xs: Vec<Integer>,
    height: usize,
    width: usize,
) -> Vec<Integer> {
    let mut out = Vec::with_capacity(xs.len());
    for j in 0..width {
        for i in 0..height {
            out.push(take(&mut xs[i * width + j]));
        }
    }
    out
}

// Writes the elements of a matrix with at least one row and one column, row by row: `write` writes
// each element, `element_separator` goes between the elements of a row, and `row_separator` between
// rows.
pub(crate) fn write_rows(
    m: &IntegerMatrix,
    f: &mut Formatter,
    element_separator: &str,
    row_separator: &str,
    write: impl Fn(&Integer, &mut Formatter) -> Result,
) -> Result {
    for (i, row) in m.elements.chunks(m.width).enumerate() {
        if i != 0 {
            f.write_str(row_separator)?;
        }
        for (j, x) in row.iter().enumerate() {
            if j != 0 {
                f.write_str(element_separator)?;
            }
            write(x, f)?;
        }
    }
    Ok(())
}

// The dimension shared by all of `vectors`, or 0 if there are none, panicking if they differ.
fn common_dimension(vectors: &[IntegerVector], what: &str) -> usize {
    let dimension = vectors.first().map_or(0, |v| v.elements.len());
    assert!(
        vectors.iter().all(|v| v.elements.len() == dimension),
        "the {what} must all have the same dimension"
    );
    dimension
}

/// Indexing an [`IntegerMatrix`] by row and column.
pub mod access;
/// Conversions to and from [`IntegerMatrix`].
pub mod conversion;
/// The transpose of an [`IntegerMatrix`].
pub mod transpose;
