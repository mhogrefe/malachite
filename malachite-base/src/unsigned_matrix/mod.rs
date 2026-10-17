// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::DivisibleBy;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};
use core::mem::swap;

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
/// A function for getting a column of an [`UnsignedMatrix`] as an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector).
pub mod column;
/// Iterators over the elements in a column of an [`UnsignedMatrix`].
pub mod column_iter;
/// Iterators over the columns of an [`UnsignedMatrix`].
pub mod columns;
/// Conversions to and from [`UnsignedMatrix`].
pub mod conversion;
/// Iterators that generate [`UnsignedMatrix`]s without repetition.
pub mod exhaustive;
/// Iterators that generate [`UnsignedMatrix`]s randomly.
#[cfg(feature = "random")]
pub mod random;
/// A function for getting a row of an [`UnsignedMatrix`] as an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector).
pub mod row;
/// Iterators over the elements in a row of an [`UnsignedMatrix`].
pub mod row_iter;
/// Iterators over the rows of an [`UnsignedMatrix`].
pub mod rows;
/// Functions for replacing a column of an [`UnsignedMatrix`] with the elements of an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector).
pub mod set_column;
/// Functions for replacing a row of an [`UnsignedMatrix`] with the elements of an
/// [`UnsignedVector`](crate::unsigned_vector::UnsignedVector).
pub mod set_row;
