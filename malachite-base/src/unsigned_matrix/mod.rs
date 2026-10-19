// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::named::Named;
use crate::num::arithmetic::traits::DivisibleBy;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use core::fmt::{Formatter, Result};
use core::mem::swap;
use core::ops::{Deref, Range};

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

// The height, width, and number of elements of a matrix with the given dimensions, panicking if any
// of them does not fit in a `usize`.
fn shape(height: u64, width: u64) -> (usize, usize, usize) {
    const TOO_LARGE: &str = "the matrix has too many elements to represent";
    let height = usize::try_from(height).expect(TOO_LARGE);
    let width = usize::try_from(width).expect(TOO_LARGE);
    (height, width, element_count(height, width))
}

// Converts the row and column ranges of a window of a matrix with the given dimensions to `usize`
// ranges, panicking if either range is decreasing or extends past the matrix.
fn window_ranges(
    height: usize,
    width: usize,
    rows: &Range<u64>,
    columns: &Range<u64>,
) -> (Range<usize>, Range<usize>) {
    assert!(
        rows.start <= rows.end,
        "the row range {rows:?} of a window is decreasing"
    );
    assert!(
        rows.end <= u64::exact_from(height),
        "the row range {rows:?} of a window extends past a matrix with {height} rows"
    );
    assert!(
        columns.start <= columns.end,
        "the column range {columns:?} of a window is decreasing"
    );
    assert!(
        columns.end <= u64::exact_from(width),
        "the column range {columns:?} of a window extends past a matrix with {width} columns"
    );
    (
        usize::exact_from(rows.start)..usize::exact_from(rows.end),
        usize::exact_from(columns.start)..usize::exact_from(columns.end),
    )
}

