// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use core::ops::{Index, IndexMut};

impl IntegerMatrix {
    // The position in `elements` of the element in row `i` and column `j`, or `None` if either
    // index is out of range.
    #[inline]
    const fn element_index(&self, i: usize, j: usize) -> Option<usize> {
        if i < self.height && j < self.width {
            Some(i * self.width + j)
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

    /// Gets a reference to the element of an [`IntegerMatrix`] in row `i` and column `j`, or
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let m = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(m.get(0, 2).unwrap(), &3);
    /// assert_eq!(m.get(1, 0).unwrap(), &4);
    /// assert_eq!(m.get(2, 0), None);
    /// assert_eq!(m.get(0, 3), None);
    /// ```
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> Option<&Integer> {
        self.element_index(i, j).map(|k| &self.elements[k])
    }

    /// Gets a mutable reference to the element of an [`IntegerMatrix`] in row `i` and column `j`,
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut m = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// *m.get_mut(1, 2).unwrap() = Integer::from(10u32);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 10]");
    /// assert!(m.get_mut(0, 3).is_none());
    /// ```
    #[inline]
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Integer> {
        self.element_index(i, j).map(|k| &mut self.elements[k])
    }
}

impl Index<(usize, usize)> for IntegerMatrix {
    type Output = Integer;

    /// Gets a reference to the element of an [`IntegerMatrix`] in row `i` and column `j`.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let m = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(m[(0, 0)], 1);
    /// assert_eq!(m[(0, 2)], 3);
    /// assert_eq!(m[(1, 1)], 5);
    /// ```
    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &Integer {
        &self.elements[self.element_index_or_panic(i, j)]
    }
}

impl IndexMut<(usize, usize)> for IntegerMatrix {
    /// Gets a mutable reference to the element of an [`IntegerMatrix`] in row `i` and column `j`.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut m = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m[(0, 1)] = Integer::from(10u32);
    /// m[(1, 2)] += Integer::from(4u32);
    /// assert_eq!(m.to_string(), "[1, 10, 3; 4, 5, 10]");
    /// ```
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Integer {
        let k = self.element_index_or_panic(i, j);
        &mut self.elements[k]
    }
}
