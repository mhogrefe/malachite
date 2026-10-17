// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};

/// A matrix whose elements are primitive unsigned integers.
///
/// The elements are stored in one [`Vec`], `elements`, in row-major order: the element in row $i$
/// and column $j$, both counted from 0, is at index $iw + j$, where $w$ is the width. This is the
/// layout of FLINT's matrices, without the stride FLINT uses for windows into other matrices.
///
/// `elements` is public, but its length must stay equal to the height times the width; a matrix
/// whose `elements` has been resized is invalid, and functions on it may panic or return wrong
/// results. The dimensions are private, so that they can only change together with the elements.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeUnsignedMatrix<T>", into = "SerdeUnsignedMatrix<T>")
)]
pub struct UnsignedMatrix<T: PrimitiveUnsigned> {
    pub elements: Vec<T>,
    height: usize,
    width: usize,
}

// The serialized form of an `UnsignedMatrix`: its dimensions and its elements in row-major order. A
// matrix whose number of elements is not the product of its dimensions is rejected.
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeUnsignedMatrix<T: PrimitiveUnsigned> {
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) elements: Vec<T>,
}

// The number of elements of a matrix with the given dimensions, panicking if it does not fit in a
// `usize`.
const fn element_count(height: usize, width: usize) -> usize {
    height
        .checked_mul(width)
        .expect("the matrix has too many elements to represent")
}

impl<T: PrimitiveUnsigned> UnsignedMatrix<T> {
    // Returns true iff `self` is valid.
    //
    // To be valid, the number of elements must be the height times the width. All `UnsignedMatrix`s
    // must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.height.checked_mul(self.width) == Some(self.elements.len())
    }

    /// Returns the number of rows of an [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .height(),
    ///     2
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,4}")
    ///         .unwrap()
    ///         .height(),
    ///     0
    /// );
    /// ```
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the number of columns of an [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .width(),
    ///     3
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,4}").unwrap().width(),
    ///     4
    /// );
    /// ```
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Creates an [`UnsignedMatrix`] with the given height and width from a slice of its elements
    /// in row-major order: the first row, then the second, and so on.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `xs.len()` is not `height * width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_elements(2, 3, &[1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// // A matrix with no rows can still have columns.
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_elements(0, 4, &[]).to_string(),
    ///     "[]_{0,4}"
    /// );
    /// ```
    #[inline]
    pub fn from_elements(height: usize, width: usize, xs: &[T]) -> Self {
        Self::from_owned_elements(height, width, xs.to_vec())
    }

    /// Creates an [`UnsignedMatrix`] with the given height and width from a [`Vec`] of its elements
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_owned_elements(3, 2, vec![1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 2; 3, 4; 5, 6]");
    /// ```
    pub fn from_owned_elements(height: usize, width: usize, xs: Vec<T>) -> Self {
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

    /// Creates an [`UnsignedMatrix`] with the given height and width from a slice of its elements
    /// in column-major order: the first column, then the second, and so on.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Panics
    /// Panics if `xs.len()` is not `height * width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_elements_column_major(2, 3, &[1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_elements_column_major(height: usize, width: usize, xs: &[T]) -> Self {
        assert_eq!(
            xs.len(),
            element_count(height, width),
            "the number of elements must be the height times the width"
        );
        let mut elements = Vec::with_capacity(xs.len());
        for i in 0..height {
            elements.extend(xs.iter().skip(i).step_by(height).copied());
        }
        Self {
            elements,
            height,
            width,
        }
    }

    /// Creates an [`UnsignedMatrix`] with the given height and width from a [`Vec`] of its elements
    /// in column-major order: the first column, then the second, and so on.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_owned_elements_column_major(2, 3, vec![1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    #[inline]
    pub fn from_owned_elements_column_major(height: usize, width: usize, xs: Vec<T>) -> Self {
        // The column-major elements of a matrix are the row-major elements of its transpose.
        Self::from_owned_elements(width, height, xs).transpose()
    }

    /// Creates an [`UnsignedMatrix`] from a slice of its rows, each an [`UnsignedVector`]. The
    /// height is the number of rows and the width is their common dimension; with no rows, the
    /// matrix is 0-by-0, and [`from_elements`](Self::from_elements) makes a matrix with no rows but
    /// some columns.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let m = UnsignedMatrix::from_rows(&[
    ///     UnsignedVector::<u64>::from_str("(1, 2, 3)").unwrap(),
    ///     UnsignedVector::<u64>::from_str("(4, 5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    /// ```
    pub fn from_rows(rows: &[UnsignedVector<T>]) -> Self {
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

    /// Creates an [`UnsignedMatrix`] from a [`Vec`] of its rows, each an [`UnsignedVector`]. The
    /// height is the number of rows and the width is their common dimension; with no rows, the
    /// matrix is 0-by-0.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let m = UnsignedMatrix::from_owned_rows(vec![
    ///     UnsignedVector::<u64>::from_str("(1, 2, 3)").unwrap(),
    ///     UnsignedVector::<u64>::from_str("(4, 5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    /// ```
    pub fn from_owned_rows(rows: Vec<UnsignedVector<T>>) -> Self {
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

    /// Creates an [`UnsignedMatrix`] from a slice of its columns, each an [`UnsignedVector`]. The
    /// width is the number of columns and the height is their common dimension; with no columns,
    /// the matrix is 0-by-0, and [`from_elements`](Self::from_elements) makes a matrix with no
    /// columns but some rows.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let m = UnsignedMatrix::from_columns(&[
    ///     UnsignedVector::<u64>::from_str("(1, 2)").unwrap(),
    ///     UnsignedVector::<u64>::from_str("(3, 4)").unwrap(),
    ///     UnsignedVector::<u64>::from_str("(5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_columns(columns: &[UnsignedVector<T>]) -> Self {
        let height = common_dimension(columns, "columns");
        let width = columns.len();
        let mut elements = Vec::with_capacity(element_count(height, width));
        for i in 0..height {
            elements.extend(columns.iter().map(|column| column.elements[i]));
        }
        Self {
            elements,
            height,
            width,
        }
    }

    /// Creates an [`UnsignedMatrix`] from a [`Vec`] of its columns, each an [`UnsignedVector`]. The
    /// width is the number of columns and the height is their common dimension; with no columns,
    /// the matrix is 0-by-0.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let m = UnsignedMatrix::from_owned_columns(vec![
    ///     UnsignedVector::<u64>::from_str("(1, 2)").unwrap(),
    ///     UnsignedVector::<u64>::from_str("(3, 4)").unwrap(),
    ///     UnsignedVector::<u64>::from_str("(5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_owned_columns(columns: Vec<UnsignedVector<T>>) -> Self {
        let height = common_dimension(&columns, "columns");
        let width = columns.len();
        let elements: Vec<T> = columns
            .into_iter()
            .flat_map(|column| column.elements)
            .collect();
        Self {
            elements: transposed_elements(&elements, width, height),
            height,
            width,
        }
    }
}