// Converts the position of a window with the given shape in a matrix with the given dimensions to
// `usize` ranges of rows and columns, panicking if the window does not fit.
fn set_window_ranges(
    height: usize,
    width: usize,
    i: u64,
    j: u64,
    window_height: usize,
    window_width: usize,
) -> (Range<usize>, Range<usize>) {
    assert!(
        i <= u64::exact_from(height) && window_height <= height - usize::exact_from(i),
        "a window of {window_height} rows from row {i} does not fit in a matrix with {height} rows"
    );
    assert!(
        j <= u64::exact_from(width) && window_width <= width - usize::exact_from(j),
        "a window of {window_width} columns from column {j} does not fit in {width} columns"
    );
    let (i, j) = (usize::exact_from(i), usize::exact_from(j));
    (i..i + window_height, j..j + window_width)
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

impl<T: PrimitiveUnsigned> Matrix for UnsignedMatrix<T> {
    type Element = T;

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
    /// Panics if `xs.len()` is not `height * width`, or if the number of elements does not fit in a
    /// [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
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
    fn from_elements(height: u64, width: u64, xs: &[T]) -> Self {
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
    /// Panics if `xs.len()` is not `height * width`, or if the number of elements does not fit in a
    /// [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_owned_elements(3, 2, vec![1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 2; 3, 4; 5, 6]");
    /// ```
    fn from_owned_elements(height: u64, width: u64, xs: Vec<T>) -> Self {
        let (height, width, count) = shape(height, width);
        assert_eq!(
            xs.len(),
            count,
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
    /// Panics if `xs.len()` is not `height * width`, or if the number of elements does not fit in a
    /// [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_elements_column_major(2, 3, &[1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    fn from_elements_column_major(height: u64, width: u64, xs: &[T]) -> Self {
        let (height, width, count) = shape(height, width);
        assert_eq!(
            xs.len(),
            count,
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
    /// Panics if `xs.len()` is not `height * width`, or if the number of elements does not fit in a
    /// [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_owned_elements_column_major(2, 3, vec![1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    #[inline]
    fn from_owned_elements_column_major(height: u64, width: u64, xs: Vec<T>) -> Self {
        // The column-major elements of a matrix are the row-major elements of its transpose.
        Self::from_owned_elements(width, height, xs).transpose()
    }

    /// Returns the zero matrix of a given height and width: every element is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
    ///
    /// # Panics
    /// Panics if the number of elements does not fit in a [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::zero(2, 3).to_string(),
    ///     "[0, 0, 0; 0, 0, 0]"
    /// );
    /// assert_eq!(UnsignedMatrix::<u64>::zero(0, 3).to_string(), "[]_{0,3}");
    /// ```
    fn zero(height: u64, width: u64) -> Self {
        let (height, width, count) = shape(height, width);
        Self {
            elements: vec![T::ZERO; count],
            height,
            width,
        }
    }

    /// Returns the identity matrix of a given size: the square matrix whose diagonal elements are 1
    /// and whose other elements are 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `size * size`.
    ///
    /// # Panics
    /// Panics if the number of elements does not fit in a [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::identity(3).to_string(),
    ///     "[1, 0, 0; 0, 1, 0; 0, 0, 1]"
    /// );
    /// assert_eq!(UnsignedMatrix::<u64>::identity(0).to_string(), "[]_{0,0}");
    /// ```
    fn identity(size: u64) -> Self {
        let mut m = Self::zero(size, size);
        // The diagonal elements are `size + 1` apart.
        for x in m.elements.iter_mut().step_by(m.width + 1) {
            *x = T::ONE;
        }
        m
    }

    /// Returns the rectangular identity matrix of a given height and width: the [`UnsignedMatrix`]
    /// whose element in row $i$ and column $j$ is 1 if $i = j$ and 0 otherwise.
    ///
    /// It is the identity matrix of the smaller dimension, padded with zero rows or columns, and
    /// the matrix that FLINT's `*_mat_one` functions produce for a matrix of that shape. When the
    /// height equals the width it is [`identity`](Self::identity).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
    ///
    /// # Panics
    /// Panics if the number of elements does not fit in a [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::rectangular_identity(2, 3).to_string(),
    ///     "[1, 0, 0; 0, 1, 0]"
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::rectangular_identity(3, 2).to_string(),
    ///     "[1, 0; 0, 1; 0, 0]"
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::rectangular_identity(0, 3).to_string(),
    ///     "[]_{0,3}"
    /// );
    /// ```
    fn rectangular_identity(height: u64, width: u64) -> Self {
        let mut m = Self::zero(height, width);
        // The diagonal elements are `width + 1` apart, and there are as many as the smaller
        // dimension.
        let diagonal_length = min(m.height, m.width);
        for x in m
            .elements
            .iter_mut()
            .step_by(m.width + 1)
            .take(diagonal_length)
        {
            *x = T::ONE;
        }
        m
    }

    /// Returns a standard basis matrix $E_{ij}$, also called a matrix unit: the matrix of a given
    /// height and width whose element in row `i` and column `j` is 1 and whose other elements are
    /// 0.
    ///
    /// Indices start at 0. The standard basis matrices of a given shape form a basis of the
    /// matrices of that shape, as the standard basis vectors do for vectors.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to `height`, if `j` is greater than or equal to
    /// `width`, or if the number of elements does not fit in a [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::standard_basis_matrix(2, 3, 1, 0).to_string(),
    ///     "[0, 0, 0; 1, 0, 0]"
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::standard_basis_matrix(1, 1, 0, 0).to_string(),
    ///     "[1]"
    /// );
    /// ```
    fn standard_basis_matrix(height: u64, width: u64, i: u64, j: u64) -> Self {
        assert!(
            i < height,
            "row index {i} is out of range for a matrix with {height} rows"
        );
        assert!(
            j < width,
            "column index {j} is out of range for a matrix with {width} columns"
        );
        let mut m = Self::zero(height, width);
        let position = usize::exact_from(i) * m.width + usize::exact_from(j);
        m.elements[position] = T::ONE;
        m
    }

    /// Returns the elements of an [`UnsignedMatrix`] in row-major order as a [`Vec`], cloning them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.to_elements(), &[1u64, 2, 3, 4]);
    /// ```
    #[inline]
    fn to_elements(&self) -> Vec<T> {
        self.elements.clone()
    }

    /// Returns the elements of an [`UnsignedMatrix`] in row-major order as a [`Vec`], taking
    /// ownership of the matrix. Nothing is copied.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.into_elements(), &[1u64, 2, 3, 4]);
    /// ```
    #[inline]
    fn into_elements(self) -> Vec<T> {
        self.elements
    }

    /// Returns a reference to the elements of an [`UnsignedMatrix`] in row-major order, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.elements_ref(), &[1u64, 2, 3, 4]);
    /// ```
    #[inline]
    fn elements_ref(&self) -> &[T] {
        &self.elements
    }

    /// Returns the number of rows of an [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
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
    fn height(&self) -> u64 {
        u64::exact_from(self.height)
    }

    /// Returns the number of columns of an [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
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
    fn width(&self) -> u64 {
        u64::exact_from(self.width)
    }

    /// Determines whether an [`UnsignedMatrix`] is square, that is, whether its height equals its
    /// width.
    ///
    /// The 0-by-0 matrix is square; the other matrices with no elements, such as a 3-by-0 matrix,
    /// are not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]")
    ///         .unwrap()
    ///         .is_square()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .is_square()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .is_square()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[]_{3,0}")
    ///         .unwrap()
    ///         .is_square()
    /// );
    /// ```
    #[inline]
    fn is_square(&self) -> bool {
        self.height == self.width
    }

    /// Determines whether every element of an [`UnsignedMatrix`] is zero. This includes every
    /// matrix with no elements.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]")
    ///         .unwrap()
    ///         .is_zero()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{3,0}")
    ///         .unwrap()
    ///         .is_zero()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[0, 0; 1, 0]")
    ///         .unwrap()
    ///         .is_zero()
    /// );
    /// ```
    #[inline]
    fn is_zero(&self) -> bool {
        self.elements.iter().all(|&x| x == T::ZERO)
    }

    /// Determines whether an [`UnsignedMatrix`] is empty, that is, whether it has no elements. A
    /// matrix is empty exactly when its height or its width is zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .is_empty()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{3,0}")
    ///         .unwrap()
    ///         .is_empty()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,3}")
    ///         .unwrap()
    ///         .is_empty()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]")
    ///         .unwrap()
    ///         .is_empty()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_is_empty` from `fmpz_mat.h`, FLINT 3.6.0.
    #[inline]
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Determines whether every element in row `i` of an [`UnsignedMatrix`] is zero. Every row of a
    /// matrix with no columns qualifies.
    ///
    /// Indices start at 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.width()`.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 2; 0, 0]")
    ///         .unwrap()
    ///         .is_zero_row(1)
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 2; 0, 0]")
    ///         .unwrap()
    ///         .is_zero_row(0)
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{3,0}")
    ///         .unwrap()
    ///         .is_zero_row(2)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_is_zero_row` from `fmpz_mat/is_zero.c`, FLINT 3.6.0, except
    /// that it panics if the row index is out of range.
    #[inline]
    fn is_zero_row(&self, i: u64) -> bool {
        self.row_iter(i).all(|&x| x == T::ZERO)
    }

    /// Determines whether rows `i` and `j` of an [`UnsignedMatrix`] are equal. A row always equals
    /// itself.
    ///
    /// Indices start at 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.width()`.
    ///
    /// # Panics
    /// Panics if `i` or `j` is greater than or equal to the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 7; 3, 4; 1, 7]")
    ///         .unwrap()
    ///         .equal_rows(0, 2)
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 7; 3, 4; 1, 7]")
    ///         .unwrap()
    ///         .equal_rows(1, 1)
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 7; 3, 4; 1, 7]")
    ///         .unwrap()
    ///         .equal_rows(0, 1)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_equal_row` from `fmpz_mat/equal.c`, FLINT 3.6.0, except that
    /// it panics if an index is out of range.
    #[inline]
    fn equal_rows(&self, i: u64, j: u64) -> bool {
        self.row_iter(i).eq(self.row_iter(j))
    }

    /// Determines whether columns `i` and `j` of an [`UnsignedMatrix`] are equal. A column always
    /// equals itself.
    ///
    /// Indices start at 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height()`.
    ///
    /// # Panics
    /// Panics if `i` or `j` is greater than or equal to the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 3, 1; 7, 4, 7]")
    ///         .unwrap()
    ///         .equal_columns(0, 2)
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 3, 1; 7, 4, 7]")
    ///         .unwrap()
    ///         .equal_columns(1, 1)
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 3, 1; 7, 4, 7]")
    ///         .unwrap()
    ///         .equal_columns(0, 1)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_equal_col` from `fmpz_mat/equal.c`, FLINT 3.6.0, except that
    /// it panics if an index is out of range.
    #[inline]
    fn equal_columns(&self, i: u64, j: u64) -> bool {
        self.column_iter(i).eq(self.column_iter(j))
    }

    /// Determines whether an [`UnsignedMatrix`] is an identity matrix: a square matrix whose
    /// diagonal elements are 1 and whose other elements are 0. This includes the 0-by-0 matrix.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 0; 0, 1]")
    ///         .unwrap()
    ///         .is_identity()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .is_identity()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 0; 1, 1]")
    ///         .unwrap()
    ///         .is_identity()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 0]")
    ///         .unwrap()
    ///         .is_identity()
    /// );
    /// ```
    fn is_identity(&self) -> bool {
        // The diagonal elements are `width + 1` apart, starting from the first.
        self.height == self.width
            && self.elements.iter().enumerate().all(|(k, x)| {
                if k.divisible_by(self.width + 1) {
                    *x == T::ONE
                } else {
                    *x == T::ZERO
                }
            })
    }

    /// Determines whether an [`UnsignedMatrix`] is a rectangular identity matrix: whether its
    /// element in row $i$ and column $j$ is 1 if $i = j$ and 0 otherwise.
    ///
    /// The matrix need not be square, and every matrix with no elements qualifies. These are the
    /// matrices that FLINT's `*_mat_is_one` functions accept. A matrix is an identity matrix
    /// exactly when it is square and a rectangular identity matrix.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 0, 0; 0, 1, 0]")
    ///         .unwrap()
    ///         .is_rectangular_identity()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 0; 0, 1; 0, 0]")
    ///         .unwrap()
    ///         .is_rectangular_identity()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,3}")
    ///         .unwrap()
    ///         .is_rectangular_identity()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 0; 0, 1; 0, 1]")
    ///         .unwrap()
    ///         .is_rectangular_identity()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 1]")
    ///         .unwrap()
    ///         .is_rectangular_identity()
    /// );
    /// ```
    #[inline]
    fn is_rectangular_identity(&self) -> bool {
        // The element at index `k` is in row `k / width` and column `k % width`. A matrix with no
        // columns has no elements, so the closure never divides by zero.
        self.elements.iter().enumerate().all(|(k, x)| {
            if k / self.width == k % self.width {
                *x == T::ONE
            } else {
                *x == T::ZERO
            }
        })
    }

    /// Determines whether an [`UnsignedMatrix`] is a standard basis matrix, returning the row and
    /// column of its 1 if it is.
    ///
    /// A standard basis matrix has a single element equal to 1 and every other element 0. Returns
    /// `None` for every other matrix, including the zero matrices and the matrices with no
    /// elements.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[0, 0, 0; 1, 0, 0]")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     Some((1, 0))
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[0, 2; 0, 0]")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     None
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 0; 0, 1]")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     None
    /// );
    /// ```
    fn standard_basis_index(&self) -> Option<(u64, u64)> {
        let k = self.elements.iter().position(|&x| x != T::ZERO)?;
        if self.elements[k] == T::ONE && self.elements[k + 1..].iter().all(|&x| x == T::ZERO) {
            Some((
                u64::exact_from(k / self.width),
                u64::exact_from(k % self.width),
            ))
        } else {
            None
        }
    }

    /// Determines whether an [`UnsignedMatrix`] is symmetric: square, and equal to its transpose.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 2; 2, 3]")
    ///         .unwrap()
    ///         .is_symmetric()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]")
    ///         .unwrap()
    ///         .is_symmetric()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 0, 0; 0, 1, 0]")
    ///         .unwrap()
    ///         .is_symmetric()
    /// );
    /// ```
    #[inline]
    fn is_symmetric(&self) -> bool {
        let n = self.width;
        self.height == n
            && (0..n)
                .all(|i| (i + 1..n).all(|j| self.elements[i * n + j] == self.elements[j * n + i]))
    }

    /// Determines whether an [`UnsignedMatrix`] is skew-symmetric: square, and equal to the
    /// negative of its transpose.
    ///
    /// No element of an [`UnsignedMatrix`] can be negative, so it is skew-symmetric only if it is a
    /// square zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]")
    ///         .unwrap()
    ///         .is_skew_symmetric()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[0, 2; 2, 0]")
    ///         .unwrap()
    ///         .is_skew_symmetric()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .is_skew_symmetric()
    /// );
    /// ```
    #[inline]
    fn is_skew_symmetric(&self) -> bool {
        // No element can be negative, so `x = -y` forces both to be zero.
        self.height == self.width && self.is_zero()
    }

    /// Determines whether an [`UnsignedMatrix`] is diagonal: every element off the diagonal is
    /// zero. The matrix need not be square.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 0; 0, 2]")
    ///         .unwrap()
    ///         .is_diagonal()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 0, 0; 0, 2, 0]")
    ///         .unwrap()
    ///         .is_diagonal()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 2; 0, 3]")
    ///         .unwrap()
    ///         .is_diagonal()
    /// );
    /// ```
    #[inline]
    fn is_diagonal(&self) -> bool {
        self.rows()
            .enumerate()
            .all(|(i, row)| row.enumerate().all(|(j, &x)| i == j || x == T::ZERO))
    }

    /// Determines whether an [`UnsignedMatrix`] is upper triangular: every element below the
    /// diagonal is zero. The matrix need not be square.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 2; 0, 3]")
    ///         .unwrap()
    ///         .is_upper_triangular()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 2, 3; 0, 4, 5]")
    ///         .unwrap()
    ///         .is_upper_triangular()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 0; 2, 3]")
    ///         .unwrap()
    ///         .is_upper_triangular()
    /// );
    /// ```
    #[inline]
    fn is_upper_triangular(&self) -> bool {
        self.rows()
            .enumerate()
            .all(|(i, row)| row.take(i).all(|&x| x == T::ZERO))
    }

    /// Determines whether an [`UnsignedMatrix`] is lower triangular: every element above the
    /// diagonal is zero. The matrix need not be square.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1, 0; 2, 3]")
    ///         .unwrap()
    ///         .is_lower_triangular()
    /// );
    /// assert!(
    ///     UnsignedMatrix::<u64>::from_str("[1; 2; 3]")
    ///         .unwrap()
    ///         .is_lower_triangular()
    /// );
    /// assert!(
    ///     !UnsignedMatrix::<u64>::from_str("[1, 2; 0, 3]")
    ///         .unwrap()
    ///         .is_lower_triangular()
    /// );
    /// ```
    #[inline]
    fn is_lower_triangular(&self) -> bool {
        self.rows()
            .enumerate()
            .all(|(i, row)| row.skip(i + 1).all(|&x| x == T::ZERO))
    }

    /// Sets every element of an [`UnsignedMatrix`] to zero, keeping its height and width.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_zero();
    /// assert_eq!(m.to_string(), "[0, 0, 0; 0, 0, 0]");
    /// ```
    #[inline]
    fn set_zero(&mut self) {
        self.elements.fill(T::ZERO);
    }

    /// Returns the transpose of an [`UnsignedMatrix`], whose rows are the columns of the original,
    /// taking the matrix by reference.
    ///
    /// $$
    /// f(A)_{ij} = A_{ji}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let t = m.transpose();
    /// assert_eq!(t.to_string(), "[1, 4; 2, 5; 3, 6]");
    /// ```
    fn transpose(&self) -> Self {
        let mut elements = Vec::with_capacity(self.elements.len());
        for j in 0..self.width {
            elements.extend(self.elements.iter().skip(j).step_by(self.width).copied());
        }
        Self {
            elements,
            height: self.width,
            width: self.height,
        }
    }

    /// Replaces an [`UnsignedMatrix`] with its transpose, whose rows are the columns of the
    /// original. A square matrix is transposed in place by swapping elements, and any other matrix
    /// by copying its elements into a new buffer.
    ///
    /// $$
    /// A \gets A^T.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.transpose_assign();
    /// assert_eq!(m.to_string(), "[1, 4; 2, 5; 3, 6]");
    /// ```
    fn transpose_assign(&mut self) {
        let (height, width) = (self.height, self.width);
        if height == width {
            for i in 0..height {
                for j in i + 1..width {
                    self.elements.swap(i * width + j, j * width + i);
                }
            }
        } else {
            self.elements = transposed_elements(&self.elements, height, width);
            swap(&mut self.height, &mut self.width);
        }
    }

    /// Swaps two rows of an [`UnsignedMatrix`]. Swapping a row with itself does nothing.
    ///
    /// Each row is a contiguous run of the elements, so the two runs are exchanged in place.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.width()`.
    ///
    /// # Panics
    /// Panics if `i` or `j` is greater than or equal to the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// a.swap_rows(0, 2);
    /// assert_eq!(a.to_string(), "[7, 8, 9; 4, 5, 6; 1, 2, 3]");
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3, 4; 5, 6, 7, 8]").unwrap();
    /// a.swap_rows(1, 1);
    /// assert_eq!(a.to_string(), "[1, 2, 3, 4; 5, 6, 7, 8]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_swap_rows` from `fmpz_mat/swap_rows.c`, FLINT 3.6.0, with a
    /// null permutation.
    fn swap_rows(&mut self, i: u64, j: u64) {
        let height = u64::exact_from(self.height);
        assert!(
            i < height && j < height,
            "row indices {i} and {j} must be less than the height of the matrix, {height}"
        );
        let (i, j) = (usize::exact_from(i), usize::exact_from(j));
        if i != j {
            let width = self.width;
            let (low, high) = (min(i, j), max(i, j));
            let (front, back) = self.elements.split_at_mut(high * width);
            front[low * width..(low + 1) * width].swap_with_slice(&mut back[..width]);
        }
    }

    /// Swaps two columns of an [`UnsignedMatrix`]. Swapping a column with itself does nothing.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height()`.
    ///
    /// # Panics
    /// Panics if `i` or `j` is greater than or equal to the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// a.swap_columns(0, 2);
    /// assert_eq!(a.to_string(), "[3, 2, 1; 6, 5, 4; 9, 8, 7]");
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3, 4; 5, 6, 7, 8]").unwrap();
    /// a.swap_columns(1, 3);
    /// assert_eq!(a.to_string(), "[1, 4, 3, 2; 5, 8, 7, 6]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_swap_cols` from `fmpz_mat/swap.c`, FLINT 3.6.0, with a null
    /// permutation.
    fn swap_columns(&mut self, i: u64, j: u64) {
        let width = u64::exact_from(self.width);
        assert!(
            i < width && j < width,
            "column indices {i} and {j} must be less than the width of the matrix, {width}"
        );
        let (i, j) = (usize::exact_from(i), usize::exact_from(j));
        if i != j {
            // The width is positive, since it exceeds `i`.
            for row in self.elements.chunks_exact_mut(self.width) {
                row.swap(i, j);
            }
        }
    }

    /// Reverses the order of the rows of an [`UnsignedMatrix`]: row $i$ of an $h$-row matrix
    /// becomes row $h - 1 - i$.
    ///
    /// Rows $i$ and $h - 1 - i$ are swapped for each $i < h/2$. FLINT calls this operation
    /// `invert_rows`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// a.reverse_rows();
    /// assert_eq!(a.to_string(), "[7, 8, 9; 4, 5, 6; 1, 2, 3]");
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3, 4; 5, 6, 7, 8]").unwrap();
    /// a.reverse_rows();
    /// assert_eq!(a.to_string(), "[5, 6, 7, 8; 1, 2, 3, 4]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_invert_rows` from `fmpz_mat/invert.c`, FLINT 3.6.0, with a
    /// null permutation.
    fn reverse_rows(&mut self) {
        let (height, width) = (self.height, self.width);
        for i in 0..height >> 1 {
            let (front, back) = self.elements.split_at_mut((height - 1 - i) * width);
            front[i * width..(i + 1) * width].swap_with_slice(&mut back[..width]);
        }
    }

    /// Reverses the order of the columns of an [`UnsignedMatrix`]: column $j$ of a $w$-column
    /// matrix becomes column $w - 1 - j$.
    ///
    /// Each row is reversed in place. FLINT calls this operation `invert_cols`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// a.reverse_columns();
    /// assert_eq!(a.to_string(), "[3, 2, 1; 6, 5, 4; 9, 8, 7]");
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[1, 2, 3, 4; 5, 6, 7, 8]").unwrap();
    /// a.reverse_columns();
    /// assert_eq!(a.to_string(), "[4, 3, 2, 1; 8, 7, 6, 5]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_invert_cols` from `fmpz_mat/invert.c`, FLINT 3.6.0, with a
    /// null permutation.
    fn reverse_columns(&mut self) {
        // A matrix with no columns has no elements, and nothing to reverse.
        if self.width != 0 {
            for row in self.elements.chunks_exact_mut(self.width) {
                row.reverse();
            }
        }
    }

    /// Returns the window of an [`UnsignedMatrix`] made of the given ranges of rows and columns, as
    /// a new [`UnsignedMatrix`], copying its elements.
    ///
    /// The element of the window in row $i$ and column $j$ is the element of the matrix in row
    /// `rows.start` $+ i$ and column `columns.start` $+ j$, so the window's height is the length of
    /// `rows` and its width is the length of `columns`. Either range may be empty.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// window.
    ///
    /// # Panics
    /// Panics if `rows.start > rows.end`, `rows.end > self.height()`, `columns.start >
    /// columns.end`, or `columns.end > self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(m.window(1..3, 0..2).to_string(), "[4, 5; 7, 8]");
    /// assert_eq!(m.window(0..2, 0..3).to_string(), "[1, 2, 3; 4, 5, 6]");
    /// assert_eq!(m.window(1..1, 0..3).to_string(), "[]_{0,3}");
    /// ```
    fn window(&self, rows: Range<u64>, columns: Range<u64>) -> Self {
        let (rows, columns) = window_ranges(self.height, self.width, &rows, &columns);
        let width = columns.len();
        let mut elements = Vec::with_capacity(rows.len() * width);
        for i in rows.clone() {
            let start = i * self.width + columns.start;
            elements.extend_from_slice(&self.elements[start..start + width]);
        }
        Self {
            elements,
            height: rows.len(),
            width,
        }
    }

    /// Returns the window of an [`UnsignedMatrix`] made of the given ranges of rows and columns, as
    /// a new [`UnsignedMatrix`], taking ownership of the matrix and reusing its storage.
    ///
    /// The result equals what [`window`](Self::window) returns. When the window has every column,
    /// its elements are contiguous, and the element [`Vec`] is truncated and its leading rows
    /// removed; in particular, a prefix of the rows only truncates it. Otherwise each row of the
    /// window is moved to its place in the front of the [`Vec`] before truncating it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `rows.start > rows.end`, `rows.end > self.height()`, `columns.start >
    /// columns.end`, or `columns.end > self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(
    ///     m.clone().into_window(1..3, 0..2).to_string(),
    ///     "[4, 5; 7, 8]"
    /// );
    /// assert_eq!(
    ///     m.clone().into_window(0..2, 0..3).to_string(),
    ///     "[1, 2, 3; 4, 5, 6]"
    /// );
    /// assert_eq!(m.into_window(1..1, 0..3).to_string(), "[]_{0,3}");
    /// ```
    fn into_window(mut self, rows: Range<u64>, columns: Range<u64>) -> Self {
        let (rows, columns) = window_ranges(self.height, self.width, &rows, &columns);
        let width = columns.len();
        if width == self.width {
            self.elements.truncate(rows.end * width);
            self.elements.drain(..rows.start * width);
        } else {
            let mut k = 0;
            for i in rows.clone() {
                let start = i * self.width + columns.start;
                self.elements.copy_within(start..start + width, k);
                k += width;
            }
            self.elements.truncate(k);
        }
        self.height = rows.len();
        self.width = width;
        self
    }

    /// Overwrites a window of an [`UnsignedMatrix`] with the elements of another
    /// [`UnsignedMatrix`], copying them.
    ///
    /// The window starts in row `i` and column `j` and has the shape of `w`: the element in row `i`
    /// $+ k$ and column `j` $+ l$ becomes the element of `w` in row $k$ and column $l$, and the
    /// other elements are unchanged. Writing back a window taken with [`window`](Self::window)
    /// restores the matrix. `w` may have no elements.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of `w`.
    ///
    /// # Panics
    /// Panics if `i + w.height() > self.height()` or `j + w.width() > self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// m.set_window(
    ///     1,
    ///     1,
    ///     &UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]").unwrap(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 0, 0; 7, 0, 0]");
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// m.set_window(
    ///     1,
    ///     0,
    ///     &UnsignedMatrix::<u64>::from_str("[10, 20, 30]").unwrap(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 2, 3; 10, 20, 30; 7, 8, 9]");
    /// ```
    fn set_window(&mut self, i: u64, j: u64, w: &Self) {
        let (rows, columns) = set_window_ranges(self.height, self.width, i, j, w.height, w.width);
        let width = columns.len();
        for (k, r) in rows.enumerate() {
            let start = r * self.width + columns.start;
            self.elements[start..start + width]
                .copy_from_slice(&w.elements[k * width..(k + 1) * width]);
        }
    }

    /// Overwrites a window of an [`UnsignedMatrix`] with the elements of another
    /// [`UnsignedMatrix`], taking ownership of the other matrix.
    ///
    /// The result equals what [`set_window`](Self::set_window) produces.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of `w`.
    ///
    /// # Panics
    /// Panics if `i + w.height() > self.height()` or `j + w.width() > self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// m.set_window_owned(
    ///     1,
    ///     1,
    ///     UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]").unwrap(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 0, 0; 7, 0, 0]");
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// m.set_window_owned(
    ///     1,
    ///     0,
    ///     UnsignedMatrix::<u64>::from_str("[10, 20, 30]").unwrap(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 2, 3; 10, 20, 30; 7, 8, 9]");
    /// ```
    fn set_window_owned(&mut self, i: u64, j: u64, w: Self) {
        self.set_window(i, j, &w);
    }

    /// Returns the vertical concatenation of two [`UnsignedMatrix`]s of the same width, copying
    /// their elements: the rows of `self` followed by the rows of `other`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// result.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different widths.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[5, 6]").unwrap();
    /// assert_eq!(a.concat_vertical(&b).to_string(), "[1, 2; 3, 4; 5, 6]");
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{0,2}").unwrap();
    /// assert_eq!(a.concat_vertical(&b).to_string(), "[1, 2]");
    /// let a = UnsignedMatrix::<u64>::from_str("[]_{2,0}").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{3,0}").unwrap();
    /// assert_eq!(a.concat_vertical(&b).to_string(), "[]_{5,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_concat_vertical` from `fmpz_mat/concat.c`, FLINT 3.6.0,
    /// except that it panics if the widths differ.
    fn concat_vertical(&self, other: &Self) -> Self {
        assert_eq!(
            self.width, other.width,
            "the widths of the matrices, {} and {}, must be equal",
            self.width, other.width
        );
        let mut elements = Vec::with_capacity(self.elements.len() + other.elements.len());
        elements.extend_from_slice(&self.elements);
        elements.extend_from_slice(&other.elements);
        Self {
            elements,
            height: self.height + other.height,
            width: self.width,
        }
    }

    /// Returns the vertical concatenation of two [`UnsignedMatrix`]s of the same width, taking
    /// ownership of both: the rows of `self` followed by the rows of `other`.
    ///
    /// The result equals what [`concat_vertical`](Self::concat_vertical) returns. The elements of
    /// `other` are moved onto the end of the element [`Vec`] of `self`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// result.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different widths.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[5, 6]").unwrap();
    /// assert_eq!(a.into_concat_vertical(b).to_string(), "[1, 2; 3, 4; 5, 6]");
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{0,2}").unwrap();
    /// assert_eq!(a.into_concat_vertical(b).to_string(), "[1, 2]");
    /// let a = UnsignedMatrix::<u64>::from_str("[]_{2,0}").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{3,0}").unwrap();
    /// assert_eq!(a.into_concat_vertical(b).to_string(), "[]_{5,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_concat_vertical` from `fmpz_mat/concat.c`, FLINT 3.6.0,
    /// except that it panics if the widths differ.
    fn into_concat_vertical(mut self, mut other: Self) -> Self {
        assert_eq!(
            self.width, other.width,
            "the widths of the matrices, {} and {}, must be equal",
            self.width, other.width
        );
        self.elements.append(&mut other.elements);
        self.height += other.height;
        self
    }

    /// Returns the horizontal concatenation of two [`UnsignedMatrix`]s of the same height, copying
    /// their elements: each row of the result is the corresponding row of `self` followed by the
    /// corresponding row of `other`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// result.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different heights.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[5; 6]").unwrap();
    /// assert_eq!(a.concat_horizontal(&b).to_string(), "[1, 2, 5; 3, 4, 6]");
    /// let a = UnsignedMatrix::<u64>::from_str("[1; 2]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(a.concat_horizontal(&b).to_string(), "[1; 2]");
    /// let a = UnsignedMatrix::<u64>::from_str("[]_{0,2}").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{0,3}").unwrap();
    /// assert_eq!(a.concat_horizontal(&b).to_string(), "[]_{0,5}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_concat_horizontal` from `fmpz_mat/concat.c`, FLINT 3.6.0,
    /// except that it panics if the heights differ.
    fn concat_horizontal(&self, other: &Self) -> Self {
        assert_eq!(
            self.height, other.height,
            "the heights of the matrices, {} and {}, must be equal",
            self.height, other.height
        );
        let width = self.width + other.width;
        let mut elements = Vec::with_capacity(self.height * width);
        for i in 0..self.height {
            elements.extend_from_slice(&self.elements[i * self.width..(i + 1) * self.width]);
            elements.extend_from_slice(&other.elements[i * other.width..(i + 1) * other.width]);
        }
        Self {
            elements,
            height: self.height,
            width,
        }
    }

    /// Returns the horizontal concatenation of two [`UnsignedMatrix`]s of the same height, taking
    /// ownership of both: each row of the result is the corresponding row of `self` followed by the
    /// corresponding row of `other`.
    ///
    /// The result equals what [`concat_horizontal`](Self::concat_horizontal) returns. The elements
    /// are moved, not copied, into a new element [`Vec`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// result.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different heights.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[5; 6]").unwrap();
    /// assert_eq!(
    ///     a.into_concat_horizontal(b).to_string(),
    ///     "[1, 2, 5; 3, 4, 6]"
    /// );
    /// let a = UnsignedMatrix::<u64>::from_str("[1; 2]").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{2,0}").unwrap();
    /// assert_eq!(a.into_concat_horizontal(b).to_string(), "[1; 2]");
    /// let a = UnsignedMatrix::<u64>::from_str("[]_{0,2}").unwrap();
    /// let b = UnsignedMatrix::<u64>::from_str("[]_{0,3}").unwrap();
    /// assert_eq!(a.into_concat_horizontal(b).to_string(), "[]_{0,5}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_concat_horizontal` from `fmpz_mat/concat.c`, FLINT 3.6.0,
    /// except that it panics if the heights differ.
    fn into_concat_horizontal(self, other: Self) -> Self {
        assert_eq!(
            self.height, other.height,
            "the heights of the matrices, {} and {}, must be equal",
            self.height, other.height
        );
        let width = self.width + other.width;
        let mut elements = Vec::with_capacity(self.height * width);
        let mut xs = self.elements.into_iter();
        let mut ys = other.elements.into_iter();
        for _ in 0..self.height {
            elements.extend((&mut xs).take(self.width));
            elements.extend((&mut ys).take(other.width));
        }
        Self {
            elements,
            height: self.height,
            width,
        }
    }
}

