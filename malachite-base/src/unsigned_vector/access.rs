// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::ops::{Index, IndexMut};

impl<T: PrimitiveUnsigned> Index<usize> for UnsignedVector<T> {
    type Output = T;

    /// Gets a reference to the element of an [`UnsignedVector`] at an index.
    ///
    /// Indices start at 0, as they do for a [`Vec`](alloc::vec::Vec).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the dimension of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v[0], 1);
    /// assert_eq!(v[2], 3);
    /// ```
    #[inline]
    fn index(&self, i: usize) -> &T {
        &self.elements[i]
    }
}

impl<T: PrimitiveUnsigned> IndexMut<usize> for UnsignedVector<T> {
    /// Gets a mutable reference to the element of an [`UnsignedVector`] at an index.
    ///
    /// Indices start at 0, as they do for a [`Vec`](alloc::vec::Vec).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the dimension of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// v[1] = 10;
    /// v[2] += 4;
    /// assert_eq!(v.to_string(), "(1, 10, 7)");
    /// ```
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.elements[i]
    }
}
