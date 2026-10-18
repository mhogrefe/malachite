// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};
use core::mem::{swap, take};
use core::ops::{Deref, Range};
use malachite_base::matrix::Matrix;
use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

/// A matrix whose elements are [`Natural`]s.
///
/// The elements are stored in one [`Vec`], `elements`, in row-major order: the element in row $i$
/// and column $j$, both counted from 0, is at index $iw + j$, where $w$ is the width. This is the
/// layout of [`IntegerMatrix`](crate::integer_matrix::IntegerMatrix).
///
/// `elements` is public, but its length must stay equal to the height times the width; a matrix
/// whose `elements` has been resized is invalid, and functions on it may panic or return wrong
/// results. The dimensions are private, so that they can only change together with the elements.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeNaturalMatrix", into = "SerdeNaturalMatrix")
)]
pub struct NaturalMatrix {
    pub elements: Vec<Natural>,
    height: usize,
    width: usize,
}

// The serialized form of a `NaturalMatrix`: its dimensions and its elements in row-major order. A
// matrix whose number of elements is not the product of its dimensions is rejected.
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeNaturalMatrix {
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) elements: Vec<Natural>,
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

impl NaturalMatrix {
    // Returns true iff `self` is valid.
    //
    // To be valid, the number of elements must be the height times the width. All `NaturalMatrix`s
    // must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.height.checked_mul(self.width) == Some(self.elements.len())
    }

    /// Creates a [`NaturalMatrix`] from a slice of its rows, each an [`NaturalVector`]. The height
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = NaturalMatrix::from_rows(&[
    ///     NaturalVector::from_str("(1, 2, 3)").unwrap(),
    ///     NaturalVector::from_str("(4, 5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    /// ```
    pub fn from_rows(rows: &[NaturalVector]) -> Self {
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

    /// Creates a [`NaturalMatrix`] from a [`Vec`] of its rows, each an [`NaturalVector`], moving
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = NaturalMatrix::from_owned_rows(vec![
    ///     NaturalVector::from_str("(1, 2, 3)").unwrap(),
    ///     NaturalVector::from_str("(4, 5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    /// ```
    pub fn from_owned_rows(rows: Vec<NaturalVector>) -> Self {
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

    /// Creates a [`NaturalMatrix`] from a slice of its columns, each an [`NaturalVector`]. The
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = NaturalMatrix::from_columns(&[
    ///     NaturalVector::from_str("(1, 2)").unwrap(),
    ///     NaturalVector::from_str("(3, 4)").unwrap(),
    ///     NaturalVector::from_str("(5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_columns(columns: &[NaturalVector]) -> Self {
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

    /// Creates a [`NaturalMatrix`] from a [`Vec`] of its columns, each an [`NaturalVector`], moving
    /// the elements rather than copying them. The width is the number of columns and the height is
    /// their common dimension; with no columns, the matrix is 0-by-0.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let m = NaturalMatrix::from_owned_columns(vec![
    ///     NaturalVector::from_str("(1, 2)").unwrap(),
    ///     NaturalVector::from_str("(3, 4)").unwrap(),
    ///     NaturalVector::from_str("(5, 6)").unwrap(),
    /// ]);
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    pub fn from_owned_columns(columns: Vec<NaturalVector>) -> Self {
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

impl Matrix for NaturalMatrix {
    type Element = Natural;

    /// Creates a [`NaturalMatrix`] with the given height and width from a slice of its elements in
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
    /// Panics if `xs.len()` is not `height * width`, or if the number of elements does not fit in a
    /// [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_elements(2, 3, &[1u32, 2, 3, 4, 5, 6].map(Natural::from));
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// // A matrix with no rows can still have columns.
    /// assert_eq!(
    ///     NaturalMatrix::from_elements(0, 4, &[]).to_string(),
    ///     "[]_{0,4}"
    /// );
    /// ```
    #[inline]
    fn from_elements(height: u64, width: u64, xs: &[Natural]) -> Self {
        Self::from_owned_elements(height, width, xs.to_vec())
    }

    /// Creates a [`NaturalMatrix`] with the given height and width from a [`Vec`] of its elements
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_owned_elements(
    ///     3,
    ///     2,
    ///     [1u32, 2, 3, 4, 5, 6].map(Natural::from).to_vec(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 2; 3, 4; 5, 6]");
    /// ```
    fn from_owned_elements(height: u64, width: u64, xs: Vec<Natural>) -> Self {
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

    /// Creates a [`NaturalMatrix`] with the given height and width from a slice of its elements in
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
    /// Panics if `xs.len()` is not `height * width`, or if the number of elements does not fit in a
    /// [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_elements_column_major(
    ///     2,
    ///     3,
    ///     &[1u32, 2, 3, 4, 5, 6].map(Natural::from),
    /// );
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    fn from_elements_column_major(height: u64, width: u64, xs: &[Natural]) -> Self {
        let (height, width, count) = shape(height, width);
        assert_eq!(
            xs.len(),
            count,
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

    /// Creates a [`NaturalMatrix`] with the given height and width from a [`Vec`] of its elements
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
    /// Panics if `xs.len()` is not `height * width`, or if the number of elements does not fit in a
    /// [`usize`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_owned_elements_column_major(
    ///     2,
    ///     3,
    ///     [1u32, 2, 3, 4, 5, 6].map(Natural::from).to_vec(),
    /// );
    /// assert_eq!(m.to_string(), "[1, 3, 5; 2, 4, 6]");
    /// ```
    fn from_owned_elements_column_major(height: u64, width: u64, xs: Vec<Natural>) -> Self {
        let (height, width, count) = shape(height, width);
        assert_eq!(
            xs.len(),
            count,
            "the number of elements must be the height times the width"
        );
        Self {
            elements: transposed_elements(xs, width, height),
            height,
            width,
        }
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(NaturalMatrix::zero(2, 3).to_string(), "[0, 0, 0; 0, 0, 0]");
    /// assert_eq!(NaturalMatrix::zero(0, 3).to_string(), "[]_{0,3}");
    /// ```
    fn zero(height: u64, width: u64) -> Self {
        let (height, width, count) = shape(height, width);
        Self {
            elements: vec![Natural::ZERO; count],
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::identity(3).to_string(),
    ///     "[1, 0, 0; 0, 1, 0; 0, 0, 1]"
    /// );
    /// assert_eq!(NaturalMatrix::identity(0).to_string(), "[]_{0,0}");
    /// ```
    fn identity(size: u64) -> Self {
        let mut m = Self::zero(size, size);
        // The diagonal elements are `size + 1` apart.
        for x in m.elements.iter_mut().step_by(m.width + 1) {
            *x = Natural::ONE;
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::standard_basis_matrix(2, 3, 1, 0).to_string(),
    ///     "[0, 0, 0; 1, 0, 0]"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::standard_basis_matrix(1, 1, 0, 0).to_string(),
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
        m.elements[position] = Natural::ONE;
        m
    }

    /// Returns the elements of a [`NaturalMatrix`] in row-major order as a [`Vec`], cloning them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.to_elements(), &[1u32, 2, 3, 4]);
    /// ```
    #[inline]
    fn to_elements(&self) -> Vec<Natural> {
        self.elements.clone()
    }

    /// Returns the elements of a [`NaturalMatrix`] in row-major order as a [`Vec`], taking
    /// ownership of the matrix. Nothing is copied.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.into_elements(), &[1u32, 2, 3, 4]);
    /// ```
    #[inline]
    fn into_elements(self) -> Vec<Natural> {
        self.elements
    }

    /// Returns a reference to the elements of a [`NaturalMatrix`] in row-major order, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.elements_ref(), &[1u32, 2, 3, 4]);
    /// ```
    #[inline]
    fn elements_ref(&self) -> &[Natural] {
        &self.elements
    }

    /// Returns the number of rows of a [`NaturalMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .height(),
    ///     2
    /// );
    /// assert_eq!(NaturalMatrix::from_str("[]_{0,4}").unwrap().height(), 0);
    /// ```
    #[inline]
    fn height(&self) -> u64 {
        u64::exact_from(self.height)
    }

    /// Returns the number of columns of a [`NaturalMatrix`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .width(),
    ///     3
    /// );
    /// assert_eq!(NaturalMatrix::from_str("[]_{0,4}").unwrap().width(), 4);
    /// ```
    #[inline]
    fn width(&self) -> u64 {
        u64::exact_from(self.width)
    }

    /// Determines whether a [`NaturalMatrix`] is square, that is, whether its height equals its
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap().is_square());
    /// assert!(
    ///     !NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .is_square()
    /// );
    /// assert!(NaturalMatrix::from_str("[]_{0,0}").unwrap().is_square());
    /// assert!(!NaturalMatrix::from_str("[]_{3,0}").unwrap().is_square());
    /// ```
    #[inline]
    fn is_square(&self) -> bool {
        self.height == self.width
    }

    /// Determines whether every element of a [`NaturalMatrix`] is zero. This includes every matrix
    /// with no elements.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(NaturalMatrix::from_str("[0, 0; 0, 0]").unwrap().is_zero());
    /// assert!(NaturalMatrix::from_str("[]_{3,0}").unwrap().is_zero());
    /// assert!(!NaturalMatrix::from_str("[0, 0; 1, 0]").unwrap().is_zero());
    /// ```
    #[inline]
    fn is_zero(&self) -> bool {
        self.elements.iter().all(|x| *x == 0u32)
    }

    /// Determines whether a [`NaturalMatrix`] is an identity matrix: a square matrix whose diagonal
    /// elements are 1 and whose other elements are 0. This includes the 0-by-0 matrix.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(
    ///     NaturalMatrix::from_str("[1, 0; 0, 1]")
    ///         .unwrap()
    ///         .is_identity()
    /// );
    /// assert!(NaturalMatrix::from_str("[]_{0,0}").unwrap().is_identity());
    /// assert!(
    ///     !NaturalMatrix::from_str("[1, 0; 1, 1]")
    ///         .unwrap()
    ///         .is_identity()
    /// );
    /// assert!(!NaturalMatrix::from_str("[1, 0]").unwrap().is_identity());
    /// ```
    fn is_identity(&self) -> bool {
        // The diagonal elements are `width + 1` apart, starting from the first.
        self.height == self.width
            && self.elements.iter().enumerate().all(|(k, x)| {
                if k.divisible_by(self.width + 1) {
                    *x == 1u32
                } else {
                    *x == 0u32
                }
            })
    }

    /// Determines whether a [`NaturalMatrix`] is a standard basis matrix, returning the row and
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[0, 0, 0; 1, 0, 0]")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     Some((1, 0))
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[0, 2; 0, 0]")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     None
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[1, 0; 0, 1]")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     None
    /// );
    /// ```
    fn standard_basis_index(&self) -> Option<(u64, u64)> {
        let k = self.elements.iter().position(|x| *x != 0u32)?;
        if self.elements[k] == 1u32 && self.elements[k + 1..].iter().all(|x| *x == 0u32) {
            Some((
                u64::exact_from(k / self.width),
                u64::exact_from(k % self.width),
            ))
        } else {
            None
        }
    }

    /// Determines whether a [`NaturalMatrix`] is symmetric: square, and equal to its transpose.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(
    ///     NaturalMatrix::from_str("[1, 2; 2, 3]")
    ///         .unwrap()
    ///         .is_symmetric()
    /// );
    /// assert!(
    ///     !NaturalMatrix::from_str("[1, 2; 3, 4]")
    ///         .unwrap()
    ///         .is_symmetric()
    /// );
    /// assert!(
    ///     !NaturalMatrix::from_str("[1, 0, 0; 0, 1, 0]")
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

    /// Determines whether a [`NaturalMatrix`] is skew-symmetric: square, and equal to the negative
    /// of its transpose.
    ///
    /// No element of a [`NaturalMatrix`] can be negative, so it is skew-symmetric only if it is a
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(
    ///     NaturalMatrix::from_str("[0, 0; 0, 0]")
    ///         .unwrap()
    ///         .is_skew_symmetric()
    /// );
    /// assert!(
    ///     !NaturalMatrix::from_str("[0, 2; 2, 0]")
    ///         .unwrap()
    ///         .is_skew_symmetric()
    /// );
    /// assert!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .is_skew_symmetric()
    /// );
    /// ```
    #[inline]
    fn is_skew_symmetric(&self) -> bool {
        // No element can be negative, so `x = -y` forces both to be zero.
        self.height == self.width && self.is_zero()
    }

    /// Determines whether a [`NaturalMatrix`] is diagonal: every element off the diagonal is zero.
    /// The matrix need not be square.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(
    ///     NaturalMatrix::from_str("[1, 0; 0, 2]")
    ///         .unwrap()
    ///         .is_diagonal()
    /// );
    /// assert!(
    ///     NaturalMatrix::from_str("[1, 0, 0; 0, 2, 0]")
    ///         .unwrap()
    ///         .is_diagonal()
    /// );
    /// assert!(
    ///     !NaturalMatrix::from_str("[1, 2; 0, 3]")
    ///         .unwrap()
    ///         .is_diagonal()
    /// );
    /// ```
    #[inline]
    fn is_diagonal(&self) -> bool {
        self.rows()
            .enumerate()
            .all(|(i, row)| row.enumerate().all(|(j, x)| i == j || *x == 0u32))
    }

    /// Determines whether a [`NaturalMatrix`] is upper triangular: every element below the diagonal
    /// is zero. The matrix need not be square.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(
    ///     NaturalMatrix::from_str("[1, 2; 0, 3]")
    ///         .unwrap()
    ///         .is_upper_triangular()
    /// );
    /// assert!(
    ///     NaturalMatrix::from_str("[1, 2, 3; 0, 4, 5]")
    ///         .unwrap()
    ///         .is_upper_triangular()
    /// );
    /// assert!(
    ///     !NaturalMatrix::from_str("[1, 0; 2, 3]")
    ///         .unwrap()
    ///         .is_upper_triangular()
    /// );
    /// ```
    #[inline]
    fn is_upper_triangular(&self) -> bool {
        self.rows()
            .enumerate()
            .all(|(i, row)| row.take(i).all(|x| *x == 0u32))
    }

    /// Determines whether a [`NaturalMatrix`] is lower triangular: every element above the diagonal
    /// is zero. The matrix need not be square.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert!(
    ///     NaturalMatrix::from_str("[1, 0; 2, 3]")
    ///         .unwrap()
    ///         .is_lower_triangular()
    /// );
    /// assert!(
    ///     NaturalMatrix::from_str("[1; 2; 3]")
    ///         .unwrap()
    ///         .is_lower_triangular()
    /// );
    /// assert!(
    ///     !NaturalMatrix::from_str("[1, 2; 0, 3]")
    ///         .unwrap()
    ///         .is_lower_triangular()
    /// );
    /// ```
    #[inline]
    fn is_lower_triangular(&self) -> bool {
        self.rows()
            .enumerate()
            .all(|(i, row)| row.skip(i + 1).all(|x| *x == 0u32))
    }

    /// Sets every element of a [`NaturalMatrix`] to zero, keeping its height and width.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.set_zero();
    /// assert_eq!(m.to_string(), "[0, 0, 0; 0, 0, 0]");
    /// ```
    #[inline]
    fn set_zero(&mut self) {
        self.elements.fill(Natural::ZERO);
    }

    /// Returns the transpose of a [`NaturalMatrix`], whose rows are the columns of the original,
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let t = m.transpose();
    /// assert_eq!(t.to_string(), "[1, 4; 2, 5; 3, 6]");
    /// ```
    fn transpose(&self) -> Self {
        let mut elements = Vec::with_capacity(self.elements.len());
        for j in 0..self.width {
            elements.extend(self.elements.iter().skip(j).step_by(self.width).cloned());
        }
        Self {
            elements,
            height: self.width,
            width: self.height,
        }
    }

    /// Replaces a [`NaturalMatrix`] with its transpose, whose rows are the columns of the original.
    /// The elements are moved, not copied: a square matrix is transposed in place by swapping
    /// elements, and any other matrix by moving each element once.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
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
            self.elements = transposed_elements(take(&mut self.elements), height, width);
            swap(&mut self.height, &mut self.width);
        }
    }

    /// Returns the window of a [`NaturalMatrix`] made of the given ranges of rows and columns, as a
    /// new [`NaturalMatrix`], cloning its elements.
    ///
    /// The element of the window in row $i$ and column $j$ is the element of the matrix in row
    /// `rows.start` $+ i$ and column `columns.start` $+ j$, so the window's height is the length of
    /// `rows` and its width is the length of `columns`. Either range may be empty.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the window,
    /// and $m$ is the maximum bit length of its elements.
    ///
    /// # Panics
    /// Panics if `rows.start > rows.end`, `rows.end > self.height()`, `columns.start >
    /// columns.end`, or `columns.end > self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
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

    /// Returns the window of a [`NaturalMatrix`] made of the given ranges of rows and columns, as a
    /// new [`NaturalMatrix`], taking ownership of the matrix and reusing its storage.
    ///
    /// The result equals what [`window`](Self::window) returns, but no element is cloned. When the
    /// window has every column, its elements are contiguous, and the element [`Vec`] is truncated
    /// and its leading rows removed; in particular, a prefix of the rows only truncates it.
    /// Otherwise each element of the window is swapped into its place in the front of the [`Vec`]
    /// before truncating it.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
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
                for j in start..start + width {
                    self.elements.swap(k, j);
                    k += 1;
                }
            }
            self.elements.truncate(k);
        }
        self.height = rows.len();
        self.width = width;
        self
    }
}

// Implements `Named` for `NaturalMatrix`.
impl_named!(NaturalMatrix);

/// `ShortlexNaturalMatrix` is a wrapper around a [`NaturalMatrix`], taking the [`NaturalMatrix`] by
/// value.
///
/// [`NaturalMatrix`] does not implement [`Ord`], because no order on matrices is canonical.
/// Wrapping a [`NaturalMatrix`] in a `ShortlexNaturalMatrix` provides one total order, a shortlex
/// order: matrices are compared first by their number of elements, then by height, then by width,
/// and then, in case of a tie, lexicographically, by their elements in row-major order. Its
/// equality agrees with [`NaturalMatrix`] equality.
///
/// The number of elements plays the part that the dimension plays in the shortlex order of vectors:
/// every matrix with fewer elements comes before every matrix with more, so that $\[5\] < \[0,
/// 0\]$, and there are only finitely many shapes with a given number of elements. The width is
/// needed as well as the height only for the matrices with no elements, such as $[]_{0,2}$ and
/// $[]_{0,5}$, since otherwise the height and the number of elements determine it. Matrices with
/// one row compare as their rows do under the shortlex order of vectors. It is a well-order: any
/// nonempty set of matrices has a least element.
///
/// `ShortlexNaturalMatrix` owns its value. This is useful in many cases, for example if you want to
/// use [`NaturalMatrix`]s as keys in a [`BTreeMap`](alloc::collections::BTreeMap). In other
/// situations, it is better to use [`ShortlexNaturalMatrixRef`], which only has a reference to its
/// value.
// Serialized as its inner `NaturalMatrix`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexNaturalMatrix(pub NaturalMatrix);

/// `ShortlexNaturalMatrixRef` is a wrapper around a [`NaturalMatrix`], taking the [`NaturalMatrix`]
/// by reference.
///
/// See the [`ShortlexNaturalMatrix`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortlexNaturalMatrixRef<'a>(pub &'a NaturalMatrix);

impl ShortlexNaturalMatrix {
    /// Borrows a [`ShortlexNaturalMatrix`] as a [`ShortlexNaturalMatrixRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::{
    ///     NaturalMatrix, ShortlexNaturalMatrix, ShortlexNaturalMatrixRef,
    /// };
    ///
    /// let m = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let x = ShortlexNaturalMatrix(m.clone());
    /// assert_eq!(x.as_ref(), ShortlexNaturalMatrixRef(&m));
    /// ```
    pub const fn as_ref(&self) -> ShortlexNaturalMatrixRef<'_> {
        ShortlexNaturalMatrixRef(&self.0)
    }
}

impl Deref for ShortlexNaturalMatrix {
    type Target = NaturalMatrix;

    /// Allows a [`ShortlexNaturalMatrix`] to dereference to a [`NaturalMatrix`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::{NaturalMatrix, ShortlexNaturalMatrix};
    ///
    /// let m = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let x = ShortlexNaturalMatrix(m.clone());
    /// assert_eq!(*x, m);
    /// ```
    fn deref(&self) -> &NaturalMatrix {
        &self.0
    }
}

impl Deref for ShortlexNaturalMatrixRef<'_> {
    type Target = NaturalMatrix;

    /// Allows a [`ShortlexNaturalMatrixRef`] to dereference to a [`NaturalMatrix`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::{NaturalMatrix, ShortlexNaturalMatrixRef};
    ///
    /// let m = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let x = ShortlexNaturalMatrixRef(&m);
    /// assert_eq!(*x, m);
    /// ```
    fn deref(&self) -> &NaturalMatrix {
        self.0
    }
}

// Moves the elements of a `height`-by-`width` matrix, given in row-major order, into the row-major
// order of its transpose.
pub(crate) fn transposed_elements(
    mut xs: Vec<Natural>,
    height: usize,
    width: usize,
) -> Vec<Natural> {
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
    m: &NaturalMatrix,
    f: &mut Formatter,
    element_separator: &str,
    row_separator: &str,
    write: impl Fn(&Natural, &mut Formatter) -> Result,
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
fn common_dimension(vectors: &[NaturalVector], what: &str) -> usize {
    let dimension = vectors.first().map_or(0, |v| v.elements.len());
    assert!(
        vectors.iter().all(|v| v.elements.len() == dimension),
        "the {what} must all have the same dimension"
    );
    dimension
}

/// Indexing a [`NaturalMatrix`] by row and column.
pub mod access;
/// Arithmetic on [`NaturalMatrix`]s.
pub mod arithmetic;
/// A function for getting a column of a [`NaturalMatrix`] as a [`NaturalVector`].
pub mod column;
/// Iterators over the elements in a column of a [`NaturalMatrix`].
pub mod column_iter;
/// Iterators over the columns of a [`NaturalMatrix`].
pub mod columns;
/// Comparisons of [`NaturalMatrix`]s with other matrices.
pub mod comparison;
/// Conversions to and from [`NaturalMatrix`].
pub mod conversion;
/// Iterators that generate [`NaturalMatrix`]s without repetition.
pub mod exhaustive;
/// Iterators that generate [`NaturalMatrix`]s randomly.
#[cfg(feature = "random")]
pub mod random;
/// A function for getting a row of a [`NaturalMatrix`] as a [`NaturalVector`].
pub mod row;
/// Iterators over the elements in a row of a [`NaturalMatrix`].
pub mod row_iter;
/// Iterators over the rows of a [`NaturalMatrix`].
pub mod rows;
/// Functions for replacing a column of a [`NaturalMatrix`] with the elements of a
/// [`NaturalVector`].
pub mod set_column;
/// Functions for replacing a row of a [`NaturalMatrix`] with the elements of a [`NaturalVector`].
pub mod set_row;