macro_rules! impl_named_unsigned_matrix {
    ($t:ident, $name:expr) => {
        impl Named for UnsignedMatrix<$t> {
            /// The name of this type, with its element type spelled out.
            const NAME: &'static str = $name;
        }
    };
}
impl_named_unsigned_matrix!(u8, "UnsignedMatrix<u8>");
impl_named_unsigned_matrix!(u16, "UnsignedMatrix<u16>");
impl_named_unsigned_matrix!(u32, "UnsignedMatrix<u32>");
impl_named_unsigned_matrix!(u64, "UnsignedMatrix<u64>");
impl_named_unsigned_matrix!(u128, "UnsignedMatrix<u128>");
impl_named_unsigned_matrix!(usize, "UnsignedMatrix<usize>");

/// `ShortlexUnsignedMatrix` is a wrapper around an [`UnsignedMatrix`], taking the
/// [`UnsignedMatrix`] by value.
///
/// [`UnsignedMatrix`] does not implement [`Ord`], because no order on matrices is canonical.
/// Wrapping an [`UnsignedMatrix`] in a `ShortlexUnsignedMatrix` provides one total order, a
/// shortlex order: matrices are compared first by their number of elements, then by height, then by
/// width, and then, in case of a tie, lexicographically, by their elements in row-major order. Its
/// equality agrees with [`UnsignedMatrix`] equality.
///
/// The number of elements plays the part that the dimension plays in the shortlex order of vectors:
/// every matrix with fewer elements comes before every matrix with more, so that $\[5\] < \[0,
/// 0\]$, and there are only finitely many shapes with a given number of elements. The width is
/// needed as well as the height only for the matrices with no elements, such as $[]_{0,2}$ and
/// $[]_{0,5}$, since otherwise the height and the number of elements determine it. Matrices with
/// one row compare as their rows do under the shortlex order of vectors. It is a well-order: any
/// nonempty set of matrices has a least element.
///
/// `ShortlexUnsignedMatrix` owns its value. This is useful in many cases, for example if you want
/// to use [`UnsignedMatrix`]s as keys in a [`BTreeMap`](alloc::collections::BTreeMap). In other
/// situations, it is better to use [`ShortlexUnsignedMatrixRef`], which only has a reference to its
/// value.
// Serialized as its inner `UnsignedMatrix`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexUnsignedMatrix<T: PrimitiveUnsigned>(pub UnsignedMatrix<T>);

