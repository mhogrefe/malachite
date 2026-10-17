// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use core::ops::{Index, IndexMut};

impl<T: PrimitiveUnsigned> UnsignedMatrix<T> {
    // The position in `elements` of the element in row `i` and column `j`, or `None` if either
    // index is out of range.
    fn element_index(&self, i: u64, j: u64) -> Option<usize> {
        if i < self.height() && j < self.width() {
            Some(usize::exact_from(i) * self.width + usize::exact_from(j))
        } else {
            None
        }
    }

    // The position in `elements` of the element in row `i` and column `j`, panicking if either
    // index is out of range.
    fn element_index_or_panic(&self, i: usize, j: usize) -> usize {
        assert!(
            i < self.height,
            "row index {i} is out of range for a matrix with {} rows",
            self.height
        );
        assert!(
            j < self.width,
            "column index {j} is out of range for a matrix with {} columns",
            self.width
        );
        i * self.width + j
    }

    /// Gets a reference to the element of an [`UnsignedMatrix`] in row `i` and column `j`, or
    /// [`None`] if either index is out of range.
    ///
    /// Indices start at 0. Unlike indexing `elements` directly, this checks each index against its
    /// own dimension, so a column index past the width never reaches into the next row.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(m.get(0, 2).unwrap(), &3);
    /// assert_eq!(m.get(1, 0).unwrap(), &4);
    /// assert_eq!(m.get(2, 0), None);
    /// assert_eq!(m.get(0, 3), None);
    /// ```
    #[inline]
    pub fn get(&self, i: u64, j: u64) -> Option<&T> {
        self.element_index(i, j).map(|k| &self.elements[k])
    }

    /// Gets a mutable reference to the element of an [`UnsignedMatrix`] in row `i` and column `j`,
    /// or [`None`] if either index is out of range.
    ///
    /// Indices start at 0. Unlike indexing `elements` directly, this checks each index against its
    /// own dimension, so a column index past the width never reaches into the next row.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// *m.get_mut(1, 2).unwrap() = 10;
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 10]");
    /// assert!(m.get_mut(0, 3).is_none());
    /// ```
    #[inline]
    pub fn get_mut(&mut self, i: u64, j: u64) -> Option<&mut T> {
        self.element_index(i, j).map(|k| &mut self.elements[k])
    }
}

impl<T: PrimitiveUnsigned> Index<(usize, usize)> for UnsignedMatrix<T> {
    type Output = T;

    /// Gets a reference to the element of an [`UnsignedMatrix`] in row `i` and column `j`.
    ///
    /// Indices start at 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix, or if `j` is greater
    /// than or equal to its width.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(m[(0, 0)], 1);
    /// assert_eq!(m[(0, 2)], 3);
    /// assert_eq!(m[(1, 1)], 5);
    /// ```
    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &T {
        &self.elements[self.element_index_or_panic(i, j)]
    }
}

impl<T: PrimitiveUnsigned> IndexMut<(usize, usize)> for UnsignedMatrix<T> {
    /// Gets a mutable reference to the element of an [`UnsignedMatrix`] in row `i` and column `j`.
    ///
    /// Indices start at 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the height of the matrix, or if `j` is greater
    /// than or equal to its width.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m[(0, 1)] = 10;
    /// m[(1, 2)] += 4;
    /// assert_eq!(m.to_string(), "[1, 10, 3; 4, 5, 10]");
    /// ```
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        let k = self.element_index_or_panic(i, j);
        &mut self.elements[k]
    }
}
