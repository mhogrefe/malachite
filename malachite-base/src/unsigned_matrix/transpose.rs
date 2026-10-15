// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::{UnsignedMatrix, transposed_elements};
use alloc::vec::Vec;
use core::mem::swap;

impl<T: PrimitiveUnsigned> UnsignedMatrix<T> {
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let t = m.transpose();
    /// assert_eq!(t.to_string(), "[1, 4; 2, 5; 3, 6]");
    /// ```
    pub fn transpose(&self) -> Self {
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// m.transpose_assign();
    /// assert_eq!(m.to_string(), "[1, 4; 2, 5; 3, 6]");
    /// ```
    pub fn transpose_assign(&mut self) {
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