/// `ShortlexUnsignedMatrixRef` is a wrapper around an [`UnsignedMatrix`], taking the
/// [`UnsignedMatrix`] by reference.
///
/// See the [`ShortlexUnsignedMatrix`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortlexUnsignedMatrixRef<'a, T: PrimitiveUnsigned>(pub &'a UnsignedMatrix<T>);

impl<T: PrimitiveUnsigned> ShortlexUnsignedMatrix<T> {
    /// Borrows a [`ShortlexUnsignedMatrix`] as a [`ShortlexUnsignedMatrixRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::{
    ///     ShortlexUnsignedMatrix, ShortlexUnsignedMatrixRef, UnsignedMatrix,
    /// };
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// let x = ShortlexUnsignedMatrix(m.clone());
    /// assert_eq!(x.as_ref(), ShortlexUnsignedMatrixRef(&m));
    /// ```
    pub const fn as_ref(&self) -> ShortlexUnsignedMatrixRef<'_, T> {
        ShortlexUnsignedMatrixRef(&self.0)
    }
}

impl<T: PrimitiveUnsigned> Deref for ShortlexUnsignedMatrix<T> {
    type Target = UnsignedMatrix<T>;

    /// Allows a [`ShortlexUnsignedMatrix`] to dereference to an [`UnsignedMatrix`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::{ShortlexUnsignedMatrix, UnsignedMatrix};
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// let x = ShortlexUnsignedMatrix(m.clone());
    /// assert_eq!(*x, m);
    /// ```
    fn deref(&self) -> &UnsignedMatrix<T> {
        &self.0
    }
}

