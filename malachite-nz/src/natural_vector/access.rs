// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use core::ops::{Index, IndexMut};

impl Index<usize> for NaturalVector {
    type Output = Natural;

    /// Gets a reference to the element of a [`NaturalVector`] at an index.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v[0], 1);
    /// assert_eq!(v[2], 3);
    /// ```
    #[inline]
    fn index(&self, i: usize) -> &Natural {
        &self.elements[i]
    }
}

impl IndexMut<usize> for NaturalVector {
    /// Gets a mutable reference to the element of a [`NaturalVector`] at an index.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v[1] = Natural::from(10u32);
    /// v[2] += Natural::from(4u32);
    /// assert_eq!(v.to_string(), "(1, 10, 7)");
    /// ```
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Natural {
        &mut self.elements[i]
    }
}