// Copies the elements of a `height`-by-`width` matrix, given in row-major order, into the row-major
// order of its transpose.
pub(crate) fn transposed_elements<T: PrimitiveUnsigned>(
    xs: &[T],
    height: usize,
    width: usize,
) -> Vec<T> {
    let mut out = Vec::with_capacity(xs.len());
    for j in 0..width {
        for i in 0..height {
            out.push(xs[i * width + j]);
        }
    }
    out
}

// Writes the elements of a matrix with at least one row and one column, row by row: `write` writes
// each element, `element_separator` goes between the elements of a row, and `row_separator` between
// rows.
pub(crate) fn write_rows<T: PrimitiveUnsigned>(
    m: &UnsignedMatrix<T>,
    f: &mut Formatter,
    element_separator: &str,
    row_separator: &str,
    write: impl Fn(&T, &mut Formatter) -> Result,
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
fn common_dimension<T: PrimitiveUnsigned>(vectors: &[UnsignedVector<T>], what: &str) -> usize {
    let dimension = vectors.first().map_or(0, |v| v.elements.len());
    assert!(
        vectors.iter().all(|v| v.elements.len() == dimension),
        "the {what} must all have the same dimension"
    );
    dimension
}

/// Indexing an [`UnsignedMatrix`] by row and column.
pub mod access;
/// Arithmetic on [`UnsignedMatrix`]s.
pub mod arithmetic;
/// Conversions to and from [`UnsignedMatrix`].
pub mod conversion;
/// Iterators that generate [`UnsignedMatrix`]s without repetition.
pub mod exhaustive;
/// A function for determining whether an [`UnsignedMatrix`] is square.
pub mod is_square;
/// Iterators that generate [`UnsignedMatrix`]s randomly.
#[cfg(feature = "random")]
pub mod random;
/// The transpose of an [`UnsignedMatrix`].
pub mod transpose;