impl<T: PrimitiveUnsigned> Deref for ShortlexUnsignedMatrixRef<'_, T> {
    type Target = UnsignedMatrix<T>;

    /// Allows a [`ShortlexUnsignedMatrixRef`] to dereference to an [`UnsignedMatrix`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::{ShortlexUnsignedMatrixRef, UnsignedMatrix};
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// let x = ShortlexUnsignedMatrixRef(&m);
    /// assert_eq!(*x, m);
    /// ```
    fn deref(&self) -> &UnsignedMatrix<T> {
        self.0
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
/// A function for getting a column of an [`UnsignedMatrix`] as an [`UnsignedVector`].
pub mod column;
/// Iterators over the elements in a column of an [`UnsignedMatrix`].
pub mod column_iter;
/// Iterators over the columns of an [`UnsignedMatrix`].
pub mod columns;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`ShortlexUnsignedMatrix`] and
/// [`ShortlexUnsignedMatrixRef`], comparing two matrices by number of elements, then height, then
/// width, and then lexicographically.
pub mod comparison;
/// Conversions to and from [`UnsignedMatrix`].
pub mod conversion;
/// Iterators that generate [`UnsignedMatrix`]s without repetition.
pub mod exhaustive;
/// Iterators that generate [`UnsignedMatrix`]s randomly.
#[cfg(feature = "random")]
pub mod random;
/// A function for getting a row of an [`UnsignedMatrix`] as an [`UnsignedVector`].
pub mod row;
/// Iterators over the elements in a row of an [`UnsignedMatrix`].
pub mod row_iter;
/// Iterators over the rows of an [`UnsignedMatrix`].
pub mod rows;
/// Functions for replacing a column of an [`UnsignedMatrix`] with the elements of an
/// [`UnsignedVector`].
pub mod set_column;
/// Functions for replacing a row of an [`UnsignedMatrix`] with the elements of an
/// [`UnsignedVector`].
pub mod set_row;
